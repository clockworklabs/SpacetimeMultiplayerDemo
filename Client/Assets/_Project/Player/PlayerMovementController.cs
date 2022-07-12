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

    private Rigidbody body;
    private static readonly int WalkingProperty = Animator.StringToHash("Walking");

    public static PlayerMovementController Local;
    
    protected void Awake()
    {
        body = GetComponent<Rigidbody>();
        player = GetComponent<NetworkPlayer>();
    }

    public void SetMove(Vector3 vec) => movementVec = vec;

    private void FixedUpdate()
    {
        if (!player.IsLocal())
        {
            return;
        }
        
        var vec = new Vector3(movementVec.x, 0.0f, movementVec.y);
        vec = CameraController.instance.transform.TransformDirection(vec);
        
        body.MovePosition(body.position + vec * (Time.fixedDeltaTime * movementSpeed));
    }

    private void Update()
    {
        if (!player.IsLocal())
        {
            return;
        }
        
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
