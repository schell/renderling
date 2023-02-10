//! Shader code for `renderling`.
#![cfg_attr(feature = "gpu", no_std)]

use glam::Mat4;

pub mod math;
pub mod pbr;
pub mod ui;

pub struct Camera {
    pub projection: Mat4,
    pub view: Mat4,
}
