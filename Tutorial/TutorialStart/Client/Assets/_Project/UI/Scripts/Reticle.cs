using UnityEngine.UI;
using UnityEngine;
using System.Linq;

public class Reticle : MonoBehaviour
{
    [SerializeField] private Image _reticle;

	[SerializeField] private float _detectionRange;

	[SerializeField] private Color _idleColor;
	[SerializeField] private Color _selectedColor;

	private Camera _camera;
	private int _detectionLayer;

	private Transform _localPlayer;

	public static GameObject SelectedTarget { private set; get; }

	private void Start()
	{
		_detectionLayer = (1 << LayerMask.NameToLayer("Resource")) + (1 << LayerMask.NameToLayer("Default"));

		gameObject.SetActive(false);
	}

	void OnPlayerInitialized()
	{
		gameObject.SetActive(true);
		_camera = Camera.main;
		_reticle.enabled = false;
		_localPlayer = PlayerMovementController.Local.transform;
	}

	void Update()
    {
		_reticle.enabled = CameraController.instance.GameCameraEnabled;
		if (CameraController.instance.GameCameraEnabled)
		{
			Ray r = new Ray(_camera.transform.position, _camera.transform.forward);
			var hits = Physics.RaycastAll(r, 100f, _detectionLayer);
			var sortedHits = hits.OrderBy(h => h.distance);
			foreach (var raycastHit in hits)
			{
				var delta = raycastHit.collider.transform.position - _localPlayer.transform.position;
				delta.y = 0f;
				if (delta.sqrMagnitude < _detectionRange * _detectionRange)
				{
					var target = raycastHit.collider.transform.parent.gameObject;

					bool valid = target.GetComponent<GameResource>() != null;
					if (valid)
					{
						_reticle.color = _selectedColor;
						SelectedTarget = target;
					}
					else
					{
						_reticle.color = _idleColor;
						SelectedTarget = null;
					}
					return;
				}
			}
		}
		_reticle.color = _idleColor;
		SelectedTarget = null;
	}
}