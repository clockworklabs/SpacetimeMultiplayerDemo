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

    [SerializeField] private string moduleAddress = "bitcraftmini";
    [SerializeField] private string hostName = "spacetimedb.com/spacetimedb";
    [SerializeField] private bool sslEnabled = true;    

    [SerializeField] private NetworkPlayer playerPrefab;
    [SerializeField] private GameObject preSpawnCamera;
    [SerializeField, Tooltip("The rate at which we are sending frequent updates on the client (in messages per second)")]
    private float clientSendRate;
    [SerializeField] private float spawnAreaRadius;
    [SerializeField] private NpcData[] npcPrefabs;    

    readonly Dictionary<ulong, NetworkPlayer> players = new Dictionary<ulong, NetworkPlayer>();
    readonly Dictionary<ulong, Npc> npcs = new Dictionary<ulong, Npc>();
    readonly Dictionary<ulong, ResourceComponent> resources = new Dictionary<ulong, ResourceComponent>();
    readonly Dictionary<ulong, GameResource> resourcesModels = new Dictionary<ulong, GameResource>();

    private float? lastMessageSendTick;
    public event Action messageSendTick;
    public static Action<ulong> OnResourceUpdated;

    public static GameObject FeatureRoot;
    
    protected void Start()
    {
        FeatureRoot = new GameObject("Features");

        Application.targetFrameRate = 60;

        NetworkManager.instance.onConnect += () =>
        {
            Debug.Log("Connected.");

            NetworkManager.instance.Subscribe(new List<string>()
            {
                "Config", "PlayerLoginComponent", "TransformComponent", "AnimationComponent", "ActiveTradeComponent",
                "TradeSessionComponent", "ChunkData", "NpcComponent", "Chunk",
                "ResourceComponent", "ServerGlobals", "InventoryComponent", "PlayerComponent",
                "PlayerChatMessage"
            });
        };
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

        NetworkManager.instance.Connect(hostName, moduleAddress, sslEnabled);
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
        // If we don't have any data for our player, then we are creating a new one.
        var player = PlayerComponent.FilterByOwnerId(NetworkPlayer.identity.Value.Bytes);
        if (!NetworkPlayer.localPlayerId.HasValue || player == null)
        {
            // Show username selection
            UIUsernameChooser.instance.Show();
        }
        NetworkManager.instance.onTransactionComplete -= CheckNewPlayer;
    }

    public void CreatePlayer(string username)
    {
        var spawnPosition = Random.insideUnitSphere * spawnAreaRadius;
        spawnPosition.y = 0.0f;

        Debug.Log("Sending request for new player.");
        Reducer.CreateNewPlayer(spawnPosition.ToStdb(), Quaternion.identity.ToStdb(), username);
    }

    public void LocalPlayerCreated()
    {
        preSpawnCamera.SetActive(false);
    }
    
    public ResourceComponent GetResourceComponent(ulong entityId)
    {
        if (resources.TryGetValue(entityId, out var res))
        {
            return res;
        }
        return null;
    }

    public GameResource GetResourceModel(ulong entityId)
    {
        if (resourcesModels.TryGetValue(entityId, out var res))
        {
            return res;
        }
        return null;
    }

    public void AssignResourceModel(ulong entityId, GameResource res)
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
