//! Debugging helpers.
use crabslab::SlabItem;

/// Used to debug shaders by early exiting the shader and attempting to display
/// the value as shaded colors.
#[repr(u32)]
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, SlabItem, core::fmt::Debug)]
pub enum DebugChannel {
    #[default]
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

    /// Displays only the calculated emissive effect (emissive_tex_color *
    /// emissive_factor * emissive_strength) of the fragment.
    Emissive,

    /// Displays only the emissive color (from the emissive map texture) of the
    /// fragment.
    UvEmissive,

    /// Displays only teh emissive factor of the fragment.
    EmissiveFactor,

    /// Displays only the emissive strength of the fragment
    /// (KHR_materials_emissive_strength).
    EmissiveStrength,
}
