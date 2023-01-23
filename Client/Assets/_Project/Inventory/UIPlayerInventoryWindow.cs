using SpacetimeDB;

public class UIPlayerInventoryWindow : UIInventoryWindow
{
    public static UIPlayerInventoryWindow instance { get; private set; }

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

	protected override void CallReducer(ulong playerEntityId, UIInventorySlot source, UIInventorySlot dest)
	{
        if (source.Owner == this)
        {
            // Move from this inventory to this inventory
            Reducer.MoveOrSwapInventorySlot(playerEntityId, InventoryEntityId, source.GetPocketIdx().Value, dest.GetPocketIdx().Value);
        }
        else
        {
            // Move from Offer to this inventory
            if (source.Owner == UITradeLocalOfferWindow.instance)
            {
                Reducer.RemoveFromTrade(playerEntityId, source.Index, dest.Index);
            }
        }
    }
}
