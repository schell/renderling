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
    gltf::{self, Vertex},
    bits::{bits, extract, insert},
    debug::*,
    pbr, slab::{self, FromSlab}, GpuToggles, Id, IsMatrix, IsVector, ID_NONE,
};


#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Entity {
    pub node: Id<gltf::Node>,
    pub info: u32,
    pub padding: [u32; 2],
    pub position: Vec4,
    pub rotation: Quat,
    pub scale: Vec4,
}

impl FromSlab for Entity {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self { node, info, padding, position, rotation, scale  } = self;
        let index = node.read_slab(index, slab);
        let index = info.read_slab(index, slab);
        let index = padding.read_slab(index, slab);
        let index = position.read_slab(index, slab);
        let index = rotation.read_slab(index, slab);
        scale.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Stage {
    pub documents: slab::Array<gltf::Document>,
    pub entities: slab::Array<Entity>
}

impl FromSlab for Stage {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self { documents, entities } = self;
        let index = documents.read_slab(index, slab);
        entities.read_slab(index, slab)
    }
}

#[allow(clippy::too_many_arguments)]
/// Scene vertex shader.
pub fn stage_vertex(
    // slab id of the entity we are drawing
    instance_index: u32,
    // which vertex are we drawing
    vertex_index: u32,

    constants: &crate::scene::GpuConstants,
    slab: &[u32],

    out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,

    gl_pos: &mut Vec4,
) {
    let mut stage = Stage::default();
    let _ = stage.read_slab(0, slab);
    let mut entity = Entity::default();
    let _ = entity.read_slab(instance_index as usize, slab);
    let mut node = gltf::Node::default();
    let _ = node.read_slab(entity.node.index(), slab);
    let mut mesh = gltf::Mesh::default();
    let _ = node.read_slab(node.mesh.index(), slab);
    let mut prim_id = Id::<gltf::Primitive>::default();
    let mut primitive = gltf::Primitive::default();
    for i in 0..mesh.primitives.len() {
        mesh.primitives.extract(&mut prim_id, i, slab);
        primitive.read_slab(prim_id.index(), slab);
        if primitive.vertices.contains_index(vertex_index as usize) {
            break;
        }
    }
    let mut vertex = gltf::Vertex::default();
    let _ = vertex.read_slab(vertex_index as usize, slab);

    let mut model_matrix = Mat4::IDENTITY;

    let (position, rotation, scale) = if entity.info.is_skin() {
        let skin_mat = vertex.get_skin_matrix(&entity.skin_joint_ids, entities);
        let (s, r, t) = skin_mat.to_scale_rotation_translation_or_id();
        (t, r, s)
    } else {
        entity.get_world_transform(entities)
    };
    let model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, position);
    *out_material = entity.material.into();
    *out_color = vertex.color;
    *out_uv0 = vertex.uv.xy();
    *out_uv1 = vertex.uv.zw();

    let scale2 = scale * scale;
    let normal = vertex.normal.xyz().alt_norm_or_zero();
    let tangent = vertex.tangent.xyz().alt_norm_or_zero();
    let normal_w = (model_matrix * (normal / scale2).extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    let tangent_w = (model_matrix * tangent.extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    let bitangent_w = normal_w.cross(tangent_w) * if vertex.tangent.w >= 0.0 { 1.0 } else { -1.0 };
    *out_tangent = tangent_w;
    *out_bitangent = bitangent_w;
    *out_norm = normal_w;

    let view_pos = model_matrix * vertex.position.xyz().extend(1.0);
    *out_pos = view_pos.xyz();
    *gl_pos = constants.camera_projection * constants.camera_view * view_pos;
}
