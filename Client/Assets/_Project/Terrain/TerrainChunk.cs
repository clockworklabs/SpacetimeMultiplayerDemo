using System;
using System.Collections;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEditor;
using UnityEngine;
using Vector3 = UnityEngine.Vector3;

public class TerrainChunk : MonoBehaviour
{
    [SerializeField] private GameObject mesh;
    private static readonly int Splat1Property = Shader.PropertyToID("_Splat1");

    private const float _viewDistance = 100;

    private SpacetimeDB.Chunk _chunk;
    
    public SpacetimeDB.Chunk GetChunk() => _chunk;
    
    byte[] GetDirtSplat(byte[] data)
    {
        var config = Config.FilterByVersion(0);
        var result = new byte[config.chunkSplatResolution * config.chunkSplatResolution];
        Array.Copy(data, config.chunkTerrainResolution * config.chunkTerrainResolution, 
            result, 0, result.Length);
        return result;
    }
    
    byte[] GetSandSplat(byte[] data)
    {
        var config = Config.FilterByVersion(0);
        var result = new byte[config.chunkSplatResolution * config.chunkSplatResolution];
        Array.Copy(data, config.chunkTerrainResolution * config.chunkTerrainResolution 
                         + config.chunkSplatResolution * config.chunkSplatResolution, result, 0, result.Length);
        return result;
    }
    
    public void Spawn(Chunk chunk, GameObject grassPrefab, GameObject treePrefab, GameObject ironDepositPrefab)
    {
        _chunk = chunk;
        var config = Config.FilterByVersion(0);
        Debug.Assert(config != null);
        var chunkPosition = chunk.position;
        var chunkTransform = transform;
        chunkTransform.position = new Vector3((float)(chunkPosition.x * config.chunkSize), 0, (float)(chunkPosition.y * config.chunkSize));
        chunkTransform.localScale =
            new Vector3((float)config.chunkSize, (float)config.chunkSize, (float)config.chunkSize);

        var chunkData = ChunkData.FilterByChunkId(_chunk.chunkId);
        var data = chunkData.data.ToArray();
        var dirtSplat = GetDirtSplat(data);
        var sandSplat = GetSandSplat(data);
        var splat1 = TextureUtil.Create(dirtSplat, sandSplat, null, null, (int)config.chunkSplatResolution,
            (int)config.chunkSplatResolution);
        
        var terrainRenderer = GetComponentInChildren<MeshRenderer>();
        var instancedMat = terrainRenderer.material;
        instancedMat.SetTexture(Splat1Property, splat1);

        foreach (var grass in chunkData.grass)
        {
            var inst = Instantiate(grassPrefab);
            inst.transform.localScale = Vector3.one * grass.scale;
            inst.transform.rotation *= Quaternion.Euler(0.0f, UnityEngine.Random.value * 360.0f, 0.0f);
            inst.transform.position = chunkTransform.position + new Vector3(grass.x, 0.0f, grass.y);
        }
        
        foreach (var tree in chunkData.trees)
        {
            var inst = Instantiate(treePrefab);
            inst.transform.localScale = Vector3.one * tree.scale;
            inst.transform.rotation *= Quaternion.Euler(0.0f, UnityEngine.Random.value * 360.0f, 0.0f);
            inst.transform.position = chunkTransform.position + new Vector3(tree.x, 0.0f, tree.y);
        }
        
        foreach (var deposit in chunkData.deposits)
        {
            var inst = Instantiate(ironDepositPrefab);
            inst.transform.localScale = Vector3.one * deposit.scale;
            inst.transform.rotation *= Quaternion.Euler(0.0f, UnityEngine.Random.value * 360.0f, 0.0f);
            inst.transform.position = chunkTransform.position + new Vector3(deposit.x, 0.0f, deposit.y);
        }
    }

    private void Update()
    {
        if (PlayerMovementController.Local == null)
        {
            return;
        }
        
        var config = SpacetimeDB.Config.FilterByVersion(0);
        Debug.Assert(config != null);
        var halfSize = (float)(config.chunkSize / 2);
        var shouldShow =
            (PlayerMovementController.Local.transform.position -
             (transform.position + new Vector3(halfSize, 0, halfSize))).sqrMagnitude < _viewDistance * _viewDistance;

        if (shouldShow != mesh.activeSelf)
        {
            mesh.SetActive(shouldShow);
        }
    }
}
