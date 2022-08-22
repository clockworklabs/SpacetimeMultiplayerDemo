using System;
using System.Collections;
using System.Collections.Generic;
using JetBrains.Annotations;
using SpacetimeDB;
using UnityEngine;

public class NetworkPlayer : MonoBehaviour
{
    private uint _playerId;
    public static uint? localPlayerId;
    public GameObject cameraRig;
    public static byte[] identity;
    public static string token;

    private void Awake()
    {
        cameraRig.SetActive(false);
        StdbNetworkManager.instance.clientTick += GameTick;
    }

    public void Spawn(uint playerId, Vector3 spawnPosition)
    {
        _playerId = playerId;
        if (IsLocal())
        {
            gameObject.name = $"Local Player - {playerId}";
            localPlayerId = playerId;
            BitCraftMiniGameManager.instance.LocalPlayerCreated();
            cameraRig.SetActive(true);
            PlayerMovementController.Local = GetComponent<PlayerMovementController>();
            PlayerInventoryController.Local = GetComponent<PlayerInventoryController>();
            PlayerInventoryController.Local.Spawn();
            transform.position = spawnPosition;
        }
        else
        {
            gameObject.name = $"Remote Player - {playerId}";
        }
    }

    public bool IsLocal() => localPlayerId.HasValue && localPlayerId.Value == _playerId;

    private Vector3? lastUpdatePosition;
    private Quaternion? lastUpdateRotation;
    
    void GameTick()
    {
        if (!IsLocal())
        {
            return;
        }

        var ourPos = PlayerMovementController.Local.GetModelTransform().position;
        var ourRot = PlayerMovementController.Local.GetModelTransform().rotation;

        if (!lastUpdatePosition.HasValue || (ourPos - lastUpdatePosition.Value).sqrMagnitude > .1f 
                                         || !lastUpdateRotation.HasValue || Quaternion.Angle(lastUpdateRotation.Value, ourRot) > 1.0f)
        {
            Reducer.MovePlayer(_playerId, ourPos.x, ourPos.y, ourPos.z, ourRot.x, ourRot.y, ourRot.z, ourRot.w);
            lastUpdatePosition = ourPos;
            lastUpdateRotation = ourRot;
        }
    }
}