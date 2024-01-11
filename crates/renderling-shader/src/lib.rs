//! Shader code for `renderling`.
//!
//! This gathers the various `renderling-shader` crates together and
//! exports them as a pair of uber shaders.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

pub mod bits;
pub mod convolution;
pub mod debug;
pub mod gltf;
pub mod pbr;
pub mod sdf;
pub mod skybox;
pub mod stage;
pub mod texture;
pub mod tonemapping;
pub mod tutorial;
pub mod ui;

pub use renderling_shader_core::*;
