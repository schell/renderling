//! Tools for wrapping pixels for sampling from the atlas.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

/// Infinitely wrap the input between 0.0 and 1.0.
///
/// Only handles `input` >= 0.0.
pub fn repeat(mut input: f32) -> f32 {
    let gto = input >= 1.0;
    input = input % 1.0;
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
        f.write_str(match self {
            &TextureAddressMode::CLAMP_TO_EDGE => "clamp to edge",
            &TextureAddressMode::REPEAT => "repeat",
            &TextureAddressMode::MIRRORED_REPEAT => "mirrored repeat",
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
            let sign = input.signum();
            let input = repeat(input.abs());
            if sign > 0.0 {
                input
            } else {
                1.0 - input
            }
        }
        TextureAddressMode::MIRRORED_REPEAT => {
            let sign = input.signum();
            let i = input.abs();
            let flip = i as u32 % 2 == 0;
            let t = repeat(i);
            if sign > 0.0 {
                if flip {
                    t
                } else {
                    1.0 - t
                }
            } else {
                if flip {
                    1.0 - t
                } else {
                    t
                }
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
}
