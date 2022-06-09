using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlayerMovementController : Singleton<PlayerMovementController>
{
    [SerializeField] private Transform modelTransform;
    [SerializeField] private float movementSpeed;
    [SerializeField] private float modelTurnSpeed;
    [SerializeField] private Animator anim;

    private Vector2 movementVec;

    private Rigidbody body;
    private static readonly int WalkingProperty = Animator.StringToHash("Walking");

    protected override void Awake()
    {
        base.Awake();
        body = GetComponent<Rigidbody>();
    }

    public void SetMove(Vector3 vec) => movementVec = vec;

    private void FixedUpdate()
    {
        var vec = new Vector3(movementVec.x, 0.0f, movementVec.y);
        vec = CameraController.instance.transform.TransformDirection(vec);
        
        body.MovePosition(body.position + vec * (Time.fixedDeltaTime * movementSpeed));
    }

    private void Update()
    {
        var vec = new Vector3(movementVec.x, 0.0f, movementVec.y);
        vec = CameraController.instance.transform.TransformDirection(vec);
        var moving = movementVec.sqrMagnitude > 0; 
        
        if (moving)
        {
            var worldMovementDirection = Quaternion.LookRotation(vec, Vector3.up);
            modelTransform.rotation = Quaternion.RotateTowards(modelTransform.rotation, 
                worldMovementDirection, modelTurnSpeed * Time.deltaTime);
        }
        
        anim.SetBool(WalkingProperty, moving);
    }
}
