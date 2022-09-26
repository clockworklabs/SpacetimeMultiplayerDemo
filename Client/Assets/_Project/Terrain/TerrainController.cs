using System;
using System.Collections.Generic;
using SpacetimeDB;
using UnityEngine;
using Object = UnityEngine.Object;

public class TerrainController : Singleton<TerrainController>
{
    [SerializeField] private TerrainChunk _terrainChunkPrefab;
    [SerializeField] private GrassPrefab _grassPrefab;
    [SerializeField] private GameObject _treePrefab;
    [SerializeField] private GameObject _ironDepositPrefab;
    [SerializeField] private float _viewDistance = 100;

    [SerializeField] private bool _enableGrass = true;

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
        StartCoroutine(terrainChunk.Spawn(chunk, _enableGrass ? _grassPrefab : null, _treePrefab, _ironDepositPrefab));
    }

    public void ChunkSpawned(TerrainChunk chunk)
    {
        chunks.Add(chunk);
    }

	private void Update()
	{
        if (PlayerMovementController.Local == null)
        {
            return;
        }

        var config = SpacetimeDB.Config.FilterByVersion(0);
		if (config == null)
		{
            return;
		}

		var halfSize = (float)(config.chunkSize / 2);
        var halfSizeVector = new Vector3(halfSize, 0, halfSize);
        var viewDistSq = _viewDistance * _viewDistance;

        foreach (var chunk in chunks)
        {
            var shouldShow =
                (PlayerMovementController.Local.transform.position -
                    (chunk.transform.position + halfSizeVector)).sqrMagnitude < viewDistSq;

            if (shouldShow != chunk.gameObject.activeInHierarchy)
            {
                chunk.gameObject.SetActive(shouldShow);
                chunk.VisibilityUpdated(shouldShow);
            }
        }
    }
}