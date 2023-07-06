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
    [SerializeField] private string moduleAddress = "bitcraftmini";
    [SerializeField] private string hostName = "localhost:3000";
    [SerializeField] private bool sslEnabled = false;

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
        SpacetimeDBClient.instance.onConnect += () =>
        {
            Debug.Log("Connected.");

            SpacetimeDBClient.instance.Subscribe(new List<string>()
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
        SpacetimeDBClient.instance.onConnectError += a =>
        {
            Debug.LogError($"Connection error: " + (a.HasValue ? a.Value.ToString() : "Null"));
        };

        // called when we are disconnected from SpacetimeDB
        SpacetimeDBClient.instance.onDisconnect += (closeStatus, error) =>
        {
            Debug.Log("Disconnected.");
        };


        // called when we receive the client identity from SpacetimeDB
        SpacetimeDBClient.instance.onIdentityReceived += (token, identity) => {
            AuthToken.SaveToken(token);
            local_identity = identity;
        };


        // called after our local cache is populated from a Subscribe call
        SpacetimeDBClient.instance.onSubscriptionApplied += OnSubscriptionApplied;

        PlayerComponent.OnInsert += PlayerComponent_OnInsert;
        PlayerComponent.OnUpdate += PlayerComponent_OnUpdate;

        Reducer.OnChatMessageEvent += OnChatMessageEvent;

        ResourceNodeComponent.OnInsert += ResourceNodeComponent_OnInsert;

        // now that we’ve registered all our callbacks, lets connect to
        // spacetimedb
        SpacetimeDBClient.instance.Connect(AuthToken.Token, hostName, moduleAddress, sslEnabled);
    }

    void OnSubscriptionApplied()
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
        SpacetimeDBClient.instance.onSubscriptionApplied -= OnSubscriptionApplied;
    }

    private void PlayerComponent_OnUpdate(PlayerComponent oldValue, PlayerComponent newValue, ReducerEvent dbEvent)
    {
        OnPlayerComponentChanged(newValue);
    }

    private void PlayerComponent_OnInsert(PlayerComponent obj, ReducerEvent dbEvent)
    {
        OnPlayerComponentChanged(obj);
    }

    private void OnChatMessageEvent(ReducerEvent dbEvent, string message)
    {
        var player = PlayerComponent.FilterByOwnerId(dbEvent.Identity.Bytes);
        if (player != null)
        {
            UIChatController.instance.OnChatMessageReceived(player.Username + ": " + message);
        }
    }

    private void OnPlayerComponentChanged(PlayerComponent obj)
    {
        // if the identity of the PlayerComponent matches our user identity then this is the local player
        if (Identity.From(obj.OwnerId) == local_identity)
        {
            // Get the MobileEntityComponent for this object and update the position to match the server
            MobileEntityComponent mobPos = MobileEntityComponent.FilterByEntityId(obj.EntityId);
            Vector3 playerPos = new Vector3(mobPos.Location.X, 0.0f, mobPos.Location.Z);
            LocalPlayer.instance.transform.position = new Vector3(playerPos.x, MathUtil.GetTerrainHeight(playerPos), playerPos.z);
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

    private void ResourceNodeComponent_OnInsert(ResourceNodeComponent insertedValue, ReducerEvent callInfo)
    {
        switch (insertedValue.ResourceType)
        {
            case ResourceNodeType.Iron:
                var iron = Instantiate(IronPrefab);
                StaticLocationComponent loc = StaticLocationComponent.FilterByEntityId(insertedValue.EntityId);
                Vector3 nodePos = new Vector3(loc.Location.X, 0.0f, loc.Location.Z);
                iron.transform.position = new Vector3(nodePos.x, MathUtil.GetTerrainHeight(nodePos), nodePos.z);
                iron.transform.rotation = Quaternion.Euler(0.0f, loc.Rotation, 0.0f);
                break;
        }
    }

    public void StartGame()
    {
        preSpawnCamera.SetActive(false);
        Reticle.instance.OnStart();
        UIChatController.instance.enabled = true;
    }
}
