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
    [SerializeField] private Vector2 minMaxAttack;
    [SerializeField] private Vector2 minMaxZoom;

    private Vector2 mouseDelta;
    private float zoomDelta;
    private float attack;
    private float currentZoom;

    public void SetMouseDelta(Vector2 delta)
    {
        mouseDelta = delta;
    }
    
    public void SetZoomDelta(float delta)
    {
        zoomDelta = delta;
    }

    void Update()
    {
        transform.Rotate(Vector3.up, rotationSpeed * Time.deltaTime * mouseDelta.x);
        attack = Mathf.Clamp(attack += mouseDelta.y * attackSpeed * Time.deltaTime, minMaxAttack.x, minMaxAttack.y);
        attackTransform.localRotation = Quaternion.Euler(-attack, 0.0f, 0.0f);
        currentZoom = Mathf.Clamp(zoomDelta * Time.deltaTime * zoomSpeed, minMaxZoom.x, minMaxZoom.y);
        cameraTransform.localPosition = new Vector3(0.0f, 0.0f, currentZoom);
    }
}
