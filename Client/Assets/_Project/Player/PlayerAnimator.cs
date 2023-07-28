using UnityEngine;
using System.Collections;
using SpacetimeDB.Types;

public class PlayerAnimator : MonoBehaviour
{

    [SerializeField] private GameObject[] _tools;
    [SerializeField] private Transform _playerRoot;

    public bool Interacting { get; private set; }

    private Animator _animator;

    private GameResource _target;

    public static PlayerAnimator Local { get; set; }

    public System.Action<bool> OnInteractionUpdate;


    private void Start()
	{
		_animator = GetComponent<Animator>();
        PlayerMovementController.localMovementDisabled.Add(() => Interacting);
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

}