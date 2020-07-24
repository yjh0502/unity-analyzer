// PresetManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct PresetManager {
    pub m_ObjectHideFlags: usize,
    pub m_DefaultList: Vec<Struct_m_DefaultList>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_DefaultList {
    #[serde(rename = "type")]
    pub field_type: Struct_type,
    pub defaultPresets: Vec<Struct_defaultPresets>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_defaultPresets {
    pub m_Preset: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Preset {
    pub fileID: String,
    pub guid: Option<String>,
    #[serde(rename = "type")]
    pub field_type: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_type {
    pub m_NativeTypeID: usize,
    pub m_ManagedTypePPtr: Struct_m_Preset,
    pub m_ManagedTypeFallback: Option<()>,
}

// AudioManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AudioManager {
    pub m_ObjectHideFlags: usize,
    pub m_Volume: usize,
    #[serde(rename = "Rolloff Scale")]
    pub Rolloff_Scale: usize,
    #[serde(rename = "Doppler Factor")]
    pub Doppler_Factor: usize,
    #[serde(rename = "Default Speaker Mode")]
    pub Default_Speaker_Mode: usize,
    pub m_SampleRate: usize,
    pub m_DSPBufferSize: usize,
    pub m_VirtualVoiceCount: usize,
    pub m_RealVoiceCount: usize,
    pub m_SpatializerPlugin: Option<()>,
    pub m_AmbisonicDecoderPlugin: Option<()>,
    pub m_DisableAudio: usize,
    pub m_VirtualizeEffects: usize,
}

// Rigidbody
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Rigidbody {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub serializedVersion: usize,
    pub m_Mass: f64,
    pub m_Drag: f64,
    pub m_AngularDrag: f64,
    pub m_UseGravity: usize,
    pub m_IsKinematic: usize,
    pub m_Interpolate: usize,
    pub m_Constraints: usize,
    pub m_CollisionDetection: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
}

// RenderSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct RenderSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Fog: usize,
    pub m_FogColor: Struct_m_FogColor,
    pub m_FogMode: usize,
    pub m_FogDensity: f64,
    pub m_LinearFogStart: usize,
    pub m_LinearFogEnd: usize,
    pub m_AmbientSkyColor: Struct_m_FogColor,
    pub m_AmbientEquatorColor: Struct_m_FogColor,
    pub m_AmbientGroundColor: Struct_m_FogColor,
    pub m_AmbientIntensity: usize,
    pub m_AmbientMode: usize,
    pub m_SubtractiveShadowColor: Option<Struct_m_FogColor>,
    pub m_SkyboxMaterial: Struct_m_Preset,
    pub m_HaloStrength: f64,
    pub m_FlareStrength: usize,
    pub m_FlareFadeSpeed: usize,
    pub m_HaloTexture: Struct_m_Preset,
    pub m_SpotCookie: Struct_m_Preset,
    pub m_DefaultReflectionMode: usize,
    pub m_DefaultReflectionResolution: usize,
    pub m_ReflectionBounces: usize,
    pub m_ReflectionIntensity: usize,
    pub m_CustomReflection: Struct_m_Preset,
    pub m_Sun: Struct_m_Preset,
    pub m_IndirectSpecularColor: Option<Struct_m_FogColor>,
    pub m_UseRadianceAmbientProbe: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FogColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

// CapsuleCollider
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct CapsuleCollider {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Material: Struct_m_Preset,
    pub m_IsTrigger: usize,
    pub m_Enabled: usize,
    pub m_Radius: f64,
    pub m_Height: f64,
    pub m_Direction: usize,
    pub m_Center: Struct_m_Center,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Center {
    pub x: f64,
    pub y: f64,
}

// Camera
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Camera {
    pub m_ObjectHideFlags: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Option<Struct_m_Preset>,
    pub m_Enabled: Option<usize>,
    pub serializedVersion: Option<usize>,
    pub m_ClearFlags: Option<usize>,
    pub m_BackGroundColor: Option<Struct_m_FogColor>,
    pub m_NormalizedViewPortRect: Option<Struct_m_Center>,
    #[serde(rename = "near clip plane")]
    pub near_clip_plane: Option<f64>,
    #[serde(rename = "far clip plane")]
    pub far_clip_plane: Option<usize>,
    #[serde(rename = "field of view")]
    pub field_of_view: Option<f64>,
    pub orthographic: Option<usize>,
    #[serde(rename = "orthographic size")]
    pub orthographic_size: Option<f64>,
    pub m_Depth: Option<isize>,
    pub m_CullingMask: Option<Struct_m_CullingMask>,
    pub m_RenderingPath: Option<isize>,
    pub m_TargetTexture: Option<Struct_m_Preset>,
    pub m_TargetDisplay: Option<usize>,
    pub m_TargetEye: Option<usize>,
    pub m_HDR: Option<usize>,
    pub m_AllowMSAA: Option<usize>,
    pub m_AllowDynamicResolution: Option<usize>,
    pub m_ForceIntoRT: Option<usize>,
    pub m_OcclusionCulling: Option<usize>,
    pub m_StereoConvergence: Option<usize>,
    pub m_StereoSeparation: Option<f64>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_projectionMatrixMode: Option<usize>,
    pub m_SensorSize: Option<Struct_m_Center>,
    pub m_LensShift: Option<Struct_m_Center>,
    pub m_GateFitMode: Option<usize>,
    pub m_FocalLength: Option<usize>,
    pub m_StereoMirrorMode: Option<usize>,
    pub m_FOVAxisMode: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_CullingMask {
    pub serializedVersion: usize,
    pub m_Bits: usize,
}

// Terrain
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Terrain {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_TerrainData: Struct_m_Preset,
    pub m_TreeDistance: usize,
    pub m_TreeBillboardDistance: usize,
    pub m_TreeCrossFadeLength: usize,
    pub m_TreeMaximumFullLODCount: usize,
    pub m_DetailObjectDistance: usize,
    pub m_DetailObjectDensity: usize,
    pub m_HeightmapPixelError: usize,
    pub m_SplatMapDistance: usize,
    pub m_HeightmapMaximumLOD: usize,
    pub m_CastShadows: usize,
    pub m_DrawHeightmap: usize,
    pub m_DrawTreesAndFoliage: usize,
    pub m_ReflectionProbeUsage: usize,
    pub m_MaterialType: usize,
    pub m_LegacySpecular: Struct_m_LegacySpecular,
    pub m_LegacyShininess: f64,
    pub m_MaterialTemplate: Struct_m_Preset,
    pub m_BakeLightProbesForTrees: usize,
    pub m_ScaleInLightmap: f64,
    pub m_LightmapParameters: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_LegacySpecular {
    pub serializedVersion: usize,
    pub rgba: usize,
}

// LightmapSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct LightmapSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_GIWorkflowMode: usize,
    pub m_GISettings: Struct_m_GISettings,
    pub m_LightmapEditorSettings: Struct_m_LightmapEditorSettings,
    pub m_LightingDataAsset: Struct_m_Preset,
    pub m_UseShadowmask: Option<usize>,
    pub m_RuntimeCPUUsage: Option<usize>,
    pub m_LightmapsMode: Option<usize>,
    pub m_ShadowMaskMode: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_LightmapEditorSettings {
    pub serializedVersion: usize,
    pub m_Resolution: usize,
    pub m_BakeResolution: usize,
    pub m_TextureWidth: Option<usize>,
    pub m_TextureHeight: Option<usize>,
    pub m_AO: Option<usize>,
    pub m_AOMaxDistance: usize,
    pub m_CompAOExponent: usize,
    pub m_CompAOExponentDirect: Option<usize>,
    pub m_Padding: usize,
    pub m_LightmapParameters: Struct_m_Preset,
    pub m_LightmapsBakeMode: Option<usize>,
    pub m_TextureCompression: usize,
    pub m_FinalGather: usize,
    pub m_FinalGatherFiltering: Option<usize>,
    pub m_FinalGatherRayCount: usize,
    pub m_ReflectionCompression: usize,
    pub m_MixedBakeMode: Option<usize>,
    pub m_BakeBackend: Option<usize>,
    pub m_PVRSampling: Option<usize>,
    pub m_PVRDirectSampleCount: Option<usize>,
    pub m_PVRSampleCount: Option<usize>,
    pub m_PVRBounces: Option<usize>,
    pub m_PVRFilterTypeDirect: Option<usize>,
    pub m_PVRFilterTypeIndirect: Option<usize>,
    pub m_PVRFilterTypeAO: Option<usize>,
    pub m_PVRFilteringMode: Option<usize>,
    pub m_PVRCulling: Option<usize>,
    pub m_PVRFilteringGaussRadiusDirect: Option<usize>,
    pub m_PVRFilteringGaussRadiusIndirect: Option<usize>,
    pub m_PVRFilteringGaussRadiusAO: Option<usize>,
    pub m_PVRFilteringAtrousPositionSigmaDirect: Option<f64>,
    pub m_PVRFilteringAtrousPositionSigmaIndirect: Option<usize>,
    pub m_PVRFilteringAtrousPositionSigmaAO: Option<usize>,
    pub m_ShowResolutionOverlay: Option<usize>,
    pub m_AtlasSize: Option<usize>,
    pub m_DirectLightInLightProbes: Option<usize>,
    pub m_ExtractAmbientOcclusion: Option<usize>,
    pub m_PVREnvironmentSampleCount: Option<usize>,
    pub m_PVREnvironmentReferencePointCount: Option<usize>,
    pub m_PVRDenoiserTypeDirect: Option<usize>,
    pub m_PVRDenoiserTypeIndirect: Option<usize>,
    pub m_PVRDenoiserTypeAO: Option<usize>,
    pub m_PVREnvironmentMIS: Option<usize>,
    pub m_ExportTrainingData: Option<usize>,
    pub m_PVRFiltering: Option<usize>,
    pub m_PVRFilteringAtrousColorSigma: Option<usize>,
    pub m_PVRFilteringAtrousNormalSigma: Option<usize>,
    pub m_PVRFilteringAtrousPositionSigma: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GISettings {
    pub serializedVersion: usize,
    pub m_BounceScale: usize,
    pub m_IndirectOutputScale: usize,
    pub m_AlbedoBoost: usize,
    pub m_TemporalCoherenceThreshold: Option<usize>,
    pub m_EnvironmentLightingMode: usize,
    pub m_EnableBakedLightmaps: usize,
    pub m_EnableRealtimeLightmaps: usize,
}

// Physics2DSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Physics2DSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Gravity: Struct_m_Center,
    pub m_DefaultMaterial: Struct_m_Preset,
    pub m_VelocityIterations: usize,
    pub m_PositionIterations: usize,
    pub m_VelocityThreshold: usize,
    pub m_MaxLinearCorrection: f64,
    pub m_MaxAngularCorrection: usize,
    pub m_MaxTranslationSpeed: usize,
    pub m_MaxRotationSpeed: usize,
    pub m_BaumgarteScale: f64,
    pub m_BaumgarteTimeOfImpactScale: f64,
    pub m_TimeToSleep: f64,
    pub m_LinearSleepTolerance: f64,
    pub m_AngularSleepTolerance: usize,
    pub m_DefaultContactOffset: f64,
    pub m_AutoSimulation: usize,
    pub m_QueriesHitTriggers: usize,
    pub m_QueriesStartInColliders: usize,
    pub m_ChangeStopsCallbacks: usize,
    pub m_CallbacksOnDisable: usize,
    pub m_ReuseCollisionCallbacks: usize,
    pub m_AutoSyncTransforms: usize,
    pub m_AlwaysShowColliders: usize,
    pub m_ShowColliderSleep: usize,
    pub m_ShowColliderContacts: usize,
    pub m_ShowColliderAABB: usize,
    pub m_ContactArrowScale: f64,
    pub m_ColliderAwakeColor: Struct_m_FogColor,
    pub m_ColliderAsleepColor: Struct_m_FogColor,
    pub m_ColliderContactColor: Struct_m_FogColor,
    pub m_ColliderAABBColor: Struct_m_FogColor,
    pub m_LayerCollisionMatrix: String,
}

// TerrainLayer
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct TerrainLayer {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_Name: String,
    pub m_DiffuseTexture: Struct_m_Preset,
    pub m_NormalMapTexture: Struct_m_Preset,
    pub m_MaskMapTexture: Struct_m_Preset,
    pub m_TileSize: Struct_m_Center,
    pub m_TileOffset: Struct_m_Center,
    pub m_Specular: Struct_m_FogColor,
    pub m_Metallic: usize,
    pub m_Smoothness: usize,
    pub m_NormalScale: usize,
    pub m_DiffuseRemapMin: Struct_m_Center,
    pub m_DiffuseRemapMax: Struct_m_Center,
    pub m_MaskMapRemapMin: Struct_m_Center,
    pub m_MaskMapRemapMax: Struct_m_Center,
}

// Mesh
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Mesh {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_Name: Option<String>,
    pub serializedVersion: usize,
    pub m_SubMeshes: Vec<Struct_m_SubMeshes>,
    pub m_Shapes: Struct_m_Shapes,
    pub m_BindPose: Vec<::serde_yaml::Value>,
    pub m_BoneNameHashes: Option<()>,
    pub m_RootBoneNameHash: usize,
    pub m_BonesAABB: Option<Vec<::serde_yaml::Value>>,
    pub m_VariableBoneCountWeights: Option<Struct_m_VariableBoneCountWeights>,
    pub m_MeshCompression: usize,
    pub m_IsReadable: usize,
    pub m_KeepVertices: usize,
    pub m_KeepIndices: usize,
    pub m_IndexFormat: Option<usize>,
    pub m_IndexBuffer: String,
    pub m_VertexData: Struct_m_VertexData,
    pub m_CompressedMesh: Struct_m_CompressedMesh,
    pub m_LocalAABB: Struct_m_LocalAABB,
    pub m_MeshUsageFlags: usize,
    pub m_BakedConvexCollisionMesh: Option<()>,
    pub m_BakedTriangleCollisionMesh: Option<()>,
    #[serde(rename = "m_MeshMetrics[0]")]
    pub m_MeshMetrics_0_: Option<usize>,
    #[serde(rename = "m_MeshMetrics[1]")]
    pub m_MeshMetrics_1_: Option<usize>,
    pub m_MeshOptimizationFlags: Option<usize>,
    pub m_StreamData: Option<Struct_m_StreamData>,
    pub m_MeshOptimized: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_Skin: Option<Vec<::serde_yaml::Value>>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_StreamData {
    pub offset: usize,
    pub size: usize,
    pub path: Option<()>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_LocalAABB {
    pub m_Center: Struct_m_Center,
    pub m_Extent: Struct_m_Center,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_CompressedMesh {
    pub m_Vertices: Struct_m_Vertices,
    pub m_UV: Struct_m_Vertices,
    pub m_Normals: Struct_m_Vertices,
    pub m_Tangents: Struct_m_Vertices,
    pub m_Weights: Struct_m_Weights,
    pub m_NormalSigns: Struct_m_Weights,
    pub m_TangentSigns: Struct_m_Weights,
    pub m_FloatColors: Struct_m_Vertices,
    pub m_BoneIndices: Struct_m_Weights,
    pub m_Triangles: Struct_m_Weights,
    pub m_UVInfo: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Weights {
    pub m_NumItems: usize,
    pub m_Data: Option<()>,
    pub m_BitSize: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Vertices {
    pub m_NumItems: usize,
    pub m_Range: usize,
    pub m_Start: usize,
    pub m_Data: Option<()>,
    pub m_BitSize: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_VertexData {
    pub serializedVersion: Option<usize>,
    pub m_VertexCount: usize,
    pub m_Channels: Vec<Struct_m_Channels>,
    pub m_DataSize: usize,
    pub _typelessdata: String,
    pub m_CurrentChannels: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Channels {
    pub stream: usize,
    pub offset: usize,
    pub format: usize,
    pub dimension: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_VariableBoneCountWeights {
    pub m_Data: Option<()>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Shapes {
    pub vertices: Vec<::serde_yaml::Value>,
    pub shapes: Vec<::serde_yaml::Value>,
    pub channels: Vec<::serde_yaml::Value>,
    pub fullWeights: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SubMeshes {
    pub serializedVersion: usize,
    pub firstByte: usize,
    pub indexCount: usize,
    pub topology: usize,
    pub baseVertex: Option<usize>,
    pub firstVertex: usize,
    pub vertexCount: usize,
    pub localAABB: Struct_m_LocalAABB,
}

// Material
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Material {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_Name: String,
    pub m_Shader: Struct_m_Preset,
    pub m_ShaderKeywords: Option<::serde_yaml::Value>,
    pub m_LightmapFlags: Option<usize>,
    pub m_CustomRenderQueue: isize,
    pub stringTagMap: Option<Struct_stringTagMap>,
    pub m_SavedProperties: Struct_m_SavedProperties,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_EnableInstancingVariants: Option<usize>,
    pub m_DoubleSidedGI: Option<usize>,
    pub disabledShaderPasses: Option<Vec<String>>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SavedProperties {
    pub serializedVersion: usize,
    pub m_TexEnvs: ::serde_yaml::Value,
    pub m_Floats: ::serde_yaml::Value,
    pub m_Colors: ::serde_yaml::Value,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_stringTagMap {
    pub RenderType: Option<String>,
}

// Animator
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Animator {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_Avatar: Struct_m_Preset,
    pub m_Controller: Struct_m_Preset,
    pub m_CullingMode: usize,
    pub m_UpdateMode: usize,
    pub m_ApplyRootMotion: usize,
    pub m_LinearVelocityBlending: usize,
    pub m_WarningMessage: Option<()>,
    pub m_HasTransformHierarchy: usize,
    pub m_AllowConstantClipSamplingOptimization: usize,
    pub m_KeepAnimatorControllerStateOnDisable: usize,
}

// VFXManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct VFXManager {
    pub m_ObjectHideFlags: usize,
    pub m_IndirectShader: Struct_m_Preset,
    pub m_CopyBufferShader: Struct_m_Preset,
    pub m_SortShader: Struct_m_Preset,
    pub m_RenderPipeSettingsPath: Option<()>,
    pub m_FixedTimeStep: String,
    pub m_MaxDeltaTime: f64,
}

// TextMesh
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct TextMesh {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Text: String,
    pub m_OffsetZ: usize,
    pub m_CharacterSize: f64,
    pub m_LineSpacing: usize,
    pub m_Anchor: usize,
    pub m_Alignment: usize,
    pub m_TabSize: usize,
    pub m_FontSize: usize,
    pub m_FontStyle: usize,
    pub m_RichText: usize,
    pub m_Font: Struct_m_Preset,
    pub m_Color: Struct_m_LegacySpecular,
}

// TagManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct TagManager {
    pub serializedVersion: usize,
    pub tags: Vec<::serde_yaml::Value>,
    pub layers: Vec<Option<String>>,
    pub m_SortingLayers: Vec<Struct_m_SortingLayers>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SortingLayers {
    pub name: String,
    pub uniqueID: usize,
    pub locked: usize,
}

// GraphicsSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct GraphicsSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Deferred: Struct_m_Deferred,
    pub m_DeferredReflections: Struct_m_Deferred,
    pub m_ScreenSpaceShadows: Struct_m_Deferred,
    pub m_LegacyDeferred: Struct_m_Deferred,
    pub m_DepthNormals: Struct_m_Deferred,
    pub m_MotionVectors: Struct_m_Deferred,
    pub m_LightHalo: Struct_m_Deferred,
    pub m_LensFlare: Struct_m_Deferred,
    pub m_AlwaysIncludedShaders: Vec<Struct_m_Preset>,
    pub m_PreloadedShaders: Vec<::serde_yaml::Value>,
    pub m_SpritesDefaultMaterial: Struct_m_Preset,
    pub m_CustomRenderPipeline: Struct_m_Preset,
    pub m_TransparencySortMode: usize,
    pub m_TransparencySortAxis: Struct_m_Center,
    pub m_DefaultRenderingPath: usize,
    pub m_DefaultMobileRenderingPath: usize,
    pub m_TierSettings: Vec<::serde_yaml::Value>,
    pub m_LightmapStripping: usize,
    pub m_FogStripping: usize,
    pub m_InstancingStripping: usize,
    pub m_LightmapKeepPlain: usize,
    pub m_LightmapKeepDirCombined: usize,
    pub m_LightmapKeepDynamicPlain: usize,
    pub m_LightmapKeepDynamicDirCombined: usize,
    pub m_LightmapKeepShadowMask: usize,
    pub m_LightmapKeepSubtractive: usize,
    pub m_FogKeepLinear: usize,
    pub m_FogKeepExp: usize,
    pub m_FogKeepExp2: usize,
    pub m_AlbedoSwatchInfos: Vec<::serde_yaml::Value>,
    pub m_LightsUseLinearIntensity: usize,
    pub m_LightsUseColorTemperature: usize,
    pub m_LogWhenShaderIsCompiled: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Deferred {
    pub m_Mode: usize,
    pub m_Shader: Struct_m_Preset,
}

// TimeManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct TimeManager {
    pub m_ObjectHideFlags: usize,
    #[serde(rename = "Fixed Timestep")]
    pub Fixed_Timestep: f64,
    #[serde(rename = "Maximum Allowed Timestep")]
    pub Maximum_Allowed_Timestep: f64,
    pub m_TimeScale: usize,
    #[serde(rename = "Maximum Particle Timestep")]
    pub Maximum_Particle_Timestep: f64,
}

// Projector
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Projector {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_NearClipPlane: f64,
    pub m_FarClipPlane: usize,
    pub m_FieldOfView: usize,
    pub m_AspectRatio: usize,
    pub m_Orthographic: usize,
    pub m_OrthographicSize: f64,
    pub m_Material: Struct_m_Preset,
    pub m_IgnoreLayers: Struct_m_CullingMask,
}

// EditorSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct EditorSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_ExternalVersionControlSupport: String,
    pub m_SerializationMode: usize,
    pub m_LineEndingsForNewScripts: usize,
    pub m_DefaultBehaviorMode: usize,
    pub m_PrefabRegularEnvironment: Struct_m_Preset,
    pub m_PrefabUIEnvironment: Struct_m_Preset,
    pub m_SpritePackerMode: usize,
    pub m_SpritePackerPaddingPower: usize,
    pub m_EtcTextureCompressorBehavior: usize,
    pub m_EtcTextureFastCompressor: usize,
    pub m_EtcTextureNormalCompressor: usize,
    pub m_EtcTextureBestCompressor: usize,
    pub m_ProjectGenerationIncludedExtensions: String,
    pub m_ProjectGenerationRootNamespace: Option<()>,
    pub m_CollabEditorSettings: Struct_m_CollabEditorSettings,
    pub m_EnableTextureStreamingInPlayMode: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_CollabEditorSettings {
    pub inProgressEnabled: usize,
}

// NavMeshProjectSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct NavMeshProjectSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub areas: Vec<Struct_areas>,
    pub m_LastAgentTypeID: String,
    pub m_Settings: Vec<Struct_m_Settings>,
    pub m_SettingNames: Vec<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings {
    pub serializedVersion: usize,
    pub agentTypeID: ::serde_yaml::Value,
    pub agentRadius: f64,
    pub agentHeight: usize,
    pub agentSlope: usize,
    pub agentClimb: f64,
    pub ledgeDropHeight: usize,
    pub maxJumpAcrossDistance: usize,
    pub minRegionArea: usize,
    pub manualCellSize: usize,
    pub cellSize: f64,
    pub manualTileSize: usize,
    pub tileSize: usize,
    pub accuratePlacement: usize,
    pub debug: Struct_debug,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_debug {
    pub m_Flags: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_areas {
    pub name: Option<String>,
    pub cost: usize,
}

// AnimationClip
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AnimationClip {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_Name: String,
    pub serializedVersion: usize,
    pub m_Legacy: usize,
    pub m_Compressed: usize,
    pub m_UseHighQualityCurve: usize,
    pub m_RotationCurves: Vec<Struct_m_RotationCurves>,
    pub m_CompressedRotationCurves: Vec<::serde_yaml::Value>,
    pub m_EulerCurves: Vec<Struct_m_EulerCurves>,
    pub m_PositionCurves: Vec<Struct_m_PositionCurves>,
    pub m_ScaleCurves: Vec<Struct_m_RotationCurves>,
    pub m_FloatCurves: Vec<Struct_m_FloatCurves>,
    pub m_PPtrCurves: Vec<Struct_m_PPtrCurves>,
    pub m_SampleRate: usize,
    pub m_WrapMode: usize,
    pub m_Bounds: Struct_m_LocalAABB,
    pub m_ClipBindingConstant: Struct_m_ClipBindingConstant,
    pub m_AnimationClipSettings: Struct_m_AnimationClipSettings,
    pub m_EditorCurves: Vec<Struct_m_EditorCurves>,
    pub m_EulerEditorCurves: Vec<Struct_m_EulerEditorCurves>,
    pub m_HasGenericRootTransform: usize,
    pub m_HasMotionFloatCurves: usize,
    pub m_Events: Vec<Struct_m_Events>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GenerateMotionCurves: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Events {
    pub time: ::serde_yaml::Value,
    pub functionName: String,
    pub data: Option<()>,
    pub objectReferenceParameter: Struct_m_Preset,
    pub floatParameter: usize,
    pub intParameter: usize,
    pub messageOptions: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_EulerEditorCurves {
    pub curve: Struct_curve,
    pub attribute: String,
    pub path: Option<String>,
    pub classID: usize,
    pub script: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: ::serde_yaml::Value,
    pub inSlope: ::serde_yaml::Value,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: ::serde_yaml::Value,
    pub outWeight: ::serde_yaml::Value,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_EditorCurves {
    pub curve: Struct_curve_1,
    pub attribute: String,
    pub path: Option<String>,
    pub classID: usize,
    pub script: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_1 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_1>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_1 {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: ::serde_yaml::Value,
    pub inSlope: ::serde_yaml::Value,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<::serde_yaml::Value>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_AnimationClipSettings {
    pub serializedVersion: usize,
    pub m_AdditiveReferencePoseClip: Struct_m_Preset,
    pub m_AdditiveReferencePoseTime: usize,
    pub m_StartTime: usize,
    pub m_StopTime: f64,
    pub m_OrientationOffsetY: usize,
    pub m_Level: usize,
    pub m_CycleOffset: usize,
    pub m_HasAdditiveReferencePose: usize,
    pub m_LoopTime: usize,
    pub m_LoopBlend: usize,
    pub m_LoopBlendOrientation: usize,
    pub m_LoopBlendPositionY: usize,
    pub m_LoopBlendPositionXZ: usize,
    pub m_KeepOriginalOrientation: usize,
    pub m_KeepOriginalPositionY: usize,
    pub m_KeepOriginalPositionXZ: usize,
    pub m_HeightFromFeet: usize,
    pub m_Mirror: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_ClipBindingConstant {
    pub genericBindings: Vec<Struct_genericBindings>,
    pub pptrCurveMapping: Vec<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_genericBindings {
    pub serializedVersion: Option<usize>,
    pub path: usize,
    pub attribute: usize,
    pub script: Struct_m_Preset,
    pub typeID: Option<usize>,
    pub customType: usize,
    pub isPPtrCurve: usize,
    pub classID: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PPtrCurves {
    pub curve: Vec<Struct_curve_2>,
    pub attribute: String,
    pub path: Option<()>,
    pub classID: usize,
    pub script: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_2 {
    pub time: f64,
    pub value: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FloatCurves {
    pub curve: Struct_curve_3,
    pub attribute: String,
    pub path: Option<String>,
    pub classID: usize,
    pub script: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_3 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_2>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_2 {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: ::serde_yaml::Value,
    pub inSlope: ::serde_yaml::Value,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: ::serde_yaml::Value,
    pub outWeight: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PositionCurves {
    pub curve: Struct_curve_4,
    pub path: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_4 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_3>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_3 {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: Struct_m_Center,
    pub inSlope: Struct_m_Center,
    pub outSlope: Struct_m_Center,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<Struct_m_Center>,
    pub outWeight: Option<Struct_m_Center>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_EulerCurves {
    pub curve: Struct_curve_4,
    pub path: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_RotationCurves {
    pub curve: Struct_curve_5,
    pub path: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_5 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_4>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_4 {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: Struct_m_Center,
    pub inSlope: Struct_m_Center,
    pub outSlope: Struct_m_Center,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: Struct_m_Center,
    pub outWeight: Struct_m_Center,
}

// InputManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct InputManager {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Axes: Vec<Struct_m_Axes>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Axes {
    pub serializedVersion: usize,
    pub m_Name: String,
    pub descriptiveName: Option<()>,
    pub descriptiveNegativeName: Option<()>,
    pub negativeButton: Option<String>,
    pub positiveButton: Option<String>,
    pub altNegativeButton: Option<String>,
    pub altPositiveButton: Option<String>,
    pub gravity: usize,
    pub dead: f64,
    pub sensitivity: f64,
    pub snap: usize,
    pub invert: usize,
    #[serde(rename = "type")]
    pub field_type: usize,
    pub axis: usize,
    pub joyNum: usize,
}

// QualitySettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct QualitySettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_CurrentQuality: usize,
    pub m_QualitySettings: Vec<Struct_m_QualitySettings>,
    pub m_PerPlatformDefaultQuality: Struct_m_PerPlatformDefaultQuality,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PerPlatformDefaultQuality {
    pub Android: usize,
    pub Standalone: usize,
    pub WebGL: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_QualitySettings {
    pub serializedVersion: usize,
    pub name: String,
    pub pixelLightCount: usize,
    pub shadows: usize,
    pub shadowResolution: usize,
    pub shadowProjection: usize,
    pub shadowCascades: usize,
    pub shadowDistance: usize,
    pub shadowNearPlaneOffset: usize,
    pub shadowCascade2Split: f64,
    pub shadowCascade4Split: Struct_m_Center,
    pub shadowmaskMode: usize,
    pub blendWeights: usize,
    pub textureQuality: usize,
    pub anisotropicTextures: usize,
    pub antiAliasing: usize,
    pub softParticles: usize,
    pub softVegetation: usize,
    pub realtimeReflectionProbes: usize,
    pub billboardsFaceCameraPosition: usize,
    pub vSyncCount: usize,
    pub lodBias: f64,
    pub maximumLODLevel: usize,
    pub streamingMipmapsActive: usize,
    pub streamingMipmapsAddAllCameras: usize,
    pub streamingMipmapsMemoryBudget: usize,
    pub streamingMipmapsRenderersPerFrame: usize,
    pub streamingMipmapsMaxLevelReduction: usize,
    pub streamingMipmapsMaxFileIORequests: usize,
    pub particleRaycastBudget: usize,
    pub asyncUploadTimeSlice: usize,
    pub asyncUploadBufferSize: usize,
    pub asyncUploadPersistentBuffer: usize,
    pub resolutionScalingFixedDPIFactor: usize,
    pub excludedTargetPlatforms: Vec<::serde_yaml::Value>,
}

// NetworkManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct NetworkManager {
    pub m_ObjectHideFlags: usize,
    pub m_DebugLevel: usize,
    pub m_Sendrate: usize,
    pub m_AssetToPrefab: Struct_m_AssetToPrefab,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_AssetToPrefab {}

// PlayerSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct PlayerSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub productGUID: String,
    pub AndroidProfiler: usize,
    pub AndroidFilterTouchesWhenObscured: usize,
    pub AndroidEnableSustainedPerformanceMode: usize,
    pub defaultScreenOrientation: usize,
    pub targetDevice: usize,
    pub useOnDemandResources: usize,
    pub accelerometerFrequency: usize,
    pub companyName: usize,
    pub productName: String,
    pub defaultCursor: Struct_m_Preset,
    pub cursorHotspot: Struct_m_Center,
    pub m_SplashScreenBackgroundColor: Struct_m_FogColor,
    pub m_ShowUnitySplashScreen: usize,
    pub m_ShowUnitySplashLogo: usize,
    pub m_SplashScreenOverlayOpacity: usize,
    pub m_SplashScreenAnimation: usize,
    pub m_SplashScreenLogoStyle: usize,
    pub m_SplashScreenDrawMode: usize,
    pub m_SplashScreenBackgroundAnimationZoom: usize,
    pub m_SplashScreenLogoAnimationZoom: usize,
    pub m_SplashScreenBackgroundLandscapeAspect: usize,
    pub m_SplashScreenBackgroundPortraitAspect: usize,
    pub m_SplashScreenBackgroundLandscapeUvs: Struct_m_Center,
    pub m_SplashScreenBackgroundPortraitUvs: Struct_m_Center,
    pub m_SplashScreenLogos: Vec<::serde_yaml::Value>,
    pub m_VirtualRealitySplashScreen: Struct_m_Preset,
    pub m_HolographicTrackingLossScreen: Struct_m_Preset,
    pub defaultScreenWidth: usize,
    pub defaultScreenHeight: usize,
    pub defaultScreenWidthWeb: usize,
    pub defaultScreenHeightWeb: usize,
    pub m_StereoRenderingPath: usize,
    pub m_ActiveColorSpace: usize,
    pub m_MTRendering: usize,
    pub m_StackTraceTypes: String,
    pub iosShowActivityIndicatorOnLoading: isize,
    pub androidShowActivityIndicatorOnLoading: usize,
    pub displayResolutionDialog: usize,
    pub iosUseCustomAppBackgroundBehavior: usize,
    pub iosAllowHTTPDownload: usize,
    pub allowedAutorotateToPortrait: usize,
    pub allowedAutorotateToPortraitUpsideDown: usize,
    pub allowedAutorotateToLandscapeRight: usize,
    pub allowedAutorotateToLandscapeLeft: usize,
    pub useOSAutorotation: usize,
    pub use32BitDisplayBuffer: usize,
    pub preserveFramebufferAlpha: usize,
    pub disableDepthAndStencilBuffers: usize,
    pub androidStartInFullscreen: usize,
    pub androidRenderOutsideSafeArea: usize,
    pub androidBlitType: usize,
    pub defaultIsNativeResolution: usize,
    pub macRetinaSupport: usize,
    pub runInBackground: usize,
    pub captureSingleScreen: usize,
    pub muteOtherAudioSources: usize,
    #[serde(rename = "Prepare IOS For Recording")]
    pub Prepare_IOS_For_Recording: usize,
    #[serde(rename = "Force IOS Speakers When Recording")]
    pub Force_IOS_Speakers_When_Recording: usize,
    pub deferSystemGesturesMode: usize,
    pub hideHomeButton: usize,
    pub submitAnalytics: usize,
    pub usePlayerLog: usize,
    pub bakeCollisionMeshes: usize,
    pub forceSingleInstance: usize,
    pub resizableWindow: usize,
    pub useMacAppStoreValidation: usize,
    pub macAppStoreCategory: String,
    pub gpuSkinning: usize,
    pub graphicsJobs: usize,
    pub xboxPIXTextureCapture: usize,
    pub xboxEnableAvatar: usize,
    pub xboxEnableKinect: usize,
    pub xboxEnableKinectAutoTracking: usize,
    pub xboxEnableFitness: usize,
    pub visibleInBackground: usize,
    pub allowFullscreenSwitch: usize,
    pub graphicsJobMode: usize,
    pub fullscreenMode: usize,
    pub xboxSpeechDB: usize,
    pub xboxEnableHeadOrientation: usize,
    pub xboxEnableGuest: usize,
    pub xboxEnablePIXSampling: usize,
    pub metalFramebufferOnly: usize,
    pub xboxOneResolution: usize,
    pub xboxOneSResolution: usize,
    pub xboxOneXResolution: usize,
    pub xboxOneMonoLoggingLevel: usize,
    pub xboxOneLoggingLevel: usize,
    pub xboxOneDisableEsram: usize,
    pub xboxOnePresentImmediateThreshold: usize,
    pub switchQueueCommandMemory: usize,
    pub switchQueueControlMemory: usize,
    pub switchQueueComputeMemory: usize,
    pub switchNVNShaderPoolsGranularity: usize,
    pub switchNVNDefaultPoolsGranularity: usize,
    pub switchNVNOtherPoolsGranularity: usize,
    pub vulkanEnableSetSRGBWrite: usize,
    pub m_SupportedAspectRatios: Struct_m_SupportedAspectRatios,
    pub bundleVersion: f64,
    pub preloadedAssets: Vec<::serde_yaml::Value>,
    pub metroInputSource: usize,
    pub wsaTransparentSwapchain: usize,
    pub m_HolographicPauseOnTrackingLoss: usize,
    pub xboxOneDisableKinectGpuReservation: usize,
    pub xboxOneEnable7thCore: usize,
    pub isWsaHolographicRemotingEnabled: usize,
    pub vrSettings: Struct_vrSettings,
    pub protectGraphicsMemory: usize,
    pub enableFrameTimingStats: usize,
    pub useHDRDisplay: usize,
    pub m_ColorGamuts: String,
    pub targetPixelDensity: usize,
    pub resolutionScalingMode: usize,
    pub androidSupportedAspectRatio: usize,
    pub androidMaxAspectRatio: f64,
    pub applicationIdentifier: Struct_applicationIdentifier,
    pub buildNumber: Struct_m_AssetToPrefab,
    pub AndroidBundleVersionCode: usize,
    pub AndroidMinSdkVersion: usize,
    pub AndroidTargetSdkVersion: usize,
    pub AndroidPreferredInstallLocation: usize,
    pub aotOptions: Option<()>,
    pub stripEngineCode: usize,
    pub iPhoneStrippingLevel: usize,
    pub iPhoneScriptCallOptimization: usize,
    pub ForceInternetPermission: usize,
    pub ForceSDCardPermission: usize,
    pub CreateWallpaper: usize,
    pub APKExpansionFiles: usize,
    pub keepLoadedShadersAlive: usize,
    pub StripUnusedMeshComponents: usize,
    pub VertexChannelCompressionMask: usize,
    pub iPhoneSdkVersion: usize,
    pub iOSTargetOSVersionString: f64,
    pub tvOSSdkVersion: usize,
    pub tvOSRequireExtendedGameController: usize,
    pub tvOSTargetOSVersionString: f64,
    pub uIPrerenderedIcon: usize,
    pub uIRequiresPersistentWiFi: usize,
    pub uIRequiresFullScreen: usize,
    pub uIStatusBarHidden: usize,
    pub uIExitOnSuspend: usize,
    pub uIStatusBarStyle: usize,
    pub iPhoneSplashScreen: Struct_m_Preset,
    pub iPhoneHighResSplashScreen: Struct_m_Preset,
    pub iPhoneTallHighResSplashScreen: Struct_m_Preset,
    pub iPhone47inSplashScreen: Struct_m_Preset,
    pub iPhone55inPortraitSplashScreen: Struct_m_Preset,
    pub iPhone55inLandscapeSplashScreen: Struct_m_Preset,
    pub iPhone58inPortraitSplashScreen: Struct_m_Preset,
    pub iPhone58inLandscapeSplashScreen: Struct_m_Preset,
    pub iPadPortraitSplashScreen: Struct_m_Preset,
    pub iPadHighResPortraitSplashScreen: Struct_m_Preset,
    pub iPadLandscapeSplashScreen: Struct_m_Preset,
    pub iPadHighResLandscapeSplashScreen: Struct_m_Preset,
    pub appleTVSplashScreen: Struct_m_Preset,
    pub appleTVSplashScreen2x: Struct_m_Preset,
    pub tvOSSmallIconLayers: Vec<::serde_yaml::Value>,
    pub tvOSSmallIconLayers2x: Vec<::serde_yaml::Value>,
    pub tvOSLargeIconLayers: Vec<::serde_yaml::Value>,
    pub tvOSLargeIconLayers2x: Vec<::serde_yaml::Value>,
    pub tvOSTopShelfImageLayers: Vec<::serde_yaml::Value>,
    pub tvOSTopShelfImageLayers2x: Vec<::serde_yaml::Value>,
    pub tvOSTopShelfImageWideLayers: Vec<::serde_yaml::Value>,
    pub tvOSTopShelfImageWideLayers2x: Vec<::serde_yaml::Value>,
    pub iOSLaunchScreenType: usize,
    pub iOSLaunchScreenPortrait: Struct_m_Preset,
    pub iOSLaunchScreenLandscape: Struct_m_Preset,
    pub iOSLaunchScreenBackgroundColor: Struct_m_LegacySpecular,
    pub iOSLaunchScreenFillPct: usize,
    pub iOSLaunchScreenSize: usize,
    pub iOSLaunchScreenCustomXibPath: Option<()>,
    pub iOSLaunchScreeniPadType: usize,
    pub iOSLaunchScreeniPadImage: Struct_m_Preset,
    pub iOSLaunchScreeniPadBackgroundColor: Struct_m_LegacySpecular,
    pub iOSLaunchScreeniPadFillPct: usize,
    pub iOSLaunchScreeniPadSize: usize,
    pub iOSLaunchScreeniPadCustomXibPath: Option<()>,
    pub iOSUseLaunchScreenStoryboard: usize,
    pub iOSLaunchScreenCustomStoryboardPath: Option<()>,
    pub iOSDeviceRequirements: Vec<::serde_yaml::Value>,
    pub iOSURLSchemes: Vec<::serde_yaml::Value>,
    pub iOSBackgroundModes: usize,
    pub iOSMetalForceHardShadows: usize,
    pub metalEditorSupport: usize,
    pub metalAPIValidation: usize,
    pub iOSRenderExtraFrameOnPause: usize,
    pub appleDeveloperTeamID: String,
    pub iOSManualSigningProvisioningProfileID: Option<()>,
    pub tvOSManualSigningProvisioningProfileID: Option<()>,
    pub iOSManualSigningProvisioningProfileType: usize,
    pub tvOSManualSigningProvisioningProfileType: usize,
    pub appleEnableAutomaticSigning: usize,
    pub iOSRequireARKit: usize,
    pub iOSAutomaticallyDetectAndAddCapabilities: usize,
    pub appleEnableProMotion: usize,
    pub clonedFromGUID: String,
    pub templatePackageId: String,
    pub templateDefaultScene: String,
    pub AndroidTargetArchitectures: usize,
    pub AndroidSplashScreenScale: usize,
    pub androidSplashScreen: Struct_m_Preset,
    pub AndroidKeystoreName: String,
    pub AndroidKeyaliasName: String,
    pub AndroidBuildApkPerCpuArchitecture: usize,
    pub AndroidTVCompatibility: usize,
    pub AndroidIsGame: usize,
    pub AndroidEnableTango: usize,
    pub androidEnableBanner: usize,
    pub androidUseLowAccuracyLocation: usize,
    pub m_AndroidBanners: Vec<Struct_m_AndroidBanners>,
    pub androidGamepadSupportLevel: usize,
    pub resolutionDialogBanner: Struct_m_Preset,
    pub m_BuildTargetIcons: Vec<Struct_m_BuildTargetIcons>,
    pub m_BuildTargetPlatformIcons: Vec<Struct_m_BuildTargetPlatformIcons>,
    pub m_BuildTargetBatching: Vec<Struct_m_BuildTargetBatching>,
    pub m_BuildTargetGraphicsAPIs: Vec<Struct_m_BuildTargetGraphicsAPIs>,
    pub m_BuildTargetVRSettings: Vec<Struct_m_BuildTargetVRSettings>,
    pub m_BuildTargetEnableVuforiaSettings: Vec<::serde_yaml::Value>,
    pub openGLRequireES31: usize,
    pub openGLRequireES31AEP: usize,
    pub m_TemplateCustomTags: Struct_m_AssetToPrefab,
    pub mobileMTRendering: Struct_mobileMTRendering,
    pub m_BuildTargetGroupLightmapEncodingQuality: Vec<::serde_yaml::Value>,
    pub m_BuildTargetGroupLightmapSettings: Vec<::serde_yaml::Value>,
    pub playModeTestRunnerEnabled: usize,
    pub runPlayModeTestAsEditModeTest: usize,
    pub actionOnDotNetUnhandledException: usize,
    pub enableInternalProfiler: usize,
    pub logObjCUncaughtExceptions: usize,
    pub enableCrashReportAPI: usize,
    pub cameraUsageDescription: Option<()>,
    pub locationUsageDescription: Option<()>,
    pub microphoneUsageDescription: Option<()>,
    pub switchNetLibKey: Option<()>,
    pub switchSocketMemoryPoolSize: usize,
    pub switchSocketAllocatorPoolSize: usize,
    pub switchSocketConcurrencyLimit: usize,
    pub switchScreenResolutionBehavior: usize,
    pub switchUseCPUProfiler: usize,
    pub switchApplicationID: usize,
    pub switchNSODependencies: Option<()>,
    pub switchTitleNames_0: Option<()>,
    pub switchTitleNames_1: Option<()>,
    pub switchTitleNames_2: Option<()>,
    pub switchTitleNames_3: Option<()>,
    pub switchTitleNames_4: Option<()>,
    pub switchTitleNames_5: Option<()>,
    pub switchTitleNames_6: Option<()>,
    pub switchTitleNames_7: Option<()>,
    pub switchTitleNames_8: Option<()>,
    pub switchTitleNames_9: Option<()>,
    pub switchTitleNames_10: Option<()>,
    pub switchTitleNames_11: Option<()>,
    pub switchTitleNames_12: Option<()>,
    pub switchTitleNames_13: Option<()>,
    pub switchTitleNames_14: Option<()>,
    pub switchPublisherNames_0: Option<()>,
    pub switchPublisherNames_1: Option<()>,
    pub switchPublisherNames_2: Option<()>,
    pub switchPublisherNames_3: Option<()>,
    pub switchPublisherNames_4: Option<()>,
    pub switchPublisherNames_5: Option<()>,
    pub switchPublisherNames_6: Option<()>,
    pub switchPublisherNames_7: Option<()>,
    pub switchPublisherNames_8: Option<()>,
    pub switchPublisherNames_9: Option<()>,
    pub switchPublisherNames_10: Option<()>,
    pub switchPublisherNames_11: Option<()>,
    pub switchPublisherNames_12: Option<()>,
    pub switchPublisherNames_13: Option<()>,
    pub switchPublisherNames_14: Option<()>,
    pub switchIcons_0: Struct_m_Preset,
    pub switchIcons_1: Struct_m_Preset,
    pub switchIcons_2: Struct_m_Preset,
    pub switchIcons_3: Struct_m_Preset,
    pub switchIcons_4: Struct_m_Preset,
    pub switchIcons_5: Struct_m_Preset,
    pub switchIcons_6: Struct_m_Preset,
    pub switchIcons_7: Struct_m_Preset,
    pub switchIcons_8: Struct_m_Preset,
    pub switchIcons_9: Struct_m_Preset,
    pub switchIcons_10: Struct_m_Preset,
    pub switchIcons_11: Struct_m_Preset,
    pub switchIcons_12: Struct_m_Preset,
    pub switchIcons_13: Struct_m_Preset,
    pub switchIcons_14: Struct_m_Preset,
    pub switchSmallIcons_0: Struct_m_Preset,
    pub switchSmallIcons_1: Struct_m_Preset,
    pub switchSmallIcons_2: Struct_m_Preset,
    pub switchSmallIcons_3: Struct_m_Preset,
    pub switchSmallIcons_4: Struct_m_Preset,
    pub switchSmallIcons_5: Struct_m_Preset,
    pub switchSmallIcons_6: Struct_m_Preset,
    pub switchSmallIcons_7: Struct_m_Preset,
    pub switchSmallIcons_8: Struct_m_Preset,
    pub switchSmallIcons_9: Struct_m_Preset,
    pub switchSmallIcons_10: Struct_m_Preset,
    pub switchSmallIcons_11: Struct_m_Preset,
    pub switchSmallIcons_12: Struct_m_Preset,
    pub switchSmallIcons_13: Struct_m_Preset,
    pub switchSmallIcons_14: Struct_m_Preset,
    pub switchManualHTML: Option<()>,
    pub switchAccessibleURLs: Option<()>,
    pub switchLegalInformation: Option<()>,
    pub switchMainThreadStackSize: usize,
    pub switchPresenceGroupId: Option<()>,
    pub switchLogoHandling: usize,
    pub switchReleaseVersion: usize,
    pub switchDisplayVersion: String,
    pub switchStartupUserAccount: usize,
    pub switchTouchScreenUsage: usize,
    pub switchSupportedLanguagesMask: usize,
    pub switchLogoType: usize,
    pub switchApplicationErrorCodeCategory: Option<()>,
    pub switchUserAccountSaveDataSize: usize,
    pub switchUserAccountSaveDataJournalSize: usize,
    pub switchApplicationAttribute: usize,
    pub switchCardSpecSize: isize,
    pub switchCardSpecClock: isize,
    pub switchRatingsMask: usize,
    pub switchRatingsInt_0: usize,
    pub switchRatingsInt_1: usize,
    pub switchRatingsInt_2: usize,
    pub switchRatingsInt_3: usize,
    pub switchRatingsInt_4: usize,
    pub switchRatingsInt_5: usize,
    pub switchRatingsInt_6: usize,
    pub switchRatingsInt_7: usize,
    pub switchRatingsInt_8: usize,
    pub switchRatingsInt_9: usize,
    pub switchRatingsInt_10: usize,
    pub switchRatingsInt_11: usize,
    pub switchRatingsInt_12: usize,
    pub switchLocalCommunicationIds_0: Option<()>,
    pub switchLocalCommunicationIds_1: Option<()>,
    pub switchLocalCommunicationIds_2: Option<()>,
    pub switchLocalCommunicationIds_3: Option<()>,
    pub switchLocalCommunicationIds_4: Option<()>,
    pub switchLocalCommunicationIds_5: Option<()>,
    pub switchLocalCommunicationIds_6: Option<()>,
    pub switchLocalCommunicationIds_7: Option<()>,
    pub switchParentalControl: usize,
    pub switchAllowsScreenshot: usize,
    pub switchAllowsVideoCapturing: usize,
    pub switchAllowsRuntimeAddOnContentInstall: usize,
    pub switchDataLossConfirmation: usize,
    pub switchUserAccountLockEnabled: usize,
    pub switchSystemResourceMemory: usize,
    pub switchSupportedNpadStyles: usize,
    pub switchNativeFsCacheSize: usize,
    pub switchIsHoldTypeHorizontal: usize,
    pub switchSupportedNpadCount: usize,
    pub switchSocketConfigEnabled: usize,
    pub switchTcpInitialSendBufferSize: usize,
    pub switchTcpInitialReceiveBufferSize: usize,
    pub switchTcpAutoSendBufferSizeMax: usize,
    pub switchTcpAutoReceiveBufferSizeMax: usize,
    pub switchUdpSendBufferSize: usize,
    pub switchUdpReceiveBufferSize: usize,
    pub switchSocketBufferEfficiency: usize,
    pub switchSocketInitializeEnabled: usize,
    pub switchNetworkInterfaceManagerInitializeEnabled: usize,
    pub switchPlayerConnectionEnabled: usize,
    pub ps4NPAgeRating: usize,
    pub ps4NPTitleSecret: Option<()>,
    pub ps4NPTrophyPackPath: Option<()>,
    pub ps4ParentalLevel: usize,
    pub ps4ContentID: String,
    pub ps4Category: usize,
    pub ps4MasterVersion: f64,
    pub ps4AppVersion: f64,
    pub ps4AppType: usize,
    pub ps4ParamSfxPath: Option<()>,
    pub ps4VideoOutPixelFormat: usize,
    pub ps4VideoOutInitialWidth: usize,
    pub ps4VideoOutBaseModeInitialWidth: usize,
    pub ps4VideoOutReprojectionRate: usize,
    pub ps4PronunciationXMLPath: Option<()>,
    pub ps4PronunciationSIGPath: Option<()>,
    pub ps4BackgroundImagePath: Option<()>,
    pub ps4StartupImagePath: Option<()>,
    pub ps4StartupImagesFolder: Option<()>,
    pub ps4IconImagesFolder: Option<()>,
    pub ps4SaveDataImagePath: Option<()>,
    pub ps4SdkOverride: Option<()>,
    pub ps4BGMPath: Option<()>,
    pub ps4ShareFilePath: Option<()>,
    pub ps4ShareOverlayImagePath: Option<()>,
    pub ps4PrivacyGuardImagePath: Option<()>,
    pub ps4NPtitleDatPath: Option<()>,
    pub ps4RemotePlayKeyAssignment: isize,
    pub ps4RemotePlayKeyMappingDir: Option<()>,
    pub ps4PlayTogetherPlayerCount: usize,
    pub ps4EnterButtonAssignment: usize,
    pub ps4ApplicationParam1: usize,
    pub ps4ApplicationParam2: usize,
    pub ps4ApplicationParam3: usize,
    pub ps4ApplicationParam4: usize,
    pub ps4DownloadDataSize: usize,
    pub ps4GarlicHeapSize: usize,
    pub ps4ProGarlicHeapSize: usize,
    pub ps4Passcode: String,
    pub ps4pnSessions: usize,
    pub ps4pnPresence: usize,
    pub ps4pnFriends: usize,
    pub ps4pnGameCustomData: usize,
    pub playerPrefsSupport: usize,
    pub enableApplicationExit: usize,
    pub resetTempFolder: usize,
    pub restrictedAudioUsageRights: usize,
    pub ps4UseResolutionFallback: usize,
    pub ps4ReprojectionSupport: usize,
    pub ps4UseAudio3dBackend: usize,
    pub ps4SocialScreenEnabled: usize,
    pub ps4ScriptOptimizationLevel: usize,
    pub ps4Audio3dVirtualSpeakerCount: usize,
    pub ps4attribCpuUsage: usize,
    pub ps4PatchPkgPath: Option<()>,
    pub ps4PatchLatestPkgPath: Option<()>,
    pub ps4PatchChangeinfoPath: Option<()>,
    pub ps4PatchDayOne: usize,
    pub ps4attribUserManagement: usize,
    pub ps4attribMoveSupport: usize,
    pub ps4attrib3DSupport: usize,
    pub ps4attribShareSupport: usize,
    pub ps4attribExclusiveVR: usize,
    pub ps4disableAutoHideSplash: usize,
    pub ps4videoRecordingFeaturesUsed: usize,
    pub ps4contentSearchFeaturesUsed: usize,
    pub ps4attribEyeToEyeDistanceSettingVR: usize,
    pub ps4IncludedModules: Vec<::serde_yaml::Value>,
    pub monoEnv: Option<()>,
    pub splashScreenBackgroundSourceLandscape: Struct_m_Preset,
    pub splashScreenBackgroundSourcePortrait: Struct_m_Preset,
    pub spritePackerPolicy: Option<()>,
    pub webGLMemorySize: usize,
    pub webGLExceptionSupport: usize,
    pub webGLNameFilesAsHashes: usize,
    pub webGLDataCaching: usize,
    pub webGLDebugSymbols: usize,
    pub webGLEmscriptenArgs: Option<()>,
    pub webGLModulesDirectory: Option<()>,
    pub webGLTemplate: String,
    pub webGLAnalyzeBuildSize: usize,
    pub webGLUseEmbeddedResources: usize,
    pub webGLCompressionFormat: usize,
    pub webGLLinkerTarget: usize,
    pub webGLThreadsSupport: usize,
    pub scriptingDefineSymbols: Struct_m_AssetToPrefab,
    pub platformArchitecture: Struct_m_AssetToPrefab,
    pub scriptingBackend: Struct_scriptingBackend,
    pub il2cppCompilerConfiguration: Struct_m_AssetToPrefab,
    pub managedStrippingLevel: Struct_scriptingBackend,
    pub incrementalIl2cppBuild: Struct_m_AssetToPrefab,
    pub allowUnsafeCode: usize,
    pub additionalIl2CppArgs: Option<()>,
    pub scriptingRuntimeVersion: usize,
    pub apiCompatibilityLevelPerPlatform: Struct_m_AssetToPrefab,
    pub m_RenderingPath: usize,
    pub m_MobileRenderingPath: usize,
    pub metroPackageName: String,
    pub metroPackageVersion: Option<()>,
    pub metroCertificatePath: Option<()>,
    pub metroCertificatePassword: Option<()>,
    pub metroCertificateSubject: Option<()>,
    pub metroCertificateIssuer: Option<()>,
    pub metroCertificateNotAfter: String,
    pub metroApplicationDescription: String,
    pub wsaImages: Struct_m_AssetToPrefab,
    pub metroTileShortName: Option<()>,
    pub metroTileShowName: usize,
    pub metroMediumTileShowName: usize,
    pub metroLargeTileShowName: usize,
    pub metroWideTileShowName: usize,
    pub metroSupportStreamingInstall: usize,
    pub metroLastRequiredScene: usize,
    pub metroDefaultTileSize: usize,
    pub metroTileForegroundText: usize,
    pub metroTileBackgroundColor: Struct_m_FogColor,
    pub metroSplashScreenBackgroundColor: Struct_m_FogColor,
    pub metroSplashScreenUseBackgroundColor: usize,
    pub platformCapabilities: Struct_m_AssetToPrefab,
    pub metroTargetDeviceFamilies: Struct_m_AssetToPrefab,
    pub metroFTAName: Option<()>,
    pub metroFTAFileTypes: Vec<::serde_yaml::Value>,
    pub metroProtocolName: Option<()>,
    pub metroCompilationOverrides: usize,
    pub XboxOneProductId: Option<()>,
    pub XboxOneUpdateKey: Option<()>,
    pub XboxOneSandboxId: Option<()>,
    pub XboxOneContentId: Option<()>,
    pub XboxOneTitleId: Option<()>,
    pub XboxOneSCId: Option<()>,
    pub XboxOneGameOsOverridePath: Option<()>,
    pub XboxOnePackagingOverridePath: Option<()>,
    pub XboxOneAppManifestOverridePath: Option<()>,
    pub XboxOneVersion: String,
    pub XboxOnePackageEncryption: usize,
    pub XboxOnePackageUpdateGranularity: usize,
    pub XboxOneDescription: Option<()>,
    pub XboxOneLanguage: Vec<String>,
    pub XboxOneCapability: Vec<::serde_yaml::Value>,
    pub XboxOneGameRating: Struct_m_AssetToPrefab,
    pub XboxOneIsContentPackage: usize,
    pub XboxOneEnableGPUVariability: usize,
    pub XboxOneSockets: Struct_m_AssetToPrefab,
    pub XboxOneSplashScreen: Struct_m_Preset,
    pub XboxOneAllowedProductIds: Vec<::serde_yaml::Value>,
    pub XboxOnePersistentLocalStorageSize: usize,
    pub XboxOneXTitleMemory: usize,
    pub xboxOneScriptCompiler: usize,
    pub XboxOneOverrideIdentityName: Option<()>,
    pub vrEditorSettings: Struct_vrEditorSettings,
    pub cloudServicesEnabled: Struct_cloudServicesEnabled,
    pub luminIcon: Struct_luminIcon,
    pub luminCert: Struct_luminCert,
    pub luminIsChannelApp: usize,
    pub luminVersion: Struct_luminVersion,
    pub facebookSdkVersion: String,
    pub facebookAppId: Option<()>,
    pub facebookCookies: usize,
    pub facebookLogging: usize,
    pub facebookStatus: usize,
    pub facebookXfbml: usize,
    pub facebookFrictionlessRequests: usize,
    pub apiCompatibilityLevel: usize,
    pub cloudProjectId: String,
    pub framebufferDepthMemorylessMode: usize,
    pub projectName: String,
    pub organizationId: String,
    pub cloudEnabled: usize,
    pub enableNativePlatformBackendsForNewInputSystem: usize,
    pub disableOldInputManagerSupport: usize,
    pub legacyClampBlendShapeWeights: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_luminVersion {
    pub m_VersionCode: usize,
    pub m_VersionName: Option<()>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_luminCert {
    pub m_CertPath: Option<()>,
    pub m_PrivateKeyPath: Option<()>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_luminIcon {
    pub m_Name: Option<()>,
    pub m_ModelFolderPath: Option<()>,
    pub m_PortalFolderPath: Option<()>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_cloudServicesEnabled {
    pub UNet: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_vrEditorSettings {
    pub daydream: Struct_daydream,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_daydream {
    pub daydreamIconForeground: Struct_m_Preset,
    pub daydreamIconBackground: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_scriptingBackend {
    pub Android: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_mobileMTRendering {
    pub Android: usize,
    pub iPhone: usize,
    pub tvOS: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BuildTargetVRSettings {
    pub m_BuildTarget: String,
    pub m_Enabled: usize,
    pub m_Devices: Vec<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BuildTargetGraphicsAPIs {
    pub m_BuildTarget: String,
    pub m_APIs: ::serde_yaml::Value,
    pub m_Automatic: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BuildTargetBatching {
    pub m_BuildTarget: String,
    pub m_StaticBatching: usize,
    pub m_DynamicBatching: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BuildTargetPlatformIcons {
    pub m_BuildTarget: String,
    pub m_Icons: Vec<Struct_m_Icons>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Icons {
    pub m_Textures: Vec<Struct_m_Preset>,
    pub m_Width: usize,
    pub m_Height: usize,
    pub m_Kind: usize,
    pub m_SubKind: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BuildTargetIcons {
    pub m_BuildTarget: Option<()>,
    pub m_Icons: Vec<Struct_m_Icons_1>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Icons_1 {
    pub serializedVersion: usize,
    pub m_Icon: Struct_m_Preset,
    pub m_Width: usize,
    pub m_Height: usize,
    pub m_Kind: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_AndroidBanners {
    pub width: usize,
    pub height: usize,
    pub banner: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_applicationIdentifier {
    pub Android: String,
    pub Standalone: String,
    pub iOS: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_vrSettings {
    pub cardboard: Struct_cardboard,
    pub daydream: Struct_daydream_1,
    pub hololens: Struct_hololens,
    pub oculus: Struct_oculus,
    pub enable360StereoCapture: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_oculus {
    pub sharedDepthBuffer: usize,
    pub dashSupport: usize,
    pub lowOverheadMode: usize,
    pub protectedContext: usize,
    pub v2Signing: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_hololens {
    pub depthFormat: usize,
    pub depthBufferSharingEnabled: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_daydream_1 {
    pub depthFormat: usize,
    pub useSustainedPerformanceMode: usize,
    pub enableVideoLayer: usize,
    pub useProtectedVideoMemory: usize,
    pub minimumSupportedHeadTracking: usize,
    pub maximumSupportedHeadTracking: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_cardboard {
    pub depthFormat: usize,
    pub enableTransitionView: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SupportedAspectRatios {
    #[serde(rename = "4:3")]
    pub field_4_3: usize,
    #[serde(rename = "5:4")]
    pub field_5_4: usize,
    #[serde(rename = "16:10")]
    pub field_16_10: usize,
    #[serde(rename = "16:9")]
    pub field_16_9: usize,
    pub Others: usize,
}

// Preset
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Preset {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_Name: String,
    pub m_TargetType: Struct_type,
    pub m_Properties: Vec<Struct_m_Properties>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Properties {
    pub target: Struct_m_Preset,
    pub propertyPath: String,
    pub value: Option<f64>,
    pub objectReference: Struct_m_Preset,
}

// OcclusionCullingSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct OcclusionCullingSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_OcclusionBakeSettings: Struct_m_OcclusionBakeSettings,
    pub m_SceneGUID: String,
    pub m_OcclusionCullingData: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_OcclusionBakeSettings {
    pub smallestOccluder: usize,
    pub smallestHole: f64,
    pub backfaceThreshold: usize,
}

// Animation
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Animation {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_Animation: Struct_m_Preset,
    pub m_Animations: Vec<Struct_m_Preset>,
    pub m_WrapMode: usize,
    pub m_PlayAutomatically: usize,
    pub m_AnimatePhysics: usize,
    pub m_CullingType: usize,
}

// NavMeshData
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct NavMeshData {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_Name: String,
    pub serializedVersion: usize,
    pub m_NavMeshTiles: Vec<Struct_m_NavMeshTiles>,
    pub m_NavMeshBuildSettings: Struct_m_NavMeshBuildSettings,
    pub m_Heightmaps: Vec<::serde_yaml::Value>,
    pub m_HeightMeshes: Vec<::serde_yaml::Value>,
    pub m_OffMeshLinks: Vec<::serde_yaml::Value>,
    pub m_SourceBounds: Struct_m_LocalAABB,
    pub m_Rotation: Struct_m_Center,
    pub m_Position: Struct_m_Center,
    pub m_AgentTypeID: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_NavMeshBuildSettings {
    pub serializedVersion: usize,
    pub agentTypeID: usize,
    pub agentRadius: f64,
    pub agentHeight: usize,
    pub agentSlope: usize,
    pub agentClimb: f64,
    pub ledgeDropHeight: usize,
    pub maxJumpAcrossDistance: usize,
    pub minRegionArea: usize,
    pub manualCellSize: usize,
    pub cellSize: f64,
    pub manualTileSize: usize,
    pub tileSize: usize,
    pub accuratePlacement: usize,
    pub debug: Struct_debug,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_NavMeshTiles {
    pub m_MeshData: String,
    pub m_Hash: Struct_m_Hash,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Hash {
    pub serializedVersion: usize,
    pub Hash: String,
}

// AnimatorStateMachine
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AnimatorStateMachine {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_Name: String,
    pub m_ChildStates: Vec<Struct_m_ChildStates>,
    pub m_ChildStateMachines: Vec<::serde_yaml::Value>,
    pub m_AnyStateTransitions: Vec<::serde_yaml::Value>,
    pub m_EntryTransitions: Vec<::serde_yaml::Value>,
    pub m_StateMachineTransitions: Struct_m_AssetToPrefab,
    pub m_StateMachineBehaviours: Vec<::serde_yaml::Value>,
    pub m_AnyStatePosition: Struct_m_Center,
    pub m_EntryPosition: Struct_m_Center,
    pub m_ExitPosition: Struct_m_Center,
    pub m_ParentStateMachinePosition: Struct_m_Center,
    pub m_DefaultState: Struct_m_Preset,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_ChildStates {
    pub serializedVersion: usize,
    pub m_State: Struct_m_Preset,
    pub m_Position: Struct_m_Center,
}

// ParticleSystemRenderer
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct ParticleSystemRenderer {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_CastShadows: usize,
    pub m_ReceiveShadows: usize,
    pub m_MotionVectors: usize,
    pub m_LightProbeUsage: usize,
    pub m_ReflectionProbeUsage: usize,
    pub m_Materials: Vec<Struct_m_Preset>,
    pub m_StaticBatchInfo: Struct_m_StaticBatchInfo,
    pub m_StaticBatchRoot: Struct_m_Preset,
    pub m_ProbeAnchor: Struct_m_Preset,
    pub m_LightProbeVolumeOverride: Struct_m_Preset,
    pub m_ScaleInLightmap: usize,
    pub m_PreserveUVs: usize,
    pub m_IgnoreNormalsForChartDetection: usize,
    pub m_ImportantGI: usize,
    pub m_SelectedEditorRenderState: Option<usize>,
    pub m_MinimumChartSize: usize,
    pub m_AutoUVMaxDistance: f64,
    pub m_AutoUVMaxAngle: usize,
    pub m_LightmapParameters: Struct_m_Preset,
    pub m_SortingLayerID: usize,
    pub m_SortingOrder: isize,
    pub m_RenderMode: usize,
    pub m_SortMode: usize,
    pub m_MinParticleSize: usize,
    pub m_MaxParticleSize: f64,
    pub m_CameraVelocityScale: usize,
    pub m_VelocityScale: f64,
    pub m_LengthScale: f64,
    pub m_SortingFudge: usize,
    pub m_NormalDirection: usize,
    pub m_RenderAlignment: usize,
    pub m_Pivot: Struct_m_Center,
    pub m_VertexStreamMask: Option<usize>,
    pub m_Mesh: Struct_m_Preset,
    pub m_Mesh1: Struct_m_Preset,
    pub m_Mesh2: Struct_m_Preset,
    pub m_Mesh3: Struct_m_Preset,
    pub serializedVersion: Option<usize>,
    pub m_DynamicOccludee: Option<usize>,
    pub m_StitchLightmapSeams: Option<usize>,
    pub m_SortingLayer: Option<usize>,
    pub m_UseCustomVertexStreams: Option<usize>,
    pub m_VertexStreams: Option<String>,
    pub m_MaskInteraction: Option<usize>,
    pub m_RenderingLayerMask: Option<usize>,
    pub m_EnableGPUInstancing: Option<usize>,
    pub m_SelectedWireframeHidden: Option<usize>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_RendererPriority: Option<usize>,
    pub m_ShadowBias: Option<usize>,
    pub m_Flip: Option<Struct_m_Center>,
    pub m_ApplyActiveColorSpace: Option<usize>,
    pub m_AllowRoll: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_StaticBatchInfo {
    pub firstSubMesh: usize,
    pub subMeshCount: usize,
}

// MeshFilter
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct MeshFilter {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Mesh: Struct_m_Preset,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// Light
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Light {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_Type: usize,
    pub m_Color: Struct_m_FogColor,
    pub m_Intensity: f64,
    pub m_Range: f64,
    pub m_SpotAngle: f64,
    pub m_CookieSize: usize,
    pub m_Shadows: Struct_m_Shadows,
    pub m_Cookie: Struct_m_Preset,
    pub m_DrawHalo: usize,
    pub m_Flare: Struct_m_Preset,
    pub m_RenderMode: usize,
    pub m_CullingMask: Struct_m_CullingMask,
    pub m_Lightmapping: usize,
    pub m_AreaSize: Struct_m_Center,
    pub m_BounceIntensity: f64,
    pub m_ShadowRadius: usize,
    pub m_ShadowAngle: usize,
    pub m_ColorTemperature: Option<usize>,
    pub m_UseColorTemperature: Option<usize>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_LightShadowCasterMode: Option<usize>,
    pub m_InnerSpotAngle: Option<f64>,
    pub m_RenderingLayerMask: Option<usize>,
    pub m_BoundingSphereOverride: Option<Struct_m_Center>,
    pub m_UseBoundingSphereOverride: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Shadows {
    pub m_Type: usize,
    pub m_Resolution: isize,
    pub m_CustomResolution: isize,
    pub m_Strength: f64,
    pub m_Bias: f64,
    pub m_NormalBias: f64,
    pub m_NearPlane: f64,
    pub m_CullingMatrixOverride: Option<Struct_m_CullingMatrixOverride>,
    pub m_UseCullingMatrixOverride: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_CullingMatrixOverride {
    pub e00: usize,
    pub e01: usize,
    pub e02: usize,
    pub e03: usize,
    pub e10: usize,
    pub e11: usize,
    pub e12: usize,
    pub e13: usize,
    pub e20: usize,
    pub e21: usize,
    pub e22: usize,
    pub e23: usize,
    pub e30: usize,
    pub e31: usize,
    pub e32: usize,
    pub e33: usize,
}

// ClusterInputManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct ClusterInputManager {
    pub m_ObjectHideFlags: usize,
    pub m_Inputs: Vec<::serde_yaml::Value>,
}

// GameObject
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct GameObject {
    pub m_ObjectHideFlags: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub serializedVersion: Option<usize>,
    pub m_Component: Option<Vec<Struct_m_Component>>,
    pub m_Layer: Option<usize>,
    pub m_Name: Option<String>,
    pub m_TagString: Option<String>,
    pub m_Icon: Option<Struct_m_Preset>,
    pub m_NavMeshLayer: Option<usize>,
    pub m_StaticEditorFlags: Option<usize>,
    pub m_IsActive: Option<usize>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Component {
    pub component: Option<Struct_m_Preset>,
}

// MeshCollider
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct MeshCollider {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Material: Struct_m_Preset,
    pub m_IsTrigger: usize,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_Convex: usize,
    pub m_CookingOptions: Option<usize>,
    pub m_SkinWidth: Option<f64>,
    pub m_Mesh: Struct_m_Preset,
    pub m_InflateMesh: Option<usize>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// SpriteRenderer
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SpriteRenderer {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_CastShadows: usize,
    pub m_ReceiveShadows: usize,
    pub m_MotionVectors: usize,
    pub m_LightProbeUsage: usize,
    pub m_ReflectionProbeUsage: usize,
    pub m_Materials: Vec<Struct_m_Preset>,
    pub m_StaticBatchInfo: Struct_m_StaticBatchInfo,
    pub m_StaticBatchRoot: Struct_m_Preset,
    pub m_ProbeAnchor: Struct_m_Preset,
    pub m_LightProbeVolumeOverride: Struct_m_Preset,
    pub m_ScaleInLightmap: usize,
    pub m_PreserveUVs: usize,
    pub m_IgnoreNormalsForChartDetection: usize,
    pub m_ImportantGI: usize,
    pub m_SelectedEditorRenderState: usize,
    pub m_MinimumChartSize: usize,
    pub m_AutoUVMaxDistance: f64,
    pub m_AutoUVMaxAngle: usize,
    pub m_LightmapParameters: Struct_m_Preset,
    pub m_SortingLayerID: usize,
    pub m_SortingOrder: usize,
    pub m_Sprite: Struct_m_Preset,
    pub m_Color: Struct_m_FogColor,
    pub m_FlipX: usize,
    pub m_FlipY: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_DynamicOccludee: Option<usize>,
    pub m_RenderingLayerMask: Option<usize>,
    pub m_RendererPriority: Option<usize>,
    pub m_StitchLightmapSeams: Option<usize>,
    pub m_SortingLayer: Option<usize>,
    pub m_DrawMode: Option<usize>,
    pub m_Size: Option<Struct_m_Center>,
    pub m_AdaptiveModeThreshold: Option<f64>,
    pub m_SpriteTileMode: Option<usize>,
    pub m_WasSpriteAssigned: Option<usize>,
    pub m_MaskInteraction: Option<usize>,
    pub m_SpriteSortPoint: Option<usize>,
}

// ConfigurableJoint
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct ConfigurableJoint {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_ConnectedBody: Struct_m_Preset,
    pub m_Anchor: Struct_m_Center,
    pub m_Axis: Struct_m_Center,
    pub m_AutoConfigureConnectedAnchor: usize,
    pub m_ConnectedAnchor: Struct_m_Center,
    pub serializedVersion: usize,
    pub m_SecondaryAxis: Struct_m_Center,
    pub m_XMotion: usize,
    pub m_YMotion: usize,
    pub m_ZMotion: usize,
    pub m_AngularXMotion: usize,
    pub m_AngularYMotion: usize,
    pub m_AngularZMotion: usize,
    pub m_LinearLimitSpring: Struct_m_LinearLimitSpring,
    pub m_LinearLimit: Struct_m_LinearLimit,
    pub m_AngularXLimitSpring: Struct_m_LinearLimitSpring,
    pub m_LowAngularXLimit: Struct_m_LinearLimit,
    pub m_HighAngularXLimit: Struct_m_LinearLimit,
    pub m_AngularYZLimitSpring: Struct_m_LinearLimitSpring,
    pub m_AngularYLimit: Struct_m_LinearLimit,
    pub m_AngularZLimit: Struct_m_LinearLimit,
    pub m_TargetPosition: Struct_m_Center,
    pub m_TargetVelocity: Struct_m_Center,
    pub m_XDrive: Struct_m_XDrive,
    pub m_YDrive: Struct_m_XDrive,
    pub m_ZDrive: Struct_m_XDrive,
    pub m_TargetRotation: Struct_m_Center,
    pub m_TargetAngularVelocity: Struct_m_Center,
    pub m_RotationDriveMode: usize,
    pub m_AngularXDrive: Struct_m_XDrive,
    pub m_AngularYZDrive: Struct_m_XDrive,
    pub m_SlerpDrive: Struct_m_SlerpDrive,
    pub m_ProjectionMode: usize,
    pub m_ProjectionDistance: f64,
    pub m_ProjectionAngle: usize,
    pub m_ConfiguredInWorldSpace: usize,
    pub m_SwapBodies: usize,
    pub m_BreakForce: String,
    pub m_BreakTorque: String,
    pub m_EnableCollision: usize,
    pub m_EnablePreprocessing: usize,
    pub m_MassScale: usize,
    pub m_ConnectedMassScale: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SlerpDrive {
    pub serializedVersion: usize,
    pub positionSpring: usize,
    pub positionDamper: f64,
    pub maximumForce: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_XDrive {
    pub serializedVersion: usize,
    pub positionSpring: usize,
    pub positionDamper: usize,
    pub maximumForce: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_LinearLimit {
    pub limit: usize,
    pub bounciness: usize,
    pub contactDistance: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_LinearLimitSpring {
    pub spring: usize,
    pub damper: usize,
}

// SceneSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SceneSettings {
    pub m_ObjectHideFlags: usize,
    pub m_PVSData: Option<()>,
    pub m_PVSObjectsArray: Vec<::serde_yaml::Value>,
    pub m_PVSPortalsArray: Vec<::serde_yaml::Value>,
    pub m_OcclusionBakeSettings: Struct_m_OcclusionBakeSettings,
}

// AudioSource
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AudioSource {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub OutputAudioMixerGroup: Struct_m_Preset,
    pub m_audioClip: Struct_m_Preset,
    pub m_PlayOnAwake: usize,
    pub m_Volume: usize,
    pub m_Pitch: usize,
    pub Loop: usize,
    pub Mute: usize,
    pub Spatialize: usize,
    pub SpatializePostEffects: usize,
    pub Priority: usize,
    pub DopplerLevel: usize,
    pub MinDistance: usize,
    pub MaxDistance: usize,
    pub Pan2D: usize,
    pub rolloffMode: usize,
    pub BypassEffects: usize,
    pub BypassListenerEffects: usize,
    pub BypassReverbZones: usize,
    pub rolloffCustomCurve: Struct_rolloffCustomCurve,
    pub panLevelCustomCurve: Struct_rolloffCustomCurve,
    pub spreadCustomCurve: Struct_rolloffCustomCurve,
    pub reverbZoneMixCustomCurve: Struct_rolloffCustomCurve,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_rolloffCustomCurve {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_5>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_5 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: usize,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: f64,
    pub outWeight: f64,
}

// NavMeshSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct NavMeshSettings {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_BuildSettings: Struct_m_BuildSettings,
    pub m_NavMeshData: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BuildSettings {
    pub serializedVersion: usize,
    pub agentTypeID: Option<usize>,
    pub agentRadius: f64,
    pub agentHeight: usize,
    pub agentSlope: usize,
    pub agentClimb: f64,
    pub ledgeDropHeight: usize,
    pub maxJumpAcrossDistance: usize,
    pub minRegionArea: usize,
    pub manualCellSize: usize,
    pub cellSize: f64,
    pub manualTileSize: Option<usize>,
    pub tileSize: Option<usize>,
    pub accuratePlacement: usize,
    pub debug: Option<Struct_debug>,
}

// SphereCollider
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SphereCollider {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Material: Struct_m_Preset,
    pub m_IsTrigger: usize,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_Radius: f64,
    pub m_Center: Struct_m_Center,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// RenderTexture
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct RenderTexture {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_Name: Option<String>,
    pub m_ImageContentsHash: Struct_m_Hash,
    pub m_ForcedFallbackFormat: usize,
    pub m_DownscaleFallback: usize,
    pub m_Width: usize,
    pub m_Height: usize,
    pub m_AntiAliasing: usize,
    pub m_DepthFormat: usize,
    pub m_ColorFormat: usize,
    pub m_MipMap: usize,
    pub m_GenerateMips: usize,
    pub m_SRGB: usize,
    pub m_UseDynamicScale: usize,
    pub m_BindMS: usize,
    pub m_TextureSettings: Struct_m_TextureSettings,
    pub m_Dimension: usize,
    pub m_VolumeDepth: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_TextureSettings {
    pub serializedVersion: usize,
    pub m_FilterMode: usize,
    pub m_Aniso: usize,
    pub m_MipBias: usize,
    pub m_WrapU: usize,
    pub m_WrapV: usize,
    pub m_WrapW: usize,
}

// EditorExtensionImpl
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct EditorExtensionImpl {
    pub serializedVersion: usize,
}

// AnimatorState
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AnimatorState {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_Name: String,
    pub m_Speed: usize,
    pub m_CycleOffset: usize,
    pub m_Transitions: Vec<Struct_m_Preset>,
    pub m_StateMachineBehaviours: Vec<::serde_yaml::Value>,
    pub m_Position: Struct_m_Center,
    pub m_IKOnFeet: usize,
    pub m_WriteDefaultValues: usize,
    pub m_Mirror: usize,
    pub m_SpeedParameterActive: usize,
    pub m_MirrorParameterActive: usize,
    pub m_CycleOffsetParameterActive: usize,
    pub m_TimeParameterActive: Option<usize>,
    pub m_Motion: Struct_m_Preset,
    pub m_Tag: Option<()>,
    pub m_SpeedParameter: Option<()>,
    pub m_MirrorParameter: Option<()>,
    pub m_CycleOffsetParameter: Option<()>,
    pub m_TimeParameter: Option<()>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
}

// PhysicsManager
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct PhysicsManager {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Gravity: Struct_m_Center,
    pub m_DefaultMaterial: Struct_m_Preset,
    pub m_BounceThreshold: usize,
    pub m_SleepThreshold: f64,
    pub m_DefaultContactOffset: f64,
    pub m_DefaultSolverIterations: usize,
    pub m_DefaultSolverVelocityIterations: usize,
    pub m_QueriesHitBackfaces: usize,
    pub m_QueriesHitTriggers: usize,
    pub m_EnableAdaptiveForce: usize,
    pub m_ClothInterCollisionDistance: usize,
    pub m_ClothInterCollisionStiffness: usize,
    pub m_ContactsGeneration: usize,
    pub m_LayerCollisionMatrix: String,
    pub m_AutoSimulation: usize,
    pub m_AutoSyncTransforms: usize,
    pub m_ReuseCollisionCallbacks: usize,
    pub m_ClothInterCollisionSettingsToggle: usize,
    pub m_ContactPairsMode: usize,
    pub m_BroadphaseType: usize,
    pub m_WorldBounds: Struct_m_LocalAABB,
    pub m_WorldSubdivisions: usize,
    pub m_FrictionType: usize,
    pub m_EnableEnhancedDeterminism: usize,
    pub m_EnableUnifiedHeightmaps: usize,
}

// BoxCollider
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct BoxCollider {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Material: Struct_m_Preset,
    pub m_IsTrigger: usize,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_Size: Struct_m_Center,
    pub m_Center: Struct_m_Center,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// UnityConnectSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct UnityConnectSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Enabled: usize,
    pub m_TestMode: usize,
    pub m_EventOldUrl: String,
    pub m_EventUrl: String,
    pub m_ConfigUrl: String,
    pub m_TestInitMode: usize,
    pub CrashReportingSettings: Struct_CrashReportingSettings,
    pub UnityPurchasingSettings: Struct_UnityPurchasingSettings,
    pub UnityAnalyticsSettings: Struct_UnityAnalyticsSettings,
    pub UnityAdsSettings: Struct_UnityAdsSettings,
    pub PerformanceReportingSettings: Struct_PerformanceReportingSettings,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_PerformanceReportingSettings {
    pub m_Enabled: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_UnityAdsSettings {
    pub m_Enabled: usize,
    pub m_InitializeOnStartup: usize,
    pub m_TestMode: usize,
    pub m_IosGameId: Option<()>,
    pub m_AndroidGameId: Option<()>,
    pub m_GameIds: Struct_m_GameIds,
    pub m_GameId: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GameIds {
    pub AndroidPlayer: usize,
    pub iPhonePlayer: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_UnityAnalyticsSettings {
    pub m_Enabled: usize,
    pub m_TestMode: usize,
    pub m_InitializeOnStartup: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_UnityPurchasingSettings {
    pub m_Enabled: usize,
    pub m_TestMode: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_CrashReportingSettings {
    pub m_EventUrl: String,
    pub m_Enabled: usize,
    pub m_LogBufferSize: usize,
    pub m_CaptureEditorExceptions: usize,
}

// ReflectionProbe
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct ReflectionProbe {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_Type: usize,
    pub m_Mode: usize,
    pub m_RefreshMode: usize,
    pub m_TimeSlicingMode: usize,
    pub m_Resolution: usize,
    pub m_UpdateFrequency: usize,
    pub m_BoxSize: Struct_m_Center,
    pub m_BoxOffset: Struct_m_Center,
    pub m_NearClip: f64,
    pub m_FarClip: usize,
    pub m_ShadowDistance: usize,
    pub m_ClearFlags: usize,
    pub m_BackGroundColor: Struct_m_FogColor,
    pub m_CullingMask: Struct_m_CullingMask,
    pub m_IntensityMultiplier: usize,
    pub m_BlendDistance: usize,
    pub m_HDR: usize,
    pub m_BoxProjection: usize,
    pub m_RenderDynamicObjects: usize,
    pub m_UseOcclusionCulling: usize,
    pub m_Importance: usize,
    pub m_CustomBakedTexture: Struct_m_Preset,
}

// Prefab
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Prefab {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Modification: Struct_m_Modification,
    pub m_ParentPrefab: Struct_m_Preset,
    pub m_RootGameObject: Option<Struct_m_Preset>,
    pub m_IsPrefabParent: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Modification {
    pub m_TransformParent: Struct_m_Preset,
    pub m_Modifications: Vec<Struct_m_Modifications>,
    pub m_RemovedComponents: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Modifications {
    pub target: Struct_m_Preset,
    pub propertyPath: String,
    pub value: Option<::serde_yaml::Value>,
    pub objectReference: Struct_m_Preset,
}

// ParticleSystem
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct ParticleSystem {
    pub m_ObjectHideFlags: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Option<Struct_m_Preset>,
    pub serializedVersion: Option<usize>,
    pub lengthInSec: Option<f64>,
    pub speed: Option<usize>,
    pub looping: Option<usize>,
    pub prewarm: Option<usize>,
    pub playOnAwake: Option<usize>,
    pub autoRandomSeed: Option<usize>,
    pub startDelay: Option<Struct_startDelay>,
    pub moveWithTransform: Option<usize>,
    pub moveWithCustomTransform: Option<Struct_m_Preset>,
    pub scalingMode: Option<usize>,
    pub randomSeed: Option<::serde_yaml::Value>,
    pub InitialModule: Option<Struct_InitialModule>,
    pub ShapeModule: Option<Struct_ShapeModule>,
    pub EmissionModule: Option<Struct_EmissionModule>,
    pub SizeModule: Option<Struct_SizeModule>,
    pub RotationModule: Option<Struct_m_Center>,
    pub ColorModule: Option<Struct_ColorModule>,
    pub UVModule: Option<Struct_UVModule>,
    pub VelocityModule: Option<Struct_m_Center>,
    pub InheritVelocityModule: Option<Struct_InheritVelocityModule>,
    pub ForceModule: Option<Struct_m_Center>,
    pub ExternalForcesModule: Option<Struct_ExternalForcesModule>,
    pub ClampVelocityModule: Option<Struct_m_Center>,
    pub NoiseModule: Option<Struct_NoiseModule>,
    pub SizeBySpeedModule: Option<Struct_SizeBySpeedModule>,
    pub RotationBySpeedModule: Option<Struct_m_Center>,
    pub ColorBySpeedModule: Option<Struct_ColorBySpeedModule>,
    pub CollisionModule: Option<Struct_CollisionModule>,
    pub TriggerModule: Option<Struct_TriggerModule>,
    pub SubModule: Option<Struct_SubModule>,
    pub LightsModule: Option<Struct_LightsModule>,
    pub TrailModule: Option<Struct_TrailModule>,
    pub simulationSpeed: Option<f64>,
    pub stopAction: Option<usize>,
    pub useUnscaledTime: Option<usize>,
    pub useRigidbodyForVelocity: Option<usize>,
    pub CustomDataModule: Option<Struct_CustomDataModule>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub cullingMode: Option<usize>,
    pub ringBufferMode: Option<usize>,
    pub ringBufferLoopRange: Option<Struct_m_Center>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_CustomDataModule {
    pub enabled: usize,
    pub mode0: usize,
    pub vectorComponentCount0: usize,
    pub color0: Struct_color0,
    pub colorLabel0: Option<String>,
    pub vector0_0: Struct_vector0_0,
    pub vectorLabel0_0: Option<String>,
    pub vector0_1: Struct_vector0_0,
    pub vectorLabel0_1: Option<String>,
    pub vector0_2: Struct_vector0_0,
    pub vectorLabel0_2: Option<String>,
    pub vector0_3: Struct_vector0_0,
    pub vectorLabel0_3: Option<String>,
    pub mode1: usize,
    pub vectorComponentCount1: usize,
    pub color1: Struct_color0,
    pub colorLabel1: Option<String>,
    pub vector1_0: Struct_vector0_0,
    pub vectorLabel1_0: Option<String>,
    pub vector1_1: Struct_vector0_0,
    pub vectorLabel1_1: Option<String>,
    pub vector1_2: Struct_vector0_0,
    pub vectorLabel1_2: Option<String>,
    pub vector1_3: Struct_vector0_0,
    pub vectorLabel1_3: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_vector0_0 {
    pub serializedVersion: Option<usize>,
    pub minMaxState: usize,
    pub scalar: usize,
    pub minScalar: Option<usize>,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_maxCurve,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_6>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_6 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: usize,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_color0 {
    pub serializedVersion: usize,
    pub minMaxState: usize,
    pub minColor: Struct_m_FogColor,
    pub maxColor: Struct_m_FogColor,
    pub maxGradient: Struct_maxGradient,
    pub minGradient: Struct_maxGradient,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxGradient {
    pub serializedVersion: usize,
    pub key0: Struct_m_FogColor,
    pub key1: Struct_m_FogColor,
    pub key2: Struct_m_FogColor,
    pub key3: Struct_m_FogColor,
    pub key4: Struct_m_FogColor,
    pub key5: Struct_m_FogColor,
    pub key6: Struct_m_FogColor,
    pub key7: Struct_m_FogColor,
    pub ctime0: usize,
    pub ctime1: usize,
    pub ctime2: usize,
    pub ctime3: usize,
    pub ctime4: usize,
    pub ctime5: usize,
    pub ctime6: usize,
    pub ctime7: usize,
    pub atime0: usize,
    pub atime1: usize,
    pub atime2: usize,
    pub atime3: usize,
    pub atime4: usize,
    pub atime5: usize,
    pub atime6: usize,
    pub atime7: usize,
    pub m_Mode: usize,
    pub m_NumColorKeys: usize,
    pub m_NumAlphaKeys: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_TrailModule {
    pub enabled: usize,
    pub ratio: f64,
    pub lifetime: Struct_lifetime,
    pub minVertexDistance: f64,
    pub textureMode: usize,
    pub worldSpace: usize,
    pub dieWithParticles: usize,
    pub sizeAffectsWidth: usize,
    pub sizeAffectsLifetime: usize,
    pub inheritParticleColor: usize,
    pub colorOverLifetime: Struct_colorOverLifetime,
    pub widthOverTrail: Struct_widthOverTrail,
    pub colorOverTrail: Struct_colorOverLifetime,
    pub generateLightingData: Option<usize>,
    pub mode: Option<usize>,
    pub ribbonCount: Option<usize>,
    pub splitSubEmitterRibbons: Option<usize>,
    pub shadowBias: Option<f64>,
    pub attachRibbonsToTransform: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_widthOverTrail {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_1,
    pub minCurve: Struct_minCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_minCurve {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_7>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_7 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: f64,
    pub inSlope: f64,
    pub outSlope: f64,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_1 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_8>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_8 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: f64,
    pub inSlope: ::serde_yaml::Value,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_colorOverLifetime {
    pub serializedVersion: usize,
    pub maxGradient: Struct_maxGradient_1,
    pub minGradient: Struct_maxGradient_1,
    pub minColor: Struct_m_FogColor,
    pub maxColor: Struct_m_FogColor,
    pub minMaxState: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxGradient_1 {
    pub key0: Struct_m_FogColor,
    pub key1: Struct_m_FogColor,
    pub key2: Struct_m_FogColor,
    pub key3: Struct_m_FogColor,
    pub key4: Struct_m_FogColor,
    pub key5: Struct_m_FogColor,
    pub key6: Struct_m_FogColor,
    pub key7: Struct_m_FogColor,
    pub ctime0: usize,
    pub ctime1: usize,
    pub ctime2: usize,
    pub ctime3: usize,
    pub ctime4: usize,
    pub ctime5: usize,
    pub ctime6: usize,
    pub ctime7: usize,
    pub atime0: usize,
    pub atime1: usize,
    pub atime2: usize,
    pub atime3: usize,
    pub atime4: usize,
    pub atime5: usize,
    pub atime6: usize,
    pub atime7: usize,
    pub m_Mode: usize,
    pub m_NumColorKeys: usize,
    pub m_NumAlphaKeys: usize,
    pub serializedVersion: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_lifetime {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_2,
    pub minCurve: Struct_maxCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_2 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_9>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_9 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: f64,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_LightsModule {
    pub enabled: usize,
    pub ratio: usize,
    pub light: Struct_m_Preset,
    pub randomDistribution: usize,
    pub color: usize,
    pub range: usize,
    pub intensity: usize,
    pub rangeCurve: Struct_startDelay,
    pub intensityCurve: Struct_intensityCurve,
    pub maxLights: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_intensityCurve {
    pub scalar: usize,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_SubModule {
    pub serializedVersion: usize,
    pub enabled: usize,
    pub subEmitters: Vec<Struct_subEmitters>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_subEmitters {
    pub emitter: Struct_m_Preset,
    #[serde(rename = "type")]
    pub field_type: usize,
    pub properties: usize,
    pub serializedVersion: Option<usize>,
    pub emitProbability: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_TriggerModule {
    pub enabled: usize,
    pub collisionShape0: Struct_m_Preset,
    pub collisionShape1: Struct_m_Preset,
    pub collisionShape2: Struct_m_Preset,
    pub collisionShape3: Struct_m_Preset,
    pub collisionShape4: Struct_m_Preset,
    pub collisionShape5: Struct_m_Preset,
    pub inside: usize,
    pub outside: usize,
    pub enter: usize,
    pub exit: usize,
    pub radiusScale: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_CollisionModule {
    pub enabled: usize,
    pub serializedVersion: usize,
    #[serde(rename = "type")]
    pub field_type: usize,
    pub collisionMode: usize,
    pub plane0: Struct_m_Preset,
    pub plane1: Struct_m_Preset,
    pub plane2: Struct_m_Preset,
    pub plane3: Struct_m_Preset,
    pub plane4: Struct_m_Preset,
    pub plane5: Struct_m_Preset,
    pub m_Dampen: Struct_startDelay,
    pub m_Bounce: Struct_m_Bounce,
    pub m_EnergyLossOnCollision: Struct_startDelay,
    pub minKillSpeed: usize,
    pub maxKillSpeed: usize,
    pub radiusScale: f64,
    pub collidesWith: Struct_m_CullingMask,
    pub maxCollisionShapes: usize,
    pub quality: usize,
    pub voxelSize: f64,
    pub collisionMessages: usize,
    pub collidesWithDynamic: usize,
    pub interiorCollisions: usize,
    pub colliderForce: Option<usize>,
    pub multiplyColliderForceByParticleSize: Option<usize>,
    pub multiplyColliderForceByParticleSpeed: Option<usize>,
    pub multiplyColliderForceByCollisionAngle: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Bounce {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_maxCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_ColorBySpeedModule {
    pub enabled: usize,
    pub gradient: Struct_colorOverLifetime,
    pub range: Struct_m_Center,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_SizeBySpeedModule {
    pub enabled: usize,
    pub curve: Struct_curve_6,
    pub y: Struct_intensityCurve,
    pub z: Struct_intensityCurve,
    pub range: Struct_m_Center,
    pub separateAxes: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_6 {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_1,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_NoiseModule {
    pub enabled: usize,
    pub strength: Struct_strength,
    pub strengthY: Struct_strengthY,
    pub strengthZ: Struct_strength,
    pub separateAxes: usize,
    pub frequency: f64,
    pub damping: usize,
    pub octaves: usize,
    pub octaveMultiplier: f64,
    pub octaveScale: usize,
    pub quality: usize,
    pub scrollSpeed: Struct_startDelay,
    pub remap: Struct_remap,
    pub remapY: Struct_remapY,
    pub remapZ: Struct_remapZ,
    pub remapEnabled: usize,
    pub positionAmount: Option<Struct_positionAmount>,
    pub rotationAmount: Option<Struct_positionAmount>,
    pub sizeAmount: Option<Struct_sizeAmount>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_sizeAmount {
    pub serializedVersion: usize,
    pub minMaxState: usize,
    pub scalar: f64,
    pub minScalar: usize,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_maxCurve,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_positionAmount {
    pub serializedVersion: usize,
    pub minMaxState: usize,
    pub scalar: usize,
    pub minScalar: usize,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_maxCurve,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_remapZ {
    pub scalar: usize,
    pub maxCurve: Struct_maxCurve_3,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_3 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_10>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_10 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: ::serde_yaml::Value,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_remapY {
    pub scalar: usize,
    pub maxCurve: Struct_maxCurve_4,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_4 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_11>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_11 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: ::serde_yaml::Value,
    pub inSlope: f64,
    pub outSlope: f64,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_remap {
    pub scalar: usize,
    pub maxCurve: Struct_maxCurve_5,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_5 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_12>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_12 {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: f64,
    pub inSlope: f64,
    pub outSlope: f64,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_strengthY {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_4,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_strength {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_6,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_6 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_13>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_13 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: ::serde_yaml::Value,
    pub inSlope: f64,
    pub outSlope: f64,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_ExternalForcesModule {
    pub enabled: usize,
    pub multiplier: usize,
    pub influenceFilter: Option<usize>,
    pub influenceMask: Option<Struct_m_CullingMask>,
    pub influenceList: Option<Vec<::serde_yaml::Value>>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_InheritVelocityModule {
    pub enabled: usize,
    pub m_Mode: usize,
    pub m_Curve: Struct_m_Curve_14,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_14 {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_7,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_7 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_15>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_15 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: ::serde_yaml::Value,
    pub inSlope: ::serde_yaml::Value,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_UVModule {
    pub enabled: usize,
    pub frameOverTime: Struct_frameOverTime,
    pub startFrame: Struct_startDelay,
    pub tilesX: usize,
    pub tilesY: usize,
    pub animationType: usize,
    pub rowIndex: usize,
    pub cycles: usize,
    pub uvChannelMask: isize,
    pub flipU: f64,
    pub flipV: f64,
    pub randomRow: usize,
    pub mode: Option<usize>,
    pub sprites: Option<Vec<Struct_sprites>>,
    pub timeMode: Option<usize>,
    pub fps: Option<usize>,
    pub speedRange: Option<Struct_m_Center>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_sprites {
    pub sprite: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_frameOverTime {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_8,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_8 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_16>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_16 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: usize,
    pub inSlope: isize,
    pub outSlope: isize,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_ColorModule {
    pub enabled: usize,
    pub gradient: Struct_colorOverLifetime,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_SizeModule {
    pub enabled: usize,
    pub curve: Struct_curve_7,
    pub y: Struct_y,
    pub z: Struct_z,
    pub separateAxes: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_z {
    pub scalar: usize,
    pub maxCurve: Struct_maxCurve_6,
    pub minCurve: Struct_minCurve_1,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_minCurve_1 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_17>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_17 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: f64,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_y {
    pub scalar: usize,
    pub maxCurve: Struct_maxCurve_1,
    pub minCurve: Struct_minCurve_1,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_7 {
    pub scalar: f64,
    pub maxCurve: Struct_curve_1,
    pub minCurve: Struct_maxCurve_6,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_EmissionModule {
    pub enabled: usize,
    pub serializedVersion: usize,
    pub rateOverTime: Struct_rateOverTime,
    pub rateOverDistance: Struct_intensityCurve,
    pub m_CombineMode: Option<usize>,
    pub cnt0: Option<usize>,
    pub cnt1: Option<usize>,
    pub cnt2: Option<usize>,
    pub cnt3: Option<usize>,
    pub cntmax0: Option<usize>,
    pub cntmax1: Option<usize>,
    pub cntmax2: Option<usize>,
    pub cntmax3: Option<usize>,
    pub time0: Option<usize>,
    pub time1: Option<f64>,
    pub time2: Option<f64>,
    pub time3: Option<f64>,
    pub m_BurstCount: usize,
    pub m_Bursts: Option<Vec<Struct_m_Bursts>>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Bursts {
    pub serializedVersion: Option<usize>,
    pub time: usize,
    pub countCurve: Option<Struct_positionAmount>,
    pub cycleCount: usize,
    pub repeatInterval: f64,
    pub minCount: Option<usize>,
    pub maxCount: Option<usize>,
    pub probability: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_rateOverTime {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_9,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_9 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_18>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_18 {
    pub serializedVersion: usize,
    pub time: ::serde_yaml::Value,
    pub value: ::serde_yaml::Value,
    pub inSlope: f64,
    pub outSlope: f64,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_ShapeModule {
    pub serializedVersion: usize,
    pub enabled: usize,
    #[serde(rename = "type")]
    pub field_type: usize,
    pub radius: ::serde_yaml::Value,
    pub angle: f64,
    pub length: f64,
    pub boxX: Option<f64>,
    pub boxY: Option<f64>,
    pub boxZ: Option<f64>,
    pub arc: ::serde_yaml::Value,
    pub placementMode: usize,
    pub m_Mesh: Struct_m_Preset,
    pub m_MeshRenderer: Struct_m_Preset,
    pub m_SkinnedMeshRenderer: Struct_m_Preset,
    pub m_MeshMaterialIndex: usize,
    pub m_MeshNormalOffset: usize,
    pub m_UseMeshMaterialIndex: usize,
    pub m_UseMeshColors: usize,
    pub alignToDirection: usize,
    pub randomDirection: Option<usize>,
    pub boxThickness: Option<Struct_m_Center>,
    pub radiusThickness: Option<f64>,
    pub donutRadius: Option<f64>,
    pub m_Position: Option<Struct_m_Center>,
    pub m_Rotation: Option<Struct_m_Center>,
    pub m_Scale: Option<Struct_m_Center>,
    pub randomDirectionAmount: Option<usize>,
    pub sphericalDirectionAmount: Option<usize>,
    pub randomPositionAmount: Option<usize>,
    pub m_Texture: Option<Struct_m_Preset>,
    pub m_TextureClipChannel: Option<usize>,
    pub m_TextureClipThreshold: Option<usize>,
    pub m_TextureUVChannel: Option<usize>,
    pub m_TextureColorAffectsParticles: Option<usize>,
    pub m_TextureAlphaAffectsParticles: Option<usize>,
    pub m_TextureBilinearFiltering: Option<usize>,
    pub m_MeshScale: Option<usize>,
    pub m_MeshSpawn: Option<Struct_m_MeshSpawn>,
    pub m_Sprite: Option<Struct_m_Preset>,
    pub m_SpriteRenderer: Option<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_MeshSpawn {
    pub mode: usize,
    pub spread: usize,
    pub speed: Struct_speed,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_speed {
    pub serializedVersion: usize,
    pub minMaxState: usize,
    pub scalar: usize,
    pub minScalar: usize,
    pub maxCurve: Struct_rolloffCustomCurve,
    pub minCurve: Struct_rolloffCustomCurve,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_InitialModule {
    pub serializedVersion: usize,
    pub enabled: usize,
    pub startLifetime: Struct_lifetime,
    pub startSpeed: Struct_startSpeed,
    pub startColor: Struct_colorOverLifetime,
    pub startSize: Struct_startSize,
    pub startSizeY: Struct_startSizeY,
    pub startSizeZ: Struct_curve_6,
    pub startRotationX: Struct_startRotationX,
    pub startRotationY: Struct_startDelay,
    pub startRotation: Struct_startRotation,
    pub randomizeRotationDirection: f64,
    pub maxNumParticles: usize,
    pub size3D: usize,
    pub rotation3D: usize,
    pub gravityModifier: Struct_gravityModifier,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_gravityModifier {
    pub scalar: f64,
    pub maxCurve: Struct_minCurve,
    pub minCurve: Struct_maxCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startRotation {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_minCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_minCurve_2 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_19>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_19 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: isize,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startRotationX {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_minCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startSizeY {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_3,
    pub minCurve: Struct_maxCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startSize {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_1,
    pub minCurve: Struct_maxCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startSpeed {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve_10,
    pub minCurve: Struct_maxCurve_2,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_maxCurve_10 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_20>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_20 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: f64,
    pub inSlope: f64,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: Option<usize>,
    pub inWeight: Option<f64>,
    pub outWeight: Option<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startDelay {
    pub scalar: f64,
    pub maxCurve: Struct_maxCurve,
    pub minCurve: Struct_maxCurve,
    pub minMaxState: usize,
    pub serializedVersion: Option<usize>,
    pub minScalar: Option<usize>,
}

// Canvas
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Canvas {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub serializedVersion: usize,
    pub m_RenderMode: usize,
    pub m_Camera: Struct_m_Preset,
    pub m_PlaneDistance: usize,
    pub m_PixelPerfect: usize,
    pub m_ReceivesEvents: usize,
    pub m_OverrideSorting: usize,
    pub m_OverridePixelPerfect: usize,
    pub m_SortingBucketNormalizedSize: usize,
    pub m_AdditionalShaderChannelsFlag: usize,
    pub m_SortingLayerID: usize,
    pub m_SortingOrder: usize,
    pub m_TargetDisplay: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// AnimatorStateTransition
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AnimatorStateTransition {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_Name: Option<()>,
    pub m_Conditions: Vec<Struct_m_Conditions>,
    pub m_DstStateMachine: Struct_m_Preset,
    pub m_DstState: Struct_m_Preset,
    pub m_Solo: usize,
    pub m_Mute: usize,
    pub m_IsExit: usize,
    pub serializedVersion: usize,
    pub m_TransitionDuration: f64,
    pub m_TransitionOffset: usize,
    pub m_ExitTime: f64,
    pub m_HasExitTime: usize,
    pub m_HasFixedDuration: usize,
    pub m_InterruptionSource: usize,
    pub m_OrderedInterruption: usize,
    pub m_CanTransitionToSelf: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Conditions {
    pub m_ConditionMode: usize,
    pub m_ConditionEvent: String,
    pub m_EventTreshold: usize,
}

// TrailRenderer
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct TrailRenderer {
    pub serializedVersion: usize,
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_CastShadows: usize,
    pub m_ReceiveShadows: usize,
    pub m_DynamicOccludee: usize,
    pub m_MotionVectors: usize,
    pub m_LightProbeUsage: usize,
    pub m_ReflectionProbeUsage: usize,
    pub m_RenderingLayerMask: usize,
    pub m_RendererPriority: usize,
    pub m_Materials: Vec<Struct_m_Preset>,
    pub m_StaticBatchInfo: Struct_m_StaticBatchInfo,
    pub m_StaticBatchRoot: Struct_m_Preset,
    pub m_ProbeAnchor: Struct_m_Preset,
    pub m_LightProbeVolumeOverride: Struct_m_Preset,
    pub m_ScaleInLightmap: usize,
    pub m_PreserveUVs: usize,
    pub m_IgnoreNormalsForChartDetection: usize,
    pub m_ImportantGI: usize,
    pub m_StitchLightmapSeams: usize,
    pub m_SelectedEditorRenderState: usize,
    pub m_MinimumChartSize: usize,
    pub m_AutoUVMaxDistance: f64,
    pub m_AutoUVMaxAngle: usize,
    pub m_LightmapParameters: Struct_m_Preset,
    pub m_SortingLayerID: usize,
    pub m_SortingLayer: usize,
    pub m_SortingOrder: usize,
    pub m_Time: usize,
    pub m_Parameters: Struct_m_Parameters,
    pub m_MinVertexDistance: f64,
    pub m_Autodestruct: usize,
    pub m_Emitting: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Parameters {
    pub serializedVersion: usize,
    pub widthMultiplier: usize,
    pub widthCurve: Struct_rolloffCustomCurve,
    pub colorGradient: Struct_maxGradient,
    pub numCornerVertices: usize,
    pub numCapVertices: usize,
    pub alignment: usize,
    pub textureMode: usize,
    pub shadowBias: usize,
    pub generateLightingData: usize,
}

// MeshRenderer
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct MeshRenderer {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_CastShadows: usize,
    pub m_ReceiveShadows: usize,
    pub m_DynamicOccludee: Option<usize>,
    pub m_MotionVectors: Option<usize>,
    pub m_LightProbeUsage: Option<usize>,
    pub m_ReflectionProbeUsage: usize,
    pub m_Materials: Vec<Struct_m_Preset>,
    pub m_StaticBatchInfo: Option<Struct_m_StaticBatchInfo>,
    pub m_StaticBatchRoot: Struct_m_Preset,
    pub m_ProbeAnchor: Struct_m_Preset,
    pub m_LightProbeVolumeOverride: Option<Struct_m_Preset>,
    pub m_ScaleInLightmap: usize,
    pub m_PreserveUVs: usize,
    pub m_IgnoreNormalsForChartDetection: Option<usize>,
    pub m_ImportantGI: usize,
    pub m_StitchLightmapSeams: Option<usize>,
    pub m_SelectedEditorRenderState: Option<usize>,
    pub m_MinimumChartSize: Option<usize>,
    pub m_AutoUVMaxDistance: f64,
    pub m_AutoUVMaxAngle: usize,
    pub m_LightmapParameters: Struct_m_Preset,
    pub m_SortingLayerID: usize,
    pub m_SortingLayer: Option<usize>,
    pub m_SortingOrder: usize,
    pub m_RenderingLayerMask: Option<usize>,
    pub m_SelectedWireframeHidden: Option<usize>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_RendererPriority: Option<usize>,
    pub m_SubsetIndices: Option<()>,
    pub m_UseLightProbes: Option<usize>,
}

// Texture2D
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Texture2D {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_Name: String,
    pub m_ImageContentsHash: Struct_m_Hash,
    pub m_ForcedFallbackFormat: usize,
    pub m_DownscaleFallback: usize,
    pub serializedVersion: usize,
    pub m_Width: usize,
    pub m_Height: usize,
    pub m_CompleteImageSize: usize,
    pub m_TextureFormat: usize,
    pub m_MipCount: usize,
    pub m_IsReadable: usize,
    pub m_StreamingMipmaps: usize,
    pub m_StreamingMipmapsPriority: usize,
    pub m_AlphaIsTransparency: usize,
    pub m_ImageCount: usize,
    pub m_TextureDimension: usize,
    pub m_TextureSettings: Struct_m_TextureSettings,
    pub m_LightmapFormat: usize,
    pub m_ColorSpace: usize,
    #[serde(rename = "image data")]
    pub image_data: usize,
    pub _typelessdata: String,
    pub m_StreamData: Struct_m_StreamData,
}

// Transform
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Transform {
    pub m_ObjectHideFlags: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Option<Struct_m_Preset>,
    pub m_LocalRotation: Option<Struct_m_Center>,
    pub m_LocalPosition: Option<Struct_m_Center>,
    pub m_LocalScale: Option<Struct_m_Center>,
    pub m_Children: Option<Vec<Struct_m_Preset>>,
    pub m_Father: Option<Struct_m_Preset>,
    pub m_RootOrder: Option<usize>,
    pub m_LocalEulerAnglesHint: Option<Struct_m_Center>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// PrefabInstance
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct PrefabInstance {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Modification: Struct_m_Modification_1,
    pub m_SourcePrefab: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Modification_1 {
    pub m_TransformParent: Struct_m_Preset,
    pub m_Modifications: Vec<Struct_m_Modifications>,
    pub m_RemovedComponents: Vec<Struct_m_Preset>,
}

// MonoBehaviour
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct MonoBehaviour {
    pub m_ObjectHideFlags: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_EditorHideFlags: usize,
    pub m_Script: Struct_m_Preset,
    pub m_Name: Option<String>,
    pub m_EditorClassIdentifier: Option<()>,
    pub startManually: Option<usize>,
    pub eventBuffering: Option<usize>,
    pub sendInBackground: Option<usize>,
    pub launchDeferredDeeplink: Option<usize>,
    pub appToken: Option<String>,
    pub logLevel: Option<usize>,
    pub environment: Option<usize>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub buildingTypeInBiomes: Option<Vec<Struct_buildingTypeInBiomes>>,
    pub passageDefaultOpenDuration: Option<usize>,
    pub characterNumPerArea: Option<usize>,
    pub isCharacterAutoSpawn: Option<usize>,
    pub isHumanAutoWork: Option<usize>,
    pub isAutoRefreshWork: Option<usize>,
    pub isHumanWorkInPassage: Option<usize>,
    pub CameraAreaExpandX: Option<usize>,
    pub CameraAreaExpandZ: Option<usize>,
    pub workSlotOpenAnimClip: Option<Struct_m_Preset>,
    pub areaOpenDelayTime: Option<f64>,
    pub synergyData: Option<Vec<Struct_synergyData>>,
    pub constructCostMultipliers: Option<Vec<usize>>,
    pub buildingGlobalDataList: Option<Vec<Struct_buildingGlobalDataList>>,
    pub techGraphicInfoList: Option<Vec<Struct_techGraphicInfoList>>,
    pub buildingIconInfoList: Option<Vec<Struct_buildingIconInfoList>>,
    pub techSelectBonusIconInfoList: Option<Vec<Struct_techSelectBonusIconInfoList>>,
    pub levelInfos: Option<Vec<Struct_levelInfos>>,
    pub openInfos: Option<Vec<Struct_openInfos>>,
    pub slotInfos: Option<Vec<Struct_slotInfos>>,
    pub upgradeInfos: Option<Vec<Struct_upgradeInfos>>,
    pub techBonusInfoList: Option<Vec<Struct_techBonusInfoList>>,
    pub infoList: Option<Vec<Struct_infoList>>,
    pub toolbarInt: Option<usize>,
    pub iOSNotificationEditorSettingsValues: Option<Struct_iOSNotificationEditorSettingsValues>,
    pub AndroidNotificationEditorSettingsValues:
        Option<Struct_AndroidNotificationEditorSettingsValues>,
    pub TrackedResourceAssets: Option<Vec<::serde_yaml::Value>>,
    pub debugViews: Option<Struct_debugViews>,
    pub fog: Option<Struct_m_Preset>,
    pub antialiasing: Option<Struct_antialiasing>,
    pub ambientOcclusion: Option<Struct_ambientOcclusion>,
    pub screenSpaceReflection: Option<Struct_screenSpaceReflection>,
    pub depthOfField: Option<Struct_depthOfField>,
    pub motionBlur: Option<Struct_motionBlur>,
    pub eyeAdaptation: Option<Struct_eyeAdaptation>,
    pub bloom: Option<Struct_bloom>,
    pub colorGrading: Option<Struct_colorGrading>,
    pub userLut: Option<Struct_userLut>,
    pub chromaticAberration: Option<Struct_chromaticAberration>,
    pub grain: Option<Struct_grain>,
    pub vignette: Option<Struct_vignette>,
    pub dithering: Option<Struct_PerformanceReportingSettings>,
    pub monitors: Option<Struct_monitors>,
    pub settings: Option<::serde_yaml::Value>,
    pub active: Option<usize>,
    pub enabled: Option<Struct_enabled>,
    pub intensity: Option<Struct_intensity>,
    pub threshold: Option<Struct_enabled>,
    pub softKnee: Option<Struct_intensity>,
    pub clamp: Option<Struct_enabled>,
    pub diffusion: Option<Struct_enabled>,
    pub anamorphicRatio: Option<Struct_enabled>,
    pub color: Option<Struct_color>,
    pub fastMode: Option<Struct_enabled>,
    pub dirtTexture: Option<Struct_dirtTexture>,
    pub dirtIntensity: Option<Struct_enabled>,
    pub gradingMode: Option<Struct_enabled>,
    pub externalLut: Option<Struct_dirtTexture>,
    pub tonemapper: Option<Struct_enabled>,
    pub toneCurveToeStrength: Option<Struct_enabled>,
    pub toneCurveToeLength: Option<Struct_intensity>,
    pub toneCurveShoulderStrength: Option<Struct_enabled>,
    pub toneCurveShoulderLength: Option<Struct_intensity>,
    pub toneCurveShoulderAngle: Option<Struct_enabled>,
    pub toneCurveGamma: Option<Struct_enabled>,
    pub ldrLut: Option<Struct_dirtTexture>,
    pub ldrLutContribution: Option<Struct_enabled>,
    pub temperature: Option<Struct_enabled>,
    pub tint: Option<Struct_tint>,
    pub colorFilter: Option<Struct_color>,
    pub hueShift: Option<Struct_enabled>,
    pub saturation: Option<Struct_tint>,
    pub brightness: Option<Struct_enabled>,
    pub postExposure: Option<Struct_intensity>,
    pub contrast: Option<Struct_tint>,
    pub mixerRedOutRedIn: Option<Struct_enabled>,
    pub mixerRedOutGreenIn: Option<Struct_enabled>,
    pub mixerRedOutBlueIn: Option<Struct_enabled>,
    pub mixerGreenOutRedIn: Option<Struct_enabled>,
    pub mixerGreenOutGreenIn: Option<Struct_enabled>,
    pub mixerGreenOutBlueIn: Option<Struct_enabled>,
    pub mixerBlueOutRedIn: Option<Struct_enabled>,
    pub mixerBlueOutGreenIn: Option<Struct_enabled>,
    pub mixerBlueOutBlueIn: Option<Struct_enabled>,
    pub lift: Option<Struct_lift>,
    pub gamma: Option<Struct_lift>,
    pub gain: Option<Struct_lift>,
    pub masterCurve: Option<Struct_masterCurve>,
    pub redCurve: Option<Struct_masterCurve>,
    pub greenCurve: Option<Struct_masterCurve>,
    pub blueCurve: Option<Struct_masterCurve>,
    pub hueVsHueCurve: Option<Struct_hueVsHueCurve>,
    pub hueVsSatCurve: Option<Struct_hueVsHueCurve>,
    pub satVsSatCurve: Option<Struct_satVsSatCurve>,
    pub lumVsSatCurve: Option<Struct_lumVsSatCurve>,
    pub m_Material: Option<Struct_m_Preset>,
    pub m_Color: Option<Struct_m_FogColor>,
    pub m_RaycastTarget: Option<usize>,
    pub m_OnCullStateChanged: Option<Struct_m_OnCullStateChanged>,
    pub m_FontData: Option<Struct_m_FontData>,
    pub m_Text: Option<String>,
    pub m_Navigation: Option<Struct_m_Navigation>,
    pub m_Transition: Option<usize>,
    pub m_Colors: Option<Struct_m_Colors>,
    pub m_SpriteState: Option<Struct_m_SpriteState>,
    pub m_AnimationTriggers: Option<Struct_m_AnimationTriggers>,
    pub m_Interactable: Option<usize>,
    pub m_TargetGraphic: Option<Struct_m_Preset>,
    pub m_OnClick: Option<Struct_m_OnClick>,
    pub m_Sprite: Option<Struct_m_Preset>,
    pub m_Type: Option<usize>,
    pub m_PreserveAspect: Option<usize>,
    pub m_FillCenter: Option<usize>,
    pub m_FillMethod: Option<usize>,
    pub m_FillAmount: Option<f64>,
    pub m_FillClockwise: Option<usize>,
    pub m_FillOrigin: Option<usize>,
    pub particleSystems: Option<Vec<Struct_particleSystems>>,
    pub gunGameObject: Option<Struct_m_Preset>,
    pub spawnLocation: Option<Struct_m_Preset>,
    pub title: Option<Struct_m_Preset>,
    pub description: Option<Struct_m_Preset>,
    pub navigationDetails: Option<Struct_m_Preset>,
    pub m_IgnoreReversedGraphics: Option<usize>,
    pub m_BlockingObjects: Option<usize>,
    pub m_BlockingMask: Option<Struct_m_CullingMask>,
    pub m_UiScaleMode: Option<usize>,
    pub m_ReferencePixelsPerUnit: Option<usize>,
    pub m_ScaleFactor: Option<usize>,
    pub m_ReferenceResolution: Option<Struct_m_Center>,
    pub m_ScreenMatchMode: Option<usize>,
    pub m_MatchWidthOrHeight: Option<usize>,
    pub m_PhysicalUnit: Option<usize>,
    pub m_FallbackScreenDPI: Option<usize>,
    pub m_DefaultSpriteDPI: Option<usize>,
    pub m_DynamicPixelsPerUnit: Option<usize>,
    pub sharedProfile: Option<Struct_m_Preset>,
    pub isGlobal: Option<usize>,
    pub blendDistance: Option<usize>,
    pub weight: Option<usize>,
    pub priority: Option<usize>,
    pub volumeTrigger: Option<Struct_m_Preset>,
    pub volumeLayer: Option<Struct_m_CullingMask>,
    pub stopNaNPropagation: Option<usize>,
    pub antialiasingMode: Option<usize>,
    pub temporalAntialiasing: Option<Struct_temporalAntialiasing>,
    pub subpixelMorphologicalAntialiasing: Option<Struct_subpixelMorphologicalAntialiasing>,
    pub fastApproximateAntialiasing: Option<Struct_fastApproximateAntialiasing>,
    pub debugLayer: Option<Struct_debugLayer>,
    pub m_Resources: Option<Struct_m_Preset>,
    pub m_ShowToolkit: Option<usize>,
    pub m_ShowCustomSorter: Option<usize>,
    pub breakBeforeColorGrading: Option<usize>,
    pub m_BeforeTransparentBundles: Option<Vec<::serde_yaml::Value>>,
    pub m_BeforeStackBundles: Option<Vec<Struct_m_BeforeStackBundles>>,
    pub m_AfterStackBundles: Option<Vec<::serde_yaml::Value>>,
    pub fireRate: Option<f64>,
    pub weaponRange: Option<usize>,
    pub gunEnd: Option<Struct_m_Preset>,
    pub muzzleFlash: Option<Struct_m_Preset>,
    pub cartridgeEjection: Option<Struct_m_Preset>,
    pub metalHitEffect: Option<Struct_m_Preset>,
    pub sandHitEffect: Option<Struct_m_Preset>,
    pub stoneHitEffect: Option<Struct_m_Preset>,
    pub waterLeakEffect: Option<Struct_m_Preset>,
    pub waterLeakExtinguishEffect: Option<Struct_m_Preset>,
    pub fleshHitEffects: Option<Vec<Struct_m_Preset>>,
    pub woodHitEffect: Option<Struct_m_Preset>,
    pub borderLeft: Option<usize>,
    pub borderRight: Option<usize>,
    pub borderTop: Option<usize>,
    pub borderBottom: Option<usize>,
    pub m_Padding: Option<Struct_m_Padding>,
    pub m_ChildAlignment: Option<usize>,
    pub m_Spacing: Option<::serde_yaml::Value>,
    pub m_ChildForceExpandWidth: Option<usize>,
    pub m_ChildForceExpandHeight: Option<usize>,
    pub m_ChildControlWidth: Option<usize>,
    pub m_ChildControlHeight: Option<usize>,
    pub m_HorizontalAxis: Option<String>,
    pub m_VerticalAxis: Option<String>,
    pub m_SubmitButton: Option<String>,
    pub m_CancelButton: Option<String>,
    pub m_InputActionsPerSecond: Option<usize>,
    pub m_RepeatDelay: Option<f64>,
    pub m_ForceModuleActive: Option<usize>,
    pub m_FirstSelected: Option<Struct_m_Preset>,
    pub m_sendNavigationEvents: Option<usize>,
    pub m_DragThreshold: Option<usize>,
    pub fireParticleSystem: Option<Struct_m_Preset>,
    pub smokeParticleSystem: Option<Struct_m_Preset>,
    pub lifeTime: Option<f64>,
    pub selectedAppIndex: Option<usize>,
    pub clientTokens: Option<Vec<Option<()>>>,
    pub appIds: Option<Vec<String>>,
    pub appLabels: Option<Vec<String>>,
    pub cookie: Option<usize>,
    pub logging: Option<usize>,
    pub status: Option<usize>,
    pub xfbml: Option<usize>,
    pub frictionlessRequests: Option<usize>,
    pub iosURLSuffix: Option<()>,
    pub appLinkSchemes: Option<Vec<Struct_appLinkSchemes>>,
    pub uploadAccessToken: Option<()>,
    pub autoLogAppEventsEnabled: Option<usize>,
    pub advertiserIDCollectionEnabled: Option<usize>,
    pub hashCode: Option<::serde_yaml::Value>,
    pub material: Option<Struct_m_Preset>,
    pub materialHashCode: Option<::serde_yaml::Value>,
    pub m_Version: Option<String>,
    pub m_SourceFontFileGUID: Option<String>,
    pub m_SourceFontFile_EditorRef: Option<Struct_m_Preset>,
    pub m_SourceFontFile: Option<Struct_m_Preset>,
    pub m_AtlasPopulationMode: Option<usize>,
    pub m_FaceInfo: Option<Struct_m_FaceInfo>,
    pub m_GlyphTable: Option<Vec<Struct_m_GlyphTable>>,
    pub m_CharacterTable: Option<Vec<Struct_m_CharacterTable>>,
    pub m_AtlasTextures: Option<Vec<Struct_m_Preset>>,
    pub m_AtlasTextureIndex: Option<usize>,
    pub m_UsedGlyphRects: Option<Vec<Struct_m_UsedGlyphRects>>,
    pub m_FreeGlyphRects: Option<Vec<Struct_m_UsedGlyphRects>>,
    pub m_fontInfo: Option<Struct_m_fontInfo>,
    pub atlas: Option<Struct_m_Preset>,
    pub m_AtlasWidth: Option<usize>,
    pub m_AtlasHeight: Option<usize>,
    pub m_AtlasPadding: Option<usize>,
    pub m_AtlasRenderMode: Option<usize>,
    pub m_glyphInfoList: Option<Vec<Struct_m_Center>>,
    pub m_KerningTable: Option<Struct_m_KerningTable>,
    pub m_FontFeatureTable: Option<Struct_m_FontFeatureTable>,
    pub fallbackFontAssets: Option<Vec<::serde_yaml::Value>>,
    pub m_FallbackFontAssetTable: Option<Vec<::serde_yaml::Value>>,
    pub m_CreationSettings: Option<Struct_m_CreationSettings>,
    pub m_FontWeightTable: Option<Vec<Struct_m_FontWeightTable>>,
    pub fontWeights: Option<Vec<Struct_m_FontWeightTable>>,
    pub normalStyle: Option<usize>,
    pub normalSpacingOffset: Option<usize>,
    pub boldStyle: Option<f64>,
    pub boldSpacing: Option<usize>,
    pub italicStyle: Option<usize>,
    pub tabSize: Option<usize>,
    pub m_UseSpriteMesh: Option<usize>,
    pub variableJoystick: Option<Struct_m_Preset>,
    pub valueText: Option<Struct_m_Preset>,
    pub background: Option<Struct_m_Preset>,
    pub axisSprites: Option<Vec<Struct_m_Preset>>,
    pub m_Template: Option<Struct_m_Preset>,
    pub m_CaptionText: Option<Struct_m_Preset>,
    pub m_CaptionImage: Option<Struct_m_Preset>,
    pub m_ItemText: Option<Struct_m_Preset>,
    pub m_ItemImage: Option<Struct_m_Preset>,
    pub m_Value: Option<usize>,
    pub m_Options: Option<Struct_m_Options>,
    pub m_OnValueChanged: Option<Struct_m_OnValueChanged>,
    pub m_HandleRect: Option<Struct_m_Preset>,
    pub m_Direction: Option<::serde_yaml::Value>,
    pub m_Size: Option<::serde_yaml::Value>,
    pub m_NumberOfSteps: Option<usize>,
    pub toggleTransition: Option<usize>,
    pub graphic: Option<Struct_m_Preset>,
    pub m_Group: Option<Struct_m_Preset>,
    pub onValueChanged: Option<Struct_onValueChanged>,
    pub m_IsOn: Option<usize>,
    pub m_Content: Option<Struct_m_Preset>,
    pub m_Horizontal: Option<usize>,
    pub m_Vertical: Option<usize>,
    pub m_MovementType: Option<usize>,
    pub m_Elasticity: Option<f64>,
    pub m_Inertia: Option<usize>,
    pub m_DecelerationRate: Option<f64>,
    pub m_ScrollSensitivity: Option<usize>,
    pub m_Viewport: Option<Struct_m_Preset>,
    pub m_HorizontalScrollbar: Option<Struct_m_Preset>,
    pub m_VerticalScrollbar: Option<Struct_m_Preset>,
    pub m_HorizontalScrollbarVisibility: Option<usize>,
    pub m_VerticalScrollbarVisibility: Option<usize>,
    pub m_HorizontalScrollbarSpacing: Option<isize>,
    pub m_VerticalScrollbarSpacing: Option<isize>,
    pub speed: Option<usize>,
    pub rb: Option<Struct_m_Preset>,
    pub m_ShowMaskGraphic: Option<usize>,
    pub handleRange: Option<usize>,
    pub deadZone: Option<usize>,
    pub axisOptions: Option<usize>,
    pub snapX: Option<usize>,
    pub snapY: Option<usize>,
    pub handle: Option<Struct_m_Preset>,
    pub moveThreshold: Option<usize>,
    pub joystickType: Option<usize>,
    pub m_AgentTypeID: Option<::serde_yaml::Value>,
    pub m_CollectObjects: Option<usize>,
    pub m_Center: Option<Struct_m_Center>,
    pub m_LayerMask: Option<Struct_m_CullingMask>,
    pub m_UseGeometry: Option<usize>,
    pub m_DefaultArea: Option<usize>,
    pub m_IgnoreNavMeshAgent: Option<usize>,
    pub m_IgnoreNavMeshObstacle: Option<usize>,
    pub m_OverrideTileSize: Option<usize>,
    pub m_TileSize: Option<usize>,
    pub m_OverrideVoxelSize: Option<usize>,
    pub m_VoxelSize: Option<f64>,
    pub m_BuildHeightMesh: Option<usize>,
    pub m_BakedNavMeshData: Option<Struct_m_Preset>,
    pub m_NavMesh: Option<Struct_m_Preset>,
    pub m_FollowTransform: Option<usize>,
    pub m_StartPoint: Option<Struct_m_Center>,
    pub m_EndPoint: Option<Struct_m_Center>,
    pub m_Width: Option<f64>,
    pub m_CostModifier: Option<isize>,
    pub m_Bidirectional: Option<usize>,
    pub m_AutoUpdatePosition: Option<usize>,
    pub m_Area: Option<usize>,
    pub m_Behaviour: Option<Struct_m_Preset>,
    pub m_OverrideArea: Option<usize>,
    pub m_IgnoreFromBuild: Option<usize>,
    pub m_AffectedAgents: Option<()>,
    pub m_Amplitude: Option<f64>,
    pub m_Period: Option<usize>,
    pub m_Tag: Option<String>,
    pub m_Prefab: Option<Struct_m_Preset>,
    pub m_KeyCode: Option<usize>,
    pub m_Tracked: Option<Struct_m_Preset>,
    pub m_PoolSize: Option<usize>,
    pub m_InstancesPerTile: Option<usize>,
    pub m_RandomPosition: Option<usize>,
    pub m_RandomOrientation: Option<usize>,
    pub m_Height: Option<usize>,
    pub m_BaseHash: Option<usize>,
    pub m_range: Option<usize>,
    pub m_relative: Option<usize>,
    pub m_Filters: Option<Vec<Struct_m_Preset>>,
    pub m_Radius: Option<f64>,
    pub m_Power: Option<usize>,
    pub m_Method: Option<usize>,
    pub axes: Option<usize>,
    pub sensitivityX: Option<usize>,
    pub sensitivityY: Option<usize>,
    pub minimumX: Option<isize>,
    pub maximumX: Option<usize>,
    pub minimumY: Option<isize>,
    pub maximumY: Option<usize>,
    pub moveSpeed: Option<usize>,
    pub lockHeight: Option<usize>,
    pub m_Tiles: Option<Vec<Struct_m_Preset>>,
    pub m_Curve: Option<Struct_m_Curve_21>,
    pub anchorSize: Option<usize>,
    pub controlSize: Option<usize>,
    pub visibleBehindObjects: Option<usize>,
    pub visibleWhenNotSelected: Option<usize>,
    pub hideAutoControls: Option<usize>,
    pub anchorShape: Option<usize>,
    pub controlShape: Option<usize>,
    pub anchor: Option<Struct_m_FogColor>,
    pub anchorHighlighted: Option<Struct_m_FogColor>,
    pub anchorSelected: Option<Struct_m_FogColor>,
    pub control: Option<Struct_m_FogColor>,
    pub controlHighlighted: Option<Struct_m_FogColor>,
    pub controlSelected: Option<Struct_m_FogColor>,
    pub handleDisabled: Option<Struct_m_FogColor>,
    pub controlLine: Option<Struct_m_FogColor>,
    pub bezierPath: Option<Struct_m_FogColor>,
    pub highlightedPath: Option<Struct_m_FogColor>,
    pub bounds: Option<Struct_m_FogColor>,
    pub segmentBounds: Option<Struct_m_FogColor>,
    pub vertexPath: Option<Struct_m_FogColor>,
    pub normals: Option<Struct_m_FogColor>,
    pub normalsLength: Option<usize>,
    pub pathCreator: Option<Struct_m_Preset>,
    pub endOfPathInstruction: Option<usize>,
    pub editorData: Option<Struct_editorData>,
    pub initialized: Option<usize>,
    pub closedLoop: Option<usize>,
    pub waypoints: Option<Vec<Struct_m_Preset>>,
    pub autoUpdate: Option<usize>,
    pub prefab: Option<Struct_m_Preset>,
    pub holder: Option<Struct_m_Preset>,
    pub spacing: Option<f64>,
    pub pathPrefab: Option<Struct_m_Preset>,
    pub followerPrefab: Option<Struct_m_Preset>,
    pub spawnPoints: Option<Vec<Struct_m_Preset>>,
    pub roadWidth: Option<usize>,
    pub thickness: Option<f64>,
    pub flattenSurface: Option<usize>,
    pub roadMaterial: Option<Struct_m_Preset>,
    pub undersideMaterial: Option<Struct_m_Preset>,
    pub textureTiling: Option<usize>,
    pub meshHolder: Option<Struct_m_Preset>,
    pub list: Option<Vec<Struct_list>>,
    pub foldoutSettings: Option<usize>,
    pub lastCamID: Option<usize>,
    pub lastCam: Option<Struct_m_Preset>,
    pub Health: Option<f64>,
    pub MyClass: Option<Struct_MyClass>,
    pub serializationData: Option<Struct_serializationData>,
    pub F: Option<Vec<Struct_F>>,
    pub Field: Option<usize>,
    pub MyInt: Option<usize>,
    pub AVector3: Option<Struct_m_Center>,
    pub AnotherInt: Option<usize>,
    pub AllTheWayAroundAndBack: Option<Struct_m_Center>,
    pub Color: Option<Struct_m_FogColor>,
    pub NotOne: Option<isize>,
    pub NumberOfBombs: Option<usize>,
    pub MyStruct: Option<Struct_MyStruct>,
    pub MyIntValue: Option<usize>,
    pub DynamicLabel: Option<String>,
    pub InvalidReference: Option<usize>,
    pub InstanceMethod: Option<usize>,
    pub StaticMethod: Option<usize>,
    pub InvalidMethod: Option<usize>,
    pub automateBeforeBuilds: Option<usize>,
    pub deleteDllAfterBuilds: Option<usize>,
    pub AutomateForAllAOTPlatforms: Option<usize>,
    pub automateForPlatforms: Option<String>,
    pub lastScan: Option<usize>,
    pub supportSerializedTypes: Option<Vec<::serde_yaml::Value>>,
    pub automateBeforeBuild: Option<usize>,
    pub enableOdinInInspector: Option<usize>,
    pub defaultEditorBehaviour: Option<usize>,
    pub processMouseMoveInInspector: Option<usize>,
    pub drawingConfig: Option<Struct_drawingConfig>,
    pub HideSerializationCautionaryMessage: Option<usize>,
    pub HidePrefabCautionaryMessage: Option<usize>,
    pub HideOdinSerializeAttributeWarningMessages: Option<usize>,
    pub HideNonSerializedShowInInspectorWarningMessages: Option<usize>,
    pub buildSerializationFormat: Option<usize>,
    pub editorSerializationFormat: Option<usize>,
    pub loggingPolicy: Option<usize>,
    pub errorHandlingPolicy: Option<usize>,
    pub Status: Option<Struct_m_Preset>,
    pub Msg: Option<Struct_m_Preset>,
    pub RewardedAd: Option<usize>,
    pub floats: Option<Struct_m_Preset>,
    pub cycleSpeed: Option<usize>,
    pub cycleSize: Option<f64>,
    pub startInterval: Option<Struct_startInterval>,
    pub interval: Option<Struct_startInterval>,
    pub pressing: Option<usize>,
    pub OnDown: Option<Struct_OnDown>,
    pub OnUp: Option<Struct_OnUp>,
    pub m_text: Option<::serde_yaml::Value>,
    pub m_isRightToLeft: Option<usize>,
    pub m_fontAsset: Option<Struct_m_Preset>,
    pub m_sharedMaterial: Option<Struct_m_Preset>,
    pub m_fontSharedMaterials: Option<Vec<::serde_yaml::Value>>,
    pub m_fontMaterial: Option<Struct_m_Preset>,
    pub m_fontMaterials: Option<Vec<::serde_yaml::Value>>,
    pub m_fontColor32: Option<Struct_m_LegacySpecular>,
    pub m_fontColor: Option<Struct_m_FogColor>,
    pub m_enableVertexGradient: Option<usize>,
    pub m_colorMode: Option<usize>,
    pub m_fontColorGradient: Option<Struct_m_fontColorGradient>,
    pub m_fontColorGradientPreset: Option<Struct_m_Preset>,
    pub m_spriteAsset: Option<Struct_m_Preset>,
    pub m_tintAllSprites: Option<usize>,
    pub m_overrideHtmlColors: Option<usize>,
    pub m_faceColor: Option<Struct_m_LegacySpecular>,
    pub m_outlineColor: Option<Struct_m_LegacySpecular>,
    pub m_fontSize: Option<f64>,
    pub m_fontSizeBase: Option<f64>,
    pub m_fontWeight: Option<usize>,
    pub m_enableAutoSizing: Option<usize>,
    pub m_fontSizeMin: Option<usize>,
    pub m_fontSizeMax: Option<usize>,
    pub m_fontStyle: Option<usize>,
    pub m_textAlignment: Option<usize>,
    pub m_isAlignmentEnumConverted: Option<usize>,
    pub m_characterSpacing: Option<f64>,
    pub m_wordSpacing: Option<f64>,
    pub m_lineSpacing: Option<f64>,
    pub m_lineSpacingMax: Option<usize>,
    pub m_paragraphSpacing: Option<usize>,
    pub m_charWidthMaxAdj: Option<usize>,
    pub m_enableWordWrapping: Option<usize>,
    pub m_wordWrappingRatios: Option<f64>,
    pub m_overflowMode: Option<usize>,
    pub m_firstOverflowCharacterIndex: Option<isize>,
    pub m_linkedTextComponent: Option<Struct_m_Preset>,
    pub m_isLinkedTextComponent: Option<usize>,
    pub m_isTextTruncated: Option<usize>,
    pub m_enableKerning: Option<usize>,
    pub m_enableExtraPadding: Option<usize>,
    pub checkPaddingRequired: Option<usize>,
    pub m_isRichText: Option<usize>,
    pub m_parseCtrlCharacters: Option<usize>,
    pub m_isOrthographic: Option<usize>,
    pub m_isCullingEnabled: Option<usize>,
    pub m_ignoreRectMaskCulling: Option<usize>,
    pub m_ignoreCulling: Option<usize>,
    pub m_horizontalMapping: Option<usize>,
    pub m_verticalMapping: Option<usize>,
    pub m_uvLineOffset: Option<usize>,
    pub m_geometrySortingOrder: Option<usize>,
    pub m_firstVisibleCharacter: Option<usize>,
    pub m_useMaxVisibleDescender: Option<usize>,
    pub m_pageToDisplay: Option<usize>,
    pub m_margin: Option<Struct_m_Center>,
    pub m_textInfo: Option<Struct_m_textInfo>,
    pub m_havePropertiesChanged: Option<usize>,
    pub m_isUsingLegacyAnimationComponent: Option<usize>,
    pub m_isVolumetricText: Option<usize>,
    pub m_spriteAnimator: Option<Struct_m_Preset>,
    pub m_isInputParsingRequired: Option<usize>,
    pub m_inputSource: Option<usize>,
    pub m_hasFontAssetChanged: Option<usize>,
    pub m_subTextObjects: Option<Vec<Struct_m_Preset>>,
    pub m_baseMaterial: Option<Struct_m_Preset>,
    pub m_maskOffset: Option<Struct_m_Center>,
    pub text: Option<Struct_m_Preset>,
    pub duration: Option<usize>,
    pub anchoredSpeed: Option<Struct_m_Center>,
    pub cam: Option<Struct_m_Preset>,
    pub scrollTime: Option<f64>,
    pub curve: Option<Struct_curve_8>,
    pub camera: Option<Struct_m_Preset>,
    pub money: Option<usize>,
    pub target: Option<Struct_m_Preset>,
    pub rotationVector: Option<Struct_m_Center>,
    pub copyTarget: Option<Struct_m_Preset>,
    pub randomVillagerCreator: Option<Struct_m_Preset>,
    pub boxCollider: Option<Struct_m_Preset>,
    pub anim: Option<Struct_m_Preset>,
    pub remainTimer: Option<usize>,
    pub spawnRateOnSelectedArea: Option<usize>,
    pub danceClip: Option<Vec<Struct_m_Preset>>,
    pub rotation: Option<Struct_m_Center>,
    pub m_VertexBufferAutoSizeReduction: Option<usize>,
    pub scale: Option<::serde_yaml::Value>,
    pub bgOn: Option<Struct_m_Preset>,
    pub bgOff: Option<Struct_m_Preset>,
    pub bgTimer: Option<Struct_m_Preset>,
    pub discountPop: Option<Struct_m_Preset>,
    pub openPop: Option<Struct_m_Preset>,
    pub costTexts: Option<Vec<Struct_m_Preset>>,
    pub timeText: Option<Struct_m_Preset>,
    pub timeTimertext: Option<Struct_m_Preset>,
    pub fillImage: Option<Struct_m_Preset>,
    pub area: Option<Struct_m_Preset>,
    pub endTime: Option<usize>,
    pub worldPos: Option<Struct_m_Center>,
    pub hide: Option<usize>,
    pub cardPrefab: Option<Struct_m_Preset>,
    pub content: Option<Struct_m_Preset>,
    pub cardElements: Option<Vec<::serde_yaml::Value>>,
    pub m_HorizontalFit: Option<usize>,
    pub m_VerticalFit: Option<usize>,
    pub techIcon: Option<Struct_m_Preset>,
    pub techRequireIcon: Option<Struct_m_Preset>,
    pub buildingImage: Option<Struct_m_Preset>,
    pub nameText: Option<Struct_m_Preset>,
    pub moneyText: Option<Struct_m_Preset>,
    pub populationText: Option<Struct_m_Preset>,
    pub techText: Option<Struct_m_Preset>,
    pub requireText: Option<Struct_m_Preset>,
    pub buttonOn: Option<Struct_m_Preset>,
    pub buttonOff: Option<Struct_m_Preset>,
    pub lockPanel: Option<Struct_m_Preset>,
    pub notConstructablePanel: Option<Struct_m_Preset>,
    pub biomeDisplays: Option<Vec<Struct_biomeDisplays>>,
    pub allBiomeDisplay: Option<Struct_m_Preset>,
    pub m_StartCorner: Option<usize>,
    pub m_StartAxis: Option<usize>,
    pub m_CellSize: Option<Struct_m_Center>,
    pub m_Constraint: Option<usize>,
    pub m_ConstraintCount: Option<usize>,
    pub buildElementPrefab: Option<Struct_m_Preset>,
    pub buildingTypeOrder: Option<String>,
    pub hidePosY: Option<usize>,
    pub showPosY: Option<usize>,
    pub showTime: Option<usize>,
    pub buildElements: Option<Vec<::serde_yaml::Value>>,
    pub errorText: Option<Struct_m_Preset>,
    pub buttonBuildOn: Option<Struct_m_Preset>,
    pub buttonBuildOff: Option<Struct_m_Preset>,
    pub buttonCancel: Option<Struct_m_Preset>,
    pub errorDesc: Option<Struct_m_Preset>,
    pub zoomSize: Option<usize>,
    pub centerOffsetZ: Option<usize>,
    pub descText: Option<Struct_m_Preset>,
    pub skillNameText: Option<Struct_m_Preset>,
    pub cooldownText: Option<Struct_m_Preset>,
    pub moneyIcon: Option<Struct_m_Preset>,
    pub techIcons: Option<Vec<Struct_m_Preset>>,
    pub skillObject: Option<Struct_m_Preset>,
    pub lvLackMessage: Option<Struct_m_Preset>,
    pub noSkillMessage: Option<Struct_m_Preset>,
    pub cooldownMessage: Option<Struct_m_Preset>,
    pub skillImage: Option<Struct_m_Preset>,
    pub image: Option<Struct_m_Preset>,
    pub cardImageInfomations: Option<Vec<Struct_buildingIconInfoList>>,
    #[serde(rename = "type")]
    pub field_type: Option<usize>,
    pub numText: Option<Struct_m_Preset>,
    pub startHeight: Option<usize>,
    pub endHeight: Option<usize>,
    pub titleText: Option<Struct_m_Preset>,
    pub adCollectButton: Option<Struct_m_Preset>,
    pub collectButton: Option<Struct_m_Preset>,
    pub adFreeButton: Option<Struct_m_Preset>,
    pub height: Option<usize>,
    pub m_renderer: Option<Struct_m_Preset>,
    pub m_maskType: Option<usize>,
    pub speedCoeff: Option<usize>,
    pub distThres: Option<usize>,
    pub prepareTime: Option<f64>,
    pub spreadPower: Option<usize>,
    pub ImitBye: Option<usize>,
    pub populationPrefab: Option<Struct_m_Preset>,
    pub populationTransform: Option<Struct_m_Preset>,
    pub earnText: Option<Struct_m_Preset>,
    pub buildingUpgradeText: Option<Struct_m_Preset>,
    pub peopleUpgradeText: Option<Struct_m_Preset>,
    pub speedUpgradeText: Option<Struct_m_Preset>,
    pub medal: Option<Struct_m_Preset>,
    pub managementElementPrefab: Option<Struct_m_Preset>,
    pub movable: Option<usize>,
    pub cycle: Option<usize>,
    pub amount: Option<usize>,
    pub progressBar: Option<Struct_m_Preset>,
    pub bg_1: Option<Struct_m_Preset>,
    pub bg_2: Option<Struct_m_Preset>,
    pub able: Option<Struct_m_Preset>,
    pub disable: Option<Struct_m_Preset>,
    pub cardElementPrefab: Option<Struct_m_Preset>,
    pub imageOn: Option<Struct_m_Preset>,
    pub imageOff: Option<Struct_m_Preset>,
    pub iconUp: Option<Struct_m_Preset>,
    pub iconDown: Option<Struct_m_Preset>,
    pub iconUpOff: Option<Struct_m_Preset>,
    pub iconDownOff: Option<Struct_m_Preset>,
    pub button: Option<Struct_m_Preset>,
    pub btnLock: Option<Struct_m_Preset>,
    pub btnOn: Option<Struct_m_Preset>,
    pub btnPurchase: Option<Struct_m_Preset>,
    pub skillIcon: Option<Struct_m_Preset>,
    pub techIconColorOn: Option<Struct_m_FogColor>,
    pub techIconColorOff: Option<Struct_m_FogColor>,
    pub buttons: Option<Vec<Struct_m_Preset>>,
    pub disableImage: Option<Struct_m_Preset>,
    pub typeIconList: Option<Vec<Struct_m_Preset>>,
    pub levelSpotList: Option<Vec<Struct_m_Preset>>,
    pub techSelectElement: Option<Struct_m_Preset>,
    pub levelDescText: Option<Struct_m_Preset>,
    pub techType: Option<usize>,
    pub techTreeElementList: Option<Vec<Struct_m_Preset>>,
    pub elementInfoList: Option<Vec<Struct_elementInfoList>>,
    pub panelButtons: Option<Vec<Struct_m_Preset>>,
    pub panels: Option<Vec<Struct_m_Preset>>,
    pub buttonOnSprites: Option<Vec<Struct_m_Preset>>,
    pub buttonOffSprites: Option<Vec<Struct_m_Preset>>,
    pub gauge: Option<Struct_m_Preset>,
    pub levelText: Option<Struct_m_Preset>,
    pub expText: Option<Struct_m_Preset>,
    pub levelUpButton: Option<Struct_m_Preset>,
    pub fillGauge: Option<Struct_m_Preset>,
    pub btn: Option<Struct_m_Preset>,
    pub bgInfos: Option<Vec<Struct_bgInfos>>,
    pub bgOffPopul: Option<Struct_m_Preset>,
    pub bgFree: Option<Struct_m_Preset>,
    pub nameTexts: Option<Vec<::serde_yaml::Value>>,
    pub priceTexts: Option<Vec<Struct_m_Preset>>,
    pub pricePopulationTexts: Option<Vec<::serde_yaml::Value>>,
    pub kind: Option<usize>,
    pub level: Option<usize>,
    pub upgradeElementPrefab: Option<Struct_m_Preset>,
    pub synergyElementPrefab: Option<Struct_m_Preset>,
    pub synergyContent: Option<Struct_m_Preset>,
    pub selectPanel: Option<Struct_m_Preset>,
    pub synergyPanel: Option<Struct_m_Preset>,
    pub selectNameText: Option<Struct_m_Preset>,
    pub selectMoneyText: Option<Struct_m_Preset>,
    pub selectPopulationText: Option<Struct_m_Preset>,
    pub selectedBuildingImage: Option<Struct_m_Preset>,
    pub skillElementPrefab: Option<Struct_m_Preset>,
    pub skillType: Option<usize>,
    pub icon: Option<Struct_m_Preset>,
    pub contents: Option<Struct_m_Preset>,
    pub worldPosGap: Option<Struct_m_Center>,
    pub p: Option<Vec<Struct_m_Preset>>,
    pub entry: Option<Struct_m_Preset>,
    pub workSlots: Option<Vec<Struct_m_Preset>>,
    pub upgrades: Option<Struct_upgrades>,
    pub facilityLevel: Option<usize>,
    pub earn: Option<usize>,
    pub population: Option<usize>,
    pub workLocation: Option<Struct_m_Preset>,
    pub humanWorkType: Option<usize>,
    pub isAnimPlayOnce: Option<usize>,
    pub worker: Option<Struct_m_Preset>,
    pub building: Option<Struct_m_Preset>,
    pub liftDownLocation: Option<Struct_m_Preset>,
    pub modelSlots: Option<Vec<Struct_modelSlots>>,
    pub tierObjectInfo: Option<Vec<Struct_tierObjectInfo>>,
    pub firstWorkSlot: Option<Struct_m_Preset>,
    pub specialBuildingModel: Option<Struct_m_Preset>,
    pub techExp: Option<usize>,
    pub isSpecialBuilding: Option<usize>,
    pub upgradeCostMultiplier: Option<usize>,
    pub humanPrefabTest: Option<Struct_m_Preset>,
    pub isOpen: Option<usize>,
    pub moneyProgressBar: Option<Struct_m_Preset>,
    pub workPosInfos: Option<Vec<Struct_workPosInfos>>,
    pub currentArea: Option<Struct_m_Preset>,
    pub goWorkSpeed: Option<usize>,
    pub creator: Option<Struct_m_Preset>,
    pub humanAnim: Option<Struct_m_Preset>,
    pub dest: Option<Struct_m_Center>,
    pub currentWorkSlotEntry: Option<Struct_m_Preset>,
    pub currentWorkSlot: Option<Struct_m_Preset>,
    pub human: Option<Struct_m_Preset>,
    pub rightHand: Option<Struct_m_Preset>,
    pub leftHand: Option<Struct_m_Preset>,
    pub body: Option<Struct_m_Preset>,
    pub walkClip: Option<Struct_m_Preset>,
    pub defaultClip: Option<Struct_m_Preset>,
    pub animationInfos: Option<Vec<Struct_animationInfos>>,
    pub carryingAnimationInfos: Option<Vec<Struct_carryingAnimationInfos>>,
    pub area1: Option<Struct_m_Preset>,
    pub area2: Option<Struct_m_Preset>,
    pub area1Slots: Option<Vec<Struct_m_Preset>>,
    pub area2Slots: Option<Vec<Struct_m_Preset>>,
    pub area1Entry: Option<Struct_m_Preset>,
    pub area2Entry: Option<Struct_m_Preset>,
    pub modelInfos: Option<Vec<Struct_modelInfos>>,
    pub tunnelPrefab: Option<Struct_m_Preset>,
    pub modelHolder: Option<Struct_m_Preset>,
    pub navMeshCollider: Option<Struct_m_Preset>,
    pub tunnelNavMeshCollider: Option<Struct_m_Preset>,
    pub modelExplodeClip: Option<Struct_m_Preset>,
    pub open: Option<usize>,
    pub requireTownHallLv: Option<usize>,
    pub isMountain: Option<usize>,
    pub model: Option<Struct_m_Preset>,
    pub cards: Option<Vec<Struct_m_Preset>>,
    pub reward: Option<Struct_reward>,
    pub rootingEffect: Option<Struct_m_Preset>,
    pub projectionCam: Option<Struct_m_Preset>,
    pub shader: Option<Struct_m_Preset>,
    pub inputTex: Option<Struct_m_Preset>,
    pub noiseShader: Option<Struct_m_Preset>,
    pub noiseTex: Option<Struct_m_Preset>,
    pub addTexture: Option<Struct_m_Preset>,
    pub camPivot: Option<Struct_m_Preset>,
    pub floor: Option<Struct_m_Preset>,
    pub autoApplyValue: Option<usize>,
    pub bgColor: Option<Struct_m_FogColor>,
    pub inkColor: Option<Struct_m_FogColor>,
    pub stepsInk: Option<Struct_m_Center>,
    pub stepsA: Option<Struct_m_Center>,
    pub stepsB: Option<Struct_m_Center>,
    pub mapping: Option<Struct_m_Center>,
    pub timeSpeed: Option<Struct_m_Center>,
    pub finalBlitToCameraTarget: Option<usize>,
    pub biomeType: Option<usize>,
    pub tileFloor: Option<Struct_m_Preset>,
    pub borders: Option<::serde_yaml::Value>,
    pub biomeInfos: Option<Vec<Struct_biomeInfos>>,
    pub tileFloors: Option<Vec<Struct_m_Preset>>,
    pub walls: Option<Vec<Struct_m_Preset>>,
    pub generatedBorders: Option<Vec<::serde_yaml::Value>>,
    pub scrap: Option<Struct_m_Preset>,
    pub size: Option<::serde_yaml::Value>,
    pub buildPos: Option<Struct_m_Preset>,
    pub biome: Option<Struct_m_Preset>,
    pub openEffect: Option<Struct_m_Preset>,
    pub scrapRemoved: Option<usize>,
    pub areaTimerRunning: Option<usize>,
    pub areaTimerIdx: Option<usize>,
    pub adjoinAreas: Option<Vec<Struct_m_Preset>>,
    pub corners: Option<Vec<Struct_m_Preset>>,
    pub adjoinColor: Option<Struct_m_FogColor>,
    pub defaultColor: Option<Struct_m_FogColor>,
    pub biomeObject: Option<Vec<Struct_m_Preset>>,
    pub areaN: Option<Struct_m_Preset>,
    pub areaE: Option<Struct_m_Preset>,
    pub areaW: Option<Struct_m_Preset>,
    pub areaS: Option<Struct_m_Preset>,
    pub head: Option<Struct_m_Preset>,
    pub eyes: Option<Struct_m_Preset>,
    pub hairBase: Option<Struct_m_Preset>,
    pub hairFront: Option<Struct_m_Preset>,
    pub hairDeco: Option<Struct_m_Preset>,
    pub handOrigins: Option<Vec<Struct_m_Preset>>,
    pub bodyModels: Option<Vec<Struct_m_Preset>>,
    pub eyeModels: Option<Vec<Struct_m_Preset>>,
    pub hairBaseModels: Option<Vec<Struct_m_Preset>>,
    pub hairFrontModels: Option<Vec<Struct_m_Preset>>,
    pub hairDecoModels: Option<Vec<Struct_m_Preset>>,
    pub facialHairModels: Option<Vec<Struct_m_Preset>>,
    pub skinMaterials: Option<Vec<Struct_m_Preset>>,
    pub bodyMaterials: Option<Vec<Struct_m_Preset>>,
    pub hairMaterials: Option<Vec<Struct_m_Preset>>,
    pub eyesMaterials: Option<Vec<Struct_m_Preset>>,
    pub TotalMessagesSubmitted: Option<usize>,
    pub TotalMessagesFailed: Option<usize>,
    pub DesignMessagesSubmitted: Option<usize>,
    pub DesignMessagesFailed: Option<usize>,
    pub QualityMessagesSubmitted: Option<usize>,
    pub QualityMessagesFailed: Option<usize>,
    pub ErrorMessagesSubmitted: Option<usize>,
    pub ErrorMessagesFailed: Option<usize>,
    pub BusinessMessagesSubmitted: Option<usize>,
    pub BusinessMessagesFailed: Option<usize>,
    pub UserMessagesSubmitted: Option<usize>,
    pub UserMessagesFailed: Option<usize>,
    pub CustomArea: Option<()>,
    pub gameKey: Option<Vec<String>>,
    pub secretKey: Option<Vec<String>>,
    pub Build: Option<Vec<f64>>,
    pub SelectedPlatformStudio: Option<Vec<Option<()>>>,
    pub SelectedPlatformGame: Option<Vec<Option<()>>>,
    pub SelectedPlatformGameID: Option<String>,
    pub SelectedStudio: Option<String>,
    pub SelectedGame: Option<String>,
    pub NewVersion: Option<String>,
    pub Changes: Option<String>,
    pub SignUpOpen: Option<usize>,
    pub StudioName: Option<()>,
    pub GameName: Option<()>,
    pub EmailGA: Option<()>,
    pub IntroScreen: Option<usize>,
    pub InfoLogEditor: Option<usize>,
    pub InfoLogBuild: Option<usize>,
    pub VerboseLogBuild: Option<usize>,
    pub UseManualSessionHandling: Option<usize>,
    pub SendExampleGameDataToMyGame: Option<usize>,
    pub UseIMEI: Option<usize>,
    pub InternetConnectivity: Option<usize>,
    pub CustomDimensions01: Option<Vec<::serde_yaml::Value>>,
    pub CustomDimensions02: Option<Vec<::serde_yaml::Value>>,
    pub CustomDimensions03: Option<Vec<::serde_yaml::Value>>,
    pub ResourceItemTypes: Option<Vec<::serde_yaml::Value>>,
    pub ResourceCurrencies: Option<Vec<::serde_yaml::Value>>,
    pub LastCreatedGamePlatform: Option<usize>,
    pub Platforms: Option<usize>,
    pub CurrentInspectorState: Option<usize>,
    pub ClosedHints: Option<()>,
    pub DisplayHints: Option<usize>,
    pub DisplayHintsScrollState: Option<Struct_m_Center>,
    pub Logo: Option<Struct_m_Preset>,
    pub UpdateIcon: Option<Struct_m_Preset>,
    pub InfoIcon: Option<Struct_m_Preset>,
    pub DeleteIcon: Option<Struct_m_Preset>,
    pub GameIcon: Option<Struct_m_Preset>,
    pub HomeIcon: Option<Struct_m_Preset>,
    pub InstrumentIcon: Option<Struct_m_Preset>,
    pub QuestionIcon: Option<Struct_m_Preset>,
    pub UserIcon: Option<Struct_m_Preset>,
    pub AmazonIcon: Option<Struct_m_Preset>,
    pub GooglePlayIcon: Option<Struct_m_Preset>,
    pub iosIcon: Option<Struct_m_Preset>,
    pub macIcon: Option<Struct_m_Preset>,
    pub windowsPhoneIcon: Option<Struct_m_Preset>,
    pub UsePlayerSettingsBuildNumber: Option<usize>,
    pub SubmitErrors: Option<usize>,
    pub MaxErrorCount: Option<usize>,
    pub SubmitFpsAverage: Option<usize>,
    pub SubmitFpsCritical: Option<usize>,
    pub IncludeGooglePlay: Option<usize>,
    pub FpsCriticalThreshold: Option<usize>,
    pub FpsCirticalSubmitInterval: Option<usize>,
    pub PlatformFoldOut: Option<usize>,
    pub CustomDimensions01FoldOut: Option<usize>,
    pub CustomDimensions02FoldOut: Option<usize>,
    pub CustomDimensions03FoldOut: Option<usize>,
    pub ResourceItemTypesFoldOut: Option<usize>,
    pub ResourceCurrenciesFoldOut: Option<usize>,
    pub mesh: Option<Struct_m_Preset>,
    pub physicMaterial: Option<Struct_m_Preset>,
    pub translation: Option<Struct_m_Center>,
    pub generateCollider: Option<usize>,
    pub updateInPlayMode: Option<usize>,
    pub curveSpace: Option<usize>,
    pub mode: Option<usize>,
    pub nodes: Option<Vec<Struct_nodes>>,
    pub curves: Option<Vec<Struct_curves>>,
    pub Length: Option<f64>,
    pub isLoop: Option<usize>,
    pub CurveChanged: Option<Struct_m_OnCullStateChanged>,
    pub mainCam: Option<Struct_m_Preset>,
    pub folderBaseName: Option<String>,
    pub frameRate: Option<usize>,
    pub framesToCapture: Option<usize>,
    pub fileName: Option<String>,
    pub center: Option<Struct_m_Center>,
    pub removeScrapUI: Option<Struct_m_Preset>,
    pub constructPanel: Option<Struct_m_Preset>,
    pub upgradePanel: Option<Struct_m_Preset>,
    pub inventory: Option<Struct_m_Preset>,
    pub m_EventMask: Option<Struct_m_CullingMask>,
    pub m_MaxRayIntersections: Option<usize>,
    pub safeRect: Option<Struct_m_Preset>,
    pub simulate: Option<usize>,
    pub ui: Option<Struct_m_Preset>,
    pub initArea: Option<Struct_m_Preset>,
    pub areaList: Option<Vec<Struct_m_Preset>>,
    pub humanList: Option<Vec<::serde_yaml::Value>>,
    pub passages: Option<Vec<Struct_m_Preset>>,
    pub buildingPrefabList: Option<Vec<Struct_m_Preset>>,
    pub humanPrefab: Option<Struct_m_Preset>,
    pub dataTable: Option<Struct_m_Preset>,
    pub humanNum: Option<usize>,
    pub selectedArea: Option<Struct_m_Preset>,
    pub game: Option<Struct_m_Preset>,
    pub passagePopup: Option<Struct_m_Preset>,
    pub passageTimerDisplay: Option<Struct_m_Preset>,
    pub globalGameData: Option<Struct_m_Preset>,
    pub selectedPassage: Option<Struct_m_Preset>,
    pub tileSize: Option<f64>,
    pub tileOrigin: Option<Struct_m_Preset>,
    pub passageOrigin: Option<Struct_m_Preset>,
    pub minTypeInfos: Option<Vec<Struct_minTypeInfos>>,
    pub tilePrefab: Option<Struct_m_Preset>,
    pub passagePrefab: Option<Struct_m_Preset>,
    pub buildPanel: Option<Struct_m_Preset>,
    pub buildPopup: Option<Struct_m_Preset>,
    pub buildingInfoPanel: Option<Struct_m_Preset>,
    pub managementPanel: Option<Struct_m_Preset>,
    pub claimPopup: Option<Struct_m_Preset>,
    pub techTreePopup: Option<Struct_m_Preset>,
    pub techSkillPopup: Option<Struct_m_Preset>,
    pub floatingsGenerator: Option<Struct_m_Preset>,
    pub areaTimerDisplayPrefab: Option<Struct_m_Preset>,
    pub plusMoneyIndicatorPrefab: Option<Struct_m_Preset>,
    pub cardGetIndicatorPrefab: Option<Struct_m_Preset>,
    pub moneyProgressBarPrefab: Option<Struct_m_Preset>,
    pub bigMoneyProgressBarPrefab: Option<Struct_m_Preset>,
    pub progressBarHolder: Option<Struct_m_Preset>,
    pub indicatorHolder: Option<Struct_m_Preset>,
    pub TimerHolder: Option<Struct_m_Preset>,
    pub startFadeInImage: Option<Struct_m_Preset>,
    pub ActiveWhileShowPopup: Option<Vec<Struct_m_Preset>>,
    pub HideWhileShowPopup: Option<Vec<Struct_m_Preset>>,
    pub debugPanel: Option<Struct_m_Preset>,
    pub globalUIData: Option<Struct_m_Preset>,
    pub isPopupShow: Option<usize>,
    pub areaTimerDisplays: Option<Vec<::serde_yaml::Value>>,
    pub sizeRange: Option<Struct_startInterval>,
    pub centerPos: Option<Struct_m_Preset>,
    pub selectZoomSize: Option<usize>,
    pub zoomSpeed: Option<f64>,
    pub touchMoveDamping: Option<usize>,
    pub isScrolling: Option<usize>,
    pub adHuman: Option<Struct_m_Preset>,
    pub areaSillhouettePrefab: Option<Struct_m_Preset>,
    pub worldMinPos: Option<Struct_m_Center>,
    pub worldMaxPos: Option<Struct_m_Center>,
    pub navMeshSurface: Option<Struct_m_Preset>,
    pub navMeshBackUp: Option<Struct_m_Preset>,
    pub worldRoot: Option<Struct_m_Preset>,
    pub areaNavMeshColliderPrefab: Option<Struct_m_Preset>,
    pub AdHumanSpawnTimer: Option<usize>,
    pub AdHumanRandomRange: Option<usize>,
    pub currentPopulation: Option<usize>,
    pub lockSelectArea: Option<usize>,
    pub lastCamMove: Option<Struct_m_Center>,
    pub lastCamDeltaSize: Option<usize>,
    pub touchManager: Option<Struct_m_Preset>,
    pub exp: Option<usize>,
    pub cash: Option<usize>,
    pub tableLevelData: Option<Struct_m_Preset>,
    pub tableSlotData: Option<Struct_m_Preset>,
    pub tableOpenData: Option<Struct_m_Preset>,
    pub tableTechLevelData: Option<Struct_m_Preset>,
    pub tableTechBonusData: Option<Struct_m_Preset>,
    pub tableTechSkillData: Option<Struct_m_Preset>,
    pub techDataList: Option<Vec<::serde_yaml::Value>>,
    pub techSkillDataList: Option<Vec<::serde_yaml::Value>>,
    pub areaTimers: Option<Vec<::serde_yaml::Value>>,
    pub isTechSkillPopupOpen: Option<usize>,
    pub defaultBiomeTypes: Option<String>,
    pub m_NavMeshData: Option<Struct_m_Preset>,
    pub Player: Option<Struct_m_Preset>,
    pub PlayerCamera: Option<Struct_m_Preset>,
    pub FirstViewPlayerCamera: Option<Struct_m_Preset>,
    pub turnSpeed: Option<usize>,
    pub recordName: Option<()>,
    pub readDelay: Option<usize>,
    pub tableUpgradeData: Option<Struct_m_Preset>,
    pub virtualLands: Option<Vec<Struct_virtualLands>>,
    pub cheaperPrice: Option<usize>,
    pub Stop: Option<usize>,
    pub passageDisplay: Option<Struct_m_Preset>,
    pub areaSelectHighlight: Option<Vec<::serde_yaml::Value>>,
    pub tileCost: Option<usize>,
    pub tileTime: Option<usize>,
    pub fogs: Option<Struct_fogs>,
    pub touchMoveSpeed: Option<usize>,
    pub passage: Option<Struct_m_Preset>,
    pub k_AssetVersion: Option<usize>,
    pub m_RequireDepthTexture: Option<usize>,
    pub m_RequireOpaqueTexture: Option<usize>,
    pub m_OpaqueDownsampling: Option<usize>,
    pub m_SupportsHDR: Option<usize>,
    pub m_MSAA: Option<usize>,
    pub m_RenderScale: Option<usize>,
    pub m_MainLightRenderingMode: Option<usize>,
    pub m_MainLightShadowsSupported: Option<usize>,
    pub m_MainLightShadowmapResolution: Option<usize>,
    pub m_AdditionalLightsRenderingMode: Option<usize>,
    pub m_AdditionalLightsPerObjectLimit: Option<usize>,
    pub m_AdditionalLightShadowsSupported: Option<usize>,
    pub m_AdditionalLightsShadowmapResolution: Option<usize>,
    pub m_ShadowDistance: Option<usize>,
    pub m_ShadowCascades: Option<usize>,
    pub m_Cascade2Split: Option<f64>,
    pub m_Cascade4Split: Option<Struct_m_Center>,
    pub m_ShadowDepthBias: Option<usize>,
    pub m_ShadowNormalBias: Option<usize>,
    pub m_SoftShadowsSupported: Option<usize>,
    pub m_SupportsDynamicBatching: Option<usize>,
    pub m_MixedLightingSupported: Option<usize>,
    pub m_ShadowType: Option<usize>,
    pub m_LocalShadowsSupported: Option<usize>,
    pub m_LocalShadowsAtlasResolution: Option<usize>,
    pub m_MaxPixelLights: Option<usize>,
    pub m_ShadowAtlasResolution: Option<usize>,
    pub m_ResourcesAsset: Option<Struct_m_Preset>,
    pub m_ShaderVariantLogLevel: Option<usize>,
    pub placeUnder: Option<usize>,
    pub parentForPrefabs: Option<Struct_m_Preset>,
    pub _lineCount: Option<usize>,
    pub _vertexCount: Option<usize>,
    pub _mesh: Option<Struct_m_Preset>,
    pub startScale: Option<f64>,
    pub DurationInSecond: Option<usize>,
    pub shapeVertices: Option<Vec<Struct_shapeVertices>>,
    pub textureScale: Option<f64>,
    pub sampleSpacing: Option<f64>,
    pub scaleRange: Option<f64>,
    pub spacingRange: Option<f64>,
    pub offset: Option<f64>,
    pub offsetRange: Option<f64>,
    pub isRandomYaw: Option<usize>,
    pub randomSeed: Option<usize>,
    pub curvature: Option<f64>,
    pub wayPoints: Option<Vec<Struct_m_Preset>>,
    pub segmentPrefab: Option<Struct_m_Preset>,
    pub segmentCount: Option<usize>,
    pub segmentSpacing: Option<f64>,
    pub endScale: Option<f64>,
    pub startRoll: Option<usize>,
    pub endRoll: Option<usize>,
    pub Follower: Option<Struct_m_Preset>,
    pub generated: Option<Struct_m_Preset>,
    pub spriteSheet: Option<Struct_m_Preset>,
    pub m_SpriteCharacterTable: Option<Vec<Struct_m_SpriteCharacterTable>>,
    pub m_SpriteGlyphTable: Option<Vec<Struct_m_SpriteGlyphTable>>,
    pub spriteInfoList: Option<Vec<Struct_m_Center>>,
    pub fallbackSpriteAssets: Option<Vec<::serde_yaml::Value>>,
    pub m_StyleList: Option<Vec<Struct_m_StyleList>>,
    pub m_enableTintAllSprites: Option<usize>,
    pub m_enableParseEscapeCharacters: Option<usize>,
    pub m_missingGlyphCharacter: Option<usize>,
    pub m_warningsDisabled: Option<usize>,
    pub m_defaultFontAsset: Option<Struct_m_Preset>,
    pub m_defaultFontAssetPath: Option<String>,
    pub m_defaultFontSize: Option<usize>,
    pub m_defaultAutoSizeMinRatio: Option<f64>,
    pub m_defaultAutoSizeMaxRatio: Option<usize>,
    pub m_defaultTextMeshProTextContainerSize: Option<Struct_m_Center>,
    pub m_defaultTextMeshProUITextContainerSize: Option<Struct_m_Center>,
    pub m_autoSizeTextContainer: Option<usize>,
    pub m_fallbackFontAssets: Option<Vec<::serde_yaml::Value>>,
    pub m_matchMaterialPreset: Option<usize>,
    pub m_defaultSpriteAsset: Option<Struct_m_Preset>,
    pub m_defaultSpriteAssetPath: Option<String>,
    pub m_defaultColorGradientPresetsPath: Option<String>,
    pub m_enableEmojiSupport: Option<usize>,
    pub m_defaultStyleSheet: Option<Struct_m_Preset>,
    pub m_leadingCharacters: Option<Struct_m_Preset>,
    pub m_followingCharacters: Option<Struct_m_Preset>,
    pub m_FontCreatorRecentSettings: Option<Vec<Struct_m_FontCreatorRecentSettings>>,
    pub m_CreationSettingsSelectionIndex: Option<usize>,
    pub m_CreationSettingsIndex: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FontCreatorRecentSettings {
    pub sourceFontFileName: Option<()>,
    pub sourceFontFileGUID: String,
    pub pointSizeSamplingMode: usize,
    pub pointSize: usize,
    pub padding: usize,
    pub packingMode: usize,
    pub atlasWidth: usize,
    pub atlasHeight: usize,
    pub characterSetSelectionMode: usize,
    pub characterSequence: ::serde_yaml::Value,
    pub fontStyle: usize,
    pub fontStyleModifier: usize,
    pub renderMode: usize,
    pub includeFontFeatures: usize,
    pub referenceFontAssetGUID: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_StyleList {
    pub m_Name: String,
    pub m_HashCode: usize,
    pub m_OpeningDefinition: String,
    pub m_ClosingDefinition: String,
    pub m_OpeningTagArray: String,
    pub m_ClosingTagArray: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SpriteGlyphTable {
    pub m_Index: usize,
    pub m_Metrics: Struct_m_Metrics,
    pub m_GlyphRect: Struct_m_UsedGlyphRects,
    pub m_Scale: usize,
    pub m_AtlasIndex: usize,
    pub sprite: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Metrics {
    pub m_Width: usize,
    pub m_Height: usize,
    pub m_HorizontalBearingX: usize,
    pub m_HorizontalBearingY: f64,
    pub m_HorizontalAdvance: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SpriteCharacterTable {
    pub m_ElementType: usize,
    pub m_Unicode: usize,
    pub m_GlyphIndex: usize,
    pub m_Scale: usize,
    pub m_Name: ::serde_yaml::Value,
    pub m_HashCode: ::serde_yaml::Value,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_shapeVertices {
    pub point: Struct_m_Center,
    pub normal: Struct_m_Center,
    pub uCoord: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_fogs {
    pub fogSE: Struct_m_Preset,
    pub fogSW: Struct_m_Preset,
    pub fogE: Struct_m_Preset,
    pub fogS: Struct_m_Preset,
    pub fogW: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_virtualLands {
    pub lockMoney: usize,
    pub lockTime: usize,
    pub isLocked: usize,
    pub buildingType: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_minTypeInfos {
    #[serde(rename = "type")]
    pub field_type: usize,
    pub num: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curves {
    pub n1: Struct_n1,
    pub n2: Struct_nodes,
    pub Changed: Struct_m_OnCullStateChanged,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_n1 {
    pub position: Struct_m_Center,
    pub direction: Struct_m_Center,
    pub up: Struct_m_Center,
    pub scale: Struct_m_Center,
    pub roll: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_nodes {
    pub position: Struct_m_Center,
    pub direction: Struct_m_Center,
    pub up: Struct_m_Center,
    pub scale: Struct_m_Center,
    pub roll: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_biomeInfos {
    pub biomeType: usize,
    pub biomeMat: Struct_m_Preset,
    pub borderPrefabs: Vec<Struct_m_Preset>,
    pub wallPrefabs: Option<Vec<Struct_m_Preset>>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_reward {
    pub rewardMoney: usize,
    pub rewardCardNum: usize,
    pub rewards: Vec<Struct_rewards>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_rewards {
    #[serde(rename = "type")]
    pub field_type: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_modelInfos {
    pub biomeType: usize,
    pub models: Vec<Struct_m_Preset>,
    pub mtnModels: Vec<Struct_m_Preset>,
    pub explodeEffectPrefab: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_carryingAnimationInfos {
    pub workType: usize,
    pub liftUpClip: Struct_m_Preset,
    pub carryClip: Struct_m_Preset,
    pub liftdownClip: Struct_m_Preset,
    pub returnClip: Struct_m_Preset,
    pub rightHandTool: Struct_m_Preset,
    pub leftHandTool: Struct_m_Preset,
    pub bodyTool: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_animationInfos {
    pub workType: usize,
    pub animationClip: Struct_m_Preset,
    pub rightHandTool: Struct_m_Preset,
    pub leftHandTool: Struct_m_Preset,
    pub bodyTool: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_workPosInfos {
    pub workLocation: Struct_m_Preset,
    pub worker: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_tierObjectInfo {
    pub requireLevel: usize,
    pub tierObjects: Vec<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_modelSlots {
    pub requireLv: usize,
    pub defaultModel: Struct_m_Preset,
    pub selectModels: Vec<Struct_m_Preset>,
    pub discardSelectModels: Vec<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_upgrades {
    pub facilityLevel: usize,
    pub peopleLevel: usize,
    pub speedLevel: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_bgInfos {
    pub kind: usize,
    pub spriteOn: Struct_m_Preset,
    pub sprietOff: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_elementInfoList {
    #[serde(rename = "type")]
    pub field_type: usize,
    pub isUp: usize,
    pub effectDescList: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_biomeDisplays {
    pub biomeType: usize,
    pub obj: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_8 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_22>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_22 {
    pub serializedVersion: usize,
    pub time: f64,
    pub value: f64,
    pub inSlope: f64,
    pub outSlope: f64,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: f64,
    pub outWeight: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_textInfo {
    pub textComponent: Struct_m_Preset,
    pub characterCount: usize,
    pub spriteCount: usize,
    pub spaceCount: usize,
    pub wordCount: usize,
    pub linkCount: usize,
    pub lineCount: usize,
    pub pageCount: usize,
    pub materialCount: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_fontColorGradient {
    pub topLeft: Struct_m_FogColor,
    pub topRight: Struct_m_FogColor,
    pub bottomLeft: Struct_m_FogColor,
    pub bottomRight: Struct_m_FogColor,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_OnUp {
    pub m_PersistentCalls: Struct_m_PersistentCalls,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PersistentCalls {
    pub m_Calls: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_OnDown {
    pub m_PersistentCalls: Struct_m_PersistentCalls_1,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PersistentCalls_1 {
    pub m_Calls: Vec<Struct_m_Calls>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Calls {
    pub m_Target: Struct_m_Preset,
    pub m_MethodName: String,
    pub m_Mode: usize,
    pub m_Arguments: Struct_m_Arguments,
    pub m_CallState: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Arguments {
    pub m_ObjectArgument: Struct_m_Preset,
    pub m_ObjectArgumentAssemblyTypeName: String,
    pub m_IntArgument: usize,
    pub m_FloatArgument: usize,
    pub m_StringArgument: Option<()>,
    pub m_BoolArgument: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_startInterval {
    pub min: usize,
    pub max: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_drawingConfig {
    pub configs: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_MyStruct {
    pub X: f64,
    pub Y: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_F {
    pub Name: Option<()>,
    pub Value: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_serializationData {
    pub SerializedFormat: usize,
    pub SerializedBytes: Option<()>,
    pub ReferencedUnityObjects: Vec<::serde_yaml::Value>,
    pub SerializedBytesString: Option<()>,
    pub Prefab: Struct_m_Preset,
    pub PrefabModificationsReferencedUnityObjects: Vec<::serde_yaml::Value>,
    pub PrefabModifications: Vec<::serde_yaml::Value>,
    pub SerializationNodes: Vec<Struct_SerializationNodes>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_SerializationNodes {
    pub Name: Option<String>,
    pub Entry: usize,
    pub Data: Option<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_MyClass {
    pub Name: String,
    pub Value: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_list {
    pub cam: Struct_m_Preset,
    pub deleteQuestion: usize,
    pub hotkey: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_editorData {
    pub _bezierPath: Struct__bezierPath,
    pub vertexPathUpToDate: usize,
    pub vertexPathMaxAngleError: f64,
    pub vertexPathMinVertexSpacing: f64,
    pub showTransformTool: Option<usize>,
    pub showPathBounds: usize,
    pub showPerSegmentBounds: usize,
    pub displayAnchorPoints: usize,
    pub displayControlPoints: usize,
    pub bezierHandleScale: f64,
    pub globalDisplaySettingsFoldout: usize,
    pub keepConstantHandleSize: usize,
    pub showNormalsInVertexMode: usize,
    pub showBezierPathInVertexMode: usize,
    pub showDisplayOptions: usize,
    pub showPathOptions: usize,
    pub showVertexPathDisplayOptions: usize,
    pub showVertexPathOptions: usize,
    pub showNormals: usize,
    pub showNormalsHelpInfo: usize,
    pub tabIndex: usize,
    pub pathTransformationEnabled: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct__bezierPath {
    pub points: Vec<Struct_m_Center>,
    pub isClosed: usize,
    pub space: usize,
    pub controlMode: usize,
    pub autoControlLength: f64,
    pub boundsUpToDate: usize,
    pub bounds: Struct_m_LocalAABB,
    pub rotation: Struct_m_Center,
    pub scale: Struct_m_Center,
    pub perAnchorNormalsAngle: Vec<f64>,
    pub globalNormalsAngle: usize,
    pub flipNormals: usize,
    pub localPosition: Option<Struct_m_Center>,
    pub pivot: Option<Struct_m_Center>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_21 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<::serde_yaml::Value>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_onValueChanged {
    pub m_PersistentCalls: Struct_m_PersistentCalls_2,
    pub m_TypeName: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PersistentCalls_2 {
    pub m_Calls: Vec<Struct_m_Calls_1>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Calls_1 {
    pub m_Target: Struct_m_Preset,
    pub m_MethodName: String,
    pub m_Mode: usize,
    pub m_Arguments: Struct_m_Arguments_1,
    pub m_CallState: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Arguments_1 {
    pub m_ObjectArgument: Struct_m_Preset,
    pub m_ObjectArgumentAssemblyTypeName: String,
    pub m_IntArgument: usize,
    pub m_FloatArgument: usize,
    pub m_StringArgument: Option<()>,
    pub m_BoolArgument: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_OnValueChanged {
    pub m_PersistentCalls: Struct_m_PersistentCalls_2,
    pub m_TypeName: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Options {
    pub m_Options: Vec<Struct_m_Options_1>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Options_1 {
    pub m_Text: String,
    pub m_Image: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FontWeightTable {
    pub regularTypeface: Struct_m_Preset,
    pub italicTypeface: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_CreationSettings {
    pub sourceFontFileName: Option<()>,
    pub sourceFontFileGUID: String,
    pub pointSizeSamplingMode: usize,
    pub pointSize: usize,
    pub padding: usize,
    pub packingMode: usize,
    pub atlasWidth: usize,
    pub atlasHeight: usize,
    pub characterSetSelectionMode: usize,
    pub characterSequence: String,
    pub referencedFontAssetGUID: Option<String>,
    pub referencedTextAssetGUID: Option<()>,
    pub fontStyle: usize,
    pub fontStyleModifier: usize,
    pub renderMode: usize,
    pub includeFontFeatures: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FontFeatureTable {
    pub m_GlyphPairAdjustmentRecords: Vec<Struct_m_GlyphPairAdjustmentRecords>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GlyphPairAdjustmentRecords {
    pub m_FirstAdjustmentRecord: Struct_m_FirstAdjustmentRecord,
    pub m_SecondAdjustmentRecord: Struct_m_SecondAdjustmentRecord,
    pub m_FeatureLookupFlags: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SecondAdjustmentRecord {
    pub m_GlyphIndex: usize,
    pub m_GlyphValueRecord: Struct_m_GlyphValueRecord,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GlyphValueRecord {
    pub m_XPlacement: usize,
    pub m_YPlacement: usize,
    pub m_XAdvance: usize,
    pub m_YAdvance: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FirstAdjustmentRecord {
    pub m_GlyphIndex: usize,
    pub m_GlyphValueRecord: Struct_m_GlyphValueRecord_1,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GlyphValueRecord_1 {
    pub m_XPlacement: usize,
    pub m_YPlacement: usize,
    pub m_XAdvance: ::serde_yaml::Value,
    pub m_YAdvance: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_KerningTable {
    pub kerningPairs: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_fontInfo {
    pub Name: Option<String>,
    pub PointSize: usize,
    pub Scale: usize,
    pub CharacterCount: usize,
    pub LineHeight: f64,
    pub Baseline: usize,
    pub Ascender: f64,
    pub CapHeight: f64,
    pub Descender: f64,
    pub CenterLine: usize,
    pub SuperscriptOffset: f64,
    pub SubscriptOffset: f64,
    pub SubSize: f64,
    pub Underline: f64,
    pub UnderlineThickness: f64,
    pub strikethrough: f64,
    pub strikethroughThickness: f64,
    pub TabWidth: f64,
    pub Padding: usize,
    pub AtlasWidth: usize,
    pub AtlasHeight: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_UsedGlyphRects {
    pub m_X: usize,
    pub m_Y: usize,
    pub m_Width: usize,
    pub m_Height: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_CharacterTable {
    pub m_ElementType: usize,
    pub m_Unicode: usize,
    pub m_GlyphIndex: usize,
    pub m_Scale: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GlyphTable {
    pub m_Index: usize,
    pub m_Metrics: Struct_m_Metrics_1,
    pub m_GlyphRect: Struct_m_GlyphRect,
    pub m_Scale: usize,
    pub m_AtlasIndex: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_GlyphRect {
    pub m_X: usize,
    pub m_Y: isize,
    pub m_Width: usize,
    pub m_Height: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Metrics_1 {
    pub m_Width: f64,
    pub m_Height: f64,
    pub m_HorizontalBearingX: ::serde_yaml::Value,
    pub m_HorizontalBearingY: f64,
    pub m_HorizontalAdvance: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FaceInfo {
    pub m_FamilyName: String,
    pub m_StyleName: Option<String>,
    pub m_PointSize: usize,
    pub m_Scale: usize,
    pub m_LineHeight: f64,
    pub m_AscentLine: f64,
    pub m_CapLine: f64,
    pub m_MeanLine: usize,
    pub m_Baseline: usize,
    pub m_DescentLine: f64,
    pub m_SuperscriptOffset: f64,
    pub m_SuperscriptSize: f64,
    pub m_SubscriptOffset: f64,
    pub m_SubscriptSize: f64,
    pub m_UnderlineOffset: f64,
    pub m_UnderlineThickness: f64,
    pub m_StrikethroughOffset: f64,
    pub m_StrikethroughThickness: f64,
    pub m_TabWidth: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_appLinkSchemes {
    pub list: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Padding {
    pub m_Left: usize,
    pub m_Right: usize,
    pub m_Top: usize,
    pub m_Bottom: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_BeforeStackBundles {
    pub assemblyQualifiedName: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_debugLayer {
    pub lightMeter: Struct_lightMeter,
    pub histogram: Struct_histogram,
    pub waveform: Struct_waveform,
    pub vectorscope: Struct_vectorscope,
    pub overlaySettings: Struct_overlaySettings,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_overlaySettings {
    pub motionColorIntensity: usize,
    pub motionGridSize: usize,
    pub colorBlindnessType: usize,
    pub colorBlindnessStrength: usize,
    pub linearDepth: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_vectorscope {
    pub size: usize,
    pub exposure: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_waveform {
    pub exposure: f64,
    pub height: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_histogram {
    pub width: usize,
    pub height: usize,
    pub channel: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_lightMeter {
    pub width: usize,
    pub height: usize,
    pub showCurves: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_fastApproximateAntialiasing {
    pub fastMode: usize,
    pub keepAlpha: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_subpixelMorphologicalAntialiasing {
    pub quality: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_temporalAntialiasing {
    pub jitterSpread: f64,
    pub sharpness: f64,
    pub stationaryBlending: f64,
    pub motionBlending: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_particleSystems {
    pub title: String,
    pub description: String,
    pub isWeaponEffect: usize,
    pub particleSystemGO: Struct_m_Preset,
    pub particlePosition: Struct_m_Center,
    pub particleRotation: Struct_m_Center,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_OnClick {
    pub m_PersistentCalls: Struct_m_PersistentCalls_3,
    pub m_TypeName: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_PersistentCalls_3 {
    pub m_Calls: Vec<Struct_m_Calls_2>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Calls_2 {
    pub m_Target: Struct_m_Preset,
    pub m_MethodName: String,
    pub m_Mode: usize,
    pub m_Arguments: Struct_m_Arguments_2,
    pub m_CallState: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Arguments_2 {
    pub m_ObjectArgument: Struct_m_Preset,
    pub m_ObjectArgumentAssemblyTypeName: String,
    pub m_IntArgument: isize,
    pub m_FloatArgument: usize,
    pub m_StringArgument: Option<()>,
    pub m_BoolArgument: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_AnimationTriggers {
    pub m_NormalTrigger: String,
    pub m_HighlightedTrigger: String,
    pub m_PressedTrigger: String,
    pub m_DisabledTrigger: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_SpriteState {
    pub m_HighlightedSprite: Struct_m_Preset,
    pub m_PressedSprite: Struct_m_Preset,
    pub m_DisabledSprite: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Colors {
    pub m_NormalColor: Struct_m_FogColor,
    pub m_HighlightedColor: Struct_m_FogColor,
    pub m_PressedColor: Struct_m_FogColor,
    pub m_DisabledColor: Struct_m_FogColor,
    pub m_ColorMultiplier: usize,
    pub m_FadeDuration: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Navigation {
    pub m_Mode: usize,
    pub m_SelectOnUp: Struct_m_Preset,
    pub m_SelectOnDown: Struct_m_Preset,
    pub m_SelectOnLeft: Struct_m_Preset,
    pub m_SelectOnRight: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_FontData {
    pub m_Font: Struct_m_Preset,
    pub m_FontSize: usize,
    pub m_FontStyle: usize,
    pub m_BestFit: usize,
    pub m_MinSize: usize,
    pub m_MaxSize: usize,
    pub m_Alignment: usize,
    pub m_AlignByGeometry: usize,
    pub m_RichText: usize,
    pub m_HorizontalOverflow: usize,
    pub m_VerticalOverflow: usize,
    pub m_LineSpacing: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_OnCullStateChanged {
    pub m_PersistentCalls: Struct_m_PersistentCalls,
    pub m_TypeName: Option<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_lumVsSatCurve {
    pub overrideState: usize,
    pub value: Struct_value,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_value {
    pub curve: Struct_curve_9,
    pub m_Loop: usize,
    pub m_ZeroValue: f64,
    pub m_Range: usize,
    pub cachedData: Vec<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_9 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_23>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_23 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: f64,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: usize,
    pub outWeight: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_satVsSatCurve {
    pub overrideState: usize,
    pub value: Struct_value_1,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_value_1 {
    pub curve: Struct_curve_10,
    pub m_Loop: usize,
    pub m_ZeroValue: f64,
    pub m_Range: usize,
    pub cachedData: Vec<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_10 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_24>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_24 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: f64,
    pub inSlope: ::serde_yaml::Value,
    pub outSlope: ::serde_yaml::Value,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: usize,
    pub outWeight: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_hueVsHueCurve {
    pub overrideState: usize,
    pub value: Struct_value_2,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_value_2 {
    pub curve: Struct_m_Curve_21,
    pub m_Loop: usize,
    pub m_ZeroValue: f64,
    pub m_Range: usize,
    pub cachedData: Vec<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_masterCurve {
    pub overrideState: usize,
    pub value: Struct_value_3,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_value_3 {
    pub curve: Struct_curve_11,
    pub m_Loop: usize,
    pub m_ZeroValue: usize,
    pub m_Range: usize,
    pub cachedData: Vec<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_11 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_25>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_25 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: usize,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
    pub weightedMode: usize,
    pub inWeight: usize,
    pub outWeight: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_lift {
    pub overrideState: usize,
    pub value: Struct_m_Center,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_tint {
    pub overrideState: usize,
    pub value: isize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_dirtTexture {
    pub overrideState: usize,
    pub value: Struct_m_Preset,
    pub defaultState: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_color {
    pub overrideState: usize,
    pub value: Struct_m_FogColor,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_intensity {
    pub overrideState: usize,
    pub value: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_enabled {
    pub overrideState: usize,
    pub value: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_monitors {
    pub currentMonitorID: usize,
    pub refreshOnPlay: usize,
    pub histogramMode: usize,
    pub waveformExposure: f64,
    pub waveformY: usize,
    pub waveformR: usize,
    pub waveformG: usize,
    pub waveformB: usize,
    pub paradeExposure: f64,
    pub vectorscopeExposure: f64,
    pub vectorscopeShowBackground: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_vignette {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_1,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_1 {
    pub mode: usize,
    pub color: Struct_m_FogColor,
    pub center: Struct_m_Center,
    pub intensity: f64,
    pub smoothness: f64,
    pub roundness: usize,
    pub mask: Struct_m_Preset,
    pub opacity: usize,
    pub rounded: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_grain {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_2,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_2 {
    pub colored: usize,
    pub intensity: f64,
    pub size: usize,
    pub luminanceContribution: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_chromaticAberration {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_3,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_3 {
    pub spectralTexture: Struct_m_Preset,
    pub intensity: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_userLut {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_4,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_4 {
    pub lut: Struct_m_Preset,
    pub contribution: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_colorGrading {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_5,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_5 {
    pub tonemapping: Struct_tonemapping,
    pub basic: Struct_basic,
    pub channelMixer: Struct_channelMixer,
    pub colorWheels: Struct_colorWheels,
    pub curves: Struct_curves_1,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curves_1 {
    pub master: Struct_master,
    pub red: Struct_master,
    pub green: Struct_master,
    pub blue: Struct_master,
    pub hueVShue: Struct_hueVShue,
    pub hueVSsat: Struct_hueVShue,
    pub satVSsat: Struct_hueVShue,
    pub lumVSsat: Struct_hueVShue,
    pub e_CurrentEditingCurve: usize,
    pub e_CurveY: usize,
    pub e_CurveR: usize,
    pub e_CurveG: usize,
    pub e_CurveB: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_hueVShue {
    pub curve: Struct_m_Curve_21,
    pub m_Loop: usize,
    pub m_ZeroValue: f64,
    pub m_Range: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_master {
    pub curve: Struct_curve_12,
    pub m_Loop: usize,
    pub m_ZeroValue: usize,
    pub m_Range: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_curve_12 {
    pub serializedVersion: usize,
    pub m_Curve: Vec<Struct_m_Curve_26>,
    pub m_PreInfinity: usize,
    pub m_PostInfinity: usize,
    pub m_RotationOrder: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Curve_26 {
    pub serializedVersion: usize,
    pub time: usize,
    pub value: usize,
    pub inSlope: usize,
    pub outSlope: usize,
    pub tangentMode: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_colorWheels {
    pub mode: usize,
    pub log: Struct_log,
    pub linear: Struct_linear,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_linear {
    pub lift: Struct_m_FogColor,
    pub gamma: Struct_m_FogColor,
    pub gain: Struct_m_FogColor,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_log {
    pub slope: Struct_m_FogColor,
    pub power: Struct_m_FogColor,
    pub offset: Struct_m_FogColor,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_channelMixer {
    pub red: Struct_m_Center,
    pub green: Struct_m_Center,
    pub blue: Struct_m_Center,
    pub currentEditingChannel: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_basic {
    pub postExposure: usize,
    pub temperature: usize,
    pub tint: usize,
    pub hueShift: usize,
    pub saturation: usize,
    pub contrast: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_tonemapping {
    pub tonemapper: usize,
    pub neutralBlackIn: f64,
    pub neutralWhiteIn: usize,
    pub neutralBlackOut: usize,
    pub neutralWhiteOut: usize,
    pub neutralWhiteLevel: f64,
    pub neutralWhiteClip: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_bloom {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_6,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_6 {
    pub bloom: Struct_bloom_1,
    pub lensDirt: Struct_lensDirt,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_lensDirt {
    pub texture: Struct_m_Preset,
    pub intensity: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_bloom_1 {
    pub intensity: f64,
    pub threshold: f64,
    pub softKnee: f64,
    pub radius: usize,
    pub antiFlicker: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_eyeAdaptation {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_7,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_7 {
    pub lowPercent: usize,
    pub highPercent: usize,
    pub minLuminance: isize,
    pub maxLuminance: usize,
    pub keyValue: f64,
    pub dynamicKeyValue: usize,
    pub adaptationType: usize,
    pub speedUp: usize,
    pub speedDown: usize,
    pub logMin: isize,
    pub logMax: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_motionBlur {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_8,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_8 {
    pub shutterAngle: usize,
    pub sampleCount: usize,
    pub frameBlending: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_depthOfField {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_9,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_9 {
    pub focusDistance: usize,
    pub aperture: f64,
    pub focalLength: usize,
    pub useCameraFov: usize,
    pub kernelSize: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_screenSpaceReflection {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_10,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_10 {
    pub reflection: Struct_reflection,
    pub intensity: Struct_intensity_1,
    pub screenEdgeMask: Struct_screenEdgeMask,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_screenEdgeMask {
    pub intensity: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_intensity_1 {
    pub reflectionMultiplier: usize,
    pub fadeDistance: usize,
    pub fresnelFade: usize,
    pub fresnelFadePower: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_reflection {
    pub blendType: usize,
    pub reflectionQuality: usize,
    pub maxDistance: usize,
    pub iterationCount: usize,
    pub stepSize: usize,
    pub widthModifier: f64,
    pub reflectionBlur: usize,
    pub reflectBackfaces: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_ambientOcclusion {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_11,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_11 {
    pub intensity: usize,
    pub radius: f64,
    pub sampleCount: usize,
    pub downsampling: usize,
    pub forceForwardCompatibility: usize,
    pub ambientOnly: usize,
    pub highPrecision: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_antialiasing {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_12,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_12 {
    pub method: usize,
    pub fxaaSettings: Struct_fxaaSettings,
    pub taaSettings: Struct_taaSettings,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_taaSettings {
    pub jitterSpread: f64,
    pub sharpen: f64,
    pub stationaryBlending: f64,
    pub motionBlending: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_fxaaSettings {
    pub preset: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_debugViews {
    pub m_Enabled: usize,
    pub m_Settings: Struct_m_Settings_13,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Settings_13 {
    pub mode: usize,
    pub depth: Struct_depth,
    pub motionVectors: Struct_motionVectors,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_motionVectors {
    pub sourceOpacity: usize,
    pub motionImageOpacity: usize,
    pub motionImageAmplitude: usize,
    pub motionVectorsOpacity: usize,
    pub motionVectorsResolution: usize,
    pub motionVectorsAmplitude: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_depth {
    pub scale: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_AndroidNotificationEditorSettingsValues {
    pub keys: Vec<String>,
    pub values: Vec<String>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_iOSNotificationEditorSettingsValues {
    pub keys: Vec<String>,
    pub values: Vec<::serde_yaml::Value>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_infoList {
    pub skillType: usize,
    pub isActiveSkill: usize,
    pub hasCooldown: usize,
    pub cooldown: usize,
    pub canStack: usize,
    pub maxStackTime: usize,
    pub effectDuration: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_techBonusInfoList {
    pub techType: usize,
    pub levelBonusList: Vec<Struct_levelBonusList>,
    pub selectableBonusList: Vec<Struct_selectableBonusList>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_selectableBonusList {
    pub requireLevel: usize,
    pub bonusList: Vec<Struct_bonusList>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_bonusList {
    pub name: String,
    pub description: String,
    pub constraintType: usize,
    pub constraintName: Option<String>,
    pub constraintMultiplier: f64,
    pub bonusEffects: Vec<Struct_bonusEffects>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_bonusEffects {
    pub targetType: usize,
    pub targetName: Option<String>,
    pub rewardType: usize,
    pub isPercent: usize,
    pub rewardAmount: isize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_levelBonusList {
    pub startLevel: usize,
    pub endLevel: usize,
    pub multiplierPerLevel: f64,
    pub bonus: Struct_bonus,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_bonus {
    pub name: String,
    pub description: String,
    pub constraintType: usize,
    pub constraintName: Option<()>,
    pub constraintMultiplier: usize,
    pub bonusEffects: Vec<Struct_bonusEffects>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_upgradeInfos {
    pub buildingType: usize,
    pub upgradeKind: usize,
    pub priceMoney: Vec<usize>,
    pub pricePopulation: String,
    pub values: Vec<f64>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_slotInfos {
    pub buildingType: usize,
    pub slotNum: usize,
    pub moneyGain: usize,
    pub moneyGainPerLevel: f64,
    pub earnInterval: f64,
    pub earnIntervalPerLevel: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_openInfos {
    pub price: usize,
    pub time: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_levelInfos {
    pub level: usize,
    pub moneySpend: Option<::serde_yaml::Value>,
    pub moneyEarn: Option<::serde_yaml::Value>,
    pub population: Option<usize>,
    pub exp: Option<usize>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_techSelectBonusIconInfoList {
    pub name: String,
    pub sprite: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_buildingIconInfoList {
    #[serde(rename = "type")]
    pub field_type: usize,
    pub sprite: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_techGraphicInfoList {
    pub techType: usize,
    pub sprite: Struct_m_Preset,
    pub textColor: Struct_m_FogColor,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_buildingGlobalDataList {
    pub buildingType: usize,
    pub isSpecialBuilding: usize,
    pub techType: usize,
    pub techExp: usize,
    pub population: usize,
    pub baseConstructCost: usize,
    pub requireLevel: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_synergyData {
    pub synergyTarget: usize,
    pub syngergyCondition: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_buildingTypeInBiomes {
    pub biomeType: usize,
    pub buildingTypes: ::serde_yaml::Value,
}

// NavMeshAgent
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct NavMeshAgent {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_AgentTypeID: ::serde_yaml::Value,
    pub m_Radius: f64,
    pub m_Speed: f64,
    pub m_Acceleration: usize,
    pub avoidancePriority: usize,
    pub m_AngularSpeed: usize,
    pub m_StoppingDistance: usize,
    pub m_AutoTraverseOffMeshLink: usize,
    pub m_AutoBraking: usize,
    pub m_AutoRepath: usize,
    pub m_Height: usize,
    pub m_BaseOffset: f64,
    pub m_WalkableMask: usize,
    pub m_ObstacleAvoidanceType: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// EditorBuildSettings
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct EditorBuildSettings {
    pub m_ObjectHideFlags: usize,
    pub serializedVersion: usize,
    pub m_Scenes: Vec<Struct_m_Scenes>,
    pub m_configObjects: Struct_m_AssetToPrefab,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_Scenes {
    pub enabled: usize,
    pub path: String,
    pub guid: String,
}

// AnimatorController
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AnimatorController {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_Name: String,
    pub serializedVersion: usize,
    pub m_AnimatorParameters: Vec<Struct_m_AnimatorParameters>,
    pub m_AnimatorLayers: Vec<Struct_m_AnimatorLayers>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_AnimatorLayers {
    pub serializedVersion: usize,
    pub m_Name: String,
    pub m_StateMachine: Struct_m_Preset,
    pub m_Mask: Struct_m_Preset,
    pub m_Motions: Vec<::serde_yaml::Value>,
    pub m_Behaviours: Vec<::serde_yaml::Value>,
    pub m_BlendingMode: usize,
    pub m_SyncedLayerIndex: isize,
    pub m_DefaultWeight: usize,
    pub m_IKPass: usize,
    pub m_SyncedLayerAffectsTiming: usize,
    pub m_Controller: Struct_m_Preset,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_AnimatorParameters {
    pub m_Name: String,
    pub m_Type: usize,
    pub m_DefaultFloat: usize,
    pub m_DefaultInt: usize,
    pub m_DefaultBool: usize,
    pub m_Controller: Struct_m_Preset,
}

// PhysicMaterial
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct PhysicMaterial {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_Name: String,
    pub dynamicFriction: f64,
    pub staticFriction: f64,
    pub bounciness: usize,
    pub frictionCombine: usize,
    pub bounceCombine: usize,
}

// RectTransform
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct RectTransform {
    pub m_ObjectHideFlags: Option<usize>,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Option<Struct_m_Preset>,
    pub m_LocalRotation: Option<Struct_m_Center>,
    pub m_LocalPosition: Option<Struct_m_Center>,
    pub m_LocalScale: Option<Struct_m_Center>,
    pub m_Children: Option<Vec<Struct_m_Preset>>,
    pub m_Father: Option<Struct_m_Preset>,
    pub m_RootOrder: Option<usize>,
    pub m_LocalEulerAnglesHint: Option<Struct_m_Center>,
    pub m_AnchorMin: Option<Struct_m_Center>,
    pub m_AnchorMax: Option<Struct_m_Center>,
    pub m_AnchoredPosition: Option<Struct_m_Center>,
    pub m_SizeDelta: Option<Struct_m_Center>,
    pub m_Pivot: Option<Struct_m_Center>,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// TerrainCollider
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct TerrainCollider {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Struct_m_Preset,
    pub m_PrefabInternal: Struct_m_Preset,
    pub m_GameObject: Struct_m_Preset,
    pub m_Material: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_TerrainData: Struct_m_Preset,
    pub m_EnableTreeColliders: usize,
}

// AudioListener
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct AudioListener {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// Behaviour
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Behaviour {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_Enabled: usize,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
}

// CanvasRenderer
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct CanvasRenderer {
    pub m_ObjectHideFlags: usize,
    pub m_PrefabParentObject: Option<Struct_m_Preset>,
    pub m_PrefabInternal: Option<Struct_m_Preset>,
    pub m_GameObject: Struct_m_Preset,
    pub m_CorrespondingSourceObject: Option<Struct_m_Preset>,
    pub m_PrefabInstance: Option<Struct_m_Preset>,
    pub m_PrefabAsset: Option<Struct_m_Preset>,
    pub m_CullTransparentMesh: Option<usize>,
}

// SpriteAtlas
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SpriteAtlas {
    pub m_ObjectHideFlags: usize,
    pub m_CorrespondingSourceObject: Struct_m_Preset,
    pub m_PrefabInstance: Struct_m_Preset,
    pub m_PrefabAsset: Struct_m_Preset,
    pub m_Name: String,
    pub m_EditorData: Struct_m_EditorData,
    pub m_MasterAtlas: Struct_m_Preset,
    pub m_PackedSprites: Vec<Struct_m_Preset>,
    pub m_PackedSpriteNamesToIndex: Vec<String>,
    pub m_Tag: String,
    pub m_IsVariant: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_m_EditorData {
    pub serializedVersion: usize,
    pub textureSettings: Struct_textureSettings,
    pub platformSettings: Vec<Struct_platformSettings>,
    pub packingSettings: Struct_packingSettings,
    pub variantMultiplier: usize,
    pub packables: Vec<Struct_m_Preset>,
    pub totalSpriteSurfaceArea: usize,
    pub bindAsDefault: usize,
    pub storedHash: Struct_storedHash,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_storedHash {
    pub serializedVersion: usize,
    pub Hash: String,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_packingSettings {
    pub serializedVersion: usize,
    pub padding: usize,
    pub blockOffset: usize,
    pub allowAlphaSplitting: usize,
    pub enableRotation: usize,
    pub enableTightPacking: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_platformSettings {
    pub serializedVersion: usize,
    pub m_BuildTarget: String,
    pub m_MaxTextureSize: usize,
    pub m_ResizeAlgorithm: usize,
    pub m_TextureFormat: isize,
    pub m_TextureCompression: usize,
    pub m_CompressionQuality: usize,
    pub m_CrunchedCompression: usize,
    pub m_AllowsAlphaSplitting: usize,
    pub m_Overridden: usize,
    pub m_AndroidETC2FallbackOverride: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_textureSettings {
    pub serializedVersion: usize,
    pub anisoLevel: usize,
    pub compressionQuality: usize,
    pub maxTextureSize: usize,
    pub textureCompression: usize,
    pub filterMode: usize,
    pub generateMipMaps: usize,
    pub readable: usize,
    pub crunchedCompression: usize,
    pub sRGB: usize,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum Root {
    PresetManager(PresetManager),
    AudioManager(AudioManager),
    Rigidbody(Rigidbody),
    RenderSettings(RenderSettings),
    CapsuleCollider(CapsuleCollider),
    Camera(Camera),
    Terrain(Terrain),
    LightmapSettings(LightmapSettings),
    Physics2DSettings(Physics2DSettings),
    TerrainLayer(TerrainLayer),
    Mesh(Mesh),
    Material(Material),
    Animator(Animator),
    VFXManager(VFXManager),
    TextMesh(TextMesh),
    TagManager(TagManager),
    GraphicsSettings(GraphicsSettings),
    TimeManager(TimeManager),
    Projector(Projector),
    EditorSettings(EditorSettings),
    NavMeshProjectSettings(NavMeshProjectSettings),
    AnimationClip(AnimationClip),
    InputManager(InputManager),
    QualitySettings(QualitySettings),
    NetworkManager(NetworkManager),
    PlayerSettings(PlayerSettings),
    Preset(Preset),
    OcclusionCullingSettings(OcclusionCullingSettings),
    Animation(Animation),
    NavMeshData(NavMeshData),
    AnimatorStateMachine(AnimatorStateMachine),
    ParticleSystemRenderer(ParticleSystemRenderer),
    MeshFilter(MeshFilter),
    Light(Light),
    ClusterInputManager(ClusterInputManager),
    GameObject(GameObject),
    MeshCollider(MeshCollider),
    SpriteRenderer(SpriteRenderer),
    ConfigurableJoint(ConfigurableJoint),
    SceneSettings(SceneSettings),
    AudioSource(AudioSource),
    NavMeshSettings(NavMeshSettings),
    SphereCollider(SphereCollider),
    RenderTexture(RenderTexture),
    EditorExtensionImpl(EditorExtensionImpl),
    AnimatorState(AnimatorState),
    PhysicsManager(PhysicsManager),
    BoxCollider(BoxCollider),
    UnityConnectSettings(UnityConnectSettings),
    ReflectionProbe(ReflectionProbe),
    Prefab(Prefab),
    ParticleSystem(ParticleSystem),
    Canvas(Canvas),
    AnimatorStateTransition(AnimatorStateTransition),
    TrailRenderer(TrailRenderer),
    MeshRenderer(MeshRenderer),
    Texture2D(Texture2D),
    Transform(Transform),
    PrefabInstance(PrefabInstance),
    MonoBehaviour(MonoBehaviour),
    NavMeshAgent(NavMeshAgent),
    EditorBuildSettings(EditorBuildSettings),
    AnimatorController(AnimatorController),
    PhysicMaterial(PhysicMaterial),
    RectTransform(RectTransform),
    TerrainCollider(TerrainCollider),
    AudioListener(AudioListener),
    Behaviour(Behaviour),
    CanvasRenderer(CanvasRenderer),
    SpriteAtlas(SpriteAtlas),
}
