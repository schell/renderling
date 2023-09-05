//! Types used to store and update an entire 3d scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{mat3, Mat4, Quat, UVec2, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    Sampler,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

use crate::{
    bits::{bits, extract, insert},
    debug::*,
    pbr, slab, GpuToggles, Id, IsMatrix, IsVector, ID_NONE,
};
