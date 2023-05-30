#ifndef INTERPOLATE_HEX_TERRAIN_TEXTURES
#define INTERPOLATE_HEX_TERRAIN_TEXTURES

#include "HexCoordinatesUtil.hlsl"
#include "Conditionals.hlsl"
#include "GradientNoise2D.hlsl"

float parameterValue(
    Texture2D<float4> textureParameters,
    SamplerState s,
    float2 paramsTexelSize,
    float layer,
    float index
) {
    float2 uv = (float2(index, layer) + 0.5) * paramsTexelSize.xy;
    return SAMPLE_TEXTURE2D_LOD(textureParameters, s, float4(uv, 0, 0), 0).r * 255;
}

void interpolateHexTerrainLayers_float(
    float3 samplePosition,

    Texture2D<float4> layerMap,
    float2 texelSize,
    SamplerState s,

    Texture2DArray<float4> textures,
    Texture2DArray<float4> normalMaps,

    Texture2DArray<float4> textures2,
    Texture2DArray<float4> normalMaps2,

    SamplerState ss,

    Texture2D<float4> textureParameters,
    float2 paramsTexelSize,
  
    out float4 albedo,
    out float3 normal) {

    float3 neighbor1W;
    float3 neighbor2W;
    float3 neighbor3W;

    interpolatedWeights(samplePosition, neighbor1W, neighbor2W, neighbor3W);

    float2 offset1 = toOffset(neighbor1W.xy);
    float2 uv1 = (offset1 + 0.5) * texelSize.xy;
    float layer1 = SAMPLE_TEXTURE2D_LOD(layerMap, s, float4(uv1, 0, 0), 0).r * 255;

    float2 offset2 = toOffset(neighbor2W.xy);
    float2 uv2 = (offset2 + 0.5) * texelSize.xy;
    float layer2 = SAMPLE_TEXTURE2D_LOD(layerMap, s, float4(uv2, 0, 0), 0).r * 255;

    float2 offset3 = toOffset(neighbor3W.xy);
    float2 uv3 = (offset3 + 0.5) * texelSize.xy;
    float layer3 = SAMPLE_TEXTURE2D_LOD(layerMap, s, float4(uv3, 0, 0), 0).r * 255;

    // Parameters

    // scale
    float scale1 = parameterValue(textureParameters, s, paramsTexelSize, layer1, 0);
    float scale2 = parameterValue(textureParameters, s, paramsTexelSize, layer2, 0);
    float scale3 = parameterValue(textureParameters, s, paramsTexelSize, layer3, 0);

    // scale2
    float scale2_1 = parameterValue(textureParameters, s, paramsTexelSize, layer1, 1);
    float scale2_2 = parameterValue(textureParameters, s, paramsTexelSize, layer2, 1);
    float scale2_3 = parameterValue(textureParameters, s, paramsTexelSize, layer3, 1);

    // normal strength
    float nStrength1 = parameterValue(textureParameters, s, paramsTexelSize, layer1, 2) / 100;
    float nStrength2 = parameterValue(textureParameters, s, paramsTexelSize, layer2, 2) / 100;
    float nStrength3 = parameterValue(textureParameters, s, paramsTexelSize, layer3, 2) / 100;

    // normal strength2
    float nStrength2_1 = parameterValue(textureParameters, s, paramsTexelSize, layer1, 3) / 100;
    float nStrength2_2 = parameterValue(textureParameters, s, paramsTexelSize, layer2, 3) / 100;
    float nStrength2_3 = parameterValue(textureParameters, s, paramsTexelSize, layer3, 3) / 100;

    // noise strength
    float noiseStrength1 = parameterValue(textureParameters, s, paramsTexelSize, layer1, 4) / 100;
    float noiseStrength2 = parameterValue(textureParameters, s, paramsTexelSize, layer2, 4) / 100;
    float noiseStrength3 = parameterValue(textureParameters, s, paramsTexelSize, layer3, 4) / 100;
    float noiseStrength = (noiseStrength1 * neighbor1W.z + noiseStrength2 * neighbor2W.z + noiseStrength3 * neighbor3W.z) / (neighbor1W.z + neighbor2W.z + neighbor3W.z);

    // noise scale
    float noiseScale1 = parameterValue(textureParameters, s, paramsTexelSize, layer1, 5);
    float noiseScale2 = parameterValue(textureParameters, s, paramsTexelSize, layer2, 5);
    float noiseScale3 = parameterValue(textureParameters, s, paramsTexelSize, layer3, 5);

    // scale value is encoded as [0.1,0.2,...,10] -> [1, 2,...,100]
    float2 texUV1 = samplePosition.xz / (10 / (scale1 / 100));
    float2 texUV2 = samplePosition.xz / (10 / (scale2 / 100));
    float2 texUV3 = samplePosition.xz / (10 / (scale3 / 100));

    float2 texUV2_1 = samplePosition.xz / (10 / (scale2_1 / 100));
    float2 texUV2_2 = samplePosition.xz / (10 / (scale2_2 / 100));
    float2 texUV2_3 = samplePosition.xz / (10 / (scale2_3 / 100));

    float4 value1 = SAMPLE_TEXTURE2D_ARRAY(textures, ss, texUV1, layer1);
    float4 value2 = SAMPLE_TEXTURE2D_ARRAY(textures, ss, texUV2, layer2);
    float4 value3 = SAMPLE_TEXTURE2D_ARRAY(textures, ss, texUV3, layer3);

    float4 value2_1 = SAMPLE_TEXTURE2D_ARRAY(textures2, ss, texUV2_1, layer1);
    float4 value2_2 = SAMPLE_TEXTURE2D_ARRAY(textures2, ss, texUV2_2, layer2);
    float4 value2_3 = SAMPLE_TEXTURE2D_ARRAY(textures2, ss, texUV2_3, layer3);

    float4 albedo_1 = (value1 * neighbor1W.z + value2 * neighbor2W.z + value3 * neighbor3W.z) / (neighbor1W.z + neighbor2W.z + neighbor3W.z);
    float4 albedo_2 = (value2_1 * neighbor1W.z + value2_2 * neighbor2W.z + value2_3 * neighbor3W.z) / (neighbor1W.z + neighbor2W.z + neighbor3W.z);

    float4 normal1 = lerp(float4(0.5, 0.5, 1, 1), SAMPLE_TEXTURE2D_ARRAY(normalMaps, ss, texUV1, layer1), nStrength1);
    float4 normal2 = lerp(float4(0.5, 0.5, 1, 1), SAMPLE_TEXTURE2D_ARRAY(normalMaps, ss, texUV2, layer2), nStrength2);
    float4 normal3 = lerp(float4(0.5, 0.5, 1, 1), SAMPLE_TEXTURE2D_ARRAY(normalMaps, ss, texUV3, layer3), nStrength3);

    float4 normal2_1 = lerp(float4(0.5, 0.5, 1, 1), SAMPLE_TEXTURE2D_ARRAY(normalMaps2, ss, texUV2_1, layer1), nStrength2_1);
    float4 normal2_2 = lerp(float4(0.5, 0.5, 1, 1), SAMPLE_TEXTURE2D_ARRAY(normalMaps2, ss, texUV2_2, layer2), nStrength2_2);
    float4 normal2_3 = lerp(float4(0.5, 0.5, 1, 1), SAMPLE_TEXTURE2D_ARRAY(normalMaps2, ss, texUV2_3, layer3), nStrength2_3);

    float4 normalPacked = (normal1 * neighbor1W.z + normal2 * neighbor2W.z + normal3 * neighbor3W.z) / (neighbor1W.z + neighbor2W.z + neighbor3W.z);
    float4 normalPacked2 = (normal2_1 * neighbor1W.z + normal2_2 * neighbor2W.z + normal2_3 * neighbor3W.z) / (neighbor1W.z + neighbor2W.z + neighbor3W.z);

    float3 normal_1 = UnpackNormal(normalPacked);
    float3 normal_2 = UnpackNormal(normalPacked2);

    float2 noiseUV = samplePosition.xz / 1000;
    float noiseValue1;
    float noiseValue2;
    float noiseValue3;
    GradientNoise2D(noiseUV, noiseScale1, noiseValue1);
    GradientNoise2D(noiseUV, noiseScale2, noiseValue2);
    GradientNoise2D(noiseUV, noiseScale3, noiseValue3);
    float noiseValue = (noiseValue1 * neighbor1W.z + noiseValue2 * neighbor2W.z + noiseValue3 * neighbor3W.z) / (neighbor1W.z + neighbor2W.z + neighbor3W.z);

    albedo = lerp(albedo_1, albedo_2, noiseValue * noiseStrength);
    normal = lerp(normal_1, normal_2, noiseValue * noiseStrength);
}

#endif