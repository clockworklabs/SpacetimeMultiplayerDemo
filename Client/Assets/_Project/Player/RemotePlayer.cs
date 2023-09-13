using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using SpacetimeDB.Types;
using TMPro;

public class RemotePlayer : MonoBehaviour
{
    public static List<RemotePlayer> Players = new List<RemotePlayer>();

    public ulong EntityId;
    public string Username { set { UsernameElement.text = value; } }

    public TMP_Text UsernameElement;

    void Start()
    {
        Players.Add(this);

        // initialize overhead name
        UsernameElement = GetComponentInChildren<TMP_Text>();
        var canvas = GetComponentInChildren<Canvas>();
        canvas.worldCamera = Camera.main;

        // register for a callback that is called when the client gets an 
        // update for a row in the MobileEntityComponent table
        MobileEntityComponent.OnUpdate += MobileEntityComponent_OnUpdate;

        // get the username for this player from the PlayerComponent table
        PlayerComponent playerComp = PlayerComponent.FilterByEntityId(EntityId);        
        Username = playerComp.Username;

        // get the last location for this player and set the initial 
        // position 
        MobileEntityComponent mobPos = MobileEntityComponent.FilterByEntityId(EntityId);
        Vector3 playerPos = new Vector3(mobPos.Location.X, 0.0f, mobPos.Location.Z);
        transform.position = new Vector3(playerPos.x, MathUtil.GetTerrainHeight(playerPos), playerPos.z);
    }

    private void MobileEntityComponent_OnUpdate(MobileEntityComponent oldObj, MobileEntityComponent obj, ReducerEvent callInfo)
    {
        // if the update was made to this object
        if (obj.EntityId == EntityId)
        {
            // update the DirectionVec in the PlayerMovementController component with the updated values
            var movementController = GetComponent<PlayerMovementController>();
            movementController.DirectionVec = new Vector3(obj.Direction.X, 0.0f, obj.Direction.Z);
            // if DirectionVec is {0,0,0} then we came to a stop so correct our position to match the server
            if (movementController.DirectionVec == Vector3.zero)
            {
                Vector3 playerPos = new Vector3(obj.Location.X, 0.0f, obj.Location.Z);
                transform.position = new Vector3(playerPos.x, MathUtil.GetTerrainHeight(playerPos), playerPos.z);
            }
        }
    }

    public void OnJump()
    {
        GetComponentInChildren<PlayerAnimator>().Jump();
        GetComponent<PlayerMovementController>().Jump();
    }
}
