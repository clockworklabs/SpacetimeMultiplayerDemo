using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using TMPro;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.UI;
using UnityEngine.UIElements;
using Button = UnityEngine.UI.Button;

public class UIChatController : Singleton<UIChatController>
{
    [SerializeField] private ScrollRect _scrollRect;
    [SerializeField] private TMP_InputField _chatInput;
    [SerializeField] private TMP_Text _messages;
    
    // Start is called before the first frame update
    void Start()
    {
        _chatInput.text = "";
        _messages.text = "";
        _chatInput.onSubmit.AddListener(arg0 =>
        {
            OnChatButtonPress();
        });
        PlayerMovementController.localMovementDisabled.Add(() => EventSystem.current.currentSelectedGameObject == _chatInput.gameObject);
        Hide();
        enabled = false;
        NetworkPlayer.OnLocalPlayerInitialized += () => enabled = true;
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

    // TODO: this redraws the whole scroll layout each time, causing a visible redraw that is ugly
    // TODO: this doesn't seem to work on initial chat load.
    private IEnumerator AutoScroll()
    {
        // Wait for end of frame and force update all canvases before setting to bottom.
        yield return new WaitForEndOfFrame();
        Canvas.ForceUpdateCanvases();
        // TODO: magic number here is because "0" was leaving the bottom message invisible.
        _scrollRect.verticalNormalizedPosition = -10;
        Canvas.ForceUpdateCanvases();
    }
    
    public void OnChatMessageReceived(ulong playerId, String message)
    {
        var player = PlayerComponent.FilterByEntityId(playerId);
        if (player != null)
        {
            _messages.text += $"{player.username} says, \"{message}\"\n";

            // Force scroll to bottom.
            if (gameObject.activeSelf)
            {
                StartCoroutine(AutoScroll());
            }   
        }
    }
    
    public void OnChatButtonPress()
    {
        Reducer.PlayerChat(NetworkPlayer.localPlayerId.Value, _chatInput.text);
        _chatInput.text = "";
    }
}
