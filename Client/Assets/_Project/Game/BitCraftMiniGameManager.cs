using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using System.Linq;
using Event = ClientApi.Event;
using Random = UnityEngine.Random;

public class BitCraftMiniGameManager : Singleton<BitCraftMiniGameManager>
{
    [SerializeField] private NetworkPlayer playerPrefab;
    [SerializeField] private GameObject preSpawnCamera;

    [SerializeField] private float spawnAreaRadius;

    readonly Dictionary<uint, NetworkPlayer> players = new Dictionary<uint, NetworkPlayer>();
    readonly Dictionary<uint, Npc> npcs = new Dictionary<uint, Npc>();
    readonly Dictionary<uint, ResourceComponent> resources = new Dictionary<uint, ResourceComponent>();

    public static GameObject FeatureRoot;

    public ResourceComponent GetResource(uint entityId)
    {
        if (resources.TryGetValue(entityId, out var res))
        {
            return res;
        }
        return null;
    }

    public static System.Action<uint> OnResourceUpdated;

    [System.Serializable]
    private class NpcData
	{
        public string npcType;
        public Npc prefab;
	}
    [SerializeField] private NpcData[] npcPrefabs;

    protected void Start()
    {
        FeatureRoot = new GameObject("Features");

        Application.targetFrameRate = 60;

        StdbNetworkManager.instance.onConnect += () => { Debug.Log("Connected."); };

        StdbNetworkManager.instance.onDisconnect += () => { };

        StdbNetworkManager.instance.tableUpdate += OnTableUpdate;
        StdbNetworkManager.instance.onEvent += OnEvent;

        StdbNetworkManager.instance.onRowUpdateComplete += () =>
        {
            // If we don't have any data for our player, then we are creating a new one.
            var player = PlayerComponent.FilterByOwnerId(NetworkPlayer.identity.Value);
            if (!NetworkPlayer.localPlayerId.HasValue || player == null)
            {
                // Show username selection
                UIUsernameChooser.instance.Show();
            }
        };

        StdbNetworkManager.instance.Connect();
    }

    void OnTableUpdate(string tableName, StdbNetworkManager.TableOp op, object oldValue, object newValue)
    {
        switch (op)
        {
            case StdbNetworkManager.TableOp.Insert:
            case StdbNetworkManager.TableOp.Update:
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
                                    networkPlayer.transform.position = entityTransform.pos.ToVector3();
                                    networkPlayer.transform.rotation = entityTransform.rot.ToQuaternion();
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
                                    // ToDo: Animation Component is probably overkill. Movement and Action can trigger the animations.
                                    networkPlayer.GetComponent<PlayerMovementController>().SetMoving(animation.moving);
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
                                    networkPlayer.GetComponent<PlayerInventoryController>()
                                        .InventoryUpdate(entityInventory);
                                }
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
                }

                break;
            case StdbNetworkManager.TableOp.Delete:
                switch (tableName)
                {
                    case "NpcComponent":
                        if (oldValue != null)
                        {
                            var npc = (NpcComponent)newValue;

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
                            var resource = (ResourceComponent)newValue;
                            resources.Remove(resource.entityId);
                            OnResourceUpdated?.Invoke(resource.entityId);
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
}
