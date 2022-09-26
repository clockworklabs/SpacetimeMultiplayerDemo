using System.Collections;
using System.Collections.Generic;
using System.Linq;
using SpacetimeDB;
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
        _maxInventorySlots = (int)config.maxPlayerInventorySlots;
        UIInventoryWindow.instance.CreateSlots(_maxInventorySlots);
        
        // Reset the inventory window to default state
        InventoryUpdate(null);

        // Give starting items if inventory is empty
        var inv = InventoryComponent.FilterByEntityId(NetworkPlayer.localPlayerId.Value);
        bool isEmpty = true;
        foreach (var p in inv.pockets)
        {
            isEmpty &= p.itemId == 0 || p.itemCount == 0;
        }

        if (isEmpty)
        {
            var pocketIdx = 0;
            foreach (var item in startingItems)
            {
                Debug.Log($"Starting item: {item.item.name}, amount: {item.amount}");
                Debug.Assert(NetworkPlayer.localPlayerId != null, "NetworkPlayer._localPlayerId != null");
                Reducer.AddItemToInventory(NetworkPlayer.localPlayerId.Value, item.item.itemId, pocketIdx, (int)item.amount);
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
            foreach (var pocket in inventory.pockets)
            {
                _pockets[pocket.pocketIdx] = pocket;
            }    
        }

        for (var x = 0; x < _maxInventorySlots; x++)
        {
            var slot = UIInventoryWindow.instance.GetSlot(x);
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
