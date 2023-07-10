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
}

impl DebugChannel {
    pub fn all() -> [Self; 8] {
        [
            Self::None,
            Self::UvCoords0,
            Self::UvCoords1,
            Self::Normals,
            Self::VertexNormals,
            Self::UvNormals,
            Self::Tangents,
            Self::Bitangents,
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
