using System;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

namespace SpacetimeDB
{
    public class EditorNetworkManager
    {
        SpacetimeDB.WebSocket webSocket;
        public bool IsConnected => isConnected;
        private bool isConnected;

        public event Action<string,ClientApi.Event.Types.Status> onTransactionComplete;

        private static string GetTokenKey()
        {
            var key = "spacetimedb.identity_token";
#if UNITY_EDITOR
            // Different editors need different keys
            key += $" - {Application.dataPath}";
#endif
            return key;
        }

        public EditorNetworkManager(string host, string database)
        {
            var options = new SpacetimeDB.ConnectOptions
            {
                //v1.bin.spacetimedb
                //v1.text.spacetimedb
                Protocol = "v1.bin.spacetimedb",
            };
            webSocket = new SpacetimeDB.WebSocket(options);
            
            var token = PlayerPrefs.HasKey(GetTokenKey()) ? PlayerPrefs.GetString(GetTokenKey()) : null;
            webSocket.OnConnect += () =>
            {
                Debug.Log("Connected");
                isConnected = true;
            };

            webSocket.OnConnectError += (message) =>
            {
                Debug.Log($"Connection error {message}");
            };

            webSocket.OnClose += (code, error) =>
            {
                Debug.Log($"Websocket closed");
                isConnected = false;
            };

            webSocket.OnMessage += OnMessageReceived;

            webSocket.Connect(token, "localhost:3000", "bitcraft", false);
        }

        private void OnMessageReceived(byte[] bytes) 
        {
            var message = ClientApi.Message.Parser.ParseFrom(bytes);
            if(message.TypeCase == ClientApi.Message.TypeOneofCase.TransactionUpdate)
            {
                var reducer = message.TransactionUpdate.Event.FunctionCall.Reducer;
                var status = message.TransactionUpdate.Event.Status;
                onTransactionComplete?.Invoke(reducer, status);
            }
        }

        public async void CallReducer(string reducer, params object[] args)
        {
            if(!isConnected)
            {
                Debug.Log("Not connected");
            }

            var _message = new NetworkManager.ReducerCallRequest
            {
                fn = reducer,
                args = args,
            };
            Newtonsoft.Json.JsonSerializerSettings _settings = new Newtonsoft.Json.JsonSerializerSettings
            {
                Converters = { new SpacetimeDB.SomeWrapperConverter(), new SpacetimeDB.EnumWrapperConverter() },
                ContractResolver = new SpacetimeDB.JsonContractResolver(),
            };
            var json = Newtonsoft.Json.JsonConvert.SerializeObject(_message, _settings);
            webSocket.Send(Encoding.ASCII.GetBytes("{ \"call\": " + json + " }"));
        }

        public void Update()
        {
            webSocket.Update();
        }

        public void Close()
        {
            if (webSocket != null)
            {
                webSocket.Close();
            }
        }        
    }
}
