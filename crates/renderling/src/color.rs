//! Color utils.

pub fn linear_xfer_u8(c: &mut u8) {
    *c = ((*c as f32 / 255.0).powf(2.2) * 255.0) as u8;
}

pub fn linear_xfer_u16(c: &mut u16) {
    *c = ((*c as f32 / 65535.0).powf(2.2) * 65535.0) as u16;
}

pub fn linear_xfer_f32(c: &mut f32) {
    *c = c.powf(2.2);
}
