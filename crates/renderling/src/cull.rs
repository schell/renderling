//! Compute based culling.
//!
//! Frustum culling as explained in
//! [the vulkan guide](https://vkguide.dev/docs/gpudriven/compute_culling/).
use crabslab::Slab;
use glam::{UVec3, Vec2, Vec4, Vec4Swizzles};
use spirv_std::{arch::IndexUnchecked, image::Image2d, spirv, Sampler};

use crate::draw::DrawIndirectArgs;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

#[cfg(feature = "compute_frustum_culling")]
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

#[cfg(feature = "compute_occlusion_culling")]
#[spirv(vertex)]
pub fn downsample_depth_pyramid_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let i = (vertex_index % 6) as usize;
    *out_clip_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
}

#[cfg(feature = "compute_occlusion_culling")]
#[spirv(fragment)]
pub fn downsample_depth_pyramid_fragment(
    #[spirv(descriptor_set = 0, binding = 0)] mip_texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 1)] mip_sampler: &Sampler,
    #[spirv(frag_coord)] frag_coord: Vec4,
    frag_color: &mut Vec4,
) {
    // Find the top-left of the 2x2 texel we're going to sample.
    let tl_texel = frag_coord.xy() * 2.0;
    // Sample te texel.
    //
    // The texel will look like this:
    //
    // a b
    // c d
    let a = mip_texture.sample(*mip_sampler, tl_texel);
    let b = mip_texture.sample(*mip_sampler, tl_texel + Vec2::new(1.0, 0.0));
    let c = mip_texture.sample(*mip_sampler, tl_texel + Vec2::new(0.0, 1.0));
    let d = mip_texture.sample(*mip_sampler, tl_texel + Vec2::new(1.0, 1.0));
    let depth_value = a.min(b).min(c).min(d);
    *frag_color = depth_value;
}

#[cfg(feature = "compute_occlusion_culling")]
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
