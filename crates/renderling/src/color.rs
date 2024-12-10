// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Color utils.

use glam::Vec4;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

pub fn linear_xfer_u8(c: &mut u8) {
    *c = ((*c as f32 / 255.0).powf(2.2) * 255.0) as u8;
}

pub fn opto_xfer_u8(c: &mut u8) {
    *c = ((*c as f32 / 255.0).powf(1.0 / 2.2) * 255.0) as u8;
}

pub fn linear_xfer_u16(c: &mut u16) {
    *c = ((*c as f32 / 65535.0).powf(2.2) * 65535.0) as u16;
}

pub fn linear_xfer_f16(c: &mut u16) {
    let mut f = half::f16::from_bits(*c).to_f32();
    linear_xfer_f32(&mut f);
    *c = half::f16::from_f32(f).to_bits();
}

pub fn linear_xfer_f32(c: &mut f32) {
    *c = c.powf(2.2);
}

pub fn linear_xfer_vec4(v: &mut Vec4) {
    linear_xfer_f32(&mut v.x);
    linear_xfer_f32(&mut v.y);
    linear_xfer_f32(&mut v.z);
}

/// Converts an RGB hex color code like `0x6DC5D1` to a Vec4 with
/// RGB components in the range `0.0` to `1.0` and an alpha of `1.0`.
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
