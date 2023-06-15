using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using SpacetimeDB;

public class GameResource : MonoBehaviour
{
	[SerializeField] private GameObject _vfx;
	[SerializeField] private GameObject _deathVfx;

	private bool _flagForDeath;

    public ulong EntityId;

    public ResourceNodeType Type = ResourceNodeType.Iron;
    public int MaxHealth = 0;
    public int Health = 0;

    public void Init()
	{
	}

	private void OnDestroy()
	{
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
