//! Shaders used in the contributor intro tutorial and in WASM tests.

use crabslab::{Array, Id, Slab, SlabItem};
use glam::{Vec3, Vec3Swizzles, Vec4};
use spirv_std::spirv;

use crate::{
    geometry::{shader::GeometryDescriptor, Vertex},
    primitive::shader::{PrimitiveDescriptor, VertexInfo},
};

/// Simple fragment shader that writes the input color to the output color.
// Inline pragma needed so this shader doesn't get optimized away:
// See <https://github.com/Rust-GPU/rust-gpu/issues/185#issuecomment-2661663722>
#[inline(never)]
#[spirv(fragment)]
pub fn passthru_fragment(in_color: Vec4, output: &mut Vec4) {
    *output = in_color;
}

/// Simple vertex shader with an implicit isosceles triangle.
///
/// This shader gets run with three indices and draws a triangle without
/// using any other data from the CPU.
#[spirv(vertex)]
pub fn implicit_isosceles_vertex(
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let pos = {
        let x = (1 - vertex_index as i32) as f32 * 0.5;
        let y = (((vertex_index & 1) as f32 * 2.0) - 1.0) * 0.5;
        Vec4::new(x, y, 0.0, 1.0)
    };
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
    let vertex_id = Id::<Vertex>::from(vertex_index as usize * Vertex::SLAB_SIZE);
    let vertex = slab.read(vertex_id);
    *clip_pos = vertex.position.extend(1.0);
    *out_color = vertex.color;
}

/// This shader uses the `instance_index` as a slab [`Id`].
/// The `instance_index` is the [`Id`] of an [`Array`] of [`Vertex`]s. The
/// `vertex_index` is the index of a [`Vertex`] within the [`Array`].
#[spirv(vertex)]
pub fn slabbed_vertices(
    // Id of the array of vertices we are rendering
    #[spirv(instance_index)] array_id: Id<Array<(Vec3, Vec4)>>,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let array = slab.read(array_id);
    let vertex_id = array.at(vertex_index as usize);
    let (position, color) = slab.read(vertex_id);
    *clip_pos = position.extend(1.0);
    *out_color = color;
}

/// This shader uses the `instance_index` as a slab id.
/// The `instance_index` is the `id` of a [`PrimitiveDescriptor`].
/// The [`PrimitiveDescriptor`] contains an [`Array`] of [`Vertex`]s
/// as its mesh, the [`Id`]s of a
/// [`MaterialDescriptor`](crate::material::shader::MaterialDescriptor) and
///[`CameraDescriptor`](crate::camera::shader::CameraDescriptor),
/// and TRS transforms.
/// The `vertex_index` is the index of a [`Vertex`] within the
/// [`PrimitiveDescriptor`]'s `vertices` [`Array`].
#[spirv(vertex)]
pub fn slabbed_renderlet(
    // Id of the array of vertices we are rendering
    #[spirv(instance_index)] primitive_id: Id<PrimitiveDescriptor>,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let prim = slab.read(primitive_id);
    let VertexInfo {
        vertex,
        model_matrix,
        ..
    } = prim.get_vertex_info(vertex_index, slab);
    let camera_id =
        slab.read_unchecked(prim.geometry_descriptor_id + GeometryDescriptor::OFFSET_OF_CAMERA_ID);
    let camera = slab.read(camera_id);
    *clip_pos = camera.view_projection() * model_matrix * vertex.position.xyz().extend(1.0);
    *out_color = vertex.color;
}
