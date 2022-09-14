//Stylized Grass Shader
//Staggart Creations (http://staggart.xyz)
//Copyright protected under Unity Asset Store EULA

struct Varyings
{
	float4 positionCS               : SV_POSITION;
#ifdef _NORMALMAP
	float4 uv                       : TEXCOORD0;
#else
	float2 uv                       : TEXCOORD0;
#endif
	float2 uvLM                     : TEXCOORD1;
	float4 color					: COLOR0;
	float3 pinningStrength			: COLOR1;
	float4 positionWSAndFogFactor   : TEXCOORD2; // xyz: positionWS, w: vertex fog factor
	half3  normalWS                 : TEXCOORD3;

#ifdef _NORMALMAP
	half3 tangentWS                 : TEXCOORD4;
	half3 bitangentWS               : TEXCOORD5;
#endif

#if defined(REQUIRES_VERTEX_SHADOW_COORD_INTERPOLATOR)
	float4 shadowCoord              : TEXCOORD6; // compute shadow coord per-vertex for the main light
#endif
#ifdef _ADDITIONAL_LIGHTS_VERTEX
	float3 vertexLight				: TEXCOORD7;
#endif
	UNITY_VERTEX_INPUT_INSTANCE_ID
	UNITY_VERTEX_OUTPUT_STEREO
};

#define SHADOW_BIAS_OFFSET 0.01

Varyings LitPassVertex(Attributes input)
{
	Varyings output;
	UNITY_SETUP_INSTANCE_ID(input);
	UNITY_TRANSFER_INSTANCE_ID(input, output);
	UNITY_INITIALIZE_VERTEX_OUTPUT_STEREO(output);

	float posOffset = ObjectPosRand01();

	WindSettings wind = PopulateWindSettings(_WindAmbientStrength, _WindSpeed, _WindDirection, _WindSwinging, BEND_MASK,
		_WindObjectRand, _WindVertexRand, _WindRandStrength, _WindGustStrength,
		_WindGustFreq, _WindPinningStrength, _WindPinningFirmness);
	BendSettings bending = PopulateBendSettings(_BendMode, BEND_MASK, _BendPushStrength, _BendFlattenStrength, _PerspectiveCorrection);

	//Object space position, normals (and tangents)
	VertexInputs vertexInputs = GetVertexInputs(input);
	//Apply transformations and bending/wind
	VertexOutput vertexData = GetVertexOutput(vertexInputs, posOffset, wind, bending);

	//Vertex color
	output.pinningStrength = vertexData.pinningStrength;
	output.color = input.color;
	output.color = ApplyVertexColor(input.positionOS, vertexData.positionWS.xyz, _BaseColor.rgb, AO_MASK, _OcclusionStrength, _VertexDarkening, _HueVariation, posOffset);

	//Apply per-vertex light if enabled in pipeline
#ifdef _ADDITIONAL_LIGHTS_VERTEX
	//Pass to fragment shader to apply in Lighting function
	output.vertexLight.rgb = VertexLighting(vertexData.positionWS, vertexData.normalWS);
#endif

	output.uv.xy = TRANSFORM_TEX(input.uv, _BaseMap);
	output.uvLM = input.uvLM.xy * unity_LightmapST.xy + unity_LightmapST.zw;
	output.positionWSAndFogFactor = float4(vertexData.positionWS.xyz, ComputeFogFactor(vertexData.positionCS.z));
	output.normalWS = vertexData.normalWS;
#ifdef _NORMALMAP
	output.uv.zw = TRANSFORM_TEX(input.uv, _BumpMap);
	output.tangentWS = vertexData.tangentWS;
	output.bitangentWS = vertexData.bitangentWS;
#endif

#if defined(REQUIRES_VERTEX_SHADOW_COORD_INTERPOLATOR) //Previously _MAIN_LIGHT_SHADOWS
	//Only when when shadow cascades are per-pixel
#if _SHADOWBIAS_CORRECTION
	//Move the shadowed position slightly away from the camera to avoid banding artifacts
	float3 shadowPos = vertexData.positionWS + (vertexData.viewDir * SHADOW_BIAS_OFFSET);
#else
	float3 shadowPos = vertexData.positionWS;
#endif

	output.shadowCoord = TransformWorldToShadowCoord(shadowPos); //vertexData.positionWS
#endif
	output.positionCS = vertexData.positionCS;

	return output;
}

SurfaceData PopulateSurfaceData(Varyings input, float3 positionWS, WindSettings wind, Light mainLight)
{
	SurfaceData surfaceData;

	float4 mainTex = SampleAlbedoAlpha(input.uv.xy, TEXTURE2D_ARGS(_BaseMap, sampler_BaseMap));

	//Albedo
	float3 albedo = mainTex.rgb;

	//Apply hue var and ambient occlusion from vertex stage
	albedo.rgb *= input.color.rgb;

	//Apply color map per-pixel
	if (_ColorMapUV.w == 1) {
		float mask = smoothstep(_ColorMapHeight, 1.0 + _ColorMapHeight, sqrt(input.color.a));
		albedo.rgb = lerp(ApplyColorMap(positionWS.xyz, albedo.rgb, _ColorMapStrength, mainLight), albedo.rgb, mask);
	}

	//Apply terrain color per-pixel
	if (_UseHexTerrainColor == 1) {
		float mask = smoothstep(_ColorMapHeight, 1.0 + _ColorMapHeight, sqrt(input.color.a));
		albedo.rgb = lerp(ApplyTerrainColor(positionWS.xyz, albedo.rgb, _ColorMapStrength, mainLight), albedo.rgb, mask);
	}

	//Tint by wind gust
	wind.gustStrength = 1;
	float gust = SampleGustMap(positionWS.xyz, wind);
	albedo += gust * _WindGustTint * 10 * mainLight.shadowAttenuation * input.color.a;

	//Albedo must be saturated, may cause sparkles otherwise
	surfaceData.albedo = saturate(albedo);
	//Not using specular setup, free to use this to pass data
	surfaceData.specular = float3(0, 0, 0);
	surfaceData.metallic = 0;
	surfaceData.smoothness = _Smoothness;
#ifdef _NORMALMAP
	surfaceData.normalTS = SampleNormal(input.uv.zw, TEXTURE2D_ARGS(_BumpMap, sampler_BumpMap));
#else
	surfaceData.normalTS = float3(0.5, 0.5, 1);
#endif
	surfaceData.emission = 0;
	surfaceData.occlusion = 1;
	surfaceData.alpha = mainTex.a;
	
	return surfaceData;
}

half4 ForwardPassFragment(Varyings input) : SV_Target
{
	UNITY_SETUP_INSTANCE_ID(input);
	UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(input);

	float3 positionWS = input.positionWSAndFogFactor.xyz;

	//Only when when shadow cascades are per-pixel
#if _SHADOWBIAS_CORRECTION && defined(MAIN_LIGHT_CALCULATE_SHADOWS)
	//Move the shadowed position slightly away from the camera to avoid banding artifacts
	half3 viewDirectionWS = SafeNormalize(GetCameraPositionWS() - positionWS);
	float3 shadowPos = positionWS + (viewDirectionWS * SHADOW_BIAS_OFFSET);
#else
	float3 shadowPos = positionWS;
#endif

	float4 shadowCoord = float4(0, 0, 0, 0);
#if defined(REQUIRES_VERTEX_SHADOW_COORD_INTERPOLATOR)
	shadowCoord = input.shadowCoord; //Per-vertex coord
#elif defined(MAIN_LIGHT_CALCULATE_SHADOWS)
	shadowCoord = TransformWorldToShadowCoord(shadowPos); //Calculate per-pixel now
#endif

	Light mainLight = GetMainLight(shadowCoord, positionWS, 1);

	WindSettings wind = PopulateWindSettings(_WindAmbientStrength, _WindSpeed, _WindDirection, _WindSwinging,
		input.color.r, _WindObjectRand, _WindVertexRand, _WindRandStrength, _WindGustStrength, _WindGustFreq,
		_WindPinningStrength, _WindPinningFirmness);

	SurfaceData surfaceData = PopulateSurfaceData(input, positionWS, wind, mainLight);

#ifdef _ALPHATEST_ON
	half ditherVal;
	Unity_Dither_float(clamp(_LodFade, 0, 1) * 2, input.positionCS, ditherVal);
	clip(surfaceData.alpha - max(_Cutoff, ditherVal));
#endif

#ifdef _NORMALMAP
	half3 normalWS = TransformTangentToWorld(surfaceData.normalTS, half3x3(input.tangentWS.xyz, input.bitangentWS.xyz, input.normalWS.xyz));
#else
	half3 normalWS = input.normalWS;
#endif

#ifdef _ADDITIONAL_LIGHTS_VERTEX 
	half3 vertexLight = input.vertexLight;
#else
	half3 vertexLight = 0;
#endif

	float translucencyMask = input.color.a * _Translucency;

	float2 screenPosNorm = 0;
	#if defined(_SCREEN_SPACE_OCCLUSION) && VERSION_GREATER_EQUAL(10,0)
	screenPosNorm = GetNormalizedScreenSpaceUV(input.positionCS);
	#endif

	float3 finalColor = ApplyLighting(surfaceData, mainLight, vertexLight, input.uvLM, normalWS, positionWS, screenPosNorm, translucencyMask);

	float fogFactor = input.positionWSAndFogFactor.w;

	finalColor = MixFog(finalColor, fogFactor);

	if(_WindPinningDebug >= 1.0f)
	{
		return half4(input.pinningStrength, 1);
	}
	return half4(finalColor, surfaceData.alpha);
}

/*
#include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/UnityGBuffer.hlsl"
#define PASS_DEFERRED

half4 DeferredPassFragment(Varyings input)
{
	UNITY_SETUP_INSTANCE_ID(input);
	UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(input);	

	WindSettings wind = PopulateWindSettings(_WindStrength, _WindSpeed, _WindDirection, _WindSwinging, input.color.r, _WindObjectRand, _WindVertexRand, _WindRandStrength, _WindGustStrength, _WindGustFreq);
	float3 positionWS = input.positionWSAndFogFactor.xyz;
	SurfaceData surfaceData = PopulateSurfaceData(input, positionWS, wind);

	AlphaClip(surfaceData.alpha, _Cutoff, input.positionCS.xyz);

#ifdef _MAIN_LIGHT_SHADOWS
	Light mainLight = GetMainLight(input.shadowCoord);
#else
	Light mainLight = GetMainLight();
#endif

#if _NORMALMAP
	half3 normalWS = TransformTangentToWorld(surfaceData.normalTS, half3x3(input.tangentWS, input.bitangentWS, input.normalWS));
#else
	half3 normalWS = input.normalWS;
#endif


#ifdef _ADDITIONAL_LIGHTS_VERTEX
	half3 vertexLight = input.vertexLight;
#else
	half3 vertexLight = 0;
#endif

	float translucencyMask = input.color.a * _Translucency;

	float3 finalColor = ApplyLighting(surfaceData, mainLight, vertexLight, input.uvLM, normalWS, positionWS, translucencyMask);

	float fogFactor = input.positionWSAndFogFactor.w;

	finalColor = MixFog(finalColor, fogFactor);

	return OutputToGBuffer(surfaceData, input, finalColor);
}

//Custom version of function to support the VertexData struct
// This will encode SurfaceData into GBuffer
FragmentOutput OutputToGBuffer(SurfaceData surfaceData, VertexData inputData, half3 globalIllumination)
{
#if PACK_NORMALS_OCT
	half2 octNormalWS = PackNormalOctQuadEncode(inputData.normalWS); // values between [-1, +1]
	half2 remappedOctNormalWS = saturate(octNormalWS * 0.5 + 0.5);   // values between [ 0,  1]
	half3 packedNormalWS = PackFloat2To888(remappedOctNormalWS);
#else
	half3 packedNormalWS = inputData.normalWS * 0.5 + 0.5;   // values between [ 0,  1]
#endif

	half2 metallicAlpha = half2(surfaceData.metallic, surfaceData.alpha);
	half packedMetallicAlpha = PackFloat2To8(metallicAlpha);

	FragmentOutput output;
	output.GBuffer0 = half4(surfaceData.albedo.rgb, surfaceData.occlusion);     // albedo          albedo          albedo          occlusion    (sRGB rendertarget)
	output.GBuffer1 = half4(surfaceData.specular.rgb, surfaceData.smoothness);  // specular        specular        specular        smoothness   (sRGB rendertarget)
	output.GBuffer2 = half4(packedNormalWS, packedMetallicAlpha);               // encoded-normal  encoded-normal  encoded-normal  encoded-metallic+alpha
	output.GBuffer3 = half4(surfaceData.emission.rgb + globalIllumination, 0);  // emission+GI     emission+GI     emission+GI     [unused]     (lighting buffer)

	return output;
}
*/