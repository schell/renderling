//! Debugging helpers.
use crate as renderling_shader;
use crate::slab::Slabbed;

/// Used to debug shaders by early exiting the shader and attempting to display
/// the value as shaded colors.
// TODO: Change DebugChannel to DebugMode and remove the previous DebugMode.
#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum DebugChannel {
    /// aoeusntahoeunsth
    None,

    /// Displays the first set of UV coordinates as a color.
    UvCoords0,

    /// Displays the second set of UV coordinates as a color.
    UvCoords1,

    /// Displays normals after normal mapping, in world space
    Normals,

    /// Displays only the vertex color for the fragment.
    VertexColor,

    /// Displays vertex normals.
    VertexNormals,

    /// Displays uv normals. These are normals coming from a normal map texture.
    /// These are the normals in tangent space.
    UvNormals,

    /// Displays vertex normals.
    Tangents,

    /// Displays bitangents as calculated from normals and tangents.
    Bitangents,

    /// Displays only the diffuse irradiance value.
    DiffuseIrradiance,

    /// Displays only the specular reflection value.
    SpecularReflection,

    /// Displays only the BRDF value for the fragment.
    Brdf,

    /// Displays only the albedo color for the fragment.
    Albedo,

    /// Displays only the roughness value for the fragment.
    Roughness,

    /// Displays only the metallic value for the fragment.
    Metallic,

    /// Displays only the occlusion color for the fragment.
    Occlusion,

    /// Displays only the calculated emissive effect (emissive_tex_color * emissive_factor * emissive_strength) of the fragment.
    Emissive,

    /// Displays only the emissive color (from the emissive map texture) of the fragment.
    UvEmissive,

    /// Displays only teh emissive factor of the fragment.
    EmissiveFactor,

    /// Displays only the emissive strength of the fragment (KHR_materials_emissive_strength).
    EmissiveStrength,
}

impl DebugChannel {
    pub fn all() -> [Self; 20] {
        [
            Self::None,
            Self::UvCoords0,
            Self::UvCoords1,
            Self::VertexColor,
            Self::Normals,
            Self::VertexNormals,
            Self::UvNormals,
            Self::Tangents,
            Self::Bitangents,
            Self::Albedo,
            Self::Roughness,
            Self::Metallic,
            Self::DiffuseIrradiance,
            Self::SpecularReflection,
            Self::Brdf,
            Self::Occlusion,
            Self::Emissive,
            Self::UvEmissive,
            Self::EmissiveFactor,
            Self::EmissiveStrength,
        ]
    }
}

/// Used to debug shaders by displaying different colors.
///
/// Create one using `DebugChannel::into`.
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct DebugMode(u32);

impl core::fmt::Debug for DebugMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DebugMode").field(&self.0).finish()
    }
}

impl From<DebugChannel> for DebugMode {
    fn from(value: DebugChannel) -> Self {
        DebugMode(value as u32)
    }
}

impl From<DebugMode> for DebugChannel {
    fn from(value: DebugMode) -> DebugChannel {
        let all = DebugChannel::all();
        #[allow(clippy::needless_range_loop)]
        for i in 0..all.len() {
            let channel = all[i];
            if channel as u32 == value.0 {
                return channel;
            }
        }
        unreachable!()
    }
}
