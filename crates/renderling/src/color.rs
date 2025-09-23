//! Color utils.

use glam::Vec4;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

/// Applies a linear transfer function to an 8-bit unsigned integer color component.
///
/// This function simulates the gamma correction process by raising the component
/// to the power of 2.2.
///
/// Converts from sRGB to linear color space.
pub fn linear_xfer_u8(c: &mut u8) {
    *c = ((*c as f32 / 255.0).powf(2.2) * 255.0) as u8;
}

/// Applies an optical transfer function to an 8-bit unsigned integer color component.
///
/// This function simulates the inverse gamma correction process by raising the component
/// to the power of 1/2.2.
///
/// Converts from linear to sRGB color space.
pub fn opto_xfer_u8(c: &mut u8) {
    *c = ((*c as f32 / 255.0).powf(1.0 / 2.2) * 255.0) as u8;
}

/// Applies a linear transfer function to a 16-bit unsigned integer color component.
///
/// This function simulates the gamma correction process by raising the component
/// to the power of 2.2.
///
/// Converts from sRGB to linear color space.
pub fn linear_xfer_u16(c: &mut u16) {
    *c = ((*c as f32 / 65535.0).powf(2.2) * 65535.0) as u16;
}

#[cfg(not(target_arch = "spirv"))]
mod cpu {
    use super::*;
    use glam::Vec3;

    /// Applies a linear transfer function to a 16-bit floating-point color component.
    ///
    /// This function simulates the gamma correction process by raising the component
    /// to the power of 2.2.
    ///
    /// Converts from sRGB to linear color space.
    pub fn linear_xfer_f16(c: &mut u16) {
        let mut f = half::f16::from_bits(*c).to_f32();
        super::linear_xfer_f32(&mut f);
        *c = half::f16::from_f32(f).to_bits();
    }

    pub fn u16_to_u8(c: u16) -> u8 {
        ((c as f32 / 65535.0) * 255.0) as u8
    }

    pub fn f32_to_u8(c: f32) -> u8 {
        (c / 255.0) as u8
    }

    pub fn u8_to_f32(c: u8) -> f32 {
        c as f32 / 255.0
    }

    /// Converts a CSS style sRGB color into a `Vec4`.
    ///
    /// This applies the linear transfer function to the input color,
    /// returning a color in linear color space.
    pub fn css_srgb_color_to_linear(r: u8, g: u8, b: u8) -> Vec4 {
        let rgb = [r, g, b].map(u8_to_f32);
        let mut color = Vec3::from_array(rgb).extend(1.0);
        linear_xfer_vec4(&mut color);
        color
    }
}
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Applies a linear transfer function to a 32-bit floating-point color component.
///
/// This function simulates the gamma correction process by raising the component
/// to the power of 2.2.
///
/// Converts from sRGB to linear color space.
pub fn linear_xfer_f32(c: &mut f32) {
    *c = c.powf(2.2);
}

/// Applies a linear transfer function to each component of a `Vec4`.
///
/// This function simulates the gamma correction process by raising each component
/// to the power of 2.2.
///
/// Converts from sRGB to linear color space for each component.
pub fn linear_xfer_vec4(v: &mut Vec4) {
    linear_xfer_f32(&mut v.x);
    linear_xfer_f32(&mut v.y);
    linear_xfer_f32(&mut v.z);
}

/// Converts an RGB hex color.
///
/// Converts a hex code like `0x6DC5D1` to a Vec4 with
/// RGB components in the range `0.0` to `1.0` and an alpha of `1.0`.
///
/// ## Note
/// This does **not** apply the linear transfer.
pub fn rgb_hex_color(hex: u32) -> Vec4 {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Vec4::new(r, g, b, 1.0)
}

#[cfg(test)]
mod test {
    use super::rgb_hex_color;

    #[test]
    fn can_rgb_hex_color() {
        let hex = 0x6dc5d1;
        let color = rgb_hex_color(hex);
        let r = (color.x * 255.0) as u8;
        let g = (color.y * 255.0) as u8;
        let b = (color.z * 255.0) as u8;
        assert_eq!(109, 0x6d);
        assert_eq!([109, 197, 209], [r, g, b]);
    }
}
