//Stylized Grass Shader
//Staggart Creations (http://staggart.xyz)
//Copyright protected under Unity Asset Store EULA

struct Varyings
{
	float2 uv           : TEXCOORD0;
	float4 positionCS   : SV_POSITION;
#ifdef _ALPHATEST_ON
	float4 positionWS   : TEXCOORD1;
#endif
	UNITY_VERTEX_INPUT_INSTANCE_ID
	UNITY_VERTEX_OUTPUT_STEREO
};

Varyings DepthOnlyVertex(Attributes input)
{
	Varyings output = (Varyings)0;
	UNITY_SETUP_INSTANCE_ID(input);
	UNITY_INITIALIZE_VERTEX_OUTPUT_STEREO(output);

	output.uv = TRANSFORM_TEX(input.uv, _BaseMap);

	float posOffset = ObjectPosRand01();

	WindSettings wind = PopulateWindSettings(_WindAmbientStrength, _WindSpeed, _WindDirection, _WindSwinging, BEND_MASK,
		_WindObjectRand, _WindVertexRand, _WindRandStrength, _WindGustStrength,
		_WindGustFreq, _WindPinningStrength, _WindPinningFirmness);
	BendSettings bending = PopulateBendSettings(_BendMode, BEND_MASK, _BendPushStrength, _BendFlattenStrength, _PerspectiveCorrection);

	VertexInputs vertexInputs = GetVertexInputs(input);
	VertexOutput vertexData = GetVertexOutput(vertexInputs, posOffset, wind, bending);

	output.positionCS = vertexData.positionCS;
#ifdef _ALPHATEST_ON
	output.positionWS.xyz = vertexData.positionWS;
#endif

	return output;
}

half4 DepthOnlyFragment(Varyings input) : SV_TARGET
{
	UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(input);

#ifdef _ALPHATEST_ON
	float alpha = SAMPLE_TEXTURE2D(_BaseMap, sampler_BaseMap, input.uv).a;
	AlphaClip(alpha, _Cutoff, input.positionCS.xyz, input.positionWS.xyz, _FadeParams);

	half ditherVal;
	Unity_Dither_float(clamp(_LodFade, 0, 1) * 2, input.positionCS, ditherVal);
	clip(alpha - max(_Cutoff, ditherVal));
#endif

	return 0;
}

#if VERSION_LOWER(10,0)
//Basically does nothing
half4 DepthNormalsFragment(Varyings input) : SV_TARGET
{
	return DepthOnlyFragment(input);
}
#else
half4 DepthNormalsFragment(Varyings input) : SV_TARGET
{
	UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(input);
	
	#ifdef _ALPHATEST_ON
	float alpha = SAMPLE_TEXTURE2D(_BaseMap, sampler_BaseMap, input.uv).a;
	AlphaClip(alpha, _Cutoff, input.positionCS.xyz, input.positionWS.xyz, _FadeParams);
	#endif

	//Normals of mesh are pointing upwards, or are slightly spherical. Derive normal perpendicular to face.
	float3 dpdx = ddx(input.positionWS.xyz);
	float3 dpdy = ddy(input.positionWS.xyz);
	float3 normalWS = normalize(cross(dpdy, dpdx));
	
	normalWS = NormalizeNormalPerPixel(normalWS);
	return half4(normalWS, 0.0);
}
#endif