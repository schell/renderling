//! Compute based culling.
//!
//! Frustum culling as explained in
//! [the vulkan guide](https://vkguide.dev/docs/gpudriven/compute_culling/).
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{UVec2, UVec3, Vec3, Vec3Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::{
    arch::IndexUnchecked,
    image::{sample_with, Image, ImageWithMethods},
    spirv, Sampler,
};

use crate::draw::DrawIndirectArgs;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

#[spirv(compute(threads(32)))]
pub fn compute_culling(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] stage_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] depth_pyramid_slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] args: &mut [DrawIndirectArgs],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let gid = global_id.x as usize;
    if gid >= args.len() {
        return;
    }

    // Get the draw arg
    let arg = unsafe { args.index_unchecked_mut(gid) };
    // Get the renderlet using the draw arg's renderlet id
    let renderlet = stage_slab.read_unchecked(arg.first_instance);

    arg.vertex_count = renderlet.get_vertex_count();
    arg.instance_count = if renderlet.visible { 1 } else { 0 };

    if renderlet.bounds.radius == 0.0 {
        return;
    }
    let camera = stage_slab.read(renderlet.camera_id);
    let model = stage_slab.read(renderlet.transform_id);
    // Compute frustum culling, and then occlusion culling, if need be
    let (renderlet_is_inside_frustum, sphere_in_world_coords) =
        renderlet.bounds.is_inside_camera_view(&camera, model);
    if renderlet_is_inside_frustum {
        // Compute occlusion culling using the hierachical z-buffer.
        let hzb_desc = depth_pyramid_slab.read_unchecked::<DepthPyramidDescriptor>(0u32.into());
        let viewprojection = camera.view_projection();

        // Find the center and radius of the bounding sphere in screen space, where
        // (0, 0) is the top-left of the screen and (1, 1) is is the bottom-left.
        //
        // z = 0 is near and z = 1 is far.
        let center_ndc = viewprojection.project_point3(sphere_in_world_coords.center);
        let center = Vec3::new(
            (center_ndc.x + 1.0) * 0.5,
            (center_ndc.y + 1.0) * -0.5,
            center_ndc.z,
        );
        // Find the radius (in screen space)
        let radius = viewprojection
            .project_point3(Vec3::new(sphere_in_world_coords.radius, 0.0, 0.0))
            .distance(Vec3::ZERO);
        let size_max_element = if hzb_desc.size.x > hzb_desc.size.y {
            hzb_desc.size.x
        } else {
            hzb_desc.size.y
        } as f32;
        let size_in_pixels = 2.0 * radius * size_max_element;
        let mip_level = size_in_pixels.log2().floor() as u32;
        let x = center.x * (hzb_desc.size.x >> mip_level) as f32;
        let y = center.y * (hzb_desc.size.y >> mip_level) as f32;
        let depth_id = hzb_desc.id_of_depth(
            mip_level,
            UVec2::new(x as u32, y as u32),
            depth_pyramid_slab,
        );

        let depth_in_hzb = depth_pyramid_slab.read_unchecked(depth_id);
        let depth_of_sphere = center.z - radius;
        let renderlet_is_behind_something = depth_of_sphere > depth_in_hzb;

        if renderlet_is_behind_something {
            arg.instance_count = 0;
        }
    } else {
        arg.instance_count = 0;
    }
}

/// A hierarchichal depth buffer.
///
/// AKA HZB
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

    #[cfg(test)]
    fn size_at(&self, mip_level: u32) -> UVec2 {
        UVec2::new(self.size.x >> mip_level, self.size.y >> mip_level)
    }

    /// Return the [`Id`] of the depth at the given `mip_level` and coordinate.
    fn id_of_depth(&self, mip_level: u32, coord: UVec2, slab: &[u32]) -> Id<f32> {
        let mip_array = slab.read(self.mip.at(mip_level as usize));
        let width_at_mip = self.size.x >> mip_level;
        let index = coord.y * width_at_mip + coord.x;
        mip_array.at(index as usize)
    }
}

pub type DepthImage2d = Image!(2D, type=f32, sampled, depth);
pub type DepthImage2dMultisampled = Image!(2D, type=f32, sampled, depth, multisampled);

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

/// Copies a depth texture to the top mip of a pyramid of mips.
///
/// It is assumed that a [`DepthPyramidDescriptor`] is stored at index `0` in
/// the given slab.
#[spirv(compute(threads(32, 32, 1)))]
pub fn compute_copy_depth_to_pyramid_multisampled(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] slab: &mut [u32],
    #[spirv(descriptor_set = 0, binding = 1)] depth_texture: &DepthImage2dMultisampled,
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let desc = slab.read_unchecked::<DepthPyramidDescriptor>(0u32.into());
    if desc.should_skip_invocation(global_id) {
        return;
    }

    let depth = depth_texture
        .fetch_with(global_id.xy(), sample_with::sample_index(0))
        .x;
    let dest_id = desc.id_of_depth(0, global_id.xy(), slab);
    slab.write(dest_id, &depth);
}

/// Downsample from `DepthPyramidDescriptor::mip_level-1` into
/// `DepthPyramidDescriptor::mip_level`.
///
/// It is assumed that a [`DepthPyramidDescriptor`] is stored at index `0` in
/// the given slab.
///
/// The `DepthPyramidDescriptor`'s `mip_level` field will point to that of the
/// mip level being downsampled to (the mip level being written into).
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
    let a_coord = global_id.xy() * 2;
    let a = slab.read(desc.id_of_depth(desc.mip_level - 1, a_coord, slab));
    let b = slab.read(desc.id_of_depth(desc.mip_level - 1, a_coord + UVec2::new(1, 0), slab));
    let c = slab.read(desc.id_of_depth(desc.mip_level - 1, a_coord + UVec2::new(0, 1), slab));
    let d = slab.read(desc.id_of_depth(desc.mip_level - 1, a_coord + UVec2::new(1, 1), slab));
    // Take the maximum depth of the region (max depth means furthest away)
    let depth_value = a.max(b).max(c).max(d);
    // Write the texel in the next mip
    let depth_id = desc.id_of_depth(desc.mip_level, global_id.xy(), slab);
    slab.write(depth_id, &depth_value);
}
