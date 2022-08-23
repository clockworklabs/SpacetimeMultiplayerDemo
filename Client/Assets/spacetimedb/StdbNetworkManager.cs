using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using ClientApi;
using SpacetimeDB;
using UnityEngine;

public class StdbNetworkManager : Singleton<StdbNetworkManager>
{
    public enum TableOp
    {
        Insert,
        Delete,
        Update
    }
    
    [Serializable]
    public class Message
    {
        public string fn;
        public object[] args;
    }
    
    [SerializeField] private float clientTicksPerSecond = 30.0f;

    public delegate void RowUpdate(uint tableId, TableOp op, TypeValue? oldValue, TypeValue? newValue);
    
    public event Action onConnect;
    public event Action onDisconnect;
    public event Action clientTick;
    public event RowUpdate tableUpdate;
    public event Action subscriptionUpdate;
    public event Action transactionUpdateComplete;

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
        webSocket.OnClose += status => onDisconnect?.Invoke();
        webSocket.OnConnect += () => onConnect?.Invoke();

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

    private struct DbEvent
    {
        public uint tableId;
        public TableOp op;
        public TypeValue? oldValue;
        public TypeValue? newValue;
    }
    
    private readonly List<DbEvent> _dbEvents = new List<DbEvent>();
    
    private void OnMessageReceived(byte[] bytes)
    {
        _dbEvents.Clear();
        var message = ClientApi.Message.Parser.ParseFrom(bytes);

        SubscriptionUpdate subscriptionUpdate = null;
        switch (message.TypeCase)
        {
            case ClientApi.Message.TypeOneofCase.SubscriptionUpdate:
                subscriptionUpdate = message.SubscriptionUpdate;
                break;
            case ClientApi.Message.TypeOneofCase.TransactionUpdate:
                subscriptionUpdate = message.TransactionUpdate.SubscriptionUpdate;
                break;
        }
        
        switch (message.TypeCase)
        {
            case ClientApi.Message.TypeOneofCase.SubscriptionUpdate:
            case ClientApi.Message.TypeOneofCase.TransactionUpdate:
                // First apply all of the state
                foreach (var update in subscriptionUpdate.TableUpdates)
                {
                    var tableId = update.TableId;
                    var table = clientDB.GetTable(tableId);
                    if (table == null)
                    {
                        continue;
                    }
                    
                    foreach (var row in update.TableRowOperations)
                    {
                        switch (row.Op)
                        {
                            case TableRowOperation.Types.OperationType.Delete:
                                var deletedValue = table.Delete(row.RowPk);
                                if (deletedValue.HasValue)
                                {
                                    _dbEvents.Add(new DbEvent()
                                    {
                                        tableId = tableId,
                                        op = TableOp.Delete,
                                        newValue = null,
                                        oldValue = deletedValue.Value
                                    });
                                }
                                break;
                            case TableRowOperation.Types.OperationType.Insert:
                                var insertedValue = table.Insert(row.RowPk, row.Row);
                                if (insertedValue.HasValue)
                                {
                                    _dbEvents.Add(new DbEvent
                                    {
                                        tableId = tableId,
                                        op = TableOp.Insert,
                                        newValue = insertedValue.Value,
                                        oldValue = null
                                    });
                                }
                                break;
                        }
                    }
                }
                
                // Send out events
                foreach (var dbEvent in _dbEvents)
                {
                    tableUpdate?.Invoke(dbEvent.tableId, dbEvent.op, dbEvent.oldValue, dbEvent.newValue);
                }

                switch (message.TypeCase)
                {
                    case ClientApi.Message.TypeOneofCase.SubscriptionUpdate:
                        this.subscriptionUpdate?.Invoke();
                        break;
                    case ClientApi.Message.TypeOneofCase.TransactionUpdate:
                        transactionUpdateComplete?.Invoke();
                        break;
                }
                break;
            case ClientApi.Message.TypeOneofCase.IdentityToken:
                NetworkPlayer.identity = Hash.From(message.IdentityToken.Identity.ToByteArray());
                NetworkPlayer.token = message.IdentityToken.Token;
                PlayerPrefs.SetString(tokenKey, NetworkPlayer.token);
                break;
        }
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