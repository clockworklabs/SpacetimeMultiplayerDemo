using UnityEngine;
using UnityEngine.UI;

public class UITradeRemoteOfferWindow : UIInventoryWindow
{
    public static UITradeRemoteOfferWindow instance { get; private set; }

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
        Debug.LogError("These slots should be locked. This should never happen.");
    }
    public void SetCheckmark(bool enabled)
    {
        _checkmark.enabled = enabled;
    }
}
