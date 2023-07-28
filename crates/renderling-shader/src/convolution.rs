//! Convolution shaders.
//!
//! These shaders convolve various functions to produce cached maps.
use glam::{Vec3, Vec4, Vec4Swizzles, Vec3Swizzles};
use spirv_std::{image::Cubemap, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
