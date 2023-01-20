using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using System.Linq;
using Event = ClientApi.Event;
using Random = UnityEngine.Random;
using System;

public partial class BitCraftMiniGameManager : Singleton<BitCraftMiniGameManager>
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

        NetworkManager.instance.onIdentityReceived += (identity) => {
            NetworkPlayer.identity = identity;
        };

        NetworkManager.instance.onTransactionComplete += CheckNewPlayer;

        // register row update and reducer event handlers
        RegisterHandlers();

        NetworkManager.instance.Connect("spacetimedb.com/spacetimedb", "bitcraftmini");
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
        // TODO: This should be handled via on-demand subscription (eg client doesn't receive any subscription by default
        // until explicitely requesting some.)
        // If we don't have any data for our player, then we are creating a new one.
        var player = PlayerComponent.FilterByOwnerId(NetworkPlayer.identity.Value);
        if (!NetworkPlayer.localPlayerId.HasValue || player == null)
        {
            // Show username selection
            UIUsernameChooser.instance.Show();
        }
        NetworkManager.instance.onTransactionComplete -= CheckNewPlayer;
    }

    public void CreatePlayer(string username)
    {
        var newPlayerId = (uint)(Random.value * uint.MaxValue);
        var spawnPosition = Random.insideUnitSphere * spawnAreaRadius;
        spawnPosition.y = 0.0f;

        Debug.Log("Sending request for new player.");
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
