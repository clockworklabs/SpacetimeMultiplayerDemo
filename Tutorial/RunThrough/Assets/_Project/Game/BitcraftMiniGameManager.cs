using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using SpacetimeDB;
using System.Linq;

public class BitcraftMiniGameManager : MonoBehaviour
{
    public static BitcraftMiniGameManager instance;

    // These are connection variables that are exposed on the GameManager
    // inspector. The cloud version of SpacetimeDB needs sslEnabled = true
    [SerializeField] private string moduleAddress = "YOUR_MODULE_DOMAIN_OR_ADDRESS";
    [SerializeField] private string hostName = "spacetimedb.com/spacetimedb";
    [SerializeField] private bool sslEnabled = true;

    // This is the identity for this player that is automatically generated
    // the first time you log in. We set this variable when the 
    // onIdentityReceived callback is triggered by the SDK after connecting
    private Identity local_identity;

    public GameObject PlayerPrefab;
    public GameObject IronPrefab;

    [SerializeField] private GameObject preSpawnCamera;

    // Start is called before the first frame update
    void Start()
    {
        instance = this;

        // When we connect to SpacetimeDB we send our subscription queries
        // to tell SpacetimeDB which tables we want to get updates for.
        NetworkManager.instance.onConnect += () =>
        {
            Debug.Log("Connected.");

            NetworkManager.instance.Subscribe(new List<string>()
            {
                "SELECT * FROM Config",
                "SELECT * FROM SpawnableEntityComponent",
                "SELECT * FROM PlayerComponent",
                "SELECT * FROM MobileEntityComponent",
                "SELECT * FROM ChatMessage",
                // Our new tables for part 2 of the tutorial
                "SELECT * FROM ResourceNodeComponent",
                "SELECT * FROM StaticLocationComponent"
            });
        };

        // called when we have an error connecting to SpacetimeDB
        NetworkManager.instance.onConnectError += a =>
        {
            Debug.LogError($"Connection error: " + (a.HasValue ? a.Value.ToString() : "Null"));
        };

        // called when we are disconnected from SpacetimeDB
        NetworkManager.instance.onDisconnect += (closeStatus, error) =>
        {
            Debug.Log("Disconnected.");
        };


        // called when we receive the client identity from SpacetimeDB
        NetworkManager.instance.onIdentityReceived += (identity) => {
            local_identity = identity;
        };


        // called after our local cache is populated from a Subscribe call
        NetworkManager.instance.onSubscriptionUpdate += OnSubscriptionUpdate;

        PlayerComponent.OnInsert += PlayerComponent_OnInsert;
        PlayerComponent.OnUpdate += PlayerComponent_OnUpdate;

        ResourceNodeComponent.OnInsert += ResourceNodeComponent_OnInsert;

        Reducer.OnChatMessageEvent += OnChatMessageEvent;

        // now that we’ve registered all our callbacks, lets connect to
        // spacetimedb
        NetworkManager.instance.Connect(hostName, moduleAddress, sslEnabled);
    }

    private void ResourceNodeComponent_OnInsert(ResourceNodeComponent insertedValue, ClientApi.Event dbEvent)
    {
        switch (insertedValue.ResourceType)
        {
            case ResourceNodeType.Iron:
                var iron = Instantiate(IronPrefab);
                StaticLocationComponent loc = StaticLocationComponent.FilterByEntityId(insertedValue.EntityId);
                iron.transform.position = new Vector3(loc.Location.X, 0.0f, loc.Location.Z);
                iron.transform.rotation = Quaternion.Euler(0.0f, loc.Rotation, 0.0f);
                break;
        }
    }

    private void PlayerComponent_OnUpdate(PlayerComponent oldValue, PlayerComponent newValue, ClientApi.Event dbEvent)
    {
        OnPlayerComponentChanged(newValue);
    }

    private void PlayerComponent_OnInsert(PlayerComponent obj, ClientApi.Event dbEvent)
    {
        OnPlayerComponentChanged(obj);
    }

    private void OnPlayerComponentChanged(PlayerComponent obj)
    {
        // if the identity of the PlayerComponent matches our user identity then this is the local player
        if (Identity.From(obj.OwnerId) == local_identity)
        {
            // Get the MobileEntityComponent for this object and update the position to match the server
            MobileEntityComponent mobPos = MobileEntityComponent.FilterByEntityId(obj.EntityId);
            LocalPlayer.instance.transform.position = new Vector3(mobPos.Location.X, 0.0f, mobPos.Location.Z);
            // Now that we have our initial position we can start the game
            StartGame();
        }
        // otherwise this is a remote player
        else
        {
            // if the remote player is logged in, spawn it 
            if (obj.LoggedIn)
            {
                // spawn the player object and attach the RemotePlayer component
                var remotePlayer = Instantiate(PlayerPrefab);
                remotePlayer.AddComponent<RemotePlayer>().EntityId = obj.EntityId;
            }
            // otherwise we need to look for the remote player object in the scene (if it exists) and destroy it
            else
            {
                var remotePlayer = FindObjectsOfType<RemotePlayer>().FirstOrDefault(item => item.EntityId == obj.EntityId);
                if (remotePlayer != null)
                {
                    Destroy(remotePlayer.gameObject);
                }
            }
        }
    }

    private void OnChatMessageEvent(ClientApi.Event.Types.Status status, Identity caller, string message)
    {
        var player = PlayerComponent.FilterByOwnerId(caller.Bytes);
        if (player != null)
        {
            UIChatController.instance.OnChatMessageReceived(player.Username + ": " + message);
        }
    }   

    void OnSubscriptionUpdate()
    {
        // If we don't have any data for our player, then we are creating a 
        // new one. Let's show the username dialog, which will then call the
        // create player reducer
        var player = PlayerComponent.FilterByOwnerId(local_identity.Bytes);
        if (player == null)
        {
            // Show username selection
            UIUsernameChooser.instance.Show();
        }

        // Show the Message of the Day in our Config table of the Client Cache
        UIChatController.instance.OnChatMessageReceived("Message of the Day: " + Config.FilterByVersion(0).MessageOfTheDay);

        // Now that we've done this work we can unregister this callback
        NetworkManager.instance.onSubscriptionUpdate -= OnSubscriptionUpdate;
    }

    public void StartGame()
    {
        preSpawnCamera.SetActive(false);
        Reticle.instance.OnStart();
        UIChatController.instance.enabled = true;
    }
}
