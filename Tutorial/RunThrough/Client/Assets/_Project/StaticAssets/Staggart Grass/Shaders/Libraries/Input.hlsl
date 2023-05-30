//Stylized Grass Shader
//Staggart Creations (http://staggart.xyz)
//Copyright protected under Unity Asset Store EULA

#include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"
#include "Packages/com.unity.render-pipelines.core/ShaderLibrary/CommonMaterial.hlsl"
#include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/SurfaceInput.hlsl"

half _LodFade;

CBUFFER_START(UnityPerMaterial)
float4 _BaseColor;
float4 _BaseMap_ST;
float4 _BumpMap_ST;
float4 _HueVariation;
float4 _FadeParams;
half4 _WindDirection;
half _ColorMapStrength;
half _ColorMapHeight;
half4 _ScalemapInfluence;
half _Cutoff;
half _Smoothness;
half _Translucency;
half _OcclusionStrength;
half _VertexDarkening;

//Bending
half _BendPushStrength;
half _BendMode;
half _BendFlattenStrength;
half _PerspectiveCorrection;

//Wind
half _WindAmbientStrength;
half _WindSpeed;
half _WindVertexRand;
half _WindObjectRand;
half _WindRandStrength;
half _WindSwinging;
half _WindGustStrength;
half _WindGustFreq;
half _WindGustTint;
half _WindPinningStrength;
half _WindPinningFirmness;
half _WindPinningDebug;

//Terrain
bool _UseHexTerrainColor;
float3 _HexTerrainSamplingOffset;
TEXTURE2D_ARRAY(_HexTerrainTextures); SAMPLER(sampler_HexTerrainTextures);
TEXTURE2D_ARRAY(_HexTerrainTextures2); SAMPLER(sampler_HexTerrainTextures2);
TEXTURE2D(_HexTerrainTextureMap); SAMPLER(sampler_HexTerrainTextureMap); float4 _HexTerrainTextureMap_TexelSize;
TEXTURE2D(_HexTerrainTextureParameters); SAMPLER(sampler_HexTerrainTextureParameters); float4 _HexTerrainTextureParameters_TexelSize;
SAMPLER(s_point_repeat_sampler);

CBUFFER_END