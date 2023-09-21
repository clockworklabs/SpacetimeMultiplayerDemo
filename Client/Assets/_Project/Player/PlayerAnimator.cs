using UnityEngine;
using System.Collections;
using SpacetimeDB.Types;
using System.Linq;

public class PlayerAnimator : MonoBehaviour
{

    [SerializeField] private GameObject[] _tools;
    [SerializeField] private Transform _playerRoot;

    public bool Interacting { get; private set; }

    private Animator _animator;

    private GameResource _target;

    public static PlayerAnimator Local { get; set; }

    public bool Grounded { get { return grounded; } }
    private bool grounded = true;

    public System.Action<bool> OnInteractionUpdate;


    private void Start()
	{
		_animator = GetComponent<Animator>();
        PlayerMovementController.localMovementDisabled.Add(() => Interacting);
    }

    private void Update()
    {
        var y = MathUtil.GetTerrainHeight(transform.position);
        if(!grounded && GetComponentInParent<PlayerMovementController>(true).VerticalVelocity < 0.0f && ((transform.position.y - 2) - y <= 0.1f))
        {
            grounded = true;
            _animator.SetBool("Grounded", true);
        }        
    }

    public void Interact(GameResource res)
    {
        if (res == null)
        {
            Interacting = false;
            for (int i = 0; i < _tools.Length; i++)
            {
                _tools[i].SetActive(false);
            }
            _target = null;
        }
        else
        {
            var resourceType = res?.Type ?? ResourceNodeType.Iron;
            switch (resourceType)
            {
                case ResourceNodeType.Iron:
                    _animator.SetTrigger("Mine");
                    Interacting = true;
                    break;
                default:
                    Interacting = false;
                    break;
            }
            for (int i = 0; i < _tools.Length; i++)
            {
                _tools[i].SetActive(((int)resourceType) == i);
            }
            _target = res;
        }
    }

    public void OnStartAction()
    {
        if (_target != null)
        {
            OnInteractionUpdate?.Invoke(true);
            StartCoroutine(FaceTarget());
        }
    }

    public void OnActionImpact()
    {
        _target.Impact(transform.position);
    }

    public void OnEndAction()
    {
        Interact(null);
        OnInteractionUpdate?.Invoke(false);
    }

    IEnumerator FaceTarget()
	{
        Vector3 start = _playerRoot.transform.forward;
        Vector3 end = _target.transform.position - _playerRoot.transform.position;
        end.y = start.y;

        float t = 0f;
        while (t < 0.3f)
        {
            t += Time.deltaTime;
            _playerRoot.transform.forward = Vector3.Lerp(start, end, t / 0.3f);
            yield return 0;
        }
    }

    public void SetRemoteAction(ulong actionTargetEntityId)
    {
        var res = DemoGameManager.instance.GameResources.FirstOrDefault(item => item.EntityId == actionTargetEntityId);
        if (res != null)
        {
            Interact(res);
        }
    }

    public void Jump()
    {
        if(!grounded)
        {
            return;
        }

        _animator.SetTrigger("Jump");
        _animator.SetBool("Grounded", false);
        grounded = false;
    }
}