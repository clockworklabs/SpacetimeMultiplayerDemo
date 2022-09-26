using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using Unity.Collections;
using UnityEngine;
using Vector3 = UnityEngine.Vector3;

public class TerrainChunk : MonoBehaviour
{
    private static readonly int Splat1Property = Shader.PropertyToID("_Splat1");

    private SpacetimeDB.Chunk _chunk;

    public SpacetimeDB.Chunk GetChunk() => _chunk;

    private readonly List<Feature> features = new List<Feature>();

    private void OnDestroy()
    {
        if (!gameObject.scene.isLoaded)
        {
            return;
        }

        foreach (var feature in features)
        {
            feature.DestroyFeature();
        }

        features.Clear();
    }

    public void VisibilityUpdated(bool visible)
    {
        foreach (var feature in features)
        {
            feature.SetFeatureVisibility(visible);
        }
    }

    public IEnumerator Spawn(Chunk chunk, GrassPrefab grassPrefab, GameObject treePrefab, GameObject ironDepositPrefab)
    {
        var terrainRenderer = GetComponentInChildren<MeshRenderer>();
        terrainRenderer.enabled = false;
        
        _chunk = chunk;
        var config = Config.FilterByVersion(0);
        Debug.Assert(config != null);
        var chunkPosition = _chunk.position;
        var chunkTransform = transform;
        chunkTransform.position = new Vector3((float)(chunkPosition.x * config.chunkSize), 0,
            (float)(chunkPosition.y * config.chunkSize));
        chunkTransform.localScale =
            new Vector3((float)config.chunkSize, (float)config.chunkSize, (float)config.chunkSize);

        var chunkData = ChunkData.FilterByChunkId(_chunk.chunkId);
        Texture2D splat1 = null;
        var nativeData = new NativeArray<byte>((int)config.chunkSplatResolution * (int)config.chunkSplatResolution * 4, Allocator.Persistent);
        for (var x = 0; x < chunkData.data.Count; x++)
        {
            nativeData[x] = chunkData.data[x];
        }

        yield return TextureUtil.Create(nativeData, (int)config.chunkSplatResolution, (int)config.chunkTerrainResolution, outputTexture => splat1 = outputTexture);
        if (splat1 == null)
        {
            yield break;
        }

        nativeData.Dispose();

        
        terrainRenderer.enabled = true;
        var instancedMat = terrainRenderer.material;
        instancedMat.SetTexture(Splat1Property, splat1);

        if (grassPrefab != null)
        {
            var grassMaterialInstance = Instantiate(grassPrefab.grass[0].sharedMaterial);
            grassMaterialInstance.SetTexture(Splat1Property, splat1);
            var grassBillboardMaterialInstance = Instantiate(grassPrefab.billboard.sharedMaterial);
            grassBillboardMaterialInstance.SetTexture(Splat1Property, splat1);
            foreach (var grass in chunkData.grass)
            {
                var feature = Feature.Create(grassPrefab.gameObject, true, true);
                feature.SetRootPosition(chunkTransform.position + new Vector3(grass.x, 0.0f, grass.y));
                feature.SetLocalScale(Vector3.one * grass.scale);
                features.Add(feature);
            }
        }

        foreach (var tree in chunkData.trees)
        {
            var feature = Feature.Create(treePrefab, false, true);
            feature.SetLocalScale(Vector3.one * tree.scale);
            feature.SetRootPosition(chunkTransform.position + new Vector3(tree.x, 0.0f, tree.y));
            feature.Model.GetComponent<GameResource>().Init(tree.entityId);
            features.Add(feature);
        }

        foreach (var deposit in chunkData.deposits)
        {
            var feature = Feature.Create(ironDepositPrefab, false, true);
            feature.SetLocalScale(Vector3.one * deposit.scale);
            feature.SetRootPosition(chunkTransform.position + new Vector3(deposit.x, 0.0f, deposit.y));
            feature.Model.GetComponent<GameResource>().Init(deposit.entityId);
            features.Add(feature);
        }
        
        TerrainController.instance.ChunkSpawned(this);
    }
}