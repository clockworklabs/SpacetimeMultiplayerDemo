using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlayerMovementController : MonoBehaviour
{
    [SerializeField] private Transform modelTransform;
    [SerializeField] private float movementSpeed;
    [SerializeField] private float modelTurnSpeed;
    [SerializeField] private Animator anim;

    public Vector3 DirectionVec;
    public float VerticalVelocity;

    private bool moving;
    private bool _interacting;

    public static CallbackBool localMovementDisabled = new CallbackBool(CallbackBool.Mode.Or);
    public static CallbackBool rigidBodyDisabled = new CallbackBool(CallbackBool.Mode.Or);
    private Rigidbody body;
    private static readonly int WalkingProperty = Animator.StringToHash("Walking");

    public static PlayerMovementController Local;

    public float JumpHeight = 5f;
    public float Gravity = -9.8f;
    private float TerminalVelocity = 53.0f;

    protected void Awake()
    {
        body = GetComponent<Rigidbody>();

        rigidBodyDisabled.Add(() =>
        {
            return false;
        });

        GetComponentInChildren<PlayerAnimator>(true).OnInteractionUpdate += OnInteractionUpdate;
    }

    private void OnDestroy()
    {
        GetComponentInChildren<PlayerAnimator>(true).OnInteractionUpdate -= OnInteractionUpdate;
    }

    public Transform GetModelTransform() => modelTransform;

    private void FixedUpdate()
    {
        // Check to see if we need to disable/enable kinematics
        if (rigidBodyDisabled.Invoke() != body.isKinematic)
        {
            body.isKinematic = !body.isKinematic;
        }

        if (localMovementDisabled.Invoke() || !CameraController.instance.GameCameraEnabled)
        {
            return;
        }

        if (DirectionVec.sqrMagnitude != 0 || VerticalVelocity != 0)
        {
            body.MovePosition(body.position + (DirectionVec * (Time.fixedDeltaTime * movementSpeed)) + (new Vector3(0.0f, VerticalVelocity, 0.0f) * Time.fixedDeltaTime));
        }
    }

    public bool IsMoving() => moving;

    public void SetMoving(bool moving)
    {
        this.moving = moving;
        anim.SetBool(WalkingProperty, moving);
    }

    void OnInteractionUpdate(bool interacting)
    {
        _interacting = interacting;
    }

    private void Update()
    {
        if (_interacting || (this == Local && !CameraController.instance.GameCameraEnabled))
        {
            moving = false;
            anim.SetBool(WalkingProperty, false);
            return;
        }

        anim.SetBool(WalkingProperty, moving);

        if (localMovementDisabled.Invoke())
        {
            return;
        }
        var wasMoving = moving;
        moving = DirectionVec.sqrMagnitude > 0;

        if (moving)
        {
            var worldMovementDirection = Quaternion.LookRotation(DirectionVec, Vector3.up);
            modelTransform.rotation = Quaternion.RotateTowards(modelTransform.rotation,
                worldMovementDirection, modelTurnSpeed * Time.deltaTime);
        }

        if (GetComponentInChildren<PlayerAnimator>(true).Grounded)
        {
            VerticalVelocity = 0.0f;
        }
        else if(VerticalVelocity < TerminalVelocity)
        {
            VerticalVelocity += Gravity * Time.deltaTime;
        }
    }

    public void Jump()
    {
        VerticalVelocity = Mathf.Sqrt(JumpHeight * -2f * Gravity);
    }
}
