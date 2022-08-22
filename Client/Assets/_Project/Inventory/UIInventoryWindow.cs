using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;

public class UIInventoryWindow : Singleton<UIInventoryWindow>
{
    [SerializeField] private UIInventorySlot _slotPrefab;
    [SerializeField] private RectTransform _slotsHierarchy;

    private bool _slotsCreated;
    private List<UIInventorySlot> _slots = new List<UIInventorySlot>();

    public void CreateSlots(int maxSlotCount)
    {
        if (_slotsCreated)
        {
            return;
        }

        _slotsCreated = true;
        ItemDragController.instance.DragDropCompleted += DragDropCompleted;

        for (var x = 0; x < maxSlotCount; x++)
        {
            var slot = Instantiate(_slotPrefab, _slotsHierarchy);
            slot.Configure((uint)x, DragDropCompleted);
            _slots.Add(slot);
        }
    }

    void DragDropCompleted(UIInventorySlot source, UIInventorySlot dest)
    {
        var sourcePocketIdx = source.GetPocketIdx();
        var destPocketIdx = dest.GetPocketIdx();
        if (!sourcePocketIdx.HasValue || !destPocketIdx.HasValue)
        {
            return;
        }

        Debug.Assert(NetworkPlayer.localPlayerId != null, "Local player not set!");
        Reducer.MoveOrSwapInventorySlot(NetworkPlayer.localPlayerId.Value, sourcePocketIdx.Value, destPocketIdx.Value);
    }

    public UIInventorySlot GetSlot(int idx)
    {
        Debug.Assert(idx >= 0 && idx < _slots.Count, $"Invalid slot idx: {idx}");
        return _slots[idx];
    }
}
