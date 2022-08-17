using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public abstract class RegisteredAsset : ScriptableObject
{
    /// <summary>
    /// Returns an id that uniquely identifies this asset.
    /// </summary>
    /// <returns>The unique identifier for this asset.</returns>
    public abstract uint GetAssetId();
}
