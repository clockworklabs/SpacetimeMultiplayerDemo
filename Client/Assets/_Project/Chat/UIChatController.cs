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
        PlayerMovementController.LocalMovementDisabled.Add(() => EventSystem.current.currentSelectedGameObject == _chatInput.gameObject);
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
    
    public void OnChatMessageReceived(uint playerId, String message)
    {
        _messages.text += $"{playerId} says, \"{message}\"\n";

        // Force scroll to bottom.
        if (gameObject.activeSelf)
        {
            StartCoroutine(AutoScroll());
        }
    }
    
    public void OnChatButtonPress()
    {
        Reducer.PlayerChat(NetworkPlayer._localPlayerId.Value, _chatInput.text);
        _chatInput.text = "";
    }
}
