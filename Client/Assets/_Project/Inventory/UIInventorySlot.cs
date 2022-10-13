using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEditor;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.Rendering.UI;
using UnityEngine.UI;
using Debug = System.Diagnostics.Debug;

public class UIInventorySlot : MonoBehaviour, IPointerDownHandler
{
    [SerializeField] private Image _itemImage;
    [SerializeField] private TMPro.TextMeshProUGUI _itemQuantity;

    private bool _displaying;
    private uint? _pocketIdx;
    private bool _locked;

    public UIInventoryWindow Owner => _owner;
    public bool Locked => _locked;
    public uint Index => _pocketIdx.Value;

    private Action<UIInventorySlot, UIInventorySlot> _callback;

    private UIInventoryWindow _owner;

    public void Configure(uint? pocketIdx, Action<UIInventorySlot, UIInventorySlot> callback, UIInventoryWindow owner, bool locked = false)
    {
        ItemDragController.instance.AddDropTarget(this);
        _callback = callback;
        _pocketIdx = pocketIdx;
        _owner = owner;
        _locked = locked;
    }

    public void Display(ItemAsset item, int quantity)
    {
        _displaying = true;
        _itemImage.sprite = item.sprite;
        _itemImage.enabled = true;
        _itemQuantity.text = quantity + "";
    }

    public void Display(Pocket pocket)
    {
        _displaying = true;
        var item = AssetRegistry.singleton.GetAsset(pocket.itemId) as ItemAsset;
        Debug.Assert(item != null, nameof(item) + " != null");
        _itemImage.sprite = item.sprite;
        _itemImage.enabled = true;
        _itemQuantity.text = pocket.itemCount + "";
    }

    public void Display(UIInventorySlot slot)
    {
        if (slot._displaying)
        {
            // Copy values from the slot
            _displaying = true;
            _itemImage.sprite = slot._itemImage.sprite;
            _itemImage.enabled = true;
            _itemQuantity.text = slot._itemQuantity.text;
        }
        else
        {
            Clear();
        }
    }

    public void Clear()
    {
        _displaying = false;
        _itemImage.enabled = false;
        _itemQuantity.text = "";
    }

    public bool IsDisplayingItem() => _displaying;
    public void OnPointerDown(PointerEventData eventData)
    {
        ItemDragController.instance.OnInventorySlotPressed(this);
    }

    public uint? GetPocketIdx() => _pocketIdx;
}
