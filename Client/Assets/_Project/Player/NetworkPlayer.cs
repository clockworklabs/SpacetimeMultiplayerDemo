using System;
using System.Collections;
using System.Collections.Generic;
using JetBrains.Annotations;
using SpacetimeDB;
using UnityEngine;

public class NetworkPlayer : MonoBehaviour
{
    [SerializeField] private GameObject cameraRig;
    [SerializeField] private GameObject[] disableWhileOffline;

    private uint? _playerId;
    public static uint? localPlayerId;
    public static Hash? identity;
    public static string token;

    public static Action OnLocalPlayerInitialized;

    public uint EntityId => _playerId.Value;

    public Vector3 targetPosition;
    public Quaternion targetRotation;

    private void Awake()
    {
        cameraRig.SetActive(false);
        BitCraftMiniGameManager.instance.messageSendTick += GameTick;
        targetPosition = transform.position;
        targetRotation = transform.rotation;
    }

    public void LoginStateChanged()
    {
        if (!_playerId.HasValue)
        {
            Debug.LogWarning("Player has not been spawned yet!");
            return;
        }

        if (IsLocal())
        {
            return;
        }
        
        var loginState = PlayerLoginComponent.FilterByEntityId(_playerId.Value);
        if (loginState != null)
        {
            Debug.Log($"Player {_playerId.Value} has changed login state: {loginState.loggedIn}");
            foreach (var mesh in disableWhileOffline)
            {
                mesh.SetActive(loginState.loggedIn);
            }

            var body = GetComponent<Rigidbody>();
            body.isKinematic = !loginState.loggedIn;
        }
    }

    public void Spawn(uint playerId)
    {
        _playerId = playerId;
        if (IsLocal())
        {
            gameObject.name = $"Local Player - {playerId}";
            BitCraftMiniGameManager.instance.LocalPlayerCreated();
            cameraRig.SetActive(true);
            PlayerMovementController.Local = GetComponent<PlayerMovementController>();
            PlayerInventoryController.Local = GetComponent<PlayerInventoryController>();
            PlayerAnimator.Local = GetComponentInChildren<PlayerAnimator>(true);

            PlayerInventoryController.Local.Spawn();
            TradeSessionController.Local.Spawn();

            // refresh player inventory
            var inventory = InventoryComponent.FilterByEntityId(playerId);
            PlayerInventoryController.Local.InventoryUpdate(inventory);

            // We are now logged in
            Reducer.PlayerUpdateLoginState(true);

            OnLocalPlayerInitialized?.Invoke();

            GpuInstancer.instance.cam = cameraRig.GetComponentInChildren<Camera>();
            GpuInstancer.instance.EnableInstancer();
        }
        else
        {
            gameObject.name = $"Remote Player - {playerId}";
            LoginStateChanged();
        }

        var entityTransform = TransformComponent.FilterByEntityId(playerId);
        if (entityTransform != null)
        {
            transform.position = entityTransform.pos.ToVector3();
            transform.rotation = entityTransform.rot.ToQuaternion();
        }
        else
        {
            Debug.LogWarning($"No transform for identity: {playerId}");
        }
    }

    public void SetTargetTransform(Vector3 position, Quaternion rotation) {
        targetPosition = position;
        targetRotation = rotation;
    }

    public bool IsLocal() => localPlayerId.HasValue && localPlayerId.Value == _playerId;

    private UnityEngine.Vector3? lastUpdatePosition;
    private Quaternion? lastUpdateRotation;

    void GameTick()
    {
        if (!IsLocal() || !localPlayerId.HasValue)
        {
            return;
        }

        var ourPos = PlayerMovementController.Local.GetModelTransform().position;
        var ourRot = PlayerMovementController.Local.GetModelTransform().rotation;

        if (!lastUpdatePosition.HasValue || (ourPos - lastUpdatePosition.Value).sqrMagnitude > .1f
                                         || !lastUpdateRotation.HasValue ||
                                         Quaternion.Angle(lastUpdateRotation.Value, ourRot) > 1.0f)
        {
            Reducer.MovePlayer(localPlayerId.Value, ourPos.ToStdb(), ourRot.ToStdb());
            lastUpdatePosition = ourPos;
            lastUpdateRotation = ourRot;
        }
    }

    void Update() {
        if (!IsLocal()) {
            transform.position = Vector3.Lerp(targetPosition, transform.position, 0.5f);
            transform.rotation = Quaternion.Lerp(targetRotation, transform.rotation, 0.5f);
        }
    }
}