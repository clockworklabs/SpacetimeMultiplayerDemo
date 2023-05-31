using System.Collections;
using UnityEngine;

public class Npc : MonoBehaviour
{
    [SerializeField] private Animator _animator;

    private Vector3 _targetPosition;
    private Quaternion _targetRotation;
    private ulong _lastTimeStamp;

    Coroutine _lerpRotation;
    Coroutine _lerpPosition;

    private void Awake()
    {
    }

	private void OnDestroy()
    {
    }
    
    public void Spawn()
    {

    }

    void GameTick()
    {
        
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