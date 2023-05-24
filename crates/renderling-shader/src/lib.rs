//! Shader code for `renderling`.
#![cfg_attr(target_arch = "spirv", no_std)]

pub mod bits;
pub mod math;
pub mod pbr;
pub mod scene;
pub mod tonemapping;
pub mod ui;

/// Boolean toggles that cause the renderer to turn on/off certain features.
///
/// CURRENTLY UNUSED.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuToggles(pub u32);

impl GpuToggles {
    const SHIFT_GAMMA_CORRECT: u32 = 31;

    pub fn set_gamma_correct(&mut self, on: bool) {
        self.0 |= if on {
            1 << Self::SHIFT_GAMMA_CORRECT
        } else {
            ! (1 << Self::SHIFT_GAMMA_CORRECT)
        };
    }

    pub fn gamma_correct(&self) -> bool {
        ((self.0 >> Self::SHIFT_GAMMA_CORRECT) & 0xb1) == 1u32
    }

}

#[cfg(test)]
mod toggles {
    use super::GpuToggles;

    const GAMMA:u32 = 0b10000000000000000000000000000000;

    #[test]
    fn toggle() {
        let mut t = GpuToggles::default();
        t.set_gamma_correct(true);
        assert_eq!(GAMMA, t.0, "0b{GAMMA:32b} != 0b{:32b}", t.0);
        assert!(t.gamma_correct());
    }
}
