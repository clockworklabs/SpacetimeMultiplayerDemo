using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GrassPrefab : MonoBehaviour
{
    public MeshRenderer[] grass;
    public MeshRenderer billboard;

    public void Assign(Material grassMaterial, Material billboardMaterial)
    {
        foreach (var renderer in grass)
        {
            renderer.sharedMaterial = grassMaterial;
        }

        billboard.sharedMaterial = billboardMaterial;
    }
}
