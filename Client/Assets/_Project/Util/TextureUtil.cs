using System.Buffers.Text;
using System.Collections.Generic;
using Unity.Collections;
using UnityEngine;

public static class TextureUtil
{
    public static Texture2D Create(byte[] r, byte[] g, byte[] b, byte[] a, int width, int height)
    {
        var texture = new Texture2D(width, height, TextureFormat.ARGB32, false);
        var colors = new Color[width * height];
        Debug.Assert(r == null || r.Length == width * height);
        Debug.Assert(g == null || g.Length == width * height);
        Debug.Assert(b == null || b.Length == width * height);
        Debug.Assert(a == null || a.Length == width * height);

        for (var y = 0; y < width; y++)
        {
            for (var x = 0; x < height; x++)
            {
                var c = Color.clear;
                var index = y * width + x;
                if (r != null)
                {
                    c.r = (float)r[index] / byte.MaxValue;
                }
                if (g != null)
                {
                    c.g = (float)g[index] / byte.MaxValue;
                }
                if (b != null)
                {
                    c.b = (float)b[index] / byte.MaxValue;
                }
                if (a != null)
                {
                    c.a = (float)a[index] / byte.MaxValue;
                }

                colors[index] = c;
            }
        }
        
        texture.SetPixels(colors);
        texture.Apply();
        return texture;
    }
}