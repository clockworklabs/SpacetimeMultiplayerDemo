using UnityEngine.UI;
using UnityEngine;

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
		_detectionLayer = 1 << LayerMask.NameToLayer("Resource");
		NetworkPlayer.OnLocalPlayerInitialized += OnPlayerInitialized;
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
			if (Physics.Raycast(r, out var raycastHit, 100f,  _detectionLayer))
			{
				var delta = raycastHit.collider.transform.position - _localPlayer.transform.position;
				delta.y = 0f;
				if (delta.sqrMagnitude < _detectionRange * _detectionRange)
				{
					_reticle.color = _selectedColor;
					SelectedTarget = raycastHit.collider.transform.parent.gameObject;
					return;
				}
			}
		}
		_reticle.color = _idleColor;
		SelectedTarget = null;
	}
}