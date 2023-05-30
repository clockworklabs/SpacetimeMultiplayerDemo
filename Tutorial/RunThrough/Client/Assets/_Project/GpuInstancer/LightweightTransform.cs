using System.Collections;
using System.Collections.Generic;
using UnityEngine;

/// <summary>
/// The purpose of the LightweightTransform is to represent the transformation data in
/// a Transform without the overhead of components and objects.
/// </summary>
public class LightweightTransform
{
    public Vector3 localScale;
    public Vector3 localPosition;
    public Quaternion localRotation;

    private LightweightTransform _parent;
    private bool _isInPool;

    private readonly HashSet<LightweightTransform> _children = new HashSet<LightweightTransform>();

    private static readonly Stack<LightweightTransform> _pool = new Stack<LightweightTransform>(1024);

    private LightweightTransform()
    {
        Reset();
    }

    private void Reset()
    {
        localScale = Vector3.one;
        localPosition = Vector3.zero;
        localRotation = Quaternion.identity;
        _parent = null;
        _children.Clear();
    }

    public void SetParent(LightweightTransform parent)
    {
        if (_parent == parent)
        {
            return;
        }

        _parent?._children.Remove(this);
        _parent = parent;
        if (parent != null && !parent._children.Contains(this))
        {
            parent._children.Add(this);
        }
    }

    public Matrix4x4 GetLocalToWorldMatrix()
    {
        var parentMatrix = Matrix4x4.identity;
        if (_parent != null)
        {
            parentMatrix = _parent.GetLocalToWorldMatrix();
        }

        return parentMatrix * Matrix4x4.TRS(localPosition, localRotation, localScale);
    }

    public void ApplyTo(Transform dest)
    {
        dest.localPosition = localPosition;
        dest.localRotation = localRotation;
        dest.localScale = localScale;
    }

    public void Combine(Transform dest)
    {
        dest.localPosition += localPosition;
        dest.localRotation *= localRotation;
        dest.localScale = Vector3.Scale(dest.localScale, localScale);
    }

    public static LightweightTransform Get()
    {
        if (_pool.Count == 0)
        {
            return new LightweightTransform();
        }

        var result = _pool.Pop();
        result._isInPool = false;
        result.Reset();
        return result;
    }

    public static void Release(LightweightTransform t)
    {
        if (t._isInPool)
        {
            return;
        }

        foreach (var child in t._children)
        {
            Release(child);
        }

        t.Reset();
        _pool.Push(t);
    }
}