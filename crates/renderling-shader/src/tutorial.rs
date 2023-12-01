//! Shaders used in the intro tutorial.
use glam::{Mat4, Vec4, Vec4Swizzles};
use spirv_std::spirv;

use crate::{
    array::Array,
    id::Id,
    slab::{Slab, Slabbed},
    stage::{RenderUnit, Vertex},
};

/// Simple fragment shader that writes the input color to the output color.
#[spirv(fragment)]
pub fn passthru_fragment(in_color: Vec4, output: &mut Vec4) {
    *output = in_color;
}

fn implicit_isosceles_triangle(vertex_index: u32) -> Vec4 {
    let x = (1 - vertex_index as i32) as f32 * 0.5;
    let y = ((vertex_index & 1) as f32 * 2.0 - 1.0) * 0.5;
    Vec4::new(x, y, 0.0, 1.0)
}

/// Simple vertex shader with an implicit isosceles triangle.
#[spirv(vertex)]
pub fn implicit_isosceles_vertex(
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    //#[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let pos = implicit_isosceles_triangle(vertex_index);
    *out_color = Vec4::new(1.0, 0.0, 0.0, 1.0);
    *clip_pos = pos;
}

/// This shader uses the vertex index as a slab [`Id`]. The [`Id`] is used to
/// read the vertex from the slab. The vertex's position and color are written
/// to the output.
#[spirv(vertex)]
pub fn slabbed_vertices_no_instance(
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let vertex_id = Id::<Vertex>::from(vertex_index as usize * Vertex::slab_size());
    let vertex = slab.read(vertex_id);
    *clip_pos = vertex.position;
    *out_color = vertex.color;
}

/// This shader uses the `instance_index` as a slab [`Id`].
/// The `instance_index` is the [`Id`] of an [`Array`] of [`Vertex`]s. The
/// `vertex_index` is the index of a [`Vertex`] within the [`Array`].
#[spirv(vertex)]
pub fn slabbed_vertices(
    // Id of the array of vertices we are rendering
    #[spirv(instance_index)] instance_index: u32,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let array_id = Id::<Array<Vertex>>::from(instance_index);
    let array = slab.read(array_id);
    let vertex_id = array.at(vertex_index as usize);
    let vertex = slab.read(vertex_id);
    *clip_pos = vertex.position;
    *out_color = vertex.color;
}

/// This shader uses the `instance_index` as a slab [`Id`].
/// The `instance_index` is the [`Id`] of a [`RenderUnit`].
/// The [`RenderUnit`] contains an [`Array`] of [`Vertex`]s
/// as its mesh, the [`Id`]s of a [`Material`] and [`Camera`],
/// and TRS transforms.
/// The `vertex_index` is the index of a [`Vertex`] within the
/// [`RenderUnit`]'s `vertices` [`Array`].
#[spirv(vertex)]
pub fn slabbed_render_unit(
    // Id of the array of vertices we are rendering
    #[spirv(instance_index)] instance_index: u32,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let unit_id = Id::<RenderUnit>::from(instance_index);
    let unit = slab.read(unit_id);
    let vertex_id = unit.vertices.at(vertex_index as usize);
    let vertex = slab.read(vertex_id);
    let camera = slab.read(unit.camera);
    let model = Mat4::from_scale_rotation_translation(unit.scale, unit.rotation, unit.position);
    *clip_pos = camera.projection * camera.view * model * vertex.position.xyz().extend(1.0);
    *out_color = vertex.color;
}
