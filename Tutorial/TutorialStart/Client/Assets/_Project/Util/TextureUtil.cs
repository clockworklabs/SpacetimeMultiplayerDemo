using System;
using System.Collections;
using System.Collections.Generic;
using Unity.Burst;
using Unity.Collections;
using Unity.Jobs;
using UnityEngine;

public static class TextureUtil
{
    public static IEnumerator Create(NativeArray<byte> data, int splatResolution, int terrainResolution, Action<Texture2D> callback)
    {
        var texture = new Texture2D(splatResolution, splatResolution, TextureFormat.ARGB32, false);
        using var colors = new NativeArray<Color>(splatResolution * splatResolution, Allocator.Persistent);

        var job = new CombineJob
        {
            data = data,
            splatSize = splatResolution,
            terrainSize = terrainResolution,
            colors = colors
        };

        var handle = job.Schedule(splatResolution * splatResolution, 128);
        while (!handle.IsCompleted)
        {
            yield return null;
        }
        handle.Complete();
        
        texture.SetPixels(colors.ToArray());
        texture.Apply();
        callback?.Invoke(texture);
    }

    [BurstCompile(FloatPrecision.Standard, FloatMode.Fast, CompileSynchronously = true, OptimizeFor = OptimizeFor.Performance)]
    struct CombineJob : IJobParallelFor
    {
        [ReadOnly] public NativeArray<byte> data;
        [ReadOnly] public int splatSize;
        [ReadOnly] public int terrainSize;
        [WriteOnly] public NativeArray<Color> colors;
        
        float GetValue(int index, int offset)
        {
            var pos = (terrainSize * terrainSize) + index + offset * (splatSize * splatSize);
            if (pos >= data.Length)
            {
                return 0;
            }
            else
            {
                return (float)data[pos] / byte.MaxValue;
            }
        }

        public void Execute(int index)
        {
            var offset = splatSize * splatSize;
            var r = GetValue(index, 0);
            var g = GetValue(index, 1);
            var b = GetValue(index, 2);
            var a = GetValue(index, 3);
            colors[index] = new Color(r, g, b, a);
        }
    }
}