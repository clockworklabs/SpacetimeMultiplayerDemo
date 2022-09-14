//Stylized Grass Shader
//Staggart Creations (http://staggart.xyz)
//Copyright protected under Unity Asset Store EULA

//Global parameters
float4 _BendMapUV;
TEXTURE2D(_BendMap); SAMPLER(sampler_BendMap);
float4 _BendMap_TexelSize;
float4 _ScaleBiasRT;

struct BendSettings
{
	uint mode;
	float mask;
	float pushStrength;
	float flattenStrength;
	float perspectiveCorrection;
};

BendSettings PopulateBendSettings(uint mode, float mask, float pushStrength, float flattenStrength, float perspCorrection)
{
	BendSettings s = (BendSettings)0;

	s.mode = mode;
	s.mask = mask;
	s.pushStrength = pushStrength;
	s.flattenStrength = flattenStrength;
	s.perspectiveCorrection = perspCorrection;

	return s;
}

//Bend map UV
float2 GetBendMapUV(in float3 wPos) {
	float2 uv = _BendMapUV.xy / _BendMapUV.z + (_BendMapUV.z / (_BendMapUV.z * _BendMapUV.z)) * wPos.xz;

#ifdef FLIP_UV
	uv.y = 1 - uv.y;
#endif

	return uv;			
}

//Texture sampling
float4 GetBendVector(float3 wPos) 
{
	if (_BendMapUV.w == 0) return float4(0.5, wPos.y, 0.5, 0.0);

	float2 uv = GetBendMapUV(wPos);

	float4 v = SAMPLE_TEXTURE2D(_BendMap, sampler_BendMap, uv).rgba;

	v.x = v.x * 2.0 - 1.0;
	v.z = v.z * 2.0 - 1.0;

	return v;
}

float4 GetBendVectorLOD(float3 wPos) 
{
	if (_BendMapUV.w == 0) return float4(0.5, wPos.y, 0.5, 0.0);

	float2 uv = GetBendMapUV(wPos);

	float4 v = SAMPLE_TEXTURE2D_LOD(_BendMap, sampler_BendMap, uv, 0).rgba;

	//Remap from 0.1 to -1.1
	v.x = v.x * 2.0 - 1.0;
	v.z = v.z * 2.0 - 1.0;

	return v;
}

float CreateDirMask(float2 uv) {
	float center = pow((uv.y * (1 - uv.y)) * 4, 4);

	return saturate(center);
}

//Creates a tube mask from the trail UV.y. Red vertex color represents lifetime strength
float CreateTrailMask(float2 uv, float lifetime)
{
	float center = saturate((uv.y * (1.0 - uv.y)) * 8.0);

	//Mask out the start of the trail, avoids grass instantly bending (assumes UV mode is set to "Stretch")
	float tip = saturate(uv.x * 16.0);

	return center * lifetime * tip;
}

void CreateTrailMask_float(float2 uv, float lifetime, out float mask)
{
	mask = CreateTrailMask(uv, lifetime);
}

float EdgeMask(float2 uv) 
{
	return saturate(((1-uv.x) *  uv.y) * (uv.x * (1 - uv.y)) * 64);
}

void EdgeMask_float(float2 uv, out float mask)
{
	mask = EdgeMask(uv);
}

float4 GetBendOffset(float3 wPos, BendSettings b) {
	float4 vec = GetBendVectorLOD(wPos);

	float4 offset = float4(wPos, vec.a);

	float grassHeight = wPos.y;
	float bendHeight = vec.y;
	float dist = grassHeight - bendHeight;

	//Note since 7.1.5 somehow this causes the grass to bend down after the bender reaches a certain height
	//dist = abs(dist); //If bender is below grass, dont bend up

	float weight = saturate(dist);

	offset.xz = vec.xz * b.mask * weight * b.pushStrength;
	offset.y = b.mask * (vec.a * 0.75) * weight * b.flattenStrength;

	float influence = 1;

	//Pass the mask, so it can be used to lerp between wind and bend offset vectors
	offset.a = vec.a * weight * influence;

	//Apply mask
	offset.xyz *= offset.a;

	return offset;
}