using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class HeightController: MonoBehaviour
{
    [SerializeField] private Transform _reference;
    [SerializeField] private float _minHeight;
	[SerializeField] private float _minDistance;

    private Vector3 _lastPos;


	private void Start()
	{
		_lastPos = transform.position;
	}

	void LateUpdate()
	{
		if (_lastPos != transform.position)
		{
			_lastPos = transform.position;
			var min = _reference.transform.position.y + _minHeight;
			transform.localPosition = Vector3.zero;
			var delta = Mathf.Max(0f, min - transform.position.y);
			if (delta > 0f)
			{
				var vec = _reference.position - transform.position;
				vec.y = 0f;
				if (vec.magnitude - delta < _minDistance)
				{
					delta = vec.magnitude - _minDistance;
				}
				vec.Normalize();
				var newPos = transform.position;
				newPos.y = min;
				newPos += vec * delta;
				transform.position = newPos;
			}
		}
	}
}
