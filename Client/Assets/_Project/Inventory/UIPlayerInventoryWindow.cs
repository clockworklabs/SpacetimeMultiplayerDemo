using SpacetimeDB.Types;

public class UIPlayerInventoryWindow : UIInventoryWindow
{
    public static UIPlayerInventoryWindow instance { get; private set; }

    public ItemAsset[] ItemAssets;

    protected override void Start()
    {
        instance = this;
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
    }
}
