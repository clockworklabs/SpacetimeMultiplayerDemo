using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using Websocket;
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
        
        StdbNetworkManager.instance.onConnect += () =>
        {
            try
            {
                Debug.Log("Sending request for new player.");
                var ourId = (uint)(Random.value * uint.MaxValue);
                NetworkPlayer._localPlayerId = ourId;
                Reducer.CreateNewPlayer(ourId);
            }
            catch (Exception e)
            {
                Debug.LogError($"Exception: {e}");
            }
        };

        StdbNetworkManager.instance.onDisconnect += () => { };

        StdbNetworkManager.clientDB.tableUpdated += (index, op, value, newValue) =>
        {
            switch (op)
            {
                case StdbClientCache.TableOp.Insert:
                    switch (index)
                    {
                        case 1:
                            if (newValue.HasValue)
                            {
                                var player = Player.From(newValue.Value);
                        
                                // check to see if this player already exists
                                if (!players.TryGetValue(player.entityId, out _))
                                {
                                    // Create a new player
                                    var newNetworkPlayer = Instantiate(playerPrefab);
                                    // spawn position
                                    var spawnPosition = Random.insideUnitSphere * spawnAreaRadius;
                                    spawnPosition.y = 0.0f;
                                    newNetworkPlayer.Spawn(player.entityId, spawnPosition);
                                    players[player.entityId] = newNetworkPlayer;
                                }
                            }
                            break;
                        case 2:
                            if (newValue.HasValue)
                            {
                                var entityTransform = EntityTransform.From(newValue.Value);
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
                                        networkPlayer.transform.position = entityTransform.ToVector3();
                                        networkPlayer.transform.rotation = entityTransform.ToQuaternion();
                                    }
                                }
                            }
                            break;
                        case 3:
                            if (newValue.HasValue)
                            {
                                var entityTransform = PlayerAnimation.From(newValue.Value);
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
                                        networkPlayer.GetComponent<PlayerMovementController>().SetMoving(entityTransform.moving);
                                    }
                                }
                            }
                            break;
                        case 4:
                            if (newValue.HasValue)
                            {
                                var entityInventory = EntityInventory.From(newValue.Value);
                                // check to see if this player already exists
                                if (players.TryGetValue(entityInventory.entityId, out var networkPlayer))
                                {
                                    // Is this our player?
                                    if (networkPlayer.IsLocal())
                                    {
                                        networkPlayer.GetComponent<PlayerInventoryController>().InventoryUpdate(entityInventory);
                                    }
                                }
                            }
                            break;
                    }
                    break;
            }
        };

        StdbNetworkManager.instance.Connect();
    }

    public void LocalPlayerCreated()
    {
        preSpawnCamera.SetActive(false);
    }
}