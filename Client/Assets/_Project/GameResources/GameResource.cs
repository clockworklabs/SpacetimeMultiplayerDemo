using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using SpacetimeDB.Types;

public class GameResource : MonoBehaviour
{
    public ulong EntityId;

    public ResourceNodeType Type => ResourceNodeComponent.FilterByEntityId(EntityId)?.ResourceType ?? ResourceNodeType.Iron;
    public int MaxHealth => ResourceNodeComponent.FilterByEntityId(EntityId)?.MaxHealth ?? 0;
    public int Health => ResourceNodeComponent.FilterByEntityId(EntityId)?.Health ?? 0;
    
	[SerializeField] private GameObject _vfx;
	[SerializeField] private GameObject _deathVfx;

	private bool _flagForDeath;

    public void Init(ResourceNodeComponent resourceNode)
	{		
		EntityId = resourceNode.EntityId;

        ResourceNodeComponent.OnDelete += OnDelete;
        ResourceNodeComponent.OnUpdate += OnUpdate;
    }

	private void OnDestroy()
	{
        ResourceNodeComponent.OnDelete -= OnDelete;
        ResourceNodeComponent.OnUpdate -= OnUpdate;
    }

	private void OnDelete(ResourceNodeComponent oldValue, ReducerEvent reducerEvent)
	{
		Debug.Log("OnDelete ResourceNodeComponent " + _flagForDeath);
	}

	public void OnUpdate(ResourceNodeComponent oldValue, ResourceNodeComponent newValue, ReducerEvent reducerEvent)
	{
        if (oldValue.EntityId == EntityId)
        {
            Debug.Log($"OnResourceUpdated {EntityId} : {Health}");
            if (Health == 1)
            {
                // this means the resource will be destroyed on the next strike
                _flagForDeath = true;
            }
        }
    }

	public void Impact(Vector3 actorPosition)
	{
		if (_flagForDeath)
		{
			if (_deathVfx != null)
			{
				_deathVfx.transform.SetParent(null, true);
				_deathVfx.SetActive(true);
			}
			StartCoroutine(KillNextFrame());
			return;
		}

		if (_vfx != null)
		{
			_vfx.gameObject.SetActive(true);
			Vector3 forward = actorPosition - transform.position;
			forward.y = 0f;
			_vfx.transform.forward = forward;
		}
	}

	IEnumerator KillNextFrame()
	{
		yield return 0;
		Destroy(gameObject);
	}
}
