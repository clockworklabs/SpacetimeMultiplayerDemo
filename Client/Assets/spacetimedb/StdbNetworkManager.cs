using System;
using System.Collections;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using Google.Protobuf;
using SpacetimeDB;
using UnityEngine;
using Websocket;

public class StdbNetworkManager : Singleton<StdbNetworkManager>
{
    public event Action onConnect;
    public event Action onDisconnect;
    public event Action<TableRowOperation> onRowUpdate;
    
    private WebSocketDispatch.WebSocket webSocket;

    protected override void Awake()
    {
        base.Awake();
        var options = new WebSocketDispatch.ConnectOptions
        {
            Url =
                "ws://localhost:3000/database/c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470/bitcraftmini/subscribe",
            Protocol = "v1.text.spacetimedb",
        };
        webSocket = new WebSocketDispatch.WebSocket(options);
        webSocket.OnMessage += OnMessageReceived;
        webSocket.OnClose += status =>
        {
            OnDisconnect();
        };
        webSocket.OnConnect += OnConnect;
    }

    public void Connect()
    {
        Task.Run(async () => await webSocket.Connect());
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
        var chars = Encoding.Default.GetString(bytes);
        Debug.Log(chars);

        var subscriptionUpdate = Newtonsoft.Json.JsonConvert.DeserializeObject<SubscriptionUpdate>(chars);
        foreach(var tableUpdate in subscriptionUpdate.TableUpdates)
        {
            var tableId = tableUpdate.TableId;
            foreach (var row in tableUpdate.TableRowOperations)
            {
                // send rows to record manager
            }
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
        Debug.LogWarning($"JSON: {json}");
        Task.Run(async () => await webSocket.Send(
            Encoding.ASCII.GetBytes(json)));
    }

    private void Update()
    {
        webSocket.Update();
    }
}
