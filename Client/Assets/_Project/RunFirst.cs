using System.Collections;
using System.Collections.Generic;
using UnityEngine;


/// <summary>
/// The purpose of this script is that it runs before all other scripts, which helps setup critical
/// singletons that other scripts need during initialization.
/// </summary>
public class RunFirst : MonoBehaviour
{
    [SerializeField] private AssetRegistry registry;
    
    protected void Awake()
    {
        registry.Initialize();
    }
}
