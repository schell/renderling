//! Texture atlas.
//!
//! All images are packed into an atlas at staging time.
//! Texture descriptors describing where in the atlas an image is,
//! and how callsites should sample pixels is packed into a buffer
//! on the GPU. This keeps the number of texture binds to a minimum.
//!
//! ## NOTE:
//! `Atlas` is a temporary work around until we can use bindless techniques
//! on web.
//!
//! `Atlas` is only available on CPU. Not available in shaders.
use crabslab::SlabItem;
use glam::{UVec2, Vec2, Vec3};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

#[cfg(not(target_arch = "spirv"))]
mod atlas_image;
#[cfg(not(target_arch = "spirv"))]
pub use atlas_image::*;
#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Method of addressing the edges of a texture.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, SlabItem)]
pub struct TextureModes {
    pub s: TextureAddressMode,
    pub t: TextureAddressMode,
}

/// A texture inside the atlas.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct AtlasTexture {
    /// The top left offset of texture in the atlas.
    pub offset_px: UVec2,
    /// The size of the texture in the atlas.
    pub size_px: UVec2,
    /// The index of the layer within the atlas that this `AtlasTexture `belongs to.
    pub layer_index: u32,
    /// The index of this frame within the layer.
    pub frame_index: u32,
    /// Various toggles of texture modes.
    pub modes: TextureModes,
}

impl AtlasTexture {
    /// Transform the given `uv` coordinates for this texture's address mode
    /// and placement in the atlas of the given size.
    pub fn uv(&self, mut uv: Vec2, atlas_size: UVec2) -> Vec3 {
        uv.x = self.modes.s.wrap(uv.x);
        uv.y = self.modes.t.wrap(uv.y);

        // get the pixel index of the uv coordinate in terms of the original image
        let mut px_index_s = (uv.x * self.size_px.x as f32) as u32;
        let mut px_index_t = (uv.y * self.size_px.y as f32) as u32;

        // convert the pixel index from image to atlas space
        px_index_s += self.offset_px.x;
        px_index_t += self.offset_px.y;

        let sx = atlas_size.x as f32;
        let sy = atlas_size.y as f32;
        // normalize the pixels by dividing by the atlas size
        let uv_s = px_index_s as f32 / sx;
        let uv_t = px_index_t as f32 / sy;

        Vec2::new(uv_s, uv_t).extend(self.layer_index as f32)
    }
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
                let flip = i as u32 % 2 == 0;
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
    use super::*;

    #[test]
    fn can_repeat() {
        assert_eq!(0.0, TextureAddressMode::Repeat.wrap(0.0));
        assert_eq!(1.0, TextureAddressMode::Repeat.wrap(2.0));
        assert_eq!(1.0, TextureAddressMode::Repeat.wrap(3.0));
    }
}
