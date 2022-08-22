using System;
using System.Text;
using System.Threading.Tasks;
using SpacetimeDB;
using UnityEngine;

public class StdbNetworkManager : Singleton<StdbNetworkManager>
{
    [SerializeField] private float clientTicksPerSecond = 30.0f;
    
    public event Action onConnect;
    public event Action onDisconnect;
    public event Action clientTick;
    
    private WebSocketDispatch.WebSocket webSocket;
    public static StdbClientCache clientDB;

    private float? lastClientTick;
    public static float clientTickInterval;

    private const string tokenKey = "bitcraftmini.identity_token";

    protected override void Awake()
    {
        base.Awake();
        var options = new WebSocketDispatch.ConnectOptions
        {
            Url =
                "ws://localhost:3000/database/c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470/bitcraftmini/subscribe",
            
            //v1.bin.spacetimedb
            //v1.text.spacetimedb
            Protocol = "v1.bin.spacetimedb",
        };
        webSocket = new WebSocketDispatch.WebSocket(options);
        webSocket.OnMessage += OnMessageReceived;
        webSocket.OnClose += status =>
        {
            OnDisconnect();
        };
        webSocket.OnConnect += OnConnect;
        
        clientDB = new StdbClientCache();
        
        // TODO: This part should be automatically generated!
        clientDB.AddTable("Player", 1, Player.GetTypeDef());
        clientDB.AddTable("EntityTransform", 2, EntityTransform.GetTypeDef());
        clientDB.AddTable("PlayerAnimation", 3, PlayerAnimation.GetTypeDef());
        clientDB.AddTable("EntityInventory", 4, EntityInventory.GetTypeDef());
        clientDB.AddTable("Config", 5, Config.GetTypeDef());
        clientDB.AddTable("PlayerChatMessage", 6, PlayerChatMessage.GetTypeDef());
        
        clientTickInterval = 1 / clientTicksPerSecond;
    }

    public void Connect()
    {
        var token = PlayerPrefs.HasKey(tokenKey) ? PlayerPrefs.GetString(tokenKey) : null;
        
        Task.Run(async () =>
        {
            try
            {
                await webSocket.Connect(token);
            }
            catch (Exception e)
            {
                Debug.LogException(e);
            }
        });
    }

    /// <summary>
    /// Called when a connection is established to a spacetimedb server
    /// </summary>
    protected virtual void OnConnect()
    {
        Debug.Log("Connected.");
        onConnect?.Invoke();
    }
    
    /// <summary>
    /// Called when the connection to the spacetimedb server is lost.
    /// </summary>
    protected virtual void OnDisconnect()
    {
        Debug.LogWarning("Connection lost.");
        onDisconnect?.Invoke();
    }
    
    /// <summary>
    /// Called when a connection is established to a spacetimedb server
    /// </summary>
    protected virtual void OnMessageReceived(byte[] bytes)
    {
        var message = ClientApi.Message.Parser.ParseFrom(bytes);

        switch (message.TypeCase)
        {
            case ClientApi.Message.TypeOneofCase.SubscriptionUpdate:
                foreach (var tableUpdate in message.SubscriptionUpdate.TableUpdates)
                {
                    var tableId = tableUpdate.TableId;
                    foreach (var row in tableUpdate.TableRowOperations)
                    {
                        clientDB.ReceiveUpdate(tableId, row);
                    }
                }
                break;
            case ClientApi.Message.TypeOneofCase.TransactionUpdate:
                foreach (var tableUpdate in message.TransactionUpdate.SubscriptionUpdate.TableUpdates)
                {
                    var tableId = tableUpdate.TableId;
                    foreach (var row in tableUpdate.TableRowOperations)
                    {
                        clientDB.ReceiveUpdate(tableId, row);
                    }
                }
                break;
            case ClientApi.Message.TypeOneofCase.IdentityToken:
                NetworkPlayer.identity = message.IdentityToken.Identity.ToByteArray();
                NetworkPlayer.token = message.IdentityToken.Token;
                PlayerPrefs.SetString(tokenKey, NetworkPlayer.token);
                break;
        }
    }

    [Serializable]
    public class Message
    {
        public string fn;
        public object[] args;
    }
    
    internal void InternalCallReducer(Message message)
    {
        var json = Newtonsoft.Json.JsonConvert.SerializeObject(message);
        webSocket.Send(Encoding.ASCII.GetBytes(json));
    }

    private void Update()
    {
        webSocket.Update();

        if (!lastClientTick.HasValue)
        {
            lastClientTick = Time.time;
        }
        else
        {
            if (Time.time - lastClientTick > clientTickInterval)
            {
                lastClientTick = Time.time;
                clientTick?.Invoke();
            }
        }
    }
}
