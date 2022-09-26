using System;
using System.Collections.Generic;
using UnityEngine;
using Object = UnityEngine.Object;
using Random = UnityEngine.Random;

public class Feature
{
    public GameObject Model
    {
        get
        {
            if (_model == null)
            {
                CreateModelInstance();
            }

            return _model;
        }

        private set => _model = value;
    }

    public GameObject ModelPrefab
    {
        get;
        private set;
    }

    private GameObject _model;
    private GpuInstancer.MeshDigest[] _meshDigest;

    // The root object for the hierarchy, this holds all renderers, colliders, etc. for the entire feature. The
    // root is only created if there is something that requires GameObjects to be present in the scene.
    private GameObject _root;

    // Transformations for building the local transform, these only matter if
    // there is no root.
    private readonly LightweightTransform _rootTransform;
    private readonly LightweightTransform _modelTransform;
    private readonly List<Collider> _instancedColliders = new List<Collider>();

    public Feature()
    {
        _rootTransform = LightweightTransform.Get();
        _modelTransform = LightweightTransform.Get();
        _modelTransform.SetParent(_rootTransform);
    }

    public void SetPrefab(GameObject prefab, bool gpuInstance)
    {
        ModelPrefab = prefab;
        if (!gpuInstance || !SystemInfo.supportsInstancing)
        {
            CreateModelInstance();
        }
        else
        {
            GpuInstancer.AddFeatureInstance(this, true);
        }

        _meshDigest = GpuInstancer.GetMeshDigest(prefab);

        if (_model == null)
        {
            GpuInstancer.UpdateFeatureInstance(this);
        }
    }

    public void SpawnColliders()
    {
        if (_instancedColliders.Count > 0)
        {
            Debug.LogWarning($"Colliders already spawned for prefab: {ModelPrefab.name}");
            return;
        }
        
        foreach (var collider in ModelPrefab.GetComponentsInChildren<CapsuleCollider>(true))
        {
            // Small issue: We want to skip grass colliders, so find them and skip them
            var checkTransform = collider.transform.parent;
            var grass = false;
            while (checkTransform != null)
            {
                // we want to skip [Gg]rass colliders
                if (checkTransform.name.Contains("rass"))
                {
                    grass = true;
                    break;
                }

                checkTransform = checkTransform.parent;
            }

            if (grass)
            {
                continue;
            }

            CreateRootInstance();
            var copyTransform = collider.transform;
            var newColliderGO = new GameObject("CapsuleCollider");
            newColliderGO.transform.parent = _root.transform;
            newColliderGO.transform.localPosition = copyTransform.localPosition;
            newColliderGO.transform.localRotation = copyTransform.localRotation;
            newColliderGO.transform.localScale = copyTransform.localScale;
            newColliderGO.layer = collider.gameObject.layer;
            var newCollider = newColliderGO.AddComponent<CapsuleCollider>();
            newCollider.center = collider.center;
            newCollider.direction = collider.direction;
            newCollider.height = collider.height;
            newCollider.radius = collider.radius;
            newCollider.sharedMaterial = collider.sharedMaterial;
            _instancedColliders.Add(newCollider);
        }
    }

    public void SetModel(GameObject instance)
    {
        if (_model != null)
        {
            return;
        }

        CreateRootInstance();
        _model = instance;
        _model.transform.parent = _root.transform;
        GpuInstancer.UpdateFeatureInstance(this);
    }

    void CreateRootInstance()
    {
        if (_root != null)
        {
            return;
        }

        _root = new GameObject(ModelPrefab.name) { transform = { parent = BitCraftMiniGameManager.FeatureRoot.transform } };
        _rootTransform.ApplyTo(_root.transform);
    }

    /// <summary>
    /// This should be called before this feature is interacted with.
    /// </summary>
    void CreateModelInstance()
    {
        if (_model != null)
        {
            return;
        }

        foreach (var collider in _instancedColliders)
        {
            Object.Destroy(collider.gameObject);
        }

        CreateRootInstance();
        _model = Object.Instantiate(ModelPrefab, _root.transform, false);
        _modelTransform.Combine(_model.transform);
        GpuInstancer.DeleteFeatureInstance(this);
    }

    public void SetFeatureVisibility(bool visible)
    {
        if (_root != null)
        {
            _root.SetActive(visible);
        }
        
        if (visible)
        {
            if (IsGpuInstanced())
            {
                GpuInstancer.AddFeatureInstance(this, true);    
            }
        }
        else
        {
            if (IsGpuInstanced())
            {
                GpuInstancer.DeleteFeatureInstance(this);    
            }
        }
    }

    public void DestroyFeature()
    {
        LightweightTransform.Release(_rootTransform);
        GpuInstancer.DeleteFeatureInstance(this);

        if (_root != null)
        {
            Object.Destroy(_root);
        }
    }

    public Vector3 GetRootPosition()
    {
        if (_root != null)
        {
            return _root.transform.position;
        }

        return _rootTransform.localPosition;
    }

    public void SetRootPosition(Vector3 position)
    {
        _rootTransform.localPosition = position;
        if (_root != null)
        {
            _root.transform.localPosition = position;
        }

        GpuInstancer.UpdateFeatureInstance(this);
    }

    public void SetRootRotation(Quaternion rotation)
    {
        _rootTransform.localRotation = rotation;
        if (_root != null)
        {
            _root.transform.localRotation = rotation;
        }

        GpuInstancer.UpdateFeatureInstance(this);
    }

    public void SetLocalScale(Vector3 scale)
    {
        _modelTransform.localScale = scale;
        if (_model != null)
        {
            _model.transform.localScale = scale;
        }

        GpuInstancer.UpdateFeatureInstance(this);
    }
    
    public Matrix4x4 GetModelTransform()
    {
        if (_model != null)
        {
            return _model.transform.localToWorldMatrix;
        }

        return _modelTransform.GetLocalToWorldMatrix();
    }

    public Matrix4x4 GetRootTransform()
    {
        if (_root != null)
        {
            return _root.transform.localToWorldMatrix;
        }

        return _rootTransform.GetLocalToWorldMatrix();
    }

    public bool IsGpuInstanced()
    {
        return _model == null;
    }

    public GpuInstancer.MeshDigest[] GetMeshDigest() => _meshDigest;
    public Matrix4x4 GetOutlineMeshTransform() => GetModelTransform();

    public GameObject GetOutlineMesh()
    {
        if (!IsGpuInstanced())
        {
            return _model;
        }

        return null;
    }

    public static Feature Create(GameObject prefab, bool gpuInstance, bool randomRotation)
    {
        var f = new Feature();
        f.SetPrefab(prefab, gpuInstance);
        if (randomRotation)
        {
            f.SetRootRotation(Quaternion.Euler(0.0f, Random.value * 360.0f, 0.0f));
        }
        return f;
    }
}