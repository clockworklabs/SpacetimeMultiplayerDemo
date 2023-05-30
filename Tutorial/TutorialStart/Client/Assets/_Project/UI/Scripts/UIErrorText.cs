using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

public class UIErrorText : MonoBehaviour
{
    [SerializeField] private float _errorShowTime = 5;

    private float _errorShowStartTime;
    private bool _showing;

    private void OnEnable()
    {
        var text = GetComponent<TMP_Text>();
        text.enabled = false;
    }
    
    public void ShowError(string error)
    {
        _errorShowStartTime = Time.time;
        _showing = true;
        var text = GetComponent<TMP_Text>();
        text.text = error;
        text.enabled = true;
    }

    void Update()
    {
        if (_showing && Time.time > _errorShowTime + _errorShowStartTime)
        {
            _showing = false;
            var text = GetComponent<TMP_Text>();
            text.text = "";
            text.enabled = false;
        }
    }
}