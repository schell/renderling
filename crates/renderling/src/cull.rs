//! Compute based culling.
//!
//! Frustum culling as explained in
//! [the vulkan guide](https://vkguide.dev/docs/gpudriven/compute_culling/).
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{UVec2, UVec3, Vec3Swizzles};
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
pub struct DepthPyramidDescriptor {
    /// Size of the top layer mip.
    size: UVec2,
    /// Current mip level.
    ///
    /// This will be updated for each run of the downsample compute shader.
    mip_level: u32,
    /// Pointer to the mip data.
    ///
    /// This points to the depth data at each mip level.
    ///
    /// The depth data itself is somewhere else in the slab.
    mip: Array<Array<f32>>,
}

impl DepthPyramidDescriptor {
    fn should_skip_invocation(&self, global_invocation: UVec3) -> bool {
        let current_size = self.size >> self.mip_level;
        !(global_invocation.x < current_size.x && global_invocation.y < current_size.y)
    }

    /// Return the [`Id`] of the depth at the given `mip_level` and coordinate.
    fn id_of_depth(&self, mip_level: u32, coord: UVec2, slab: &[u32]) -> Id<f32> {
        let mip_array = slab.read(self.mip.at(mip_level as usize));
        let width_at_mip = self.size.x >> mip_level;
        let index = coord.y * width_at_mip + coord.x;
        mip_array.at(index as usize)
    }
}

pub type DepthImage2d = Image!(2D, type=f32, sampled=true, depth=true);
pub type DepthPyramidImage = Image!(2D, format = r32f, sampled = true, depth = false);
pub type DepthPyramidImageMut = Image!(2D, format = r32f, depth = false);

/// Copies a depth texture to the top mip of a pyramid of mips.
///
/// It is assumed that a [`DepthPyramidDescriptor`] is stored at index `0` in
/// the given slab.
#[spirv(compute(threads(32, 32, 1)))]
pub fn compute_copy_depth_to_pyramid(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] slab: &mut [u32],
    #[spirv(descriptor_set = 0, binding = 1)] depth_texture: &DepthImage2d,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let desc = slab.read_unchecked::<DepthPyramidDescriptor>(0u32.into());
    if desc.should_skip_invocation(global_id) {
        return;
    }

    let depth = depth_texture
        .fetch_with(global_id.xy(), sample_with::lod(0))
        .x;
    let dest_id = desc.id_of_depth(0, global_id.xy(), slab);
    slab.write(dest_id, &depth);
}

/// Downsample from `DepthPyramidDescriptor::mip_level` into
/// `DepthPyramidDescriptor::mip_level + 1`.
///
/// It is assumed that a [`DepthPyramidDescriptor`] is stored at index `0` in
/// the given slab.
///
/// The `DepthPyramidDescriptor`'s `mip_level` field will point to that of the
/// level being sampled.
///
/// This shader should be called in a loop from from `1..mip_count`.
#[spirv(compute(threads(32, 32, 1)))]
pub fn compute_downsample_depth_pyramid(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] slab: &mut [u32],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let desc = slab.read_unchecked::<DepthPyramidDescriptor>(0u32.into());
    if desc.should_skip_invocation(global_id) {
        return;
    }
    // Sample the texel.
    //
    // The texel will look like this:
    //
    // a b
    // c d
    let a = slab.read(desc.id_of_depth(desc.mip_level, global_id.xy(), slab));
    let b = slab.read(desc.id_of_depth(desc.mip_level, global_id.xy() + UVec2::new(1, 0), slab));
    let c = slab.read(desc.id_of_depth(desc.mip_level, global_id.xy() + UVec2::new(0, 1), slab));
    let d = slab.read(desc.id_of_depth(desc.mip_level, global_id.xy() + UVec2::new(1, 1), slab));
    let depth_value = a.min(b).min(c).min(d);
    // Write the texel in the current mip
    let current_mip_id = desc.id_of_depth(desc.mip_level + 1, global_id.xy() / 2, slab);
    slab.write(current_mip_id, &depth_value);
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
