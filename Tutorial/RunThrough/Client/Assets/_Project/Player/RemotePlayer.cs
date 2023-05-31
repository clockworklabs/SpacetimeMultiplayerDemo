using SpacetimeDB;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class RemotePlayer : MonoBehaviour
{
    public ulong EntityId { get; set; }

    // Start is called before the first frame update
    void Start()
    {
        MobileEntityComponent.OnUpdate += MobileEntityComponent_OnUpdate;        

        MobileEntityComponent mobPos = MobileEntityComponent.FilterByEntityId(EntityId);
        transform.position = new Vector3(mobPos.Location.X, 0.0f, mobPos.Location.Z);
    }

    private void MobileEntityComponent_OnUpdate(MobileEntityComponent oldObj, MobileEntityComponent obj)
    {
        if(obj.EntityId == EntityId)
        {
            var movementController = GetComponent<PlayerMovementController>(); 
            movementController.DirectionVec = new Vector3(obj.Direction.X, 0.0f, obj.Direction.Z);
            // correct the position when stopped
            if (movementController.DirectionVec == Vector3.zero)
            {
                transform.position = new Vector3(obj.Location.X, 0.0f, obj.Location.Z);
            }
        }
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
