void ToonShading_float(in float3 Normal, in float ToonRampSmoothness, in float3 ClipSpacePos, in float3 WorldPos, in float3 ToonRampTinting,
in float ToonRampOffset, in float ToonRampOffsetPoint, in float Ambient, out float3 ToonRampOutput, out float3 Direction)
{

	// set the shader graph node previews
	#ifdef SHADERGRAPH_PREVIEW
		ToonRampOutput = float3(0.5,0.5,0);
		Direction = float3(0.5,0.5,0);
	#else

		// grab the shadow coordinates
		#if SHADOWS_SCREEN
			half4 shadowCoord = ComputeScreenPos(ClipSpacePos);
		#else
			half4 shadowCoord = TransformWorldToShadowCoord(WorldPos);
		#endif 
		// grab the main light
		#if _MAIN_LIGHT_SHADOWS_CASCADE || _MAIN_LIGHT_SHADOWS
			Light light = GetMainLight(shadowCoord);
		#else
			Light light = GetMainLight();
		#endif

		// dot product for toonramp
		half d = dot(Normal, light.direction) * 0.5 + 0.5;
		
		
		// toonramp in a smoothstep
		half toonRamp = smoothstep(ToonRampOffset, ToonRampOffset+ ToonRampSmoothness, d );
		
		

		float3 extraLights;
		// get the number of point/spot lights
		int pixelLightCount = GetAdditionalLightsCount();
		// loop over every light
		for (int j = 0; j < pixelLightCount; ++j) {
			// grab the point light
			// If you get an error here V remove the ", half4(1,1,1,1)" part
			Light aLight = GetAdditionalLight(j, WorldPos, half4(1, 1, 1, 1));
			
			// grab the light, shadows ,and light color
			float3 attenuatedLightColor = aLight.color * (aLight.distanceAttenuation * aLight.shadowAttenuation);
			
			// dot product for toonramp
			half d = dot(Normal, aLight.direction) * 0.5 + 0.5;
			
			// toonramp in a smoothstep
			half toonRampExtra = smoothstep(ToonRampOffsetPoint, ToonRampOffsetPoint+ ToonRampSmoothness, d );

			// add them all together
			extraLights += (attenuatedLightColor * toonRampExtra);
		}
		
		// multiply with shadows;
		toonRamp *= light.shadowAttenuation;

		// add in lights and extra tinting
		ToonRampOutput = light.color * (toonRamp + ToonRampTinting)  + Ambient;

		// also add in point/spot lights
		ToonRampOutput += extraLights;
		// output direction for rimlight
		
		#if MAIN_LIGHT
			Direction = normalize(light.direction);
		#else
		// if no main light, use a side down angle
			Direction = float3(0.5,0.5,0);
		#endif

	#endif
}