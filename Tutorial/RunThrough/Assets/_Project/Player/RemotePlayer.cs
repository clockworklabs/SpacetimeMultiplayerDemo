using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using SpacetimeDB;

public class RemotePlayer : MonoBehaviour
{
    public ulong EntityId;
    public string Username;

    void Start()
    {
        // register for a callback that is called when the client gets an 
        // update for a row in the MobileEntityComponent table
        MobileEntityComponent.OnUpdate += MobileEntityComponent_OnUpdate;


        // get the last location for this player and set the initial 
        // position 
        MobileEntityComponent mobPos = MobileEntityComponent.FilterByEntityId(EntityId);
        transform.position = new Vector3(mobPos.Location.X, 0.0f, mobPos.Location.Z);
    }

    private void MobileEntityComponent_OnUpdate(MobileEntityComponent oldObj, MobileEntityComponent obj, ClientApi.Event dbEvent)
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
                transform.position = new Vector3(obj.Location.X, 0.0f, obj.Location.Z);
            }
        }
    }
}
