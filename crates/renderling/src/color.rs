//! Color utils.#[cfg(target_arch = "spirv")]
#[allow(unused_imports)]
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
