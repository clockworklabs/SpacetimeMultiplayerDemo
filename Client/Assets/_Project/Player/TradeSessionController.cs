using System.Collections;
using System.Collections.Generic;
using System.Linq;
using SpacetimeDB;
using UnityEngine;

public class TradeSessionController : MonoBehaviour
{

    public static TradeSessionController Local;

    private ulong _localOfferEntityId;
    private ulong _remoteOfferEntityId;
    private ulong _sessionEntityId;

    private readonly Dictionary<uint, Pocket> _localPockets = new Dictionary<uint, Pocket>();
    private readonly Dictionary<uint, Pocket> _remotePockets = new Dictionary<uint, Pocket>();

    private int _tradingSlots;

	private void Start()
	{
        Local = this;
	}

	public void Spawn()
    {
        var config = Config.FilterByVersion(0);
        Debug.Assert(config != null, "Server config missing!");
        _tradingSlots = (int)config.TradingSlots;
        UITradeLocalOfferWindow.instance.CreateSlots(_tradingSlots, false);
        UITradeRemoteOfferWindow.instance.CreateSlots(_tradingSlots, true);

        // Reset the inventory window to default state
        InventoryUpdate(null);
	}

	public void Initiate(ulong sessionEntityId, ulong localOfferEntityId, ulong remoteOfferEntityId)
	{
        _sessionEntityId = sessionEntityId;

        UITradeLocalOfferWindow.instance.InventoryEntityId = localOfferEntityId;
        UITradeRemoteOfferWindow.instance.InventoryEntityId = remoteOfferEntityId;

        _remoteOfferEntityId = remoteOfferEntityId;
        _localOfferEntityId = localOfferEntityId;
        UITradeLocalOfferWindow.instance.Show();
        UITradeLocalOfferWindow.instance.SetCheckmark(false);
        UITradeRemoteOfferWindow.instance.Show();
        UITradeRemoteOfferWindow.instance.SetCheckmark(false);
        UIPlayerInventoryWindow.instance.Show();

        // Clear slots on a new session
        for (var x = 0; x < _tradingSlots; x++)
        {
            var slot = UITradeLocalOfferWindow.instance.GetSlot(x);
            slot.Clear();
            slot = UITradeRemoteOfferWindow.instance.GetSlot(x);
            slot.Clear();
		}
	}

	public void UpdateSession(ulong sessionEntityId)
	{
		if (sessionEntityId != _sessionEntityId)
		{
            // Someone else trade session. Ignore.
            return;
		}

		var session = TradeSessionComponent.FilterByEntityId(_sessionEntityId);

        // update checkboxes
        var isInitiator = session.InitiatorEntityId == NetworkPlayer.localPlayerId.Value;
        UITradeLocalOfferWindow.instance.SetCheckmark(isInitiator ? session.ApprovedByInitiator : session.ApprovedByAcceptor);
        UITradeRemoteOfferWindow.instance.SetCheckmark(isInitiator ? session.ApprovedByAcceptor : session.ApprovedByInitiator);
    }


    public void Terminate(bool successful)
	{
        _remoteOfferEntityId = 0;
        _localOfferEntityId = 0;
        UITradeLocalOfferWindow.instance.Hide();
        UITradeRemoteOfferWindow.instance.Hide();
        if (!successful) // successful is always false because the update gets discarded by the delete happening in the same transaction.
        {
            UIPlayerInventoryWindow.instance.Hide();
        }
    }

    public bool InventoryUpdate(InventoryComponent inventory)
    {
        if (Local != this || inventory == null)
        {
            return false;
        }

        Dictionary<uint, Pocket> pockets;
        UIInventoryWindow ui;
        if (inventory.EntityId == _localOfferEntityId)
        {
            pockets = _localPockets;
            ui = UITradeLocalOfferWindow.instance;
        }
        else if (inventory.EntityId == _remoteOfferEntityId)
        {
            pockets = _remotePockets;
            ui = UITradeRemoteOfferWindow.instance;
        }
        else
        {
            // Updated inventory is not part of this trade session window
            return false;
        }

        pockets.Clear();
        foreach (var pocket in inventory.Pockets)
        {
            pockets[pocket.PocketIdx] = pocket;
        }

        for (var x = 0; x < _tradingSlots; x++)
        {
            var slot = ui.GetSlot(x);
            if (pockets.TryGetValue((uint)x, out var pocket))
            {
                slot.Display(pocket);
            }
            else
            {
                slot.Clear();
            }
        }
    
        return true;
    }
}
