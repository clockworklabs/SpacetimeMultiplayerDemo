using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GameResource : MonoBehaviour
{

	public uint EntityId { get; private set; }
	public int Type => BitCraftMiniGameManager.instance.GetResourceComponent(EntityId)?.resourceId ?? 0;
	public int MaxHealth => BitCraftMiniGameManager.instance.GetResourceComponent(EntityId)?.maxHealth ?? 0;
	public int Health => BitCraftMiniGameManager.instance.GetResourceComponent(EntityId)?.health ?? 0;

	[SerializeField] private GameObject _vfx;
	[SerializeField] private GameObject _deathVfx;

	private bool _flagForDeath;

	public void Init(uint entityId)
	{
		EntityId = entityId;
		BitCraftMiniGameManager.instance.AssignResourceModel(EntityId, this);
		StartCoroutine(WaitForDespawn());
	}

	private void OnDestroy()
	{
		BitCraftMiniGameManager.OnResourceUpdated -= OnResourceUpdated;
		BitCraftMiniGameManager.instance.AssignResourceModel(EntityId, null);
	}

	private void OnResourceUpdated(uint entityId)
	{
		if (entityId == EntityId)
		{
			if (Health == 0)
			{
				// Note: this will fail if latency is very bad or if extract animation is very short.
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

	IEnumerator WaitForDespawn()
	{
		yield return 0;
		BitCraftMiniGameManager.OnResourceUpdated += OnResourceUpdated;

		if (Health == 0)
		{
			Destroy(gameObject);
		}
	}

	IEnumerator KillNextFrame()
	{
		yield return 0;
		Destroy(gameObject);
	}



}
