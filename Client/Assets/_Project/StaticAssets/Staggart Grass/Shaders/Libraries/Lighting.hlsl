//Stylized Grass Shader
//Staggart Creations (http://staggart.xyz)
//Copyright protected under Unity Asset Store EULA

void ApplyTranslucency(inout float3 color, float3 viewDirectionWS, Light light, float amount)
{
	float VdotL = saturate(dot(-viewDirectionWS, light.direction));
	VdotL = pow(VdotL, 4);

	//Translucency masked by shadows and grass mesh bottom
	float tMask = VdotL * light.shadowAttenuation * light.distanceAttenuation;

	//Fade the effect out as the sun approaches the horizon (75 to 90 degrees)
	half sunAngle = dot(float3(0, 1, 0), light.direction);
	half angleMask = saturate(sunAngle * 6.666); /* 1.0/0.15 = 6.666 */

	tMask *= angleMask;

	float3 tColor = color + BlendOverlay((light.color), color);
	color = lerp(color, tColor, tMask * (amount * 4.0));
}

//Blinn-phong shading with SH
half3 SimpleLighting(Light light, half3 normalWS, half3 bakedGI, half3 albedo, half occlusion, half3 emission)
{
	light.color *= light.distanceAttenuation * light.shadowAttenuation;

	half3 diffuseColor = bakedGI + LightingLambert(light.color, light.direction, normalWS);

	return (albedo * diffuseColor) + emission;
}

// General function to apply lighting based on the configured mode
half3 ApplyLighting(SurfaceData surfaceData, Light mainLight, half3 vertexLight, float2 lightmapUV, half3 normalWS, half3 positionWS, float2 normalizedScreenSpaceUV, half translucency)
{
	half3 color = 0;

#if defined(_SCREEN_SPACE_OCCLUSION) && VERSION_GREATER_EQUAL(10,0)
	AmbientOcclusionFactor aoFactor = GetScreenSpaceAmbientOcclusion(normalizedScreenSpaceUV);
	surfaceData.occlusion = min(surfaceData.occlusion, aoFactor.indirectAmbientOcclusion);

	#ifdef _UNLIT
	surfaceData.albedo *= min(surfaceData.occlusion, aoFactor.indirectAmbientOcclusion);
	#endif
	
	#endif

	#ifdef _UNLIT
	color = surfaceData.albedo;
	#endif

	half3 bakedGI = 0;
#ifndef _UNLIT
#ifdef LIGHTMAP_ON
	bakedGI = SampleLightmap(lightmapUV, normalWS);
#else
	//Spherical harmonics, skybox lighting and light probes
	bakedGI = SampleSH(normalWS);
#endif
	
	half3 viewDirectionWS = SafeNormalize(GetCameraPositionWS() - positionWS);

	#if defined(_SCREEN_SPACE_OCCLUSION) && VERSION_GREATER_EQUAL(10,0)
	mainLight.color *= aoFactor.directAmbientOcclusion;
	#endif
	
#if _ADVANCED_LIGHTING
	// BRDFData holds energy conserving diffuse and specular material reflections and its roughness.
	BRDFData brdfData;
	//Note: _SPECULARHIGHLIGHTS_OFF is forced off
	InitializeBRDFData(surfaceData.albedo, 0.0 /* metallic */, 0, surfaceData.smoothness, surfaceData.alpha, brdfData);

	// Mix diffuse GI with environment reflections.
	color = GlobalIllumination(brdfData, bakedGI, surfaceData.occlusion, normalWS, viewDirectionWS);

	// LightingPhysicallyBased computes direct light contribution.
#if VERSION_GREATER_EQUAL(9,0)
	color += LightingPhysicallyBased(brdfData, mainLight, normalWS, viewDirectionWS, true);
#else
	color += LightingPhysicallyBased(brdfData, mainLight, normalWS, viewDirectionWS);
#endif
#endif
	
#if _SIMPLE_LIGHTING
	#if defined(_SCREEN_SPACE_OCCLUSION) && VERSION_GREATER_EQUAL(10,0)
	//MixRealtimeAndBakedGI has no occlusion factor, multiply GI by occlusion to emulate the behaviour of LightingPhysicallyBased
	bakedGI *= surfaceData.occlusion;
	#endif
	//Simple diffuse and specular shading
	MixRealtimeAndBakedGI(mainLight, normalWS, bakedGI, 0);

	color = SimpleLighting(mainLight, normalWS, bakedGI, surfaceData.albedo.rgb, surfaceData.occlusion, surfaceData.emission);
#endif

#ifdef _ADDITIONAL_LIGHTS_VERTEX
	//Apply light color, previously calculated in vertex shader
	color += vertexLight;
#endif // Vertex lights

	// Additional lights loop per-pixel
#if _ADDITIONAL_LIGHTS

	// Returns the amount of lights affecting the object being renderer.
	// These lights are culled per-object in the forward renderer
	uint additionalLightsCount = GetAdditionalLightsCount();
	for (uint i = 0u; i < additionalLightsCount; ++i)
	{
		// Similar to GetMainLight, but it takes a for-loop index. This figures out the
		// per-object light index and samples the light buffer accordingly to initialized the
		// Light struct. If _ADDITIONAL_LIGHT_SHADOWS is defined it will also compute shadows.
		Light light = GetAdditionalLight(i, positionWS);

		#if defined(_SCREEN_SPACE_OCCLUSION) && VERSION_GREATER_EQUAL(10,0)
		light.color *= aoFactor.directAmbientOcclusion;
		#endif
		
#if _ADVANCED_LIGHTING
		// Same functions used to shade the main light.
#if VERSION_GREATER_EQUAL(9,0)
		color += LightingPhysicallyBased(brdfData, light, normalWS, viewDirectionWS, true);
#else
		color += LightingPhysicallyBased(brdfData, light, normalWS, viewDirectionWS);
#endif
		
		// Apply translucency for additional lights?
		//ApplyTranslucency(color, viewDirectionWS, light, translucency);
#endif
		
#if _SIMPLE_LIGHTING
		//Diffuse + specular lighting
		color += SimpleLighting(light, normalWS, bakedGI, surfaceData.albedo.rgb, surfaceData.occlusion, surfaceData.emission);
#endif
	}
#endif //Additional lights

	ApplyTranslucency(color, viewDirectionWS, mainLight, translucency);
#endif //No unlit

	// Emission (wind gust tint)
	color += surfaceData.emission;

	return color;
}