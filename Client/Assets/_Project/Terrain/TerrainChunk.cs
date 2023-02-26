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
        var chunkPosition = _chunk.Position;
        var chunkTransform = transform;
        chunkTransform.position = new Vector3((float)(chunkPosition.X * config.ChunkSize), 0,
            (float)(chunkPosition.Y * config.ChunkSize));
        chunkTransform.localScale =
            new Vector3((float)config.ChunkSize, (float)config.ChunkSize, (float)config.ChunkSize);

        var chunkData = ChunkData.FilterByChunkId(_chunk.ChunkId);
        Texture2D splat1 = null;
        var nativeData = new NativeArray<byte>((int)config.ChunkSplatResolution * (int)config.ChunkSplatResolution * 4, Allocator.Persistent);
        for (var x = 0; x < chunkData.Data.Length; x++)
        {
            nativeData[x] = chunkData.Data[x];
        }

        yield return TextureUtil.Create(nativeData, (int)config.ChunkSplatResolution, (int)config.ChunkTerrainResolution, outputTexture => splat1 = outputTexture);
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
            foreach (var grass in chunkData.Grass)
            {
                var feature = Feature.Create(grassPrefab.gameObject, true, true);
                feature.SetRootPosition(chunkTransform.position + new Vector3(grass.X, 0.0f, grass.Y));
                feature.SetLocalScale(Vector3.one * grass.Scale);
                features.Add(feature);
            }
        }

        foreach (var tree in chunkData.Trees)
        {
            var feature = Feature.Create(treePrefab, false, true);
            feature.SetLocalScale(Vector3.one * tree.Scale);
            feature.SetRootPosition(chunkTransform.position + new Vector3(tree.X, 0.0f, tree.Y));
            feature.Model.GetComponent<GameResource>().Init(tree.EntityId);
            features.Add(feature);
        }

        foreach (var deposit in chunkData.Deposits)
        {
            var feature = Feature.Create(ironDepositPrefab, false, true);
            feature.SetLocalScale(Vector3.one * deposit.Scale);
            feature.SetRootPosition(chunkTransform.position + new Vector3(deposit.X, 0.0f, deposit.Y));
            feature.Model.GetComponent<GameResource>().Init(deposit.EntityId);
            features.Add(feature);
        }
        
        TerrainController.instance.ChunkSpawned(this);
    }
}
