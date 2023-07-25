//! Convolution shaders.
//!
//! These shaders convolve various functions to produce cached maps.
use glam::{Mat3, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{math, scene::GpuConstants};

pub fn diffuse_irradiance(
    _environment_equirectangular: &Image2d,
    _envoronment_sampler: &Sampler,
) {}
