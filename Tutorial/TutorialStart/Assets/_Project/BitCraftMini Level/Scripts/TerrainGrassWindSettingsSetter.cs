using UnityEngine;

namespace Core.Utilities
{
    // Creates a new instance of terrain data that can be safely changed.
    // Applies wind wettings specified in inspector.
    public class TerrainGrassWindSettingsSetter : MonoBehaviour
    {
        [SerializeField]
        private Terrain _terrain;

        [SerializeField]
        private TerrainGrassWindSettings _windSettings;

        private TerrainData _originalTerrainData;
        private TerrainData _terrainData;

        private void OnEnable()
        {
            if (_terrain == null)
            {
                Debug.LogError("Terrain is not specified!");
                return;
            }

            _originalTerrainData = _terrain.terrainData;

            _terrainData = Instantiate(_originalTerrainData);
            _windSettings.SetToTerrainData(_terrainData);
            _terrain.terrainData = _terrainData;
        }

        private void OnDisable()
        {
            if (_originalTerrainData != null)
            {
                _terrain.terrainData = _originalTerrainData;
            }

            if (_terrainData != null)
            {
                Destroy(_terrainData);
            }

            _terrainData = null;
        }

        [System.Serializable]
        private class TerrainGrassWindSettings
        {
            [Range(0f, 1f)]
            public float speed;

            [Range(0f, 10f)]
            public float size;

            [Range(0f, 1f)]
            public float bending;

            public Color tint = new Color32(178, 153, 128, 255);

            public void SetToTerrainData(TerrainData terrainData)
            {
                terrainData.wavingGrassStrength = speed;
                terrainData.wavingGrassSpeed = size;
                terrainData.wavingGrassAmount = bending;
                terrainData.wavingGrassTint = tint;
            }

            public static TerrainGrassWindSettings FromTerrainData(TerrainData terrainData)
            {
                return new TerrainGrassWindSettings()
                {
                    speed = terrainData.wavingGrassStrength,
                    size = terrainData.wavingGrassSpeed,
                    bending = terrainData.wavingGrassAmount,
                    tint = terrainData.wavingGrassTint
                };
            }
        }
    }
}