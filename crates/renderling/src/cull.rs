//! Compute based culling.
//!
//! Frustum culling as explained in
//! [the vulkan guide](https://vkguide.dev/docs/gpudriven/compute_culling/).
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{UVec2, UVec3, Vec2, Vec3Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::{
    arch::IndexUnchecked,
    image::{sample_with, Image, ImageWithMethods},
    spirv,
};

use crate::{draw::DrawIndirectArgs, pbr::PbrConfig, stage::Renderlet};

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

    crate::println!("gid: {gid}");
    // Get the draw arg
    let arg = unsafe { args.index_unchecked_mut(gid) };
    // Get the renderlet using the draw arg's renderlet id
    let renderlet_id = Id::<Renderlet>::new(arg.first_instance);
    let renderlet = stage_slab.read_unchecked(renderlet_id);
    crate::println!("renderlet: {renderlet_id:?}");

    arg.vertex_count = renderlet.get_vertex_count();
    arg.instance_count = if renderlet.visible { 1 } else { 0 };

    if renderlet.bounds.radius == 0.0 {
        crate::println!("renderlet bounding radius is zero, cannot cull");
        return;
    }

    let config: PbrConfig = stage_slab.read(Id::new(0));
    if !config.perform_frustum_culling {
        return;
    }

    let camera = stage_slab.read(renderlet.camera_id);
    let model = stage_slab.read(renderlet.transform_id);
    // Compute frustum culling, and then occlusion culling, if need be
    let (renderlet_is_inside_frustum, sphere_in_world_coords) =
        renderlet.bounds.is_inside_camera_view(&camera, model);

    if renderlet_is_inside_frustum {
        arg.instance_count = 1;
        crate::println!("renderlet is inside frustum");
        crate::println!("znear: {}", camera.frustum().planes[0]);
        crate::println!(" zfar: {}", camera.frustum().planes[5]);
        if !config.perform_occlusion_culling {
            return;
        }

        // Compute occlusion culling using the hierachical z-buffer.
        let hzb_desc = depth_pyramid_slab.read_unchecked::<DepthPyramidDescriptor>(0u32.into());
        let viewport_size = Vec2::new(hzb_desc.size.x as f32, hzb_desc.size.y as f32);
        let sphere_aabb = sphere_in_world_coords.project_onto_viewport(&camera, viewport_size);
        crate::println!("sphere_aabb: {sphere_aabb:#?}");

        let size_in_pixels = sphere_aabb.max.xy() - sphere_aabb.min.xy();
        let size_in_pixels = if size_in_pixels.x > size_in_pixels.y {
            size_in_pixels.x
        } else {
            size_in_pixels.y
        };
        crate::println!("renderlet size in pixels: {size_in_pixels}");

        let mip_level = size_in_pixels.log2().floor() as u32;
        let max_mip_level = hzb_desc.mip.len() as u32 - 1;
        let mip_level = if mip_level > max_mip_level {
            crate::println!("mip_level maxed out at {mip_level}, setting to {max_mip_level}");
            max_mip_level
        } else {
            mip_level
        };
        crate::println!(
            "selected mip level: {mip_level} {}x{}",
            viewport_size.x as u32 >> mip_level,
            viewport_size.y as u32 >> mip_level
        );

        let center = sphere_aabb.center().xy();
        crate::println!("center: {center}");

        let x = center.x.round() as u32 >> mip_level;
        let y = center.y.round() as u32 >> mip_level;
        crate::println!("mip (x, y): ({x}, {y})");

        let depth_id = hzb_desc.id_of_depth(mip_level, UVec2::new(x, y), depth_pyramid_slab);
        let depth_in_hzb = depth_pyramid_slab.read_unchecked(depth_id);
        crate::println!("depth_in_hzb: {depth_in_hzb}");

        let depth_of_sphere = sphere_aabb.min.z;
        crate::println!("depth_of_sphere: {depth_of_sphere}");

        let renderlet_is_behind_something = depth_of_sphere > depth_in_hzb;
        let renderlet_surrounds_camera = depth_of_sphere > 1.0;

        if renderlet_is_behind_something || renderlet_surrounds_camera {
            crate::println!("CULLED");
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
