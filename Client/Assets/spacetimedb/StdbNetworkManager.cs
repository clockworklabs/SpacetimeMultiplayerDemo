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
    [SerializeField] private float clientTicksPerSecond = 30.0f;
    
    public event Action onConnect;
    public event Action onDisconnect;
    public event Action<uint, TableRowOperation> onRowUpdate;
    
    public event Action clientTick;
    
    private WebSocketDispatch.WebSocket webSocket;

    private float? lastClientTick;
    public static float clientTickInterval;

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

        clientTickInterval = 1 / clientTicksPerSecond;
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
        var message = Websocket.Message.Parser.ParseFrom(bytes);

        switch (message.TypeCase)
        {
            case Websocket.Message.TypeOneofCase.SubscriptionUpdate:
                foreach (var tableUpdate in message.SubscriptionUpdate.TableUpdates)
                {
                    var tableId = tableUpdate.TableId;
                    foreach (var row in tableUpdate.TableRowOperations)
                    {
                        onRowUpdate?.Invoke(tableId, row);
                    }
                }
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
        Debug.LogWarning($"JSON: {json}");
        Task.Run(async () => await webSocket.Send(
            Encoding.ASCII.GetBytes(json)));
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
