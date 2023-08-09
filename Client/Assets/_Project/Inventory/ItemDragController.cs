using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.UI;

public class ItemDragController : Singleton<ItemDragController>
{
    [SerializeField] private UIInventorySlot _dragSlot;

    enum State
    {
        WaitingForClick,
        ClickDown,
        Dragging,
    }

    private State _state;
    private const float _dragThreshold = 1.5f;
    private Vector2 _dragOffset;
    private Vector2 _lastClickPoint;
    private UIInventorySlot _sourceSlot;
    private RectTransform canvasRect;

    public event Action<UIInventorySlot, UIInventorySlot> DragDropCompleted;

    private readonly List<UIInventorySlot> _dropTargets = new List<UIInventorySlot>();

    protected override void Awake()
    {
        base.Awake();
        canvasRect = transform as RectTransform;
    }

    public void AddDropTarget(UIInventorySlot slot)
    {
        if (!_dropTargets.Contains(slot))
        {
            _dropTargets.Add(slot);
        }
    }
    
    public void OnInventorySlotPressed(UIInventorySlot slot)
    {
        _lastClickPoint = Mouse.current.position.ReadValue();
        _sourceSlot = slot;
        _state = State.ClickDown;
    }

    private void ResetState()
    {
        _state = State.WaitingForClick;
        _dragSlot.gameObject.SetActive(false);
    }

    private void CompleteDrag()
    {
        var mousePos = Mouse.current.position.ReadValue();
        foreach (var target in _dropTargets)
        {
            var rect = target.transform as RectTransform;
            if (RectTransformUtility.RectangleContainsScreenPoint(rect, mousePos))
            {
                DragDropCompleted?.Invoke(_sourceSlot, target);
                break;
            }
        }
        
        ResetState();
    }

    private void Update()
    {
        switch (_state)
        {
            case State.ClickDown:
                if (!Mouse.current.leftButton.isPressed)
                {
                    ResetState();
                    break;
                }

                if ((_lastClickPoint - Mouse.current.position.ReadValue()).sqrMagnitude >
                    _dragThreshold * _dragThreshold)
                {
                    _dragSlot.gameObject.SetActive(true);
                    _dragSlot.Display(_sourceSlot);
                    _state = State.Dragging;

                    var targetRectTransform = _dragSlot.transform as RectTransform;
                    var sourceRectTransform = _sourceSlot.transform as RectTransform;
                    targetRectTransform!.sizeDelta = sourceRectTransform!.sizeDelta;
                    _dragOffset = sourceRectTransform.sizeDelta / 2;
                }
                break;
            case State.Dragging:
                var rectTransform = _dragSlot.transform as RectTransform;
                RectTransformUtility.ScreenPointToLocalPointInRectangle(canvasRect, Mouse.current.position.ReadValue(),
                    null, out var canvasPoint);
                rectTransform!.anchoredPosition = canvasPoint - _dragOffset;
                
                if (!Mouse.current.leftButton.isPressed)
                {
                    CompleteDrag();
                }
                break;
        }
    }
}
