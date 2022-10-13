using SpacetimeDB;
using UnityEngine;
using UnityEngine.UI;

public class UITradeLocalOfferWindow : UIInventoryWindow
{
    public static UITradeLocalOfferWindow instance { get; private set; }

    [SerializeField] private Image _checkmark;

    protected override void Start()
    {
        instance = this;
        NetworkPlayer.OnLocalPlayerInitialized += () => enabled = true;
        base.Start();
    }

    protected override void OnDestroy()
    {
        base.OnDestroy();
        instance = null;
    }

    protected override void CallReducer(uint playerEntityId, UIInventorySlot source, UIInventorySlot dest)
    {
        if (source.Owner == this)
        {
            // Move from this inventory to this inventory
            Reducer.MoveOrSwapInventorySlot(playerEntityId, InventoryEntityId, source.GetPocketIdx().Value, dest.GetPocketIdx().Value);
        }
        else
        {
            if (source.Owner == UIPlayerInventoryWindow.instance)
            {
                // Move from player inventory to this inventory
                Reducer.AddToTrade(playerEntityId, source.Index, dest.Index);
            }
            else
            {
                Debug.LogError("Where does this item come from?");
            }
		}
	}

	public void OnDecline()
	{
        Reducer.RefuseTrade(NetworkPlayer.localPlayerId.Value);
	}

	public void OnAccept()
	{
        Reducer.ToggleAcceptTrade(NetworkPlayer.localPlayerId.Value);
	}

	public void SetCheckmark(bool enabled)
	{
        _checkmark.enabled = enabled;
    }



}
