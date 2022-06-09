using System.Collections;
using System.Collections.Generic;
using Google.Protobuf;
using UnityEngine;

public class SpacetimeDBManager : Singleton<SpacetimeDBManager>
{
    /// <summary>
    /// Called when a connection is established to a spacetimedb server
    /// </summary>
    protected virtual void OnConnect()
    {
        
    }
    
    /// <summary>
    /// Called when the connection to the spacetimedb server is lost.
    /// </summary>
    protected virtual void OnDisconnect()
    {
        
    }
    
    /// <summary>
    /// Called when a connection is established to a spacetimedb server
    /// </summary>
    protected virtual void OnMessageReceived()
    {
        
    }

    protected void CallReducer(string reducerName, IMessage message)
    {
        
    }
}
