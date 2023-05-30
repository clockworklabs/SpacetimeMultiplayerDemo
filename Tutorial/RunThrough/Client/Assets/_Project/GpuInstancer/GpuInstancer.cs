using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Net.Mime;
using Unity.Burst;
using Unity.Collections;
using Unity.Collections.LowLevel.Unsafe;
using Unity.Jobs;
using UnityEngine;
using UnityEngine.PlayerLoop;
using UnityEngine.Rendering;
using UnityEngine.XR;

#if UNITY_EDITOR
using UnityEditor;
using UnityEditor.Rendering;
#endif

public class GpuInstancer : Singleton<GpuInstancer>
{
    public enum CameraMode
    {
        GameView,
        AllCameras,
    }

    [Flags]
    public enum InstanceRenderFlags
    {
        Instances = 1,
        FadeInInstances = 2,
        FadeOutInstances = 4,

        All = -1
    }

    [BurstCompatible, BurstCompile]
    struct DisplayInstance : IComparer<DisplayInstance>
    {
        public int instancerId;
        public int instanceId;
        public int targetLod;
        public bool fadeIn;
        public bool fadeOut;
        public Matrix4x4 matrix;

        public int Compare(DisplayInstance x, DisplayInstance y)
        {
            return y.instancerId - x.instancerId;
        }
    }

    public struct MeshDigest
    {
        public GameObject prefabObj;
        public Mesh mesh;
        public int submeshIndex;
        public Material material;
        public Matrix4x4 localMatrix;
        public ShadowCastingMode shadowCastingMode;
        public bool receiveShadows;
        public int layer;
        public int lodLevel;
    }

    [BurstCompatible, BurstCompile]
    struct PreviousDisplayedInstance
    {
        public int instancerId;
        public int instanceId;
    }

    [Tooltip("The game camera to render instances on")]
    public Camera cam;

    public CameraMode mode;
    public InstanceRenderFlags instanceRenderFlags = InstanceRenderFlags.All;
    private bool _instancerEnabled;
    private bool _forceNoFade;
    private bool _initialLodCalculationComplete;

    [Tooltip("The amount of time it takes to fade from LOD level to another")]
    public float lodFadeInTime = 1.0f;

    public float lodFadeOutTime = 0.20f;

    private static Dictionary<InstanceTemplate.Key, InstanceRenderer> _gpuInstancers =
        new Dictionary<InstanceTemplate.Key, InstanceRenderer>();

    private static Dictionary<int, InstanceRenderer> _instancerIdToGpuInstancer =
        new Dictionary<int, InstanceRenderer>();

    // These are all of the features we are currently instancing
    private static readonly HashSet<Feature> _allFeatureInstances = new HashSet<Feature>();

    // Conversion from a feature instance to a feature instance ID
    private static readonly Dictionary<Feature, int> _featureInstanceToInstanceId = new Dictionary<Feature, int>();

    // Converts from a feature prefab to a prefab ID
    private static readonly Dictionary<GameObject, int> _prefabToPrefabId = new Dictionary<GameObject, int>();

    // Converts from a feature prefab to a prefab ID
    private static readonly Dictionary<GameObject, MeshDigest[]> _prefabToDigest =
        new Dictionary<GameObject, MeshDigest[]>();

    private static readonly List<Matrix4x4[]> _blockPool = new List<Matrix4x4[]>();

    // This maps a feature PREFAB to a description of its assets
    private static NativeList<BlittableAssetDef> _cachedAssets;

    private static NativeParallelHashMap<int, RegisteredInstance> _registeredInstances;

    // Mapping from instanceId to current LOD value
    private static NativeParallelHashMap<int, int> _registeredInstanceLod;

    // This is a temporary list for burst jobs, allocated globally to prevent garbage
    private static NativeList<DisplayInstance> _displayedInstances;

    private static NativeList<PreviousDisplayedInstance> _previousDisplayedInstances;

#if UNITY_EDITOR
    private static readonly HashSet<Material> _editorUnsupportedFadeMaterials = new HashSet<Material>();
#endif

    private static MaterialPropertyBlock _fadeInMPB;
    private static MaterialPropertyBlock _fadeOutMPB;
    private static NativeList<Matrix4x4> _outputInstances;
    private static NativeList<Matrix4x4> _outputFadeInInstances;
    private static NativeList<Matrix4x4> _outputFadeOutInstances;
    private static NativeList<BlittableInstanceBlock> _outputBlocks;
    private static NativeList<BlittableInstanceBlock> _outputFadeInBlocks;
    private static NativeList<BlittableInstanceBlock> _outputFadeOutBlocks;
    private static float _lastUpdateTime;
    private static readonly float[] _fadeInMPBValues = new float[1023];
    private static readonly float[] _fadeOutMPBValues = new float[1023];

    // The player's last known position
    private static Vector3? lastPlayerPosition;

    // The position of the player when we last updated the LODs
    private static Vector3? lastInstanceUpdatePlayerPosition;

    // Whether or not the instance renderer is enabled
    private static bool _enabled = true;

    private static bool _isDirty;

    private static bool _jobRunning;

    private static bool _fadeRunning;

    private static int _nextUniqueIdentifier = 1;

    private static float _lastJobTime = float.PositiveInfinity;

    private static readonly Queue<IGpuInstancerCommand> _commandQueue = new Queue<IGpuInstancerCommand>();
    private static readonly int LodFadeProperty = Shader.PropertyToID("_LodFade");

    // Maximum LOD level, 0-3 (4 levels total)
    private const int MAX_LOD = 3;

    [BurstCompatible]
    struct RegisteredInstance
    {
        // The id of the prefab
        public int prefabId;

        // The ID of the prefab instance that registered this instance
        public int instanceId;

        // Defines the global transform of this feature instance
        public Matrix4x4 featureGlobalMatrix;
    }

    /// <summary>
    /// This is a blittable representation of part of an asset. Depending on the distance between a feature and the
    /// player a certain LOD will be chosen. There will be one BlittableAssetDef for each LOD level for an asset. For
    /// example, here is how a tree might be broken down:
    ///
    ///  - Tree
    ///    - Leaves
    ///      - Leaves LOD 0 - only shown for LOD value 0
    ///      - Leaves LOD 1 - only shown for LOD value 1
    ///      - Leaves LOD 2 - only shown for LOD value 2 and greater
    ///    - Bark - no LODs here so its always present. We use lod=-1 to represent this state
    ///
    /// Each one of the 4 labeled components above will be in its own BlittableAssetDef.
    /// </summary>
    [BurstCompatible]
    public struct BlittableAssetDef : IEquatable<BlittableAssetDef>
    {
        // The prefab ID that this asset def is associated with 
        public int prefabId;

        // The effective LOD level where this asset is active, the special value -1 here means that it is active
        // for all LOD values.
        public int lod;

        // The id of the instancer that should be used for this asset
        public int instancerId;

        // Whether or not this asset def can be faded in or out
        public bool fadeSupported;

        // The transform for this asset
        public Matrix4x4 localInstanceMatrix;

        public bool Equals(BlittableAssetDef otherAsset)
        {
            return prefabId == otherAsset.prefabId && lod == otherAsset.lod && instancerId == otherAsset.instancerId;
        }
    }

    /// <summary>
    /// The output of the burst job is one contiguous list of instances for ALL instance types. These block
    /// descriptions let us know where each instancer starts/stops.
    /// </summary>
    [BurstCompatible]
    struct BlittableInstanceBlock
    {
        public int instancerId;
        public int count;
    }

    /// <summary>
    /// This is a bundle of all of the information that is required to render an instance
    /// </summary>
    public struct InstanceTemplate
    {
        public struct Key
        {
            public Mesh mesh;
            public Material material;
            public int subMeshIndex;
        }

        public int instancerId;
        public Key key;
        public ShadowCastingMode shadowCastingMode;
        public bool receiveShadows;
        public int renderingLayer;
    }

    class InstanceRenderer
    {
        public InstanceTemplate InstanceTemplate { get; private set; }

        private int _incompleteCount;

        // We have to split instances into blocks of 1023 due to a limitation in DrawInstanced, we could get
        // around this by using DrawInstancedIndirect but this would require custom shaders for EVERY material
        // we want to render instanced. This may be worth it for extremely common meshes like grass.
        private Matrix4x4[] _incompleteInstanceBlock;
        private readonly List<Matrix4x4[]> _completeInstanceBlocks = new List<Matrix4x4[]>();

        private readonly Matrix4x4[] _fadeInBlock;
        private int _fadeInCount;
        private readonly Matrix4x4[] _fadeOutBlock;
        private int _fadeOutCount;

        private Material _fadeInMat;
        private Material _fadeOutMat;

        public bool IsEnabled { get; set; }

        public InstanceRenderer(InstanceTemplate instanceTemplate)
        {
            InstanceTemplate = instanceTemplate;
            IsEnabled = true;
            _incompleteCount = 0;
            _incompleteInstanceBlock = NewBlock();
            _fadeInBlock = NewBlock();
            _fadeOutBlock = NewBlock();
            _fadeInMPB = new MaterialPropertyBlock();
            _fadeOutMPB = new MaterialPropertyBlock();

            _fadeInMat = Instantiate(instanceTemplate.key.material);
            _fadeOutMat = Instantiate(instanceTemplate.key.material);

            if (!InstanceTemplate.key.material.enableInstancing)
            {
                InstanceTemplate.key.material.enableInstancing = true;
#if UNITY_EDITOR
                Debug.LogWarning($"Please commit changes to this material (instancing enabled):" +
                                 $" {InstanceTemplate.key.material.name}");
#endif
            }
        }

        Matrix4x4[] NewBlock()
        {
            if (_blockPool.Count > 0)
            {
                var result = _blockPool[0];
                _blockPool.RemoveAt(0);
                return result;
            }

            return new Matrix4x4[1023];
        }

        public void ResetInstances()
        {
            _incompleteCount = 0;
            _blockPool.AddRange(_completeInstanceBlocks);
            _completeInstanceBlocks.Clear();
            _fadeInCount = 0;
            _fadeOutCount = 0;
        }

        private void AddInstance(Matrix4x4 matrix)
        {
            if (_incompleteCount >= 1023)
            {
                _completeInstanceBlocks.Add(_incompleteInstanceBlock);
                _incompleteCount = 0;
                _incompleteInstanceBlock = NewBlock();
            }

            _incompleteInstanceBlock[_incompleteCount++] = matrix;
        }

        public void BulkAddInstances(NativeArray<Matrix4x4> instances, int start, int count)
        {
            while (count >= 1023)
            {
                var newCompleteBlock = NewBlock();
                var slice = instances.Slice(start, 1023);
                slice.CopyTo(newCompleteBlock);
                _completeInstanceBlocks.Add(newCompleteBlock);

                start += 1023;
                count -= 1023;
            }

            if (count > 0)
            {
                _incompleteCount = count;
                for (var x = 0; x < count; x++)
                {
                    _incompleteInstanceBlock[x] = instances[start + x];
                }
            }
            else
            {
                _incompleteCount = 0;
            }
        }

        public void AddFadeIn(NativeArray<Matrix4x4> instances, int start, int count)
        {
            _fadeInCount = Mathf.Min(count, _fadeInBlock.Length);
            for (var x = 0; x < _fadeInCount; x++)
            {
                _fadeInBlock[x] = instances[x + start];
            }
        }

        public void AddFadeOut(NativeArray<Matrix4x4> instances, int start, int count)
        {
            _fadeOutCount = Mathf.Min(count, _fadeOutBlock.Length);
            for (var x = 0; x < _fadeOutCount; x++)
            {
                _fadeOutBlock[x] = instances[x + start];
            }
        }

        public int GetInstanceCount() => _completeInstanceBlocks.Count * 1023 + _incompleteCount;

#if UNITY_EDITOR

        public void DumpEditorDebug()
        {
            GUILayout.Label($"\t{InstanceTemplate.key.mesh.name}, {InstanceTemplate.key.subMeshIndex} " +
                            $"{InstanceTemplate.key.material.name} = {GetInstanceCount()}");
        }
#endif

        public void CompleteFade()
        {
            for (var x = 0; x < _fadeInCount; x++)
            {
                AddInstance(_fadeInBlock[x]);
            }

            _fadeOutCount = 0;
            _fadeInCount = 0;
        }

        public void Render(Camera cam, InstanceRenderFlags flags, float fadeInProgress, float fadeOutProgress)
        {
            if (!IsEnabled)
            {
                return;
            }

            if (_fadeInCount > 0 && (flags & InstanceRenderFlags.FadeInInstances) != 0)
            {
                _fadeInMat.SetFloat(LodFadeProperty, 1 - fadeInProgress);
                Graphics.DrawMeshInstanced(InstanceTemplate.key.mesh, InstanceTemplate.key.subMeshIndex, _fadeInMat,
                                           _fadeInBlock, _fadeInCount, null, InstanceTemplate.shadowCastingMode,
                                           InstanceTemplate.receiveShadows, InstanceTemplate.renderingLayer, cam);
            }

            if (_fadeOutCount > 0 && (flags & InstanceRenderFlags.FadeOutInstances) != 0)
            {
                _fadeOutMat.SetFloat(LodFadeProperty, fadeOutProgress);
                Graphics.DrawMeshInstanced(InstanceTemplate.key.mesh, InstanceTemplate.key.subMeshIndex, _fadeOutMat,
                                           _fadeOutBlock, _fadeOutCount, null, InstanceTemplate.shadowCastingMode,
                                           InstanceTemplate.receiveShadows, InstanceTemplate.renderingLayer, cam);
            }

            // TODO: Use DrawMeshInstancedIndirect here, I don't know how for now
            if ((flags & InstanceRenderFlags.Instances) != 0)
            {
                if (_incompleteCount > 0)
                {
                    Graphics.DrawMeshInstanced(InstanceTemplate.key.mesh, InstanceTemplate.key.subMeshIndex,
                                               InstanceTemplate.key.material, _incompleteInstanceBlock,
                                               _incompleteCount, null, InstanceTemplate.shadowCastingMode,
                                               InstanceTemplate.receiveShadows, InstanceTemplate.renderingLayer, cam);
                }

                foreach (var completeBlock in _completeInstanceBlocks)
                {
                    Graphics.DrawMeshInstanced(InstanceTemplate.key.mesh, InstanceTemplate.key.subMeshIndex,
                                               InstanceTemplate.key.material, completeBlock, completeBlock.Length, null,
                                               InstanceTemplate.shadowCastingMode, InstanceTemplate.receiveShadows,
                                               InstanceTemplate.renderingLayer, cam);
                }
            }
        }
    }

    private void Start()
    {
        _registeredInstances = new NativeParallelHashMap<int, RegisteredInstance>(100000, Allocator.Persistent);
        _registeredInstanceLod = new NativeParallelHashMap<int, int>(100000, Allocator.Persistent);
        _cachedAssets = new NativeList<BlittableAssetDef>(Allocator.Persistent);
        _outputInstances = new NativeList<Matrix4x4>(Allocator.Persistent);
        _outputFadeInInstances = new NativeList<Matrix4x4>(Allocator.Persistent);
        _outputFadeInBlocks = new NativeList<BlittableInstanceBlock>(Allocator.Persistent);
        _outputFadeOutInstances = new NativeList<Matrix4x4>(Allocator.Persistent);
        _outputFadeOutBlocks = new NativeList<BlittableInstanceBlock>(Allocator.Persistent);
        _fadeInMPB = new MaterialPropertyBlock();
        _fadeOutMPB = new MaterialPropertyBlock();

        _outputBlocks = new NativeList<BlittableInstanceBlock>(Allocator.Persistent);
        _displayedInstances = new NativeList<DisplayInstance>(Allocator.Persistent);
        _previousDisplayedInstances = new NativeList<PreviousDisplayedInstance>(Allocator.Persistent);
    }

    private void OnDestroy()
    {
        _registeredInstances.Dispose();
        _registeredInstanceLod.Dispose();
        _cachedAssets.Dispose();
        _outputInstances.Dispose();
        _outputFadeInInstances.Dispose();
        _outputFadeInBlocks.Dispose();
        _outputFadeOutInstances.Dispose();
        _outputFadeOutBlocks.Dispose();

        _outputBlocks.Dispose();
        _displayedInstances.Dispose();
        _previousDisplayedInstances.Dispose();
    }

    public void EnableInstancer()
    {
        _instancerEnabled = true;
        _forceNoFade = true;
    }

    public void DisableInstancer()
    {
        _instancerEnabled = false;
        _initialLodCalculationComplete = false;
    }

    public bool HasCompletedInitialGeneration()
    {
        return _initialLodCalculationComplete;
    }

    public static void RemovePrefabInstance(int instance)
    {
        if (_jobRunning)
        {
            _commandQueue.Enqueue(new RemoveInstance(instance));
        }
        else
        {
            _isDirty = true;
            _registeredInstances.Remove(instance);
        }
    }

    public static void RemovePrefabInstances(IList<int> instances)
    {
        if (_jobRunning)
        {
            foreach (var instance in instances)
            {
                _commandQueue.Enqueue(new RemoveInstance(instance));
            }
        }
        else
        {
            _isDirty = true;
            foreach (var instance in instances)
            {
                _registeredInstances.Remove(instance);
            }
        }
    }

    public static int AddPrefabInstance(GameObject prefab, Matrix4x4 transform, bool allowFade)
    {
        _isDirty = true;
        var instanceId = _nextUniqueIdentifier++;

        if (!_prefabToPrefabId.TryGetValue(prefab, out var featurePrefabId))
        {
            featurePrefabId = InternalCreateAssetDefinition(prefab, allowFade);
            _prefabToPrefabId[prefab] = featurePrefabId;
        }

        var registeredInstance = new RegisteredInstance
        {
            featureGlobalMatrix = transform,
            instanceId = instanceId,
            prefabId = featurePrefabId,
        };

        if (_jobRunning)
        {
            _commandQueue.Enqueue(new AddRegisteredInstance(registeredInstance));
        }
        else
        {
            _registeredInstances.Add(registeredInstance.instanceId, registeredInstance);
        }

        return instanceId;
    }

    public static void AddFeatureInstance(Feature feature, bool allowFade)
    {
        _isDirty = true;
        if (_allFeatureInstances.Add(feature))
        {
            _featureInstanceToInstanceId[feature] = AddPrefabInstance(
                feature.ModelPrefab, feature.GetModelTransform(), allowFade);
        }
        else
        {
            UpdateFeatureInstance(feature);
        }
    }

    public static void UpdateFeatureInstance(Feature feature)
    {
        if (_allFeatureInstances.Contains(feature))
        {
            _isDirty = true;
            var featureInstanceId = _featureInstanceToInstanceId[feature];
            var featurePrefabId = _prefabToPrefabId[feature.ModelPrefab];

            var featureInstance = new RegisteredInstance
            {
                instanceId = featureInstanceId,
                prefabId = featurePrefabId,
                featureGlobalMatrix = feature.GetModelTransform(),
            };

            if (_jobRunning)
            {
                _commandQueue.Enqueue(new UpdateRegisteredInstance(featureInstance));
            }
            else
            {
                // TODO: again hash map here?
                _registeredInstances[featureInstanceId] = new RegisteredInstance
                {
                    instanceId = featureInstanceId,
                    prefabId = featurePrefabId,
                    featureGlobalMatrix = feature.GetModelTransform(),
                };
            }
        }
    }

    public static void DeleteFeatureInstance(Feature feature)
    {
        if (_allFeatureInstances.Remove(feature))
        {
            _isDirty = true;
            var featureInstanceId = _featureInstanceToInstanceId[feature];
            _featureInstanceToInstanceId.Remove(feature);

            if (_jobRunning)
            {
                _commandQueue.Enqueue(new RemoveInstance(featureInstanceId));
            }
            else
            {
                // Remove the instance from the registered instances array
                _registeredInstances.Remove(featureInstanceId);
            }
        }
    }

    /// <summary>
    /// Returns the mesh digest for the given prefab. If the prefab has not been digested yet it will be digested now.
    /// </summary>
    /// <param name="prefab">The prefab to digest</param>
    /// <returns>The mesh digest representation of the prefab</returns>
    public static MeshDigest[] GetMeshDigest(GameObject prefab)
    {
        if (_prefabToDigest.TryGetValue(prefab, out var result))
        {
            return result;
        }

        InternalCreateAssetDefinition(prefab, true);
        return _prefabToDigest[prefab];
    }

    /// <summary>
    /// This extracts all LOD information from the prefab, so that we can quickly recalculate LOD states for this
    /// prefab. This only needs to be done once per prefab. The results ends up in `featureLODCache`.
    /// </summary>
    /// <param name="prefab">The prefab to calculate LOD states for.</param>
    /// <param name="allowFade"></param>
    private static int InternalCreateAssetDefinition(GameObject prefab, bool allowFade)
    {
        prefab.transform.position = Vector3.zero;
        var featurePrefabId = _nextUniqueIdentifier++;

        var lodGroups = prefab.GetComponentsInChildren<LODGroup>();
        var lodGroupRenderers = new HashSet<MeshRenderer>();
        var digestList = new List<MeshDigest>();

        void AddRenderer(MeshRenderer meshRenderer, int lodIndex)
        {
            var filter = meshRenderer.GetComponent<MeshFilter>();
            if (filter == null || filter.sharedMesh == null)
            {
                return;
            }

            var go = meshRenderer.gameObject;
            lodGroupRenderers.Add(meshRenderer);
            var mesh = filter.sharedMesh;
            for (var subMeshIndex = 0; subMeshIndex < mesh.subMeshCount; subMeshIndex++)
            {
                var cachedAsset = new BlittableAssetDef
                {
                    lod = lodIndex,
                    instancerId = GetInstancerIdForRenderer(meshRenderer, subMeshIndex),
                    localInstanceMatrix = meshRenderer.localToWorldMatrix,
                    prefabId = featurePrefabId,
                    fadeSupported = allowFade,
                };

                digestList.Add(new MeshDigest
                {
                    prefabObj = go,
                    mesh = mesh,
                    submeshIndex = subMeshIndex,
                    material = meshRenderer.sharedMaterials[subMeshIndex],
                    localMatrix = meshRenderer.localToWorldMatrix,
                    shadowCastingMode = meshRenderer.shadowCastingMode,
                    receiveShadows = meshRenderer.receiveShadows,
                    layer = meshRenderer.gameObject.layer,
                    lodLevel = lodIndex,
                });

                if (_jobRunning)
                {
                    _commandQueue.Enqueue(new AddCachedAsset(cachedAsset));
                }
                else
                {
                    _cachedAssets.Add(cachedAsset);
                }
            }
        }

        // Create definitions for all renderers that are within LOD groups
        foreach (var group in lodGroups)
        {
            var lods = group.GetLODs();
            Renderer[] activeLods = null;
            for (var lodIndex = 0; lodIndex <= MAX_LOD; lodIndex++)
            {
                if (lodIndex < lods.Length)
                {
                    var newRenderers = lods[lodIndex].renderers;
                    if (newRenderers != null && newRenderers.Length > 0)
                    {
                        activeLods = newRenderers;
                    }
                }

                if (activeLods == null)
                {
                    continue;
                }

                foreach (var renderer in activeLods)
                {
                    var meshRenderer = renderer as MeshRenderer;
                    if (meshRenderer == null)
                    {
                        continue;
                    }

                    AddRenderer(meshRenderer, lodIndex);
                }
            }
        }

        // Create definitions for all renderers that are NOT part of an LOD group - when a renderer is not part of an
        // LOD group it is always displayed (lod=-1)
        foreach (var renderer in prefab.GetComponentsInChildren<MeshRenderer>())
        {
            if (lodGroupRenderers.Contains(renderer) || !renderer.enabled)
            {
                continue;
            }

            AddRenderer(renderer, -1);
        }

        _prefabToDigest[prefab] = digestList.ToArray();

        return featurePrefabId;
    }

    private static int GetInstancerIdForRenderer(MeshRenderer meshRenderer, int submeshIndex = -1)
    {
        if (meshRenderer == null)
        {
            return -1;
        }

        var filter = meshRenderer.GetComponent<MeshFilter>();
        if (filter == null || filter.sharedMesh == null)
        {
            return -1;
        }

        var key = new InstanceTemplate.Key
        {
            subMeshIndex = submeshIndex,
            mesh = filter.sharedMesh,
            material = submeshIndex >= 0 ? meshRenderer.sharedMaterials[submeshIndex] : meshRenderer.sharedMaterial,
        };

        if (_gpuInstancers.TryGetValue(key, out var renderer))
        {
            return renderer.InstanceTemplate.instancerId;
        }

        renderer = _gpuInstancers[key] = new InstanceRenderer(new InstanceTemplate
        {
            instancerId = _nextUniqueIdentifier++,
            key = key,
            shadowCastingMode =
                meshRenderer.shadowCastingMode,
            receiveShadows = meshRenderer.receiveShadows,
            renderingLayer = meshRenderer.gameObject.layer,
        });

        _instancerIdToGpuInstancer[renderer.InstanceTemplate.instancerId] = renderer;
        return renderer.InstanceTemplate.instancerId;
    }

    IEnumerator RecalculateLods()
    {
        _outputInstances.Clear();
        _outputBlocks.Clear();
        _outputFadeInInstances.Clear();
        _outputFadeInBlocks.Clear();
        _outputFadeOutInstances.Clear();
        _outputFadeOutBlocks.Clear();
        _displayedInstances.Clear();
        var job = new GpuInstancerJob
        {
            lodModifierSqr = QualitySettings.lodBias,
            cachedAssets = _cachedAssets,
            playerPosition = lastPlayerPosition ?? Vector3.zero,
            usePlayerPosition = lastPlayerPosition.HasValue,
            registeredInstancesLod = _registeredInstanceLod,
            registeredInstancesEnumerator = _registeredInstances.GetEnumerator(),
            fadeInList = _outputFadeInInstances,
            fadeInBlocks = _outputFadeInBlocks,
            fadeOutList = _outputFadeOutInstances,
            fadeOutBlocks = _outputFadeOutBlocks,
            displayList = _displayedInstances,
            gpuInstanceList = _outputInstances,
            gpuInstanceBlocks = _outputBlocks,
            forceNoFade = _forceNoFade,
            previousDisplayedInstances = _previousDisplayedInstances,
        };

        // Uncomment to run job in in scheduled mode
        var handle = job.Schedule();
        while (!handle.IsCompleted)
        {
            yield return null;
        }
        
        handle.Complete();

        // Uncomment to debug job
        // yield return null;
        // job.Run();

        foreach (var instancer in _gpuInstancers)
        {
            instancer.Value.ResetInstances();
        }

#if UNITY_EDITOR
        Debug.Assert(_outputBlocks.Length == _outputFadeInBlocks.Length &&
                     _outputBlocks.Length == _outputFadeOutBlocks.Length);
#endif
        var outputInstancesArray = _outputInstances.AsArray();
        var fadeInInstancesArray = _outputFadeInInstances.AsArray();
        var fadeOutInstancesArray = _outputFadeOutInstances.AsArray();

        var instanceIndex = 0;
        var fadeInIndex = 0;
        var fadeOutIndex = 0;

        for (var x = 0; x < _outputBlocks.Length; x++)
        {
            var instanceBlock = _outputBlocks[x];
            var fadeInBlock = _outputFadeInBlocks[x];
            var fadeOutBlock = _outputFadeOutBlocks[x];

            if (_instancerIdToGpuInstancer.TryGetValue(instanceBlock.instancerId, out var instancer))
            {
                instancer.BulkAddInstances(outputInstancesArray, instanceIndex, instanceBlock.count);
                instancer.AddFadeIn(fadeInInstancesArray, fadeInIndex, fadeInBlock.count);
                instancer.AddFadeOut(fadeOutInstancesArray, fadeOutIndex, fadeOutBlock.count);
            }
            else
            {
                Debug.LogError($"Failed to find instancer: {instanceBlock.instancerId}");
            }

            instanceIndex += instanceBlock.count;
            fadeInIndex += fadeInBlock.count;
            fadeOutIndex += fadeOutBlock.count;
        }

        // Complete any commands that we queued up while waiting for the job to complete
        while (_commandQueue.Count > 0)
        {
            _commandQueue.Dequeue().Execute();
        }

        _lastUpdateTime = Time.time;
        _fadeRunning = true;
        _jobRunning = false;
        _forceNoFade = false;
        _initialLodCalculationComplete = true;
    }

    public static int GetLodForPrefab(Vector3 position)
    {
        if (!lastPlayerPosition.HasValue)
        {
            return 0;
        }

        var lod = 0;
        var diffSqr = (position - lastPlayerPosition.Value).sqrMagnitude;
        if (diffSqr > GpuInstancerJob.LOD1 * GpuInstancerJob.LOD1)
        {
            lod = 1;
        }

        if (diffSqr > GpuInstancerJob.LOD2 * GpuInstancerJob.LOD2)
        {
            lod = 2;
        }

        if (diffSqr > GpuInstancerJob.LOD3 * GpuInstancerJob.LOD3)
        {
            lod = 3;
        }

        return lod;
    }

    private void Update()
    {
        if (PlayerMovementController.Local == null || !_instancerEnabled)
        {
            return;
        }

        var totalFadeTime = Mathf.Max(lodFadeInTime, lodFadeOutTime);
        if (!_jobRunning && _fadeRunning && Time.time >= _lastUpdateTime + totalFadeTime)
        {
            foreach (var instancer in _gpuInstancers.Values)
            {
                instancer.CompleteFade();
            }

            _fadeRunning = false;
        }

        if (_isDirty && !_jobRunning && !_fadeRunning)
        {
            _isDirty = false;
            _jobRunning = true;
            StartCoroutine(RecalculateLods());
        }

        lastPlayerPosition = PlayerMovementController.Local.transform.position;
        if (!lastInstanceUpdatePlayerPosition.HasValue)
            lastInstanceUpdatePlayerPosition = lastPlayerPosition;

        if ((lastPlayerPosition.Value - lastInstanceUpdatePlayerPosition.Value).sqrMagnitude > 25.0f * 25.0f)
        {
            lastInstanceUpdatePlayerPosition = lastPlayerPosition;
            _isDirty = true;
        }
    }

    private void LateUpdate()
    {
        var fadeInProgress = Mathf.Clamp01((Time.time - _lastUpdateTime) / lodFadeInTime);
        var fadeOutProgress = Mathf.Clamp01((Time.time - _lastUpdateTime - lodFadeInTime) / lodFadeOutTime);
        foreach (var gpuInstancer in _gpuInstancers)
        {
            gpuInstancer.Value.Render(mode == CameraMode.AllCameras ? null : cam, instanceRenderFlags, fadeInProgress,
                                      fadeOutProgress);
        }
    }

    [BurstCompile(CompileSynchronously = true, FloatMode = FloatMode.Fast)]
    // [BurstCompile(Debug = true)]
    private struct GpuInstancerJob : IJob
    {
        [ReadOnly]
        public NativeList<BlittableAssetDef> cachedAssets;
        [ReadOnly]
        public NativeParallelHashMap<int, RegisteredInstance>.Enumerator registeredInstancesEnumerator;
        public NativeParallelHashMap<int, int> registeredInstancesLod;
        [ReadOnly]
        public Vector3 playerPosition;
        [ReadOnly]
        public bool usePlayerPosition;
        [ReadOnly]
        public bool forceNoFade;

        [ReadOnly]
        public float lodModifierSqr;

        public NativeList<Matrix4x4> gpuInstanceList;
        public NativeList<DisplayInstance> displayList;
        public NativeList<BlittableInstanceBlock> gpuInstanceBlocks;
        public NativeList<Matrix4x4> fadeInList;
        public NativeList<BlittableInstanceBlock> fadeInBlocks;
        public NativeList<Matrix4x4> fadeOutList;
        public NativeList<BlittableInstanceBlock> fadeOutBlocks;
        public NativeList<PreviousDisplayedInstance> previousDisplayedInstances;

        public const float LOD1 = 15.0f;
        public const float LOD2 = 25.0f;
        public const float LOD3 = 35.0f;

        // bool IsDisplayed(int registeredInstanceId, int instancerId)
        // {
        //     foreach (var entry in previousDisplayedInstances)
        //     {
        //         if (entry.instanceId == registeredInstanceId && entry.instancerId == instancerId)
        //         {
        //             return true;
        //         }
        //     }
        //
        //     return false;
        // }

        bool IsDisplayed(int registeredInstanceId, int instancerId)
        {
            foreach (var entry in previousDisplayedInstances)
            {
                if (entry.instanceId == registeredInstanceId && entry.instancerId == instancerId)
                {
                    return true;
                }
            }

            return false;
        }

        public void Execute()
        {
            // Calculate the actual display list
            registeredInstancesEnumerator.Reset();

            // The objective of this first loop is to populate the display list with all of the instances
            // we want to display. When we are transitioning between lods we add 2 lods at the same position.
            while (registeredInstancesEnumerator.MoveNext())
            {
                var registeredInstance = registeredInstancesEnumerator.Current;
                var globalFeatureMatrix = registeredInstance.Value.featureGlobalMatrix;
                var featurePoint = globalFeatureMatrix.MultiplyPoint(Vector3.zero);
                var distanceSqr = (featurePoint - playerPosition).sqrMagnitude;
                var hasPreviousLodValue = true;
                if (!registeredInstancesLod.TryGetValue(registeredInstance.Key, out var previousLodValue))
                {
                    previousLodValue = -1;
                    hasPreviousLodValue = false;
                }

                // Calculate the LOD value
                var selectedLod = 0;
                if (distanceSqr > LOD1 * LOD1 * lodModifierSqr)
                {
                    selectedLod = 1;
                }

                if (distanceSqr > LOD2 * LOD2 * lodModifierSqr)
                {
                    selectedLod = 2;
                }

                if (distanceSqr > LOD3 * LOD3 * lodModifierSqr)
                {
                    selectedLod = 3;
                }

                if (!usePlayerPosition)
                {
                    selectedLod = 0;
                }

                foreach (var cachedAsset in cachedAssets)
                {
                    if (cachedAsset.prefabId != registeredInstance.Value.prefabId)
                    {
                        continue;
                    }

                    var isSelectedAsset = cachedAsset.lod < 0 || cachedAsset.lod == selectedLod;
                    var isPreviousAsset = hasPreviousLodValue && previousLodValue == cachedAsset.lod;
                    // var isAlreadyDisplayed = IsDisplayed(registeredInstance.Key, cachedAsset.instancerId);
                    var isAlreadyDisplayed = false;

                    if (isSelectedAsset || isPreviousAsset)
                    {
                        var fadeIn = false;
                        var fadeOut = false;

                        if (hasPreviousLodValue && cachedAsset.lod >= 0)
                        {
                            if (isPreviousAsset)
                            {
                                if (!isSelectedAsset)
                                {
                                    fadeOut = true;
                                }
                            }
                            else
                            {
                                fadeIn = true;
                            }
                        }
                        else
                        {
                            if (!hasPreviousLodValue)
                            {
                                fadeIn = true;
                            }
                        }

                        if (forceNoFade)
                        {
                            if (!isSelectedAsset)
                            {
                                continue;
                            }

                            fadeIn = fadeOut = false;
                        }

                        if (isAlreadyDisplayed || !cachedAsset.fadeSupported)
                        {
                            fadeIn = fadeOut = false;
                        }

                        displayList.Add(new DisplayInstance
                        {
                            instancerId = cachedAsset.instancerId,
                            instanceId = registeredInstance.Key,
                            fadeIn = fadeIn,
                            fadeOut = fadeOut,
                            targetLod = selectedLod,
                            matrix = globalFeatureMatrix * cachedAsset.localInstanceMatrix,
                        });
                    }
                }
            }

            if (displayList.Length <= 0)
            {
                return;
            }

            // First sort the registered instances by instance ID
            displayList.Sort(new DisplayInstance());

            var currentInstancer = displayList[0].instancerId;
            var currentInstanceCount = 0;
            var fadeInCount = 0;
            var fadeOutCount = 0;

            registeredInstancesLod.Clear();
            // previousDisplayedInstances.Clear();

            foreach (var displayedInstance in displayList)
            {
                // Have we hit a new feature type?
                if (displayedInstance.instancerId != currentInstancer)
                {
                    gpuInstanceBlocks.Add(new BlittableInstanceBlock
                    {
                        instancerId = currentInstancer,
                        count = currentInstanceCount,
                    });

                    fadeInBlocks.Add(new BlittableInstanceBlock
                    {
                        instancerId = currentInstancer,
                        count = fadeInCount,
                    });

                    fadeOutBlocks.Add(new BlittableInstanceBlock
                    {
                        instancerId = currentInstancer,
                        count = fadeOutCount,
                    });

                    // We are on a new feature
                    currentInstancer = displayedInstance.instancerId;
                    currentInstanceCount = 0;
                    fadeInCount = 0;
                    fadeOutCount = 0;
                }

                // Update the registered instance LOD
                registeredInstancesLod[displayedInstance.instanceId] = displayedInstance.targetLod;

                if (displayedInstance.fadeIn && fadeInCount < 1023)
                {
                    // previousDisplayedInstances.Add(new PreviousDisplayedInstance
                    // {
                    //     instanceId = displayedInstance.instanceId,
                    //     instancerId = currentInstancer,
                    // });

                    fadeInCount++;
                    fadeInList.Add(displayedInstance.matrix);
                }
                else if (displayedInstance.fadeOut && fadeOutCount < 1023)
                {
                    fadeOutCount++;
                    fadeOutList.Add(displayedInstance.matrix);
                }
                else
                {
                    // previousDisplayedInstances.Add(new PreviousDisplayedInstance
                    // {
                    //     instanceId = displayedInstance.instanceId,
                    //     instancerId = currentInstancer,
                    // });

                    currentInstanceCount++;
                    gpuInstanceList.Add(displayedInstance.matrix);
                }
            }

            // Add the block for the final feature
            gpuInstanceBlocks.Add(new BlittableInstanceBlock
            {
                instancerId = currentInstancer,
                count = currentInstanceCount,
            });

            fadeInBlocks.Add(new BlittableInstanceBlock
            {
                instancerId = currentInstancer,
                count = fadeInCount,
            });

            fadeOutBlocks.Add(new BlittableInstanceBlock
            {
                instancerId = currentInstancer,
                count = fadeOutCount,
            });
        }
    }

    #region Commands

    private interface IGpuInstancerCommand
    {
        public void Execute();
    }

    private struct AddCachedAsset : IGpuInstancerCommand
    {
        private BlittableAssetDef _asset;

        public AddCachedAsset(BlittableAssetDef asset)
        {
            _asset = asset;
        }

        public void Execute()
        {
            _cachedAssets.Add(_asset);
            _isDirty = true;
        }
    }

    private struct AddRegisteredInstance : IGpuInstancerCommand
    {
        private RegisteredInstance _instance;

        public AddRegisteredInstance(RegisteredInstance instance)
        {
            _instance = instance;
        }

        public void Execute()
        {
            _registeredInstances[_instance.instanceId] = _instance;
            _isDirty = true;
        }
    }

    private struct UpdateRegisteredInstance : IGpuInstancerCommand
    {
        private RegisteredInstance _instance;

        public UpdateRegisteredInstance(RegisteredInstance instance)
        {
            _instance = instance;
        }

        public void Execute()
        {
            _registeredInstances[_instance.instanceId] = _instance;
            _isDirty = true;
        }
    }

    private struct RemoveInstance : IGpuInstancerCommand
    {
        private int _instanceId;

        public RemoveInstance(int instanceId)
        {
            _instanceId = instanceId;
        }

        public void Execute()
        {
            _registeredInstances.Remove(_instanceId);
            _isDirty = true;
        }
    }

    #endregion

#if UNITY_EDITOR

    [CustomEditor(typeof(GpuInstancer))]
    public class GpuInstancerEditor : Editor
    {
        public override void OnInspectorGUI()
        {
            base.OnInspectorGUI();

            if (!Application.isPlaying)
            {
                GUILayout.Label("Only available during play mode");
                return;
            }

            GUILayout.Label($"Last Job Time: {_lastJobTime}");
            _enabled = EditorGUILayout.Toggle("GPU Instancing", _enabled);

            foreach (var instancer in _gpuInstancers)
            {
                GUILayout.BeginHorizontal();
                instancer.Value.IsEnabled = EditorGUILayout.Toggle(instancer.Value.IsEnabled);
                instancer.Value.DumpEditorDebug();
                GUILayout.EndHorizontal();
            }

            GUILayout.Label($"Unsupported Fade Materials");
            foreach (var mat in _editorUnsupportedFadeMaterials)
            {
                GUILayout.Label($"\t{mat.name}: No _LodFade property.");
            }
        }
    }

#endif
}