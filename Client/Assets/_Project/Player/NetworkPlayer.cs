using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class NetworkPlayer : MonoBehaviour
{
    private uint _playerId;
    public static uint? _localPlayerId;
    
    public void Spawn(uint playerId, bool isLocal)
    {
        this._playerId = playerId;
        if (isLocal)
        {
            _localPlayerId = playerId;
        }
    }

    public bool IsLocal() => _localPlayerId.HasValue && _localPlayerId.Value == _playerId;
}
