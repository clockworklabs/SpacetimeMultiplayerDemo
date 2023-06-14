using UnityEngine;

[RequireComponent(typeof(ParticleSystem))]
public class AutoDeactivate : MonoBehaviour
{
    private ParticleSystem _particleSystem;

    [SerializeField] private bool _destroyModel = false;

    [SerializeField] private float _minDuration;
    private float _destroyTimestamp;

    private void OnEnable()
    {
        _particleSystem = GetComponent<ParticleSystem>();
        if (_minDuration > 0f)
        {
            _destroyTimestamp = Time.realtimeSinceStartup + _minDuration;
        }
    }

    private void Update()
    {
        if (_destroyTimestamp > 0f && Time.realtimeSinceStartup < _destroyTimestamp)
        {
            return;
        }

        bool end = !_particleSystem.isPlaying;
        if (end)
		{
            if (_destroyModel)
            {
                Destroy(this.gameObject);
            }
            else
            {
                _particleSystem.Stop();
                gameObject.SetActive(false);
            }
        }
    }
}