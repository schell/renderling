//! GPU textures.
use glam::{UVec2, Vec2};

use crate::bits::{bits, extract, insert};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureModes(u32);

impl TextureModes {
    const BITS_WRAP_S: (u32, u32) = bits(0..=1);
    const BITS_WRAP_T: (u32, u32) = bits(2..=3);

    pub fn get_wrap_s(&self) -> TextureAddressMode {
        TextureAddressMode(extract(self.0, Self::BITS_WRAP_S))
    }

    pub fn set_wrap_s(&mut self, wrap_s: TextureAddressMode) {
        insert(&mut self.0, Self::BITS_WRAP_S, wrap_s.0)
    }

    pub fn get_wrap_t(&self) -> TextureAddressMode {
        TextureAddressMode(extract(self.0, Self::BITS_WRAP_T))
    }

    pub fn set_wrap_t(&mut self, wrap_t: TextureAddressMode) {
        insert(&mut self.0, Self::BITS_WRAP_T, wrap_t.0)
    }
}

/// A GPU texture.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuTexture {
    // The top left offset of texture in the atlas.
    pub offset_px: UVec2,
    // The size of the texture in the atlas.
    pub size_px: UVec2,
    // Various toggles of texture modes.
    pub modes: TextureModes,

    pub padding: u32,
}

impl GpuTexture {
    /// Transform the given `uv` coordinates for this texture's address mode
    /// and placement in the atlas of the given size.
    pub fn uv(&self, mut uv: Vec2, atlas_size: UVec2) -> Vec2 {
        uv.x = wrap(uv.x, self.modes.get_wrap_s());
        uv.y = wrap(uv.y, self.modes.get_wrap_t());

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

        Vec2::new(uv_s, uv_t)
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
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureAddressMode(u32);

impl core::fmt::Display for TextureAddressMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match *self {
            TextureAddressMode::CLAMP_TO_EDGE => "clamp to edge",
            TextureAddressMode::REPEAT => "repeat",
            TextureAddressMode::MIRRORED_REPEAT => "mirrored repeat",
            _ => "unknown",
        })
    }
}

impl TextureAddressMode {
    /// Clamp the value to the edge of the texture.
    pub const CLAMP_TO_EDGE: TextureAddressMode = TextureAddressMode(0);
    /// Repeat the texture in a tiling fashion.
    pub const REPEAT: TextureAddressMode = TextureAddressMode(1);
    /// Repeat the texture, mirroring it every integer wrap.
    pub const MIRRORED_REPEAT: TextureAddressMode = TextureAddressMode(2);
}

/// Wrap the given s/t coord into a pixel index according to texture addressing.
pub fn wrap(input: f32, mode: TextureAddressMode) -> f32 {
    match mode {
        TextureAddressMode::REPEAT => {
            let sign = if input >= 0.0 { 1.0f32 } else { -1.0 };
            let input = repeat(input.abs());
            if sign > 0.0 {
                input
            } else {
                1.0 - input
            }
        }
        TextureAddressMode::MIRRORED_REPEAT => {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_repeat() {
        assert_eq!(0.0, wrap(0.0, TextureAddressMode::REPEAT));
        assert_eq!(1.0, wrap(2.0, TextureAddressMode::REPEAT));
        assert_eq!(1.0, wrap(3.0, TextureAddressMode::REPEAT));
    }

    #[test]
    fn modes() {
        let mut modes = TextureModes::default();
        assert_eq!(TextureAddressMode::CLAMP_TO_EDGE, modes.get_wrap_s());
        assert_eq!(TextureAddressMode::CLAMP_TO_EDGE, modes.get_wrap_t());

        let wrappings = 0..=2;
        let gets = &[
            Box::new(TextureModes::get_wrap_s) as Box<dyn Fn(&TextureModes) -> TextureAddressMode>,
            Box::new(TextureModes::get_wrap_t),
        ];
        let sets = [
            Box::new(TextureModes::set_wrap_s)
                as Box<dyn Fn(&mut TextureModes, TextureAddressMode)>,
            Box::new(TextureModes::set_wrap_t),
        ];

        for n in wrappings {
            let tex_mode = TextureAddressMode(n);
            for (i, (get, set)) in gets.iter().zip(sets.iter()).enumerate() {
                set(&mut modes, tex_mode);
                let new = get(&modes);
                assert_eq!(
                    tex_mode, new,
                    "i:{i} n:{n} {new} does not equal expected {tex_mode}"
                );
            }
        }
    }
}
