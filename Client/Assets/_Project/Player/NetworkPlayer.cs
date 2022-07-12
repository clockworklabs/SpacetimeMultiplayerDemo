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
        StdbNetworkManager.instance.clientTick += GameTick;
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

    void GameTick()
    {
        if (!IsLocal())
        {
            return;
        }

        var ourPos = PlayerMovementController.Local.GetModelTransform().position.ToStdb();
        var ourRot = PlayerMovementController.Local.GetModelTransform().rotation.ToStdb();
        Reducer.MovePlayer(_playerId, ourPos, ourRot, PlayerMovementController.Local.IsMoving());
    }
}