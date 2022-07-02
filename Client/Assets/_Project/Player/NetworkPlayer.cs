using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;

public class NetworkPlayer : MonoBehaviour
{
    private uint _playerId;
    public static uint? _localPlayerId;
    
    public void Spawn(uint playerId, bool isLocal)
    {
        _playerId = playerId;
        if (isLocal)
        {
            _localPlayerId = playerId;
        }
    }

    public bool IsLocal() => _localPlayerId.HasValue && _localPlayerId.Value == _playerId;

    private void Update()
    {
        if (IsLocal())
        {
            var ourPos = transform.position;
            Reducer.MovePlayer(_playerId, ourPos.x, ourPos.y, ourPos.z);
        }
    }
}
