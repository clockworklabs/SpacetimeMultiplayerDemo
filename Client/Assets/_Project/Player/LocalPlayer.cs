using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using SpacetimeDB.Types;

public class LocalPlayer : MonoBehaviour
{
    [SerializeField] private GameObject cameraRig;

    public ulong EntityId { get; set; }

    public static LocalPlayer instance;
    public string Username { set { UsernameElement.text = value; } }

    public TMP_Text UsernameElement;

    private Vector2 movementVec;

    // Start is called before the first frame update
    void Start()
    {
        instance = this;
        cameraRig.SetActive(true);
        PlayerMovementController.Local = GetComponent<PlayerMovementController>();
        PlayerInventoryController.Local = GetComponent<PlayerInventoryController>();
        PlayerAnimator.Local = GetComponentInChildren<PlayerAnimator>(true);        
    }

    private Vector3? lastUpdateDirection;

    private void FixedUpdate()
    {
        var directionVec = GetDirectionVec();
        PlayerMovementController.Local.DirectionVec = directionVec;

        // first get the position of the player
        var ourPos = PlayerMovementController.Local.GetModelTransform().position;
        // if we are moving , and we haven't updated our destination yet, or we've moved more than .1 units, update our destination
        if (directionVec.sqrMagnitude != 0 && (!lastUpdateDirection.HasValue || (directionVec - lastUpdateDirection.Value).sqrMagnitude > .1f))
        {
            Reducer.MovePlayer(new StdbVector2() { X = ourPos.x, Z = ourPos.z }, new StdbVector2() { X = directionVec.x, Z = directionVec.z });
            lastUpdateDirection = directionVec;
        }
        // if we stopped moving, send the update
        else if (directionVec.sqrMagnitude == 0 && lastUpdateDirection != null)
        {
            Reducer.StopPlayer(new StdbVector2() { X = ourPos.x, Z = ourPos.z });
            lastUpdateDirection = null;
        }
    }

    public void SetMove(UnityEngine.Vector3 vec) => movementVec = vec;

    public Vector3 GetDirectionVec()
    {
        var vec = new Vector3(movementVec.x, 0, movementVec.y);
        return CameraController.instance.transform.TransformDirection(vec);
    }

    public void OnJump()
    {   
        PlayerAnimator.Local.Jump();
        PlayerMovementController.Local.Jump();

        Reducer.Jump(EntityId);
    }
}
