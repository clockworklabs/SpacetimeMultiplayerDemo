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
    private bool _interacting;

    public static CallbackBool localMovementDisabled = new CallbackBool(CallbackBool.Mode.Or);
    public static CallbackBool rigidBodyDisabled = new CallbackBool(CallbackBool.Mode.Or);
    private Rigidbody body;
    private static readonly int WalkingProperty = Animator.StringToHash("Walking");

    public static PlayerMovementController Local;

    protected void Awake()
    {
        body = GetComponent<Rigidbody>();
        player = GetComponent<NetworkPlayer>();

        // We must be standing in a valid chunk
        rigidBodyDisabled.Add(() =>
        {
            var config = SpacetimeDB.Config.FilterByVersion(0);
            var position = transform.position;
            var chunkPosX = MathUtil.RoundNegInf(position.x / config.chunkSize);
            var chunkPosY = MathUtil.RoundNegInf(position.z / config.chunkSize);
            var chunk = TerrainController.instance.GetChunk(chunkPosX, chunkPosY);
            return chunk == null;
        });

        GetComponentInChildren<PlayerAnimator>().OnInteractionUpdate += OnInteractionUpdate;
    }

	private void OnDestroy()
	{
        GetComponentInChildren<PlayerAnimator>().OnInteractionUpdate -= OnInteractionUpdate;
    }

	public Transform GetModelTransform() => modelTransform;
    
    public void SetMove(UnityEngine.Vector3 vec) => movementVec = vec;

    private void FixedUpdate()
    {
        // Check to see if we need to disable/enable kinematics
        if (rigidBodyDisabled.Invoke() != body.isKinematic)
        {
            body.isKinematic = !body.isKinematic;
        }
        
        if (!player.IsLocal() || localMovementDisabled.Invoke() || !CameraController.instance.GameCameraEnabled)
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
        anim.SetBool(WalkingProperty, moving);
	}

	void OnInteractionUpdate(bool interacting)
	{
        _interacting = interacting;
	}

    private void Update()
    {
        if (_interacting || !CameraController.instance.GameCameraEnabled)
        {
            moving = false;
            anim.SetBool(WalkingProperty, false);
            return;
        }

        anim.SetBool(WalkingProperty, moving);

        if (!player.IsLocal() || localMovementDisabled.Invoke() || !NetworkPlayer.localPlayerId.HasValue)
        {
            return;
        }

        var vec = new Vector3(movementVec.x, 0.0f, movementVec.y);
        vec = CameraController.instance.transform.TransformDirection(vec);
        var wasMoving = moving;
        moving = movementVec.sqrMagnitude > 0; 
        
        if (moving)
        {
            var worldMovementDirection = Quaternion.LookRotation(vec, Vector3.up);
            modelTransform.rotation = Quaternion.RotateTowards(modelTransform.rotation, 
                worldMovementDirection, modelTurnSpeed * Time.deltaTime);
        }

        if (moving != wasMoving)
        {
            SpacetimeDB.Reducer.UpdateAnimation(NetworkPlayer.localPlayerId.Value, moving, 0);
        }
    }
}
