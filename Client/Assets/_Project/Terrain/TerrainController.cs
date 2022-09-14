using System;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using Object = UnityEngine.Object;

public class TerrainController : Singleton<TerrainController>
{
    [SerializeField] private TerrainChunk _terrainChunkPrefab;
    [SerializeField] private GameObject _grassPrefab;
    [SerializeField] private GameObject _treePrefab;
    [SerializeField] private GameObject _ironDepositPrefab;
    
    // TODO: Convert this to a dictionary
    private readonly List<TerrainChunk> chunks = new List<TerrainChunk>();

    public TerrainChunk GetChunk(ChunkPosition pos)
    {
        foreach (var chunk in chunks)
        {
            if (chunk.GetChunk().position.Equals(pos))
            {
                return chunk;
            }   
        }

        return null;
    }

    public TerrainChunk GetChunk(int x, int y) => GetChunk(new ChunkPosition() { x = x, y = y });

    public void AddChunk(Chunk chunk)
    {
        var terrainChunk = Instantiate(_terrainChunkPrefab);
        chunks.Add(terrainChunk);
        terrainChunk.Spawn(chunk, _grassPrefab, _treePrefab, _ironDepositPrefab);
    }
}