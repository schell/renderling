//! Debugging helpers.

/// Used to debug shaders by early exiting the shader and attempting to display
/// the value as shaded colors.
#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum DebugChannel {
    /// aoeusntahoeunsth
    None = 0,

    /// Displays the first set of UV coordinates as a color.
    UvCoords0 = 1,

    /// Displays the second set of UV coordinates as a color.
    UvCoords1 = 2,

    /// Displays normals after normal mapping, in world space
    Normals = 3,

    /// Displays vertex normals.
    VertexNormals = 4,

    /// Displays uv normals. These are normals coming from a normal map texture.
    /// These are the normals in tangent space.
    UvNormals = 5,

    /// Displays vertex normals.
    Tangents = 6,

    /// Displays bitangents as calculated from normals and tangents.
    Bitangents = 7,

    /// Displays only the diffuse irradiance value.
    DiffuseIrradiance = 8,

    /// Displays only the specular reflection value.
    SpecularReflection = 9,

    /// Displays only the BRDF value for the fragment.
    Brdf = 10,

    /// Displays only the roughness value for the fragment.
    Roughness = 11,

    /// Displays only the metallic value for the fragment.
    Metallic = 12,

    /// Displays only the albedo color for the fragment.
    Albedo = 13,

    /// Displays only the occlusion color for the fragment.
    Occlusion = 14,

    /// Displays only the emissive color for the fragment.
    Emissive = 15,
}

impl DebugChannel {
    pub fn all() -> [Self; 16] {
        [
            Self::None,
            Self::UvCoords0,
            Self::UvCoords1,
            Self::Normals,
            Self::VertexNormals,
            Self::UvNormals,
            Self::Tangents,
            Self::Bitangents,
            Self::DiffuseIrradiance,
            Self::SpecularReflection,
            Self::Brdf,
            Self::Roughness,
            Self::Metallic,
            Self::Albedo,
            Self::Occlusion,
            Self::Emissive,
        ]
    }
}

/// Used to debug shaders by displaying different colors.
///
/// Create one using `DebugChannel::into`.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DebugMode(u32);

impl From<DebugChannel> for DebugMode {
    fn from(value: DebugChannel) -> Self {
        DebugMode(value as u32)
    }
}

impl From<DebugMode> for DebugChannel {
    fn from(value: DebugMode) -> DebugChannel {
        let all = DebugChannel::all();
        for i in 0..all.len() {
            let channel = all[i];
            if channel as u32 == value.0 {
                return channel;
            }
        }
        unreachable!()
    }
}
