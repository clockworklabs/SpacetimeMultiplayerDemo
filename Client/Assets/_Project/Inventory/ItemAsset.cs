using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[CreateAssetMenu(menuName = "BitCraftMini/ItemAsset")]
public class ItemAsset : RegisteredAsset
{
    public uint itemId;
    public Sprite sprite;

    public static ItemAsset GetItem(uint itemId)
    {
        return AssetRegistry.singleton.GetAsset(itemId) as ItemAsset;
    }

    public override uint GetAssetId() => itemId;
}
