using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraController : Singleton<CameraController>
{
    [SerializeField] private Transform attackTransform;
    [SerializeField] private Transform cameraTransform;
    [SerializeField] private float rotationSpeed;
    [SerializeField] private float attackSpeed;
    [SerializeField] private float zoomSpeed;
    [SerializeField] private float initialZoom = -4f;
    [SerializeField] private float initialAttack = -20;
    [SerializeField] private Vector2 minMaxAttack;
    [SerializeField] private Vector2 minMaxZoom;

    [SerializeField] private Transform target;
    [SerializeField] private Vector3 offset;

    private Camera _camera;

    private Vector2 mouseDelta;
    private float zoomDelta;
    private float attack;
    private float currentZoom;

    public bool GameCameraEnabled { get { return _disablers.Count == 0; } }

    public static void AddDisabler(int hash)
	{
        _disablers.Add(hash);
	}

	public static void RemoveDisabler(int hash)
	{
        _disablers.Remove(hash);
	}

    public static bool HasDisabler(int hash)
    {
        return _disablers.Contains(hash);
    }

    private static HashSet<int> _disablers = new HashSet<int>();


	private void Start()
	{
        currentZoom = initialZoom;
        attack = initialAttack;
        transform.parent = null;
        _camera = Camera.main;
    }

	public void SetMouseDelta(Vector2 delta)
    {
        mouseDelta = delta;
    }
    
    public void SetZoomDelta(float delta)
    {
        zoomDelta = delta;
	}

	public void IncreaseAttack(float delta)
	{
        attack = Mathf.Clamp(attack += delta * attackSpeed * Time.deltaTime, minMaxAttack.x, minMaxAttack.y);
        attackTransform.localRotation = Quaternion.Euler(-attack, 0.0f, 0.0f);
    }
	
    void LateUpdate()
	{
        Cursor.lockState = GameCameraEnabled ? CursorLockMode.Locked : CursorLockMode.None;
        if (!GameCameraEnabled)
		{
            return;
		}
		
        Vector3 forward = _camera.transform.forward;
        forward.y = 0f;
        forward.Normalize();

        Vector3 right = _camera.transform.right;
        right.y = 0f;
        right.Normalize();

        Vector3 up = Vector3.up;

        transform.position = target.position + forward * offset.x + up * offset.y + right * offset.z;
        transform.Rotate(Vector3.up, rotationSpeed * Time.deltaTime * mouseDelta.x);
        attack = Mathf.Clamp(attack += mouseDelta.y * attackSpeed * Time.deltaTime, minMaxAttack.x, minMaxAttack.y);
        attackTransform.localRotation = Quaternion.Euler(-attack, 0.0f, 0.0f);
        currentZoom = Mathf.Clamp(currentZoom + zoomDelta * Time.deltaTime * zoomSpeed, minMaxZoom.x, minMaxZoom.y);
        cameraTransform.localPosition = new Vector3(0.0f, 0.0f, currentZoom);

        _camera.transform.forward = transform.position - _camera.transform.position;
    }
}
