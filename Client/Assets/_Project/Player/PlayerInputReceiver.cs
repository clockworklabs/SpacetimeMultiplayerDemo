using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerInputReceiver : MonoBehaviour
{
    private bool cameraMouseButtonDown;
    private Vector2 mouseDelta;
    private Vector2 pointerPosition;
    private Vector2 lastPointerPosition;
    private float zoomDelta;

    void OnMove(InputValue value)
    {
        PlayerMovementController.instance.SetMove(value.Get<Vector2>());
    }

    void OnPointerPosition(InputValue value)
    {
        pointerPosition = value.Get<Vector2>();
    }

    void OnCameraMouseButton(InputValue value)
    {
        cameraMouseButtonDown = value.isPressed;
    }

    void OnZoom(InputValue value)
    {
        zoomDelta = value.Get<float>();
    }
    
    private void Update()
    {
        if (!CameraController.HasInstance())
        {
            return;
        }
        mouseDelta = pointerPosition - lastPointerPosition;
        lastPointerPosition = pointerPosition;
        CameraController.instance.SetMouseDelta(cameraMouseButtonDown ? mouseDelta : Vector2.zero);
        CameraController.instance.SetZoomDelta(zoomDelta);

        zoomDelta = 0.0f;
    }
}
