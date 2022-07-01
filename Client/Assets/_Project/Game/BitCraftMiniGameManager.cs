using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using Random = UnityEngine.Random;

public class BitCraftMiniGameManager : Singleton<BitCraftMiniGameManager>
{
    [SerializeField] private NetworkPlayer playerPrefab;
    [SerializeField] private StdbNetworkManager networkManager;
    [SerializeField] private GameObject spawnPosition;

    protected void Start()
    {
        networkManager.onConnect += () =>
        {
            try
            {
                Debug.Log("Sending request for new player.");
                var ourId = (uint)(Random.value * uint.MaxValue);
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
        
        networkManager.Connect();
    }
}
