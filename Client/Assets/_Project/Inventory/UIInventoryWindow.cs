using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;

public abstract class UIInventoryWindow : MonoBehaviour
{

    [SerializeField] private UIInventorySlot _slotPrefab;
    [SerializeField] private RectTransform _slotsHierarchy;

    private bool _slotsCreated;
    private List<UIInventorySlot> _slots = new List<UIInventorySlot>();

    public ulong InventoryEntityId { get; set; }

    protected abstract void CallReducer(ulong playerEntityId, UIInventorySlot source, UIInventorySlot dest);

    protected virtual void Start()
	{
        enabled = false;
    }

    protected virtual void OnDestroy()
	{
	}

	public void Show()
    {
        if (enabled)
        {
            GetComponent<UIFade>().FadeIn();
            CameraController.AddDisabler(GetHashCode());
        }
    }

    public void Hide()
    {
        if (enabled)
        {
            GetComponent<UIFade>().FadeOut();
            CameraController.RemoveDisabler(GetHashCode());
        }
    }

    public void Toggle()
    {
        if (GetComponent<UIFade>().IsShowing())
        {
            Hide();
        }
        else
        {
            Show();
        }
    }

    public void CreateSlots(int maxSlotCount, bool locked)
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
            slot.Configure((uint)x, DragDropCompleted, this, locked);
            _slots.Add(slot);
        }
    }

    void DragDropCompleted(UIInventorySlot source, UIInventorySlot dest)
    {
        if (dest.Owner != this)
        {
            return;
        }

        if (dest.Locked)
        {
            return;
        }

        var sourcePocketIdx = source.GetPocketIdx();
        var destPocketIdx = dest.GetPocketIdx();
        if (!sourcePocketIdx.HasValue || !destPocketIdx.HasValue)
        {
            return;
        }
        // Prevent drag and drop on itself
        if (dest.Owner == source.Owner && sourcePocketIdx.Value == destPocketIdx.Value)
        {
            return;
        }

        Debug.Assert(LocalPlayer.instance != null, "Local player not set!");
        CallReducer(LocalPlayer.instance.EntityId, source, dest);
	}

	public UIInventorySlot GetSlot(int idx)
    {
        Debug.Assert(idx >= 0 && idx < _slots.Count, $"Invalid slot idx: {idx}");
        return _slots[idx];
    }
}
