using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class LocalPlayer : MonoBehaviour
{
    [SerializeField] private GameObject cameraRig;

    public static LocalPlayer instance;
    public string username;

    private Vector2 movementVec;

    // Start is called before the first frame update
    void Start()
    {
        instance = this;
        cameraRig.SetActive(true);
        PlayerMovementController.Local = GetComponent<PlayerMovementController>();
        PlayerAnimator.Local = GetComponentInChildren<PlayerAnimator>(true);        
    }

    private void FixedUpdate()
    {
        var directionVec = GetDirectionVec();
        PlayerMovementController.Local.DirectionVec = directionVec;
    }

    public void SetMove(UnityEngine.Vector3 vec) => movementVec = vec;

    public Vector3 GetDirectionVec()
    {
        var vec = new Vector3(movementVec.x, 0, movementVec.y);
        return CameraController.instance.transform.TransformDirection(vec);
    }
}
