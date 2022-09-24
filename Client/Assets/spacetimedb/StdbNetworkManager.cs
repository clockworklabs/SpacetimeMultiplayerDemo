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

    public delegate void RowUpdate(string tableName, TableOp op, TypeValue? oldValue, TypeValue? newValue);

    public event Action onConnect;
    public event Action onDisconnect;
    public event Action clientTick;
    
    /// <summary>
    /// Invoked on each row update to each table.
    /// </summary>
    public event RowUpdate tableUpdate;
    
    /// <summary>
    /// Callback is invoked after a transaction or subscription update is received and all updates have been applied.
    /// </summary>
    public event Action onRowUpdateComplete;
    /// <summary>
    /// Invoked when an event message is received or at the end of a transaction update.
    /// </summary>
    public event Action<ClientApi.Event> onEvent;

    private WebSocketDispatch.WebSocket webSocket;
    private bool connectionClosed;
    public static StdbClientCache clientDB;

    private float? lastClientTick;
    public static float clientTickInterval;

    protected override void Awake()
    {
        base.Awake();
        var options = new WebSocketDispatch.ConnectOptions
        {
            Url =
                "ws://localhost:3000/database/subscribe?name=bitcraftmini",

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
        clientDB.AddTable("PlayerComponent", PlayerComponent.GetTypeDef());
        clientDB.AddTable("TransformComponent", TransformComponent.GetTypeDef());
        clientDB.AddTable("AnimationComponent", AnimationComponent.GetTypeDef());
        clientDB.AddTable("InventoryComponent", InventoryComponent.GetTypeDef());
        clientDB.AddTable("PlayerLoginComponent", PlayerLoginComponent.GetTypeDef());
        clientDB.AddTable("Config", Config.GetTypeDef());
        clientDB.AddTable("PlayerChatMessage", PlayerChatMessage.GetTypeDef());
        clientDB.AddTable("Chunk", Chunk.GetTypeDef());
        clientDB.AddTable("ChunkData", ChunkData.GetTypeDef());
        clientDB.AddTable("NpcComponent", NpcComponent.GetTypeDef());
        clientDB.AddTable("ResourceComponent", ResourceComponent.GetTypeDef());

        clientTickInterval = 1 / clientTicksPerSecond;
    }

    private void OnDestroy()
    {
        connectionClosed = true;
        webSocket.Close();
        webSocket = null;
    }

    public void Connect()
    {
        var token = PlayerPrefs.HasKey(GetTokenKey()) ? PlayerPrefs.GetString(GetTokenKey()) : null;

        Task.Run(async () =>
        {
            try
            {
                await webSocket.Connect(token);
            }
            catch (Exception e)
            {
                if (connectionClosed)
                {
                    Debug.Log("Connection closed gracefully.");
                    return;
                }
                
                Debug.LogException(e);
            }
        });
    }

    private struct DbEvent
    {
        public string tableName;
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
                    var tableName = update.TableName;
                    var tableId = update.TableId;
                    var table = clientDB.GetTable(tableName);
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
                                        tableName = tableName,
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
                                        tableName = tableName,
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
                var eventCount = _dbEvents.Count;
                for (int i = 0; i < eventCount; i++)
                {
                    bool isUpdate = false;
                    if (i < eventCount - 1)
                    {
                        if (_dbEvents[i].op == TableOp.Delete && _dbEvents[i + 1].op == TableOp.Insert)
                        {
                            // somewhat hacky: Delete followed by an insert on the same table is considered an update.
                            isUpdate = _dbEvents[i].tableName.Equals(_dbEvents[i + 1].tableName);
                        }
                    }
                    if (isUpdate)
                    {
                        // Merge delete and insert in one update
                        tableUpdate?.Invoke(_dbEvents[i].tableName, TableOp.Update, _dbEvents[i].oldValue, _dbEvents[i+1].newValue);
                        i++;
                    }
                    else
                    {
                        tableUpdate?.Invoke(_dbEvents[i].tableName, _dbEvents[i].op, _dbEvents[i].oldValue, _dbEvents[i].newValue);
                    }
                }

                switch (message.TypeCase)
                {
                    case ClientApi.Message.TypeOneofCase.SubscriptionUpdate:
                        onRowUpdateComplete?.Invoke();
                        break;
                    case ClientApi.Message.TypeOneofCase.TransactionUpdate:
                        onRowUpdateComplete?.Invoke();
                        onEvent?.Invoke(message.TransactionUpdate.Event);
                        break;
                }

                break;
            case ClientApi.Message.TypeOneofCase.IdentityToken:
                NetworkPlayer.identity = Hash.From(message.IdentityToken.Identity.ToByteArray());
                NetworkPlayer.token = message.IdentityToken.Token;

                PlayerPrefs.SetString(GetTokenKey(), NetworkPlayer.token);
                break;
            case ClientApi.Message.TypeOneofCase.Event:
                onEvent?.Invoke(message.Event);
                break;
        }
    }

    private string GetTokenKey()
    {
        var key = "bitcraftmini.identity_token";
#if UNITY_EDITOR
        // Different editors need different keys
        key += $" - {Application.dataPath}";
#endif
        return key;
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