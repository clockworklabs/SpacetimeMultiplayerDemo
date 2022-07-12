using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;

public class NetworkPlayer : MonoBehaviour
{
    private uint _playerId;
    public static uint? _localPlayerId;
    public GameObject cameraRig;

    private void Awake()
    {
        cameraRig.SetActive(false);
    }

    public void Spawn(uint playerId)
    {
        _playerId = playerId;
        if (IsLocal())
        {
            gameObject.name = $"Local Player - {playerId}";
            _localPlayerId = playerId;
            BitCraftMiniGameManager.instance.LocalPlayerCreated();
            cameraRig.SetActive(true);
            PlayerMovementController.Local = GetComponent<PlayerMovementController>();
        }
        else
        {
            gameObject.name = $"Remote Player - {playerId}";
        }
    }

    public bool IsLocal() => _localPlayerId.HasValue && _localPlayerId.Value == _playerId;

    private void Update()
    {
        if (!IsLocal())
        {
            return;
        }

        var ourPos = transform.position;
        Reducer.MovePlayer(_playerId, ourPos.x, ourPos.y, ourPos.z);
    }
}