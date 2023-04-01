//! Shader code for `renderling`.
#![cfg_attr(target_arch = "spirv", no_std)]

use glam::Mat4;

pub mod light;
pub mod math;
pub mod mesh;
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

#[cfg_attr(not(target_arch = "spirv"), derive(encase::ShaderType))]
#[derive(Default)]
pub struct TestStorage {
    pub position: glam::Vec4,
}

pub fn compute_test_storage(test: &mut TestStorage) {
    test.position = glam::Vec4::new(6.0, 6.0, 6.0, 666.0);
}
