using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class UIFade : MonoBehaviour
{
    private enum StartingState
    {
        Visible,
        Invisible
    }

    private enum State
    {
        FadeIn,
        FadeOut
    }

    [SerializeField] private StartingState _startingState;
    
    private State _state;
    private CanvasGroup _group;
    private float _fadeValue;

    private const float _fadeTime = 0.10f;

    private void Awake()
    {
        _group = GetComponent<CanvasGroup>();
        Debug.Assert(_group != null, "UIFade requires a canvas group!");

        switch (_startingState)
        {
            case StartingState.Invisible:
                _fadeValue = 0;
                _state = State.FadeOut;
                break;
            case StartingState.Visible:
                _fadeValue = 1;
                _state = State.FadeIn;
                break;
        }
        _group.alpha = _fadeValue;
    }

    public bool IsShowing() => _state == State.FadeIn;
    
    public void Toggle()
    {
        switch (_state)
        {
            case State.FadeIn:
                _state = State.FadeOut;
                break;
            case State.FadeOut:
                _state = State.FadeIn;
                break;
        }
    }
    
    public void FadeIn() => _state = State.FadeIn;
    public void FadeOut() => _state = State.FadeOut;

    public void Update()
    {
        switch (_state)
        {
            case State.FadeIn:
                _fadeValue = Mathf.Clamp01(_fadeValue + Time.deltaTime / _fadeTime);
                break;
            case State.FadeOut:
                _fadeValue = Mathf.Clamp01(_fadeValue - Time.deltaTime / _fadeTime);
                break;
        }

        _group.alpha = _fadeValue;
    }
}
