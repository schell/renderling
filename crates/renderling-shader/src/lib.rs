//! Shader code for `renderling`.
#![cfg_attr(target_arch = "spirv", no_std)]

use bits::{bits, extract, insert};

pub mod bits;
pub mod convolution;
pub mod debug;
mod id;
pub use id::*;
pub mod math;
pub mod pbr;
pub mod scene;
pub mod skybox;
pub mod tonemapping;
pub mod ui;

/// Boolean toggles that cause the renderer to turn on/off certain features.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuToggles(pub u32);

impl GpuToggles {
    const BITS_HAS_SKYBOX: (u32, u32) = bits(0..=0);

    pub fn get_has_skybox(&self) -> bool {
        extract(self.0, Self::BITS_HAS_SKYBOX) == 1
    }

    pub fn set_has_skybox(&mut self, has: bool) {
        insert(&mut self.0, Self::BITS_HAS_SKYBOX, if has { 1 } else { 0 })
    }
}
