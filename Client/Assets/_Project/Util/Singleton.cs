using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Singleton<T> : MonoBehaviour where T : Singleton<T>
{
    public static T instance;

    protected virtual void Awake()
    {
        if (instance != null)
        {
            Debug.LogWarning($"Instance already exists: {GetType()}");
            return;
        }

        instance = (T)this;
    }
}
