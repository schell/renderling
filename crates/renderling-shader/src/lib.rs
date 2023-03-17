//! Shader code for `renderling`.
#![cfg_attr(target_arch = "spirv", no_std)]

use glam::Mat4;

pub mod array;
pub mod light;
pub mod math;
pub mod primitive;
pub mod pbr;
pub mod ui;

/// A camera projection and view.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
pub struct CameraRaw {
    pub projection: Mat4,
    pub view: Mat4,
}
