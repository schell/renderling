//! Texture atlas.
//!
//! All images are packed into an atlas at staging time.
//! Texture descriptors describe where in the atlas an image is,
//! and how it should sample pixels. These descriptors are packed into a buffer
//! on the GPU. This keeps the number of texture binds to a minimum (one, in most cases).
//!
//! ## NOTE:
//! `Atlas` is a temporary work around until we can use bindless techniques
//! on web.
//!
//! `Atlas` is only available on CPU. Not available in shaders.
use crabslab::SlabItem;

#[cfg(cpu)]
mod atlas_image;
#[cfg(cpu)]
pub use atlas_image::*;
#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod shader;

/// Method of addressing the edges of a texture.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, SlabItem)]
pub struct TextureModes {
    pub s: TextureAddressMode,
    pub t: TextureAddressMode,
}

/// Infinitely wrap the input between 0.0 and 1.0.
///
/// Only handles `input` >= 0.0.
pub fn repeat(mut input: f32) -> f32 {
    let gto = input >= 1.0;
    input %= 1.0;
    if gto && input == 0.0 {
        1.0
    } else {
        input
    }
}

/// Clamp the input between 0.0 and 1.0.
pub fn clamp(input: f32) -> f32 {
    if input > 1.0 {
        1.0 - f32::EPSILON
    } else if input < 0.0 {
        0.0 + f32::EPSILON
    } else {
        input
    }
}

/// How edges should be handled in texture addressing/wrapping.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(u32)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, SlabItem)]
pub enum TextureAddressMode {
    #[default]
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

impl core::fmt::Display for TextureAddressMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match *self {
            TextureAddressMode::ClampToEdge => "clamp to edge",
            TextureAddressMode::Repeat => "repeat",
            TextureAddressMode::MirroredRepeat => "mirrored repeat",
        })
    }
}

impl TextureAddressMode {
    /// Wrap the given s/t coord into a pixel index according to texture
    /// addressing.
    pub fn wrap(&self, input: f32) -> f32 {
        match self {
            TextureAddressMode::Repeat => {
                let sign = if input >= 0.0 { 1.0f32 } else { -1.0 };
                let input = repeat(input.abs());
                if sign > 0.0 {
                    input
                } else {
                    1.0 - input
                }
            }
            TextureAddressMode::MirroredRepeat => {
                let sign = if input >= 0.0 { 1.0f32 } else { -1.0 };
                let i = input.abs();
                let flip = (i as u32).is_multiple_of(2);
                let t = repeat(i);
                if sign > 0.0 {
                    if flip {
                        t
                    } else {
                        1.0 - t
                    }
                } else if flip {
                    1.0 - t
                } else {
                    t
                }
            }
            _ => clamp(input),
        }
    }
}

#[cfg(test)]
mod test {
    use glam::{UVec2, UVec3, Vec2, Vec3Swizzles, Vec4Swizzles};

    use crate::atlas::shader::AtlasTextureDescriptor;

    use super::*;

    #[test]
    fn can_repeat() {
        assert_eq!(0.0, TextureAddressMode::Repeat.wrap(0.0));
        assert_eq!(1.0, TextureAddressMode::Repeat.wrap(2.0));
        assert_eq!(1.0, TextureAddressMode::Repeat.wrap(3.0));
    }

    #[test]
    /// Tests that clip coordinates can be converted into texture coords within
    /// a specific `AtlasTexture`, and back again.
    fn constrain_clip_coords_sanity() {
        let atlas_texture = AtlasTextureDescriptor {
            offset_px: UVec2::splat(0),
            size_px: UVec2::splat(800),
            layer_index: 0,
            frame_index: 0,
            modes: TextureModes {
                s: TextureAddressMode::ClampToEdge,
                t: TextureAddressMode::ClampToEdge,
            },
        };
        let atlas_size = UVec3::new(1024, 1024, 4);
        let corners @ [tl, tr, br, bl] = [
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TL,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BL,
        ]
        .map(|coord| {
            atlas_texture.constrain_clip_coords_to_texture_space(coord.xy(), atlas_size.xy())
        });
        log::info!("uv_corners: {corners:#?}");

        let clip_br = crate::math::CLIP_SPACE_COORD_QUAD_CCW_BR.xy();
        log::info!("clip_br: {clip_br}");
        let input_uv_br = (clip_br * Vec2::new(1.0, -1.0) + Vec2::splat(1.0)) * Vec2::splat(0.5);
        log::info!("input_uv_br: {input_uv_br}");
        assert_eq!(Vec2::ONE, input_uv_br, "incorrect uv");

        let d = 800.0 / 1024.0;
        assert_eq!(Vec2::splat(0.0), tl, "incorrect tl");
        assert_eq!(Vec2::new(d, 0.0), tr, "incorrect tr");
        assert_eq!(Vec2::new(d, d), br, "incorrect br");
        assert_eq!(Vec2::new(0.0, d), bl, "incorrect bl");

        let corners = [
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TL,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BL,
        ]
        .map(|coord| atlas_texture.constrain_clip_coords(coord.xy(), atlas_size.xy()));
        log::info!("clip_corners: {corners:#?}");
        //     [
        //     Vec2(
        //         -1.0,
        //         1.0,
        //     ),
        //     Vec2(
        //         0.5625,
        //         1.0,
        //     ),
        //     Vec2(
        //         0.5625,
        //         -0.5625,
        //     ),
        //     Vec2(
        //         -1.0,
        //         -0.5625,
        //     ),
        // ]
    }
}
