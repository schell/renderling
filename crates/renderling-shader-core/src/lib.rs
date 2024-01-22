//! Core types for `renderling-shader` and `renderling`.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

use crabslab::SlabItem;
use glam::{Mat4, Quat, Vec3};
use spirv_std::{
    image::{Cubemap, Image2d},
    Sampler,
};

pub mod math;
pub mod texture;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        #[cfg(not(target_arch = "spirv"))]
        {
            std::println!($($arg)*);
        }
    }
}

pub trait IsSampler: Copy + Clone {}

impl IsSampler for Sampler {}

pub trait Sample2d {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec2, lod: f32) -> glam::Vec4;
}

impl Sample2d for Image2d {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec2, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

pub trait SampleCube {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: Vec3, lod: f32) -> glam::Vec4;
}

impl SampleCube for Cubemap {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: Vec3, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, SlabItem)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

/// A camera used for transforming the stage during rendering.
///
/// Use `Camera::new(projection, view)` to create a new camera.
/// Or use `Camera::default` followed by `Camera::with_projection_and_view`
/// to set the projection and view matrices. Using the `with_*` or `set_*`
/// methods is preferred over setting the fields directly because they will
/// also update the camera's position.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, SlabItem)]
pub struct Camera {
    pub projection: Mat4,
    pub view: Mat4,
    pub position: Vec3,
}

impl Camera {
    pub fn new(projection: Mat4, view: Mat4) -> Self {
        Camera::default().with_projection_and_view(projection, view)
    }

    pub fn set_projection_and_view(&mut self, projection: Mat4, view: Mat4) {
        self.projection = projection;
        self.view = view;
        self.position = view.inverse().transform_point3(Vec3::ZERO);
    }

    pub fn with_projection_and_view(mut self, projection: Mat4, view: Mat4) -> Self {
        self.set_projection_and_view(projection, view);
        self
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.set_projection_and_view(projection, self.view);
    }

    pub fn with_projection(mut self, projection: Mat4) -> Self {
        self.set_projection(projection);
        self
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.set_projection_and_view(self.projection, view);
    }

    pub fn with_view(mut self, view: Mat4) -> Self {
        self.set_view(view);
        self
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
