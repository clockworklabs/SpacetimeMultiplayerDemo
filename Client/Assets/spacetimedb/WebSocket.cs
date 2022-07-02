using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using UnityEngine;

namespace WebSocketDispatch
{
    internal abstract class MainThreadDispatch
    {
        public abstract void Execute();
    }

    class OnConnectMessage : MainThreadDispatch
    {
        private WebSocketOpenEventHandler receiver;
        
        public OnConnectMessage(WebSocketOpenEventHandler receiver)
        {
            this.receiver = receiver;
        }
        
        public override void Execute()
        {
            receiver.Invoke();
        }
    }
    
    class OnDisconnectMessage : MainThreadDispatch
    {
        private WebSocketCloseEventHandler receiver;
        private WebSocketCloseStatus? status;
        
        public OnDisconnectMessage(WebSocketCloseEventHandler receiver, WebSocketCloseStatus? status)
        {
            this.receiver = receiver;
            this.status = status;
        }
        
        public override void Execute()
        {
            receiver.Invoke(status);
        }
    }
    
    class OnMessage : MainThreadDispatch
    {
        private WebSocketMessageEventHandler receiver;
        private byte[] message;
        
        public OnMessage(WebSocketMessageEventHandler receiver, byte[] message)
        {
            this.receiver = receiver;
            this.message = message;
        }
        
        public override void Execute()
        {
            receiver.Invoke(message);
        }
    }
    
    public delegate void WebSocketOpenEventHandler();

    public delegate void WebSocketMessageEventHandler(byte[] message);

    public delegate void WebSocketErrorEventHandler(string errorMsg);

    public delegate void WebSocketCloseEventHandler(WebSocketCloseStatus? closeCode);

    public struct ConnectOptions
    {
        public string Url;
        public string Protocol;
    }

    public static class WebSocketHelpers
    {
        public static WebSocketCloseStatus ParseCloseCodeEnum(int closeCode)
        {
            if (Enum.IsDefined(typeof(WebSocketCloseStatus), closeCode))
            {
                return (WebSocketCloseStatus)closeCode;
            }

            return WebSocketCloseStatus.Empty;
        }

        public static WebSocketException GetErrorMessageFromCode(int errorCode)
        {
            switch (errorCode)
            {
                case -1:
                    return new WebSocketException(errorCode, "WebSocket instance not found.");
                case -2:
                    return new WebSocketException(errorCode, "WebSocket is already connected or in connecting state.");
                case -3:
                    return new WebSocketException(errorCode, "WebSocket is not connected.");
                case -4:
                    return new WebSocketException(errorCode, "WebSocket is already closing.");
                case -5:
                    return new WebSocketException(errorCode, "WebSocket is already closed.");
                case -6:
                    return new WebSocketException(errorCode, "WebSocket is not in open state.");
                case -7:
                    return new WebSocketException(errorCode,
                        "Cannot close WebSocket. An invalid code was specified or reason is too long.");
                default:
                    return new WebSocketException(errorCode, "Unknown error.");
            }
        }
    }


    public class WebSocket
    {
        // WebSocket buffer for incoming messages
        private static readonly int MAXMessageSize = 1024 * 1024 * 10; //10 MB

        // Connection parameters
        private readonly ConnectOptions _options;
        private readonly byte[] _receiveBuffer = new byte[MAXMessageSize];
        private readonly ConcurrentQueue<MainThreadDispatch> dispatchQueue = new ConcurrentQueue<MainThreadDispatch>();
        
        protected ClientWebSocket Ws;

        public WebSocket(ConnectOptions options)
        {
            Ws = new ClientWebSocket();
            _options = options;
        }

        public event WebSocketOpenEventHandler OnConnect;
        public event WebSocketMessageEventHandler OnMessage;
        public event WebSocketErrorEventHandler OnError;
        public event WebSocketCloseEventHandler OnClose;

        public async Task Connect()
        {
            var url = new Uri(_options.Url);
            Ws.Options.AddSubProtocol(_options.Protocol);
            Ws.Options.UseDefaultCredentials = true;

            var source = new CancellationTokenSource(10000);
            // var auth = Convert.ToBase64String(
            //     Encoding.Default.GetBytes(_options.Username + ":" + _options.Password));
            // Ws.Options.SetRequestHeader("Authorization", "Basic " + auth);

            try
            {
                await Ws.ConnectAsync(url, source.Token);
                dispatchQueue.Enqueue(new OnConnectMessage(OnConnect));
            }
            catch (WebSocketException ex)
            {
                if (ex.WebSocketErrorCode == WebSocketError.UnsupportedProtocol)
                {
                    Debug.LogError("Unsupported protocol.");
                    return;
                }

                dispatchQueue.Enqueue(new OnDisconnectMessage(OnClose, WebSocketCloseStatus.EndpointUnavailable));
                Debug.LogError("Error connecting: " + ex);
                return;
            }
            catch (Exception e)
            {
                dispatchQueue.Enqueue(new OnDisconnectMessage(OnClose, WebSocketCloseStatus.EndpointUnavailable));
                Debug.LogError("Other error: " + e);
                return;
            }

            while (Ws.State == WebSocketState.Open)
            {
                try
                {
                    var receiveResult = await Ws.ReceiveAsync(new ArraySegment<byte>(_receiveBuffer),
                        CancellationToken.None);
                    if (receiveResult.MessageType == WebSocketMessageType.Close)
                    {
                        await Ws.CloseAsync(WebSocketCloseStatus.NormalClosure, string.Empty,
                            CancellationToken.None);
                        if (receiveResult.CloseStatus != WebSocketCloseStatus.NormalClosure)
                        {
                            Debug.LogError("Server closed connection abnormally.");
                            dispatchQueue.Enqueue(new OnDisconnectMessage(OnClose, receiveResult.CloseStatus));
                        }
                    }
                    else
                    {
                        var count = receiveResult.Count;
                        while (receiveResult.EndOfMessage == false)
                        {
                            if (count >= MAXMessageSize)
                            {
                                var closeMessage = $"Maximum message size: {MAXMessageSize} bytes.";
                                await Ws.CloseAsync(WebSocketCloseStatus.MessageTooBig, closeMessage,
                                    CancellationToken.None);
                                return;
                            }
                            
                            receiveResult = await Ws.ReceiveAsync(
                                new ArraySegment<byte>(_receiveBuffer, count, MAXMessageSize - count),
                                CancellationToken.None);
                            count += receiveResult.Count;
                        }
                        
                        var buffCopy = new byte[count];
                        for (var x = 0; x < count; x++)
                            buffCopy[x] = _receiveBuffer[x];
                        dispatchQueue.Enqueue(new OnMessage(OnMessage, buffCopy));
                    }
                }
                catch (WebSocketException ex)
                {
                    if (ex.WebSocketErrorCode == WebSocketError.ConnectionClosedPrematurely)
                    {
                        Debug.LogError("Server closed connection prematurely.");
                        dispatchQueue.Enqueue(new OnDisconnectMessage(OnClose, null));
                        break;
                    }

                    Debug.LogError(ex);
                }
            }
        }

        public Task Close(WebSocketCloseStatus code = WebSocketCloseStatus.NormalClosure, string reason = null)
        {
            Ws?.CloseAsync(code, "Disconnecting normally.", CancellationToken.None);
            Ws = null;

            return Task.CompletedTask;
        }

        public async Task Send(byte[] message)
        {
            await Ws!.SendAsync(new ArraySegment<byte>(message), WebSocketMessageType.Text, true,
                CancellationToken.None);
        }

        public WebSocketState GetState()
        {
            return Ws!.State;
        }

        public void Update()
        {
            while (dispatchQueue.TryDequeue(out var result))
            {
                result.Execute();
            }
        }
    }
}