//! Compute based culling.
//!
//! Frustum culling as explained in
//! [the vulkan guide](https://vkguide.dev/docs/gpudriven/compute_culling/).
use crabslab::{Slab, SlabItem};
use glam::{UVec2, UVec3, Vec3Swizzles, Vec4};
use spirv_std::{
    arch::IndexUnchecked,
    image::{sample_with, Image, ImageWithMethods},
    spirv,
};

use crate::draw::DrawIndirectArgs;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

#[spirv(compute(threads(32)))]
pub fn compute_frustum_culling(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] args: &mut [DrawIndirectArgs],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let gid = global_id.x as usize;
    if gid >= args.len() {
        return;
    }

    // Get the draw arg
    let arg = unsafe { args.index_unchecked_mut(gid) };
    // Get the renderlet using the draw arg's renderlet id
    let renderlet = slab.read_unchecked(arg.first_instance);

    arg.vertex_count = renderlet.get_vertex_count();
    arg.instance_count = if renderlet.visible { 1 } else { 0 };

    if renderlet.bounds.radius == 0.0 {
        return;
    }
    let camera = slab.read(renderlet.camera_id);
    let model = slab.read(renderlet.transform_id);
    if !renderlet.bounds.is_inside_camera_view(&camera, model) {
        arg.instance_count = 0;
    }
}

#[derive(Clone, Copy, Default, SlabItem)]
pub struct PyramidDescriptor {
    /// Size of the top layer mip.
    size: UVec2,
    /// Current mip level.
    ///
    /// This will be updated for each run of the downsample compute shader.
    mip_level: u32,
}

impl PyramidDescriptor {
    fn should_skip_invocation(&self, global_invocation: UVec3) -> bool {
        let current_size = self.size >> self.mip_level;
        global_invocation.x < current_size.x && global_invocation.y < current_size.y
    }
}

pub type DepthImage2d = Image!(2D, type=f32, sampled=true, depth=true);
pub type DepthPyramidImage = Image!(2D, format = r32f, sampled = true, depth = false);
pub type DepthPyramidImageMut = Image!(2D, format = r32f, sampled = false, depth = false);

/// Copies a depth texture to the top mip of a pyramid of mips.
#[spirv(compute(threads(32)))]
pub fn compute_copy_depth_to_pyramid(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] desc: &PyramidDescriptor,
    #[spirv(descriptor_set = 0, binding = 1)] depth_texture: &DepthImage2d,
    #[spirv(descriptor_set = 0, binding = 2)] mip: &DepthPyramidImageMut,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    if desc.should_skip_invocation(global_id) {
        return;
    }

    let depth: Vec4 = depth_texture.fetch_with(global_id.xy(), sample_with::lod(0));
    unsafe {
        mip.write(global_id.xy(), depth);
    }
}

/// Downsample from `previous_mip` into `current_mip`.
#[spirv(compute(threads(32)))]
pub fn compute_downsample_depth_pyramid(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] desc: &PyramidDescriptor,
    #[spirv(descriptor_set = 0, binding = 1)] previous_mip: &DepthPyramidImage,
    #[spirv(descriptor_set = 0, binding = 2)] current_mip: &DepthPyramidImageMut,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    if desc.should_skip_invocation(global_id) {
        return;
    }
    // Sample the texel.
    //
    // The texel will look like this:
    //
    // a b
    // c d
    let coord_in_previous_mip = global_id.xy() * 2;
    let a = previous_mip.fetch(coord_in_previous_mip);
    let b = previous_mip.fetch(coord_in_previous_mip + UVec2::new(1, 0));
    let c = previous_mip.fetch(coord_in_previous_mip + UVec2::new(0, 1));
    let d = previous_mip.fetch(coord_in_previous_mip + UVec2::new(1, 1));
    let depth_value = a.min(b).min(c).min(d);
    // Write the texel in the current mip
    unsafe {
        current_mip.write(global_id.xy(), Vec4::splat(depth_value));
    }
}

#[spirv(compute(threads(32)))]
pub fn compute_occlusion_culling(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] args: &mut [DrawIndirectArgs],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let gid = global_id.x as usize;
    if gid >= args.len() {
        return;
    }

    // Get the draw arg
    let arg = unsafe { args.index_unchecked_mut(gid) };
    // Get the renderlet using the draw arg's renderlet id
    let renderlet = slab.read_unchecked(arg.first_instance);

    arg.vertex_count = renderlet.get_vertex_count();
    arg.instance_count = if renderlet.visible { 1 } else { 0 };
}
