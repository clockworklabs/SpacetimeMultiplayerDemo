using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using System.Linq;
using Event = ClientApi.Event;
using Random = UnityEngine.Random;
using System;

public class BitCraftMiniGameManager : Singleton<BitCraftMiniGameManager>
{
    [System.Serializable]
    private class NpcData
    {
        public string npcType;
        public Npc prefab;
    }
    
    [SerializeField] private NetworkPlayer playerPrefab;
    [SerializeField] private GameObject preSpawnCamera;
    [SerializeField, Tooltip("The rate at which we are sending frequent updates on the client (in messages per second)")]
    private float clientSendRate;
    [SerializeField] private float spawnAreaRadius;
    [SerializeField] private NpcData[] npcPrefabs;

    readonly Dictionary<uint, NetworkPlayer> players = new Dictionary<uint, NetworkPlayer>();
    readonly Dictionary<uint, Npc> npcs = new Dictionary<uint, Npc>();
    readonly Dictionary<uint, ResourceComponent> resources = new Dictionary<uint, ResourceComponent>();
    readonly Dictionary<uint, GameResource> resourcesModels = new Dictionary<uint, GameResource>();

    private float? lastMessageSendTick;
    public event Action messageSendTick;
    public static Action<uint> OnResourceUpdated;

    public static GameObject FeatureRoot;
    
    protected void Start()
    {
        FeatureRoot = new GameObject("Features");

        Application.targetFrameRate = 60;

        NetworkManager.instance.onConnect += () => { Debug.Log("Connected."); };
        NetworkManager.instance.onConnectError += a =>
        {
            Debug.LogError($"Connection error: " + (a.HasValue ? a.Value.ToString() : "Null"));
        };
        NetworkManager.instance.onDisconnect += (closeStatus, error) =>
        {
            Debug.Log("Disconnected.");
        };

        NetworkManager.instance.onRowUpdate += OnRowUpdate;
        NetworkManager.instance.onEvent += OnEvent;

        NetworkManager.instance.onIdentityReceived += (identity) => {
            NetworkPlayer.identity = identity;
        };

        NetworkManager.instance.onTransactionComplete += CheckNewPlayer;

        NetworkManager.instance.Connect("spacetime.spacetimedb.net:3000", "bitcraftmini");
	}

    void Update() {
        if (!lastMessageSendTick.HasValue)
        {
            lastMessageSendTick = Time.time;
        }
        else
        {
            if (clientSendRate > 0 && Time.time - lastMessageSendTick > 1.0f / clientSendRate)
            {
                lastMessageSendTick = Time.time;
                messageSendTick?.Invoke();
            }
        }
    }

	void CheckNewPlayer()
	{
        var count = NetworkManager.clientDB.GetEntries("Chunk").Count();
        // skip random table updates that come before the world status update containing the chunks
        // TODO: This should be handled via on-demand subscription (eg client doesn't receive any subscription by default
        // until explicitely requesting some.)
        if (count > 0)
        {
            // If we don't have any data for our player, then we are creating a new one.
            var player = PlayerComponent.FilterByOwnerId(NetworkPlayer.identity.Value);
            if (!NetworkPlayer.localPlayerId.HasValue || player == null)
            {
                // Show username selection
                UIUsernameChooser.instance.Show();
            }
            NetworkManager.instance.onTransactionComplete -= CheckNewPlayer;
        };
    }

    void OnRowUpdate(string tableName, NetworkManager.TableOp op, object oldValue, object newValue)
    {
        switch (op)
        {
            case NetworkManager.TableOp.Insert:
            case NetworkManager.TableOp.Update:
                switch (tableName)
                {
                    case "NpcComponent":
                        if (newValue != null)
                        {
                            var npc = (NpcComponent)newValue;

                            // check to see if this player already exists
                            if (!npcs.TryGetValue(npc.entityId, out _))
                            {
                                // Create a new npc
                                var prefabData = npcPrefabs.FirstOrDefault(npcData => npcData.npcType == npc.model);
                                if (prefabData != null)
                                {
                                    var newNpc = Instantiate(prefabData.prefab);
                                    newNpc.Spawn(npc.entityId);
                                    npcs[npc.entityId] = newNpc;
                                }
                                else
                                {
                                    Debug.LogError($"Did not find npc prefab for {npc.model}");
                                }
                            }
                        }
                        break;

                    case "PlayerComponent":
                        if (newValue != null)
                        {
                            var player = (PlayerComponent)newValue;

                            // check to see if this player already exists
                            if (!players.TryGetValue(player.entityId, out _))
                            {
                                // Create a new player
                                var newNetworkPlayer = Instantiate(playerPrefab);

                                // Do we own this player?
                                if (NetworkPlayer.identity.HasValue &&
                                    player.ownerId.Equals(NetworkPlayer.identity.Value))
                                {
                                    if (NetworkPlayer.localPlayerId.HasValue)
                                    {
                                        Debug.LogWarning("This identity has more than one player!");
                                        return;
                                    }

                                    Debug.Log($"Attaching to player with id: {player.entityId}");
                                    NetworkPlayer.localPlayerId = player.entityId;
                                }

                                newNetworkPlayer.Spawn(player.entityId);
                                players[player.entityId] = newNetworkPlayer;
                            }
                        }

                        break;
                    case "TransformComponent":
                        if (newValue != null)
                        {
                            var entityTransform = (TransformComponent)newValue;
                            // check to see if this player already exists
                            if (players.TryGetValue(entityTransform.entityId, out var networkPlayer))
                            {
                                // Is this our player?
                                if (networkPlayer.IsLocal())
                                {
                                    // Ignore local updates
                                }
                                else
                                {
                                    networkPlayer.SetTargetTransform(
                                        entityTransform.pos.ToVector3(),
                                        entityTransform.rot.ToQuaternion()
                                    );
                                }
                            }
                        }

                        break;
                    case "AnimationComponent":
                        if (newValue != null)
                        {
                            var animation = (AnimationComponent)newValue;
                            // check to see if this player already exists
                            if (players.TryGetValue(animation.entityId, out var networkPlayer))
                            {
                                // Is this our player?
                                if (networkPlayer.IsLocal())
                                {
                                    // Ignore local updates
                                }
                                else
                                {
                                    networkPlayer.GetComponent<PlayerMovementController>().SetMoving(animation.moving);
                                    networkPlayer.GetComponentInChildren<PlayerAnimator>(true).SetRemoteAction(animation.actionTargetEntityId);
                                }
                            }
                        }

                        break;
                    case "InventoryComponent":
                        if (newValue != null)
                        {
                            var entityInventory = (InventoryComponent)newValue;
                            // check to see if this player already exists
                            if (players.TryGetValue(entityInventory.entityId, out var networkPlayer))
                            {
                                // Is this our player?
                                if (networkPlayer.IsLocal())
                                {
                                    PlayerInventoryController.Local.InventoryUpdate(entityInventory);
                                }
                            } 
                            else // attempt to update the trade session inventories
							{
                                TradeSessionController.Local.InventoryUpdate(entityInventory);
							}
                        }

                        break;
                    case "PlayerLoginComponent":
                        if (newValue != null)
                        {
                            var loginState = (PlayerLoginComponent)newValue;
                            // check to see if this player already exists
                            if (players.TryGetValue(loginState.entityId, out var networkPlayer))
                            {
                                networkPlayer.LoginStateChanged();
                            }
                        }

                        break;
                    case "PlayerChatMessage":
                        if (newValue != null)
                        {
                            var chatMessage = (PlayerChatMessage)newValue;
                            UIChatController.instance.OnChatMessageReceived(chatMessage.playerId, chatMessage.message);
                        }

                        break;
                    case "Chunk":
                        if (newValue != null)
                        {
                            var chunk = (Chunk)newValue;
                            TerrainController.instance.AddChunk(chunk);
                        }
                        break;
                    case "ResourceComponent":
                        if (newValue != null)
                        {
                            var resource = (ResourceComponent)newValue;
                            resources[resource.entityId] = resource;
                            OnResourceUpdated?.Invoke(resource.entityId);
                        }
                        break;

                    case "TradeSessionComponent":
                        if (newValue != null)
                        {
                            var session = (TradeSessionComponent)newValue;
                            if (NetworkPlayer.localPlayerId.HasValue)
                            {
                                var localId = NetworkPlayer.localPlayerId.Value;
                                if (session.acceptorEntityId == localId || session.initiatorEntityId == localId)
                                {
                                    var local = session.acceptorEntityId == localId ? session.acceptorOfferInventoryEntityId : session.initiatorOfferInventoryEntityId;
                                    var remote = session.acceptorEntityId == localId ? session.initiatorOfferInventoryEntityId : session.acceptorOfferInventoryEntityId;

                                    if (op == NetworkManager.TableOp.Insert)
                                    {
                                        TradeSessionController.Local.Initiate(session.entityId, local, remote);
                                    }
                                    else
                                    {
                                        TradeSessionController.Local.UpdateSession(session.entityId);
                                    }
                                }
                            }
                        }
                        break;
                }

                break;
            case NetworkManager.TableOp.Delete:
                switch (tableName)
                {
                    case "NpcComponent":
                        if (oldValue != null)
                        {
                            var npc = (NpcComponent)oldValue;

                            // check to see if this player already exists
                            if (npcs.TryGetValue(npc.entityId, out var npcModel))
                            {
                                Destroy(npcModel.gameObject);
                                npcs.Remove(npc.entityId);
                            }
                        }
                        break;
                    case "ResourceComponent":
                        if (oldValue != null)
                        {
                            var resource = (ResourceComponent)oldValue;
                            resources.Remove(resource.entityId);
                            OnResourceUpdated?.Invoke(resource.entityId);
                        }
                        break;

                    case "TradeSessionComponent":
                        if (oldValue != null)
                        {
                            var session = (TradeSessionComponent)oldValue;
                            var localId = NetworkPlayer.localPlayerId.Value;
                            if (session.acceptorEntityId == localId || session.initiatorEntityId == localId)
                            {
                                TradeSessionController.Local.Terminate(session.approvedByInitiator && session.approvedByAcceptor);
                            }
                        }
                        break;
                }
                break;
        }
    }

    // TODO: Unjankify this later
    private string _createUserUsername;

    private void OnEvent(Event dbEvent)
    {
        if (dbEvent.FunctionCall.Reducer == "create_new_player")
        {
            var jsonString = dbEvent.FunctionCall.ArgBytes.ToStringUtf8();
            var args = Newtonsoft.Json.JsonConvert.DeserializeObject<object[]>(jsonString);
            if ((string)args[3] == _createUserUsername)
            {
                switch (dbEvent.Status)
                {
                    case Event.Types.Status.Committed:
                        Debug.Log($"Create player success: {_createUserUsername}");
                        break;
                    case Event.Types.Status.Failed:
                        UIUsernameChooser.instance.ShowError($"Failed to create user with username: {_createUserUsername}");
                        break;
                }
            }
        }
    }

    public void CreatePlayer(string username)
    {
        var newPlayerId = (uint)(Random.value * uint.MaxValue);
        var spawnPosition = Random.insideUnitSphere * spawnAreaRadius;
        spawnPosition.y = 0.0f;

        Debug.Log("Sending request for new player.");
        _createUserUsername = username;
        Reducer.CreateNewPlayer(newPlayerId, spawnPosition.ToStdb(), Quaternion.identity.ToStdb(), username);
    }

    public void LocalPlayerCreated()
    {
        preSpawnCamera.SetActive(false);
    }
    
    public ResourceComponent GetResourceComponent(uint entityId)
    {
        if (resources.TryGetValue(entityId, out var res))
        {
            return res;
        }
        return null;
    }

    public GameResource GetResourceModel(uint entityId)
    {
        if (resourcesModels.TryGetValue(entityId, out var res))
        {
            return res;
        }
        return null;
    }

    public void AssignResourceModel(uint entityId, GameResource res)
    {
        if (res == null)
        {
            resourcesModels.Remove(entityId);
        }
        else
        {
            resourcesModels[entityId] = res;
        }
    }
}
