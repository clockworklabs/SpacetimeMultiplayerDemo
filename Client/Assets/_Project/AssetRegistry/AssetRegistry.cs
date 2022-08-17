using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;

public class AssetRegistry : ScriptableObject
{
    [SerializeField] private RegisteredAsset[] allAssets;

    public static AssetRegistry singleton;

    private Dictionary<uint, RegisteredAsset> lookupTable = new Dictionary<uint, RegisteredAsset>();

    public void Initialize()
    {
        singleton = this;
        lookupTable.Clear();

        foreach (var asset in allAssets)
        {
            if (asset == null)
            {
                continue;
            }

            var assetId = asset.GetAssetId();
            if (lookupTable.TryGetValue(assetId, out var dupAsset))
            {
                Debug.LogError($"Assets have the same ID: {asset.name}, {dupAsset.name}");
                continue;
            }

            lookupTable[assetId] = asset;
        }
    }

    /// <summary>
    /// Returns the asset with the given asset ID.
    /// </summary>
    /// <param name="assetId"></param>
    /// <returns></returns>
    public RegisteredAsset GetAsset(uint assetId)
    {
        if (lookupTable.TryGetValue(assetId, out var foundAsset))
        {
#if UNITY_EDITOR
            if (foundAsset.GetAssetId() == assetId)
            {
                return foundAsset;
            }
#else
            return asset;
#endif
        }

        // We want to allow the programmer to add new assets while the game is running (or change IDs), so
        // we will try the expensive lookup now
        foreach(var asset in allAssets)
        {
            if (asset.GetAssetId() == assetId)
            {
                return asset;
            }
        }

        Debug.LogWarning($"We did not find an asset with this ID: {assetId}");
        return null;
    }
}