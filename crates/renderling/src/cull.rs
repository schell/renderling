//! Compute based culling.
//!
//! Frustum culling as explained in
//! [the vulkan guide](https://vkguide.dev/docs/gpudriven/compute_culling/).
use crabslab::Slab;
use glam::Mat4;
use glam::UVec3;
use spirv_std::{arch::IndexUnchecked, spirv};

use crate::bvol::Aabb;
use crate::{camera::Camera, stage::DrawIndirectArgs, transform::Transform};

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Determine (roughly) if an AABB is within the clip space after transformation.
pub fn is_visible_in_clip_space(aabb: Aabb, camera: Camera, model: Transform) -> bool {
    let transform = camera.projection * camera.view * Mat4::from(model);
    let min = transform.project_point3(aabb.min);
    let max = transform.project_point3(aabb.max);
    !Aabb::new(min, max).is_outside_frustum(camera.frustum)
}

#[cfg(feature = "compute_frustum_culling")]
#[spirv(compute(threads(32)))]
pub fn compute_frustum_culling(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] args: &mut [DrawIndirectArgs],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let gid = global_id.x as usize;
    if gid >= args.len() {
        return;
    }

    // Get the draw arg
    let arg = unsafe { args.index_unchecked_mut(gid) };
    arg.instance_count = 1;

    // Get the renderlet using the draw arg's renderlet id
    let renderlet = slab.read_unchecked(arg.first_instance);
    if renderlet.bounds.is_zero() {
        return;
    }
    let camera = slab.read(renderlet.camera_id);
    let model = slab.read(renderlet.transform_id);
    if is_visible_in_clip_space(renderlet.bounds, camera, model) {
        arg.instance_count = 0;
    }
}
