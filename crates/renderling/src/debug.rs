//! Debug overlay.
use crabslab::{Id, Slab};
use glam::{Vec2, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::{arch::IndexUnchecked, spirv};

use crate::{
    draw::DrawIndirectArgs, geometry::GeometryDescriptor, sdf, stage::RenderletDescriptor,
    transform::TransformDescriptor,
};

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Renders an implicit quad.
#[spirv(vertex)]
pub fn debug_overlay_vertex(
    #[spirv(vertex_index)] vertex_id: u32,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    *clip_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[vertex_id as usize % 6];
}

/// Renders a debug overlay on top of the current framebuffer.
///
/// Displays useful information in real time.
#[spirv(fragment)]
pub fn debug_overlay_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] draw_calls: &[DrawIndirectArgs],
    #[spirv(frag_coord)] frag_coord: Vec4,
    frag_color: &mut Vec4,
) {
    let camera_id_id = Id::from(GeometryDescriptor::OFFSET_OF_CAMERA_ID);
    let camera_id = slab.read_unchecked(camera_id_id);
    let camera = slab.read_unchecked(camera_id);
    let resolution_id = Id::from(GeometryDescriptor::OFFSET_OF_RESOLUTION);
    let viewport_size = slab.read_unchecked(resolution_id);

    *frag_color = Vec4::ZERO;

    for i in 0..draw_calls.len() {
        let draw_call = unsafe { draw_calls.index_unchecked(i) };
        let renderlet_id = Id::<RenderletDescriptor>::new(draw_call.first_instance);
        let transform_id =
            slab.read_unchecked(renderlet_id + RenderletDescriptor::OFFSET_OF_TRANSFORM_ID);
        let mut model = TransformDescriptor::IDENTITY;
        slab.read_into_if_some(transform_id, &mut model);
        let bounds = slab.read_unchecked(renderlet_id + RenderletDescriptor::OFFSET_OF_BOUNDS);

        let (_, sphere_in_world_coords) = bounds.is_inside_camera_view(&camera, model);
        let sphere_aabb = sphere_in_world_coords.project_onto_viewport(
            &camera,
            Vec2::new(viewport_size.x as f32, viewport_size.y as f32),
        );

        let sdf_circle = sdf::Box {
            center: sphere_aabb.center().xy(),
            half_extent: (sphere_aabb.max.xy() - sphere_aabb.min.xy()) * 0.5,
        };

        let distance = sdf_circle.distance(frag_coord.xy() + 0.5);

        // Here we use `step_le`, which I have annotated with `#inline(always)`.
        // I did this because without it, it seems to do the opposite of expected.
        // I found this by inlining by hand.
        let alpha = crate::math::step_le(sphere_aabb.max.z, 1.0);
        if distance.abs() < 0.5 {
            *frag_color = Vec4::new(0.0, 0.0, 0.0, 1.0 * alpha);
        } else if distance.abs() <= 2.0 {
            *frag_color = Vec4::new(1.0, 1.0, 1.0, 0.5 * alpha);
        } else if distance.abs() <= 3.0 {
            *frag_color = Vec4::new(0.5, 0.5, 0.5, 1.0 * alpha);
        }
    }
}
