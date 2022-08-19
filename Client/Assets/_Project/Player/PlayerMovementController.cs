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

    private Vector2 movementVec;
    private NetworkPlayer player;
    private bool moving;

    public static CallbackBool LocalMovementDisabled = new CallbackBool(CallbackBool.Mode.Or);
    private Rigidbody body;
    private static readonly int WalkingProperty = Animator.StringToHash("Walking");

    public static PlayerMovementController Local;
    
    protected void Awake()
    {
        body = GetComponent<Rigidbody>();
        player = GetComponent<NetworkPlayer>();
    }

    public Transform GetModelTransform() => modelTransform;
    
    public void SetMove(Vector3 vec) => movementVec = vec;

    private void FixedUpdate()
    {
        if (!player.IsLocal() || LocalMovementDisabled.Invoke())
        {
            return;
        }
        
        var vec = new Vector3(movementVec.x, 0.0f, movementVec.y);
        vec = CameraController.instance.transform.TransformDirection(vec);
        
        body.MovePosition(body.position + vec * (Time.fixedDeltaTime * movementSpeed));
    }

    public bool IsMoving() => moving;
    
    public void SetMoving(bool moving)
    {
        if (player.IsLocal())
        {
            return;
        }

        this.moving = moving;
    }
    
    private void Update()
    {
        anim.SetBool(WalkingProperty, moving);
        
        if (!player.IsLocal() || LocalMovementDisabled.Invoke())
        {
            return;
        }

        var vec = new Vector3(movementVec.x, 0.0f, movementVec.y);
        vec = CameraController.instance.transform.TransformDirection(vec);
        moving = movementVec.sqrMagnitude > 0; 
        
        if (moving)
        {
            var worldMovementDirection = Quaternion.LookRotation(vec, Vector3.up);
            modelTransform.rotation = Quaternion.RotateTowards(modelTransform.rotation, 
                worldMovementDirection, modelTurnSpeed * Time.deltaTime);
        }
    }
}
