//Stylized Grass Shader
//Staggart Creations (http://staggart.xyz)
//Copyright protected under Unity Asset Store EULA

//Single channel overlay
float BlendOverlay(float a, float b)
{
	return (b < 0.5) ? 2.0 * a * b : 1.0 - 2.0 * (1.0 - a) * (1.0 - b);
}

//RGB overlay
float3 BlendOverlay(float3 a, float3 b)
{
	float3 color;
	color.r = BlendOverlay(a.r, b.r);
	color.g = BlendOverlay(a.g, b.g);
	color.b = BlendOverlay(a.b, b.b);
	return color;
}

float4 SampleColorMapTexture(in float3 wPos, Light mainLight) 
{
	float2 uv = GetColorMapUV(wPos);
	return SAMPLE_TEXTURE2D(_ColorMap, sampler_ColorMap, uv).rgba;
}

float4 SampleTerrainColor(in float3 wPos, Light mainLight) 
{
	// Get terrain color
	float4 albedo;
	float3 normal;

	float3 AbsoluteWorldSpacePosition = GetAbsolutePositionWS(wPos).xyz;
	interpolateHexTerrainLayers_float(
		float3(_HexTerrainSamplingOffset.x,0,_HexTerrainSamplingOffset.y) + AbsoluteWorldSpacePosition,
		_HexTerrainTextureMap,
		1 / _HexTerrainTextureMap_TexelSize.zw,
		s_point_repeat_sampler,

		_HexTerrainTextures,
		_HexTerrainTextures, // normal maps input (not needed here so just passing in textures again)
		_HexTerrainTextures2,
		_HexTerrainTextures2, // normal maps input (not needed here so just passing in textures again)
		s_point_repeat_sampler,

		_HexTerrainTextureParameters,
		1 / _HexTerrainTextureParameters_TexelSize.zw,
		albedo,
		normal
	);
	return albedo;
}

//---------------------------------------------------------------//

//Shading (RGB=hue - A=brightness)
float4 ApplyVertexColor(in float4 vertexPos, in float3 wPos, in float3 baseColor, in float mask, in float aoAmount, in float darkening, in float4 hue, in float posOffset)
{
	float4 col = float4(baseColor, 1);

	//Apply hue
	col.rgb = lerp(col.rgb, hue.rgb, posOffset * hue.a);
	//Apply darkening
	float rand = frac(vertexPos.r * 4);

	float vertexDarkening = lerp(col.a, col.a * rand, darkening * mask); //Only apply to top vertices
	//Apply ambient occlusion
	float ambientOcclusion = lerp(col.a, col.a * mask, aoAmount);

	col.rgb *= vertexDarkening * ambientOcclusion;

	//Pass vertex color alpha-channel to fragment stage. Used in some shading functions such as translucency
	col.a = mask;

	return col;
}

float3 ApplyAmbientOcclusion(in float3 color, in float mask, in float amount) {
	return lerp(color, color * mask, amount);
}

float3 ApplyDarkening(in float3 vertexPos, in float3 color, in float amount)
{
	float rand = frac(vertexPos.r * 4.0);

	return lerp(color, color * rand, amount);
}

float3 ApplyColorMap(float3 wPos, float3 iColor, float s, Light mainLight) 
{
	return lerp(iColor, SampleColorMapTexture(wPos, mainLight).rgb, s);
}

float3 ApplyTerrainColor(float3 wPos, float3 iColor, float s, Light mainLight) 
{
	return lerp(iColor, SampleTerrainColor(wPos, mainLight).rgb, s);
}

//Apply object and vertex hue colors
float3 ApplyHue(in float4 iColor, in float3 oColor)
{
	return lerp(oColor, iColor.rgb, ObjectPosRand01() * iColor.a);
}

void ApplyObjectHueVariation(in float4 hue, in float3 color, out float3 output) {
	output = lerp(color.rgb, hue.rgb, hue.a * ObjectPosRand01());
}