using SpacetimeDB;
using System.Collections;
using UnityEngine;

public class Npc : MonoBehaviour
{
    [SerializeField] private Animator _animator;

    private uint? _entityId;
    private Vector3 _targetPosition;
    private Quaternion _targetRotation;
    private ulong _lastTimeStamp;

    Coroutine _lerpRotation;
    Coroutine _lerpPosition;

    private void Awake()
    {
        BitCraftMiniGameManager.instance.messageSendTick += GameTick;
        // We can't realistically call OnEvent and de-jsonify the events in real-time for every animation or position update for the npcs
    }

	private void OnDestroy()
    {
        BitCraftMiniGameManager.instance.messageSendTick -= GameTick;
    }
    
    public void Spawn(uint entityId)
    {
        _entityId = entityId;
        gameObject.name = $"Npc - {entityId}";

        var entityTransform = TransformComponent.FilterByEntityId(entityId);
        if (entityTransform != null)
        {
            transform.position = entityTransform.pos.ToVector3();
            transform.rotation = entityTransform.rot.ToQuaternion();
            _targetRotation = transform.rotation;
            _targetPosition = transform.position;
        }
        else
        {
            Debug.LogWarning($"No transform for identity: {entityId}");
        }

        var npc = NpcComponent.FilterByEntityId(entityId);
        if (npc != null)
        {
            _lastTimeStamp = npc.nextAction;
        }
    }

    void GameTick()
    {
        // Note: registering to table update will likely trigger too often.
        // ToDo: register to TransformComponent updates?
        if (!_entityId.HasValue)
        {
            return;
        }
        var entityId = _entityId.Value;
        var npc = NpcComponent.FilterByEntityId(entityId);
        if (npc == null)
        {
            return;
        }

        var entityTransform = TransformComponent.FilterByEntityId(entityId);
        if (entityTransform != null)
        {
            var newRotation = entityTransform.rot.ToQuaternion();
            if (newRotation != _targetRotation)
            {
                _targetRotation = newRotation;
                if (_lerpRotation != null)
                {
                    StopCoroutine(_lerpRotation);
                }
                _lerpRotation = StartCoroutine(LerpRotation(transform.rotation, _targetRotation, npc.nextAction - _lastTimeStamp));    
            }

            var newPosition = entityTransform.pos.ToVector3();
            if (newPosition != _targetPosition)
            {
                _targetPosition = newPosition;
                if (_lerpPosition != null)
                {
                    StopCoroutine(_lerpPosition);
                }
                _lerpPosition = StartCoroutine(LerpPosition(transform.position, _targetPosition));
            }
        }

        var entityAnimation = AnimationComponent.FilterByEntityId(entityId);
        if (entityAnimation != null)
        {
            // Move animation is handled by LerpPosition.
            // ToDo: Npc actions.
        }
        _lastTimeStamp = npc.nextAction;
	}

	IEnumerator LerpRotation(Quaternion start, Quaternion end, ulong duration)
	{
        float t = 0f;
        float dur = (float)duration / 1000000f;
		while (t < dur)
		{
            t += Time.deltaTime;
            transform.rotation = Quaternion.Lerp(start, end, t / dur);
            yield return 0;
        }
	}

	IEnumerator LerpPosition(Vector3 start, Vector3 end)
    {
        _animator.SetFloat("Forward", 1);
        float t = 0f;
        float dur = 0.15f;
        while (t < dur)
        {
            t += Time.deltaTime;
            transform.position = Vector3.Lerp(start, end, t / dur);
            yield return 0;
        }
        _animator.SetFloat("Forward", 0);
    }

}