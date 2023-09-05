//! GLTF assets storable on the GPU.
//!
//! Storage buffers:
//! - assets
//! - scenes
//! - animations
//! - cameras
//! - lights
//! - skins
//! - meshes
//! - primitives
//! - materials
//! - vertices
//! - nodes
//! - slab
use glam::{Mat4, Quat, Vec4};

use crate::{
    id::*,
    scene::LightType,
    slab::{self, FromSlab},
};

pub type Material = crate::pbr::PbrMaterial;

/// A GLTF asset living on the GPU.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Document {
    pub scenes: slab::Array<Id<Scene>, 1>,
    pub cameras: slab::Array<Id<Camera>, 1>,
    pub lights: slab::Array<Id<Light>, 1>,
    pub skins: slab::Array<Id<Skin>, 1>,
    pub materials: slab::Array<Id<Material>, 1>,
    pub meshes: slab::Array<Id<Mesh>, 1>,
    pub nodes: slab::Array<Id<Node>, 1>,
    pub animations: slab::Array<Id<Animation>, 1>,
    pub default_scene: Id<Scene>,
}

#[repr(u32)]
pub enum PropertyType {
    Translation,
    Rotation,
    Scale,
    MorphTargetWeights,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Property(u32);

impl Property {
    pub fn as_enum(&self) -> PropertyType {
        match self.0 {
            0 => PropertyType::Translation,
            1 => PropertyType::Rotation,
            2 => PropertyType::Scale,
            4 => PropertyType::MorphTargetWeights,
            _ => PropertyType::Translation,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
// 3 u32s
pub struct Target {
    pub animation: Id<Animation>,
    pub node: Id<Node>,
    pub property: Property,
}

impl FromSlab<3> for Target {
    fn read_slab(&mut self, [a, n, p]: [u32; 3]) {
        let Self {
            animation,
            node,
            property,
        } = self;
        *animation = Id::new(a);
        *node = Id::new(n);
        *property = Property(p);
    }
}

#[repr(u32)]
pub enum InterpolationType {
    Linear,
    Step,
    CubicSpline,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Interpolation(u32);

impl Interpolation {
    pub fn as_enum(&self) -> InterpolationType {
        match self.0 {
            0 => InterpolationType::Linear,
            1 => InterpolationType::Step,
            2 => InterpolationType::CubicSpline,
            _ => InterpolationType::Linear,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
// 12 u32s
pub struct Sampler {
    pub animation: Id<Animation>,
    // keyframe times
    pub input: slab::Array<f32, 1>,
    pub interpolation: Interpolation,
    pub output_translation: slab::Array<Vec4, 4>,
    pub output_rotation: slab::Array<Quat, 4>,
    pub output_scale: slab::Array<Vec4, 4>,
    pub output_morph_target_weights: slab::Array<slab::Array<f32, 1>, 2>,
}

impl FromSlab<12> for Sampler {
    fn read_slab(&mut self, [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11]: [u32; 12]) {
        let Self {
            animation,
            input,
            interpolation,
            output_translation,
            output_rotation,
            output_scale,
            output_morph_target_weights,
        } = self;
        Id::read_slab(animation, [n0]);
        slab::Array::read_slab(input, [n1, n2]);
        *interpolation = Interpolation(n3);
        slab::Array::read_slab(output_translation, [n4, n5]);
        slab::Array::read_slab(output_rotation, [n6, n7]);
        slab::Array::read_slab(output_scale, [n8, n9]);
        slab::Array::read_slab(output_morph_target_weights, [n10, n11]);
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
// 16 u32s
pub struct Channel {
    pub animation: Id<Animation>,
    pub target: Target,
    pub sampler: Sampler,
}

impl FromSlab<16> for Channel {
    fn read_slab(
        &mut self,
        [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15]: [u32; 16],
    ) {
        let Self { animation, target, sampler } = self;
        Id::read_slab(animation, [n0]);
        Target::read_slab(target, [n1, n2, n3]);
        Sampler::read_slab(sampler, [n4, n5, n6, n7, n8, n9, n10, n11]);
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Animation {
    pub id: Id<Animation>,
    pub channels: slab::Array<Channel, 16>,
    pub samplers: slab::Array<Sampler, 12>,
}

#[repr(u32)]
pub enum ProjectionType {
    Orthographic,
    Perspective,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Projection(u32);

impl Projection {
    pub fn as_enum(&self) -> ProjectionType {
        match self.0 {
            0 => ProjectionType::Orthographic,
            1 => ProjectionType::Perspective,
            _ => ProjectionType::Orthographic,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub id: Id<Camera>,
    pub projection: u32,
    pub xmag_or_aspect: f32,
    pub ymag_or_yfov: f32,
    pub zfar: f32,
    pub znear: f32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub id: Id<Light>,
    // Intensity
    pub intensity: f32,
    // A distance cutoff at which the light's intensity may be considered to have reached
    // zero. A value of f32::MAX means that there is **no range**.
    pub range: f32,
    // The subcategory of light
    pub kind: LightType,
    // Color of the light source (uses only xyz components, w is unsused)
    pub color: Vec4,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Skin {
    pub id: Id<Skin>,
    pub skeleton: Id<Node>,
    pub joints: slab::Array<Id<Node>, 1>,
    pub inverse_bind_matrices: slab::Array<Mat4, 16>,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4,
    pub joints: slab::Array<Id<Node>, 1>,
    pub weights: slab::Array<f32, 1>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
            tangent: Vec4::Y,
            joints: Default::default(),
            weights: Default::default(),
        }
    }
}

impl FromSlab<24> for Vertex {
    fn read_slab(&mut self, slab: [u32; 24]) {
        let Self { position, color, uv, normal, tangent, joints, weights } = self;
        //Vec4::read_slab(position, slab)
    }
}

impl Vertex {
    ///// Return the matrix needed to bring vertices into the coordinate space of
    ///// the joint node.
    // pub fn get_joint_matrix(
    //    &self,
    //    i: usize,
    //    joint_ids: &[Id<GpuEntity>; 32],
    //    entities: &[GpuEntity],
    //) -> Mat4 {
    //    if i >= self.joints.len() {
    //        return Mat4::IDENTITY;
    //    }
    //    let joint_index = self.joints[i];
    //    let joint_id = if joint_index as usize >= joint_ids.len() {
    //        Id::NONE
    //    } else {
    //        joint_ids[joint_index as usize]
    //    };
    //    if joint_id.is_none() {
    //        return Mat4::IDENTITY;
    //    }
    //    let entity_index = joint_id.index();
    //    if entity_index >= entities.len() {
    //        return Mat4::IDENTITY;
    //    }
    //    let joint_entity = &entities[entity_index];
    //    let (t, r, s) = joint_entity.get_world_transform(entities);
    //    let trs = Mat4::from_scale_rotation_translation(s, r, t);
    //    trs * joint_entity.inverse_bind_matrix
    //}

    ///// Return the result of adding all joint matrices multiplied by their
    ///// weights for the given vertex.
    //// See the [khronos gltf viewer reference](https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/47a191931461a6f2e14de48d6da0f0eb6ec2d147/source/Renderer/shaders/animation.glsl#L47)
    // pub fn get_skin_matrix(&self, joint_ids: &[Id<GpuEntity>; 32], entities:
    // &[GpuEntity]) -> Mat4 {    let mut mat = Mat4::ZERO;
    //    for i in 0..self.joints.len() {
    //        mat += self.weights[i] * self.get_joint_matrix(i, joint_ids,
    // entities);    }
    //    if mat == Mat4::ZERO {
    //        return Mat4::IDENTITY;
    //    }
    //    mat
    //}

    // pub fn generate_normal(a: GpuVertex, b: GpuVertex, c: GpuVertex) -> Vec3 {
    //    let ab = a.position.xyz() - b.position.xyz();
    //    let ac = a.position.xyz() - c.position.xyz();
    //    ab.cross(ac).normalize()
    //}

    // pub fn generate_tangent(a: GpuVertex, b: GpuVertex, c: GpuVertex) -> Vec4 {
    //    let ab = b.position.xyz() - a.position.xyz();
    //    let ac = c.position.xyz() - a.position.xyz();
    //    let n = ab.cross(ac);
    //    let d_uv1 = b.uv.xy() - a.uv.xy();
    //    let d_uv2 = c.uv.xy() - a.uv.xy();
    //    let denom = d_uv1.x * d_uv2.y - d_uv2.x * d_uv1.y;
    //    let denom_sign = if denom >= 0.0 { 1.0 } else { -1.0 };
    //    let denom = denom.abs().max(f32::EPSILON) * denom_sign;
    //    let f = 1.0 / denom;
    //    let s = f * Vec3::new(
    //        d_uv2.y * ab.x - d_uv1.y * ac.x,
    //        d_uv2.y * ab.y - d_uv1.y * ac.y,
    //        d_uv2.y * ab.z - d_uv1.y * ac.z,
    //    );
    //    let t = f * Vec3::new(
    //        d_uv1.x * ac.x - d_uv2.x * ab.x,
    //        d_uv1.x * ac.y - d_uv2.x * ab.y,
    //        d_uv1.x * ac.z - d_uv2.x * ab.z,
    //    );
    //    let n_cross_t_dot_s_sign = if n.cross(t).dot(s) >= 0.0 { 1.0 } else { -1.0
    // };    (s - s.dot(n) * n)
    //        .alt_norm_or_zero()
    //        .extend(n_cross_t_dot_s_sign)
    //}
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MorphTarget {
    pub position: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Primitive {
    pub id: Id<Primitive>,
    pub material: Id<Material>,
    pub padding: [u32; 2],
    pub attributes: slab::Array<Vertex, 24>,
    //pub morph_targets: slab::Array<MorphTarget, 12>,
    // bounds of the POSITION attribute, uses only xyz
    pub bounding_box: Vec4,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mesh {
    pub id: Id<Mesh>,
    pub primitives: slab::Array<Id<Primitive>, 1>,
    pub weights: slab::Array<f32, 1>,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node {
    pub id: Id<Node>,
    // index of possible camera
    pub camera: Id<Camera>,
    // index of possible light
    pub light: Id<Light>,
    // index of possible skin
    pub skin: Id<Skin>,
    // index of possible mesh
    pub mesh: Id<Mesh>,
    pub padding: [u32; 3],
    // indices of children
    pub children: slab::Array<Id<Node>, 1>,
    // Weights of the instantiated morph target
    pub weights: slab::Array<f32, 1>,
    // transform of this node
    pub transform: Mat4,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Scene {
    pub id: Id<Scene>,
    // indices of contiguous nodes
    pub nodes: slab::Array<Id<Node>, 1>,
}
