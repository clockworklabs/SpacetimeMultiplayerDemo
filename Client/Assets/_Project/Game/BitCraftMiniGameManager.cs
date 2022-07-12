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
    [SerializeField] private StdbNetworkManager networkManager;
    [SerializeField] private GameObject spawnPosition;

    readonly Dictionary<uint, NetworkPlayer> players = new Dictionary<uint, NetworkPlayer>();

    protected void Start()
    {
        networkManager.onConnect += () =>
        {
            try
            {
                Debug.Log("Sending request for new player.");
                var ourId = (uint)(Random.value * uint.MaxValue);
                NetworkPlayer._localPlayerId = ourId;
                Reducer.CreateNewPlayer(ourId, spawnPosition.transform.position.ToStdb());
            }
            catch (Exception e)
            {
                Debug.LogError($"Exception: {e}");
            }
        };
        
        networkManager.onDisconnect += () =>
        {

        };
        
        networkManager.onRowUpdate += (tableId, row) =>
        {
            switch (row.Op)
            {
                case TableRowOperation.Types.OperationType.Insert:
                    if (tableId == 1)
                    {
                        // Player deserialized from row update
                        Player player = new Player();
                        
                        // check to see if this player already exists
                        if (players.TryGetValue(player.playerId, out var networkPlayer))
                        {
                            networkPlayer.transform.position = player.position.ToVector3();
                        }
                        else
                        {
                            // Create a new player
                            var newNetworkPlayer = Instantiate(playerPrefab);
                            newNetworkPlayer.Spawn(player.playerId);
                        }
                    }
                    
                    break;
            }
        };
        
        networkManager.Connect();
    }
}
