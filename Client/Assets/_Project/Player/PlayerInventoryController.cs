using System.Collections;
using System.Collections.Generic;
using System.Linq;
using SpacetimeDB.Types;
using UnityEngine;

public class PlayerInventoryController : MonoBehaviour
{
    [System.Serializable]
    public struct StartingItem
    {
        public ItemAsset item;
        public uint amount;
    }

    [SerializeField] private StartingItem[] startingItems;

    public static PlayerInventoryController Local;

    private readonly Dictionary<uint, Pocket> _pockets = new Dictionary<uint, Pocket>();

    private int _maxInventorySlots;

    public void Spawn()
    {
        Local = this;
        var config = Config.FilterByVersion(0);
        Debug.Assert(config != null, "Server config missing!");
        _maxInventorySlots = (int)config.MaxPlayerInventorySlots;
        UIPlayerInventoryWindow.instance.CreateSlots(_maxInventorySlots, false);
        UIPlayerInventoryWindow.instance.InventoryEntityId = LocalPlayer.instance.EntityId;
        // Reset the inventory window to default state
        InventoryUpdate(null);

        // Give starting items if inventory is empty
        var inv = InventoryComponent.FilterByEntityId(LocalPlayer.instance.EntityId);
        bool isEmpty = true;
        foreach (var p in inv.Pockets)
        {
            isEmpty &= p.ItemId == 0 || p.ItemCount == 0;
        }

        if (isEmpty)
        {
            var pocketIdx = 0;
            foreach (var item in startingItems)
            {
                Debug.Log($"Starting item: {item.item.name}, amount: {item.amount}");
                Debug.Assert(LocalPlayer.instance != null, "NetworkPlayer._localPlayerId != null");
                Reducer.AddItemToInventory(LocalPlayer.instance.EntityId, item.item.itemId, pocketIdx, (int)item.amount);
                pocketIdx++;
            }
        }
    }
    
    public void InventoryUpdate(InventoryComponent inventory)
    {
        if (Local != this)
        {
            return;
        }
        
        _pockets.Clear();
        if (inventory != null)
        {
            foreach (var pocket in inventory.Pockets)
            {
                _pockets[pocket.PocketIdx] = pocket;
            }    
        }

        for (var x = 0; x < _maxInventorySlots; x++)
        {
            var slot = UIPlayerInventoryWindow.instance.GetSlot(x);
            if (_pockets.TryGetValue((uint)x, out var pocket))
            {
                slot.Display(pocket);
            }
            else
            {
                slot.Clear();
            }
        }
    }
}
