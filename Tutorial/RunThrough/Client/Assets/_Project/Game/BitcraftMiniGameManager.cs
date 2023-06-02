using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using SpacetimeDB;

public class BitcraftMiniGameManager : MonoBehaviour
{
    public static BitcraftMiniGameManager instance;

    public GameObject PlayerPrefab;

    [SerializeField] private GameObject preSpawnCamera;

    [SerializeField] private string moduleAddress = "bitcraftmini";
    [SerializeField] private string hostName = "spacetimedb.com/spacetimedb";
    [SerializeField] private bool sslEnabled = true;

    private Identity local_identity;

    // Start is called before the first frame update
    void Start()
    {
        instance = this;

        NetworkManager.instance.onConnect += () =>
        {
            Debug.Log("Connected.");

            NetworkManager.instance.Subscribe(new List<string>()
            {
                "SELECT * FROM Config",
                "SELECT * FROM SpawnableEntityComponent",
                "SELECT * FROM PlayerComponent",
                "SELECT * FROM MobileEntityComponent",
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
            local_identity = identity;
        };

        NetworkManager.instance.onTransactionComplete += CheckNewPlayer;

        PlayerComponent.OnInsert += PlayerComponent_OnInsert;

        NetworkManager.instance.Connect(hostName, moduleAddress, sslEnabled);
    }

    private void PlayerComponent_OnInsert(PlayerComponent obj)  
    {
        if(Identity.From(obj.OwnerId) == local_identity)
        {
            MobileEntityComponent mobPos = MobileEntityComponent.FilterByEntityId(obj.EntityId);
            LocalPlayer.instance.transform.position = new Vector3(mobPos.Location.X, 0.0f, mobPos.Location.Z);   
            StartGame();
        }
        else
        {
            var remotePlayer = Instantiate(PlayerPrefab);
            remotePlayer.AddComponent<RemotePlayer>().EntityId = obj.EntityId;            
        }
    }

    public void StartGame()
    {
        UIChatController.instance.enabled = true;
        preSpawnCamera.SetActive(false);

        UIChatController.instance.OnChatMessageReceived("Message of the Day: " + Config.FilterByVersion(0).MessageOfTheDay);
    }

    void CheckNewPlayer()
    {
        // If we don't have any data for our player, then we are creating a new one.
        var player = PlayerComponent.FilterByOwnerId(local_identity.Bytes);
        if (player == null)
        {
            // Show username selection
            UIUsernameChooser.instance.Show();
        }
        NetworkManager.instance.onTransactionComplete -= CheckNewPlayer;
    }
}
