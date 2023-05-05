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
    if input >= 1.0 {
        1.0
    } else if input <= 0.0 {
        0.0
    } else {
        input
    }
}

/// How edges should be handled in texture addressing/wrapping.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureAddressMode(u32);

impl TextureAddressMode {
    /// Clamp the value to the edge of the texture:
    pub const CLAMP_TO_EDGE: TextureAddressMode = TextureAddressMode(0);
    /// Repeat the texture in a tiling fashion
    pub const REPEAT: TextureAddressMode = TextureAddressMode(1);
    /// Repeat the texture, mirroring it every repeat
    pub const MIRRORED_REPEAT: TextureAddressMode = TextureAddressMode(2);
}

/// Wrap the given s/t coord into a pixel index according to texture addressing.
pub fn wrap(input: f32, mode: TextureAddressMode) -> f32 {
    match mode {
        TextureAddressMode::CLAMP_TO_EDGE => clamp(input),
        TextureAddressMode::REPEAT => {
            let sign = input.signum();
            let input = repeat(input);
            if sign > 0.0 {
                input
            } else {
                1.0 - input
            }
        },
        // TODO: implement mirrored repeat wrapping
        TextureAddressMode::MIRRORED_REPEAT => repeat(input * input.signum()),
        _ => clamp(input)
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
