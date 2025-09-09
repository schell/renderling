//! Physically based bloom.
//!
//! As described in [learnopengl.com's Physically Based Bloom
//! article](https://learnopengl.com/Guest-Articles/2022/Phys.-Based-Bloom).
#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

pub mod shaders;
