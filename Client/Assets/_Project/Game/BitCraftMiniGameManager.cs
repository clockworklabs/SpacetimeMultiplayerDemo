using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using Event = ClientApi.Event;
using Random = UnityEngine.Random;

public class BitCraftMiniGameManager : Singleton<BitCraftMiniGameManager>
{
    [SerializeField] private NetworkPlayer playerPrefab;
    [SerializeField] private GameObject preSpawnCamera;

    [SerializeField] private float spawnAreaRadius;

    readonly Dictionary<uint, NetworkPlayer> players = new Dictionary<uint, NetworkPlayer>();

    protected void Start()
    {
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

    void OnTableUpdate(string tableName, StdbNetworkManager.TableOp op, TypeValue? oldVAlue, TypeValue? newValue)
    {
        switch (op)
        {
            case StdbNetworkManager.TableOp.Insert:
                switch (tableName)
                {
                    case "PlayerComponent":
                        if (newValue.HasValue)
                        {
                            var player = PlayerComponent.From(newValue.Value);

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
                        if (newValue.HasValue)
                        {
                            var entityTransform = TransformComponent.From(newValue.Value);
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
                    case "PlayerAnimationComponent":
                        if (newValue.HasValue)
                        {
                            var playerAnimation = PlayerAnimationComponent.From(newValue.Value);
                            // check to see if this player already exists
                            if (players.TryGetValue(playerAnimation.entityId, out var networkPlayer))
                            {
                                // Is this our player?
                                if (networkPlayer.IsLocal())
                                {
                                    // Ignore local updates
                                }
                                else
                                {
                                    networkPlayer.GetComponent<PlayerMovementController>()
                                        .SetMoving(playerAnimation.moving);
                                }
                            }
                        }

                        break;
                    case "InventoryComponent":
                        if (newValue.HasValue)
                        {
                            var entityInventory = InventoryComponent.From(newValue.Value);
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
                        if (newValue.HasValue)
                        {
                            var loginState = PlayerLoginComponent.From(newValue.Value);
                            // check to see if this player already exists
                            if (players.TryGetValue(loginState.entityId, out var networkPlayer))
                            {
                                networkPlayer.LoginStateChanged();
                            }
                        }

                        break;
                    case "PlayerChatMessage":
                        if (newValue.HasValue)
                        {
                            var chatMessage = PlayerChatMessage.From(newValue.Value);
                            UIChatController.instance.OnChatMessageReceived(chatMessage.playerId, chatMessage.message);
                        }

                        break;
                    case "Chunk":
                        if (newValue.HasValue)
                        {
                            var chunk = Chunk.From(newValue.Value);
                            TerrainController.instance.AddChunk(chunk);
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
