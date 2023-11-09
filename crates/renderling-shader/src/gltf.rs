//! GLTF assets storable on the GPU.
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
    pub scenes: slab::Array<Id<Scene>>,
    pub cameras: slab::Array<Id<Camera>>,
    pub lights: slab::Array<Id<Light>>,
    pub skins: slab::Array<Id<Skin>>,
    pub materials: slab::Array<Id<Material>>,
    pub meshes: slab::Array<Id<Mesh>>,
    pub nodes: slab::Array<Id<Node>>,
    pub animations: slab::Array<Id<Animation>>,
    pub default_scene: Id<Scene>,
}

impl FromSlab for Document {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            scenes,
            cameras,
            lights,
            skins,
            materials,
            meshes,
            nodes,
            animations,
            default_scene,
        } = self;
        let index = scenes.read_slab(index, slab);
        let index = cameras.read_slab(index, slab);
        let index = lights.read_slab(index, slab);
        let index = skins.read_slab(index, slab);
        let index = materials.read_slab(index, slab);
        let index = meshes.read_slab(index, slab);
        let index = nodes.read_slab(index, slab);
        let index = animations.read_slab(index, slab);
        default_scene.read_slab(index, slab)
    }
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

impl FromSlab for Target {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            animation,
            node,
            property,
        } = self;
        let index = animation.read_slab(index, slab);
        let index = node.read_slab(index, slab);
        property.0.read_slab(index, slab)
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
    pub input: slab::Array<f32>,
    pub interpolation: Interpolation,
    pub output_translation: slab::Array<Vec4>,
    pub output_rotation: slab::Array<Quat>,
    pub output_scale: slab::Array<Vec4>,
    pub output_morph_target_weights: slab::Array<slab::Array<f32>>,
}

impl FromSlab for Sampler {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            animation,
            input,
            interpolation,
            output_translation,
            output_rotation,
            output_scale,
            output_morph_target_weights,
        } = self;
        let index = animation.read_slab(index, slab);
        let index = input.read_slab(index, slab);
        let index = interpolation.0.read_slab(index, slab);
        let index = output_translation.read_slab(index, slab);
        let index = output_rotation.read_slab(index, slab);
        let index = output_scale.read_slab(index, slab);
        output_morph_target_weights.read_slab(index, slab)
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

impl FromSlab for Channel {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            animation,
            target,
            sampler,
        } = self;
        let index = animation.read_slab(index, slab);
        let index = target.read_slab(index, slab);
        sampler.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Animation {
    pub id: Id<Animation>,
    pub channels: slab::Array<Channel>,
    pub samplers: slab::Array<Sampler>,
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
    pub joints: slab::Array<Id<Node>>,
    pub inverse_bind_matrices: slab::Array<Mat4>,
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
    pub joints: slab::Array<Id<Node>>,
    pub weights: slab::Array<f32>,
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

impl FromSlab for Vertex {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            position,
            color,
            uv,
            normal,
            tangent,
            joints,
            weights,
        } = self;
        let index = position.read_slab(index, slab);
        let index = color.read_slab(index, slab);
        let index = uv.read_slab(index, slab);
        let index = normal.read_slab(index, slab);
        let index = tangent.read_slab(index, slab);
        let index = joints.read_slab(index, slab);
        weights.read_slab(index, slab)
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

impl FromSlab for MorphTarget {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            position,
            normal,
            tangent,
        } = self;
        let index = position.read_slab(index, slab);
        let index = normal.read_slab(index, slab);
        tangent.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Primitive {
    pub id: Id<Primitive>,
    pub material: Id<Material>,
    pub padding: [u32; 2],
    pub vertices: slab::Array<Vertex>,
    pub morph_targets: slab::Array<MorphTarget>,
    // bounds of the POSITION attribute, uses only xyz
    pub bounding_box: Vec4,
}

impl FromSlab for Primitive {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            id,
            material,
            padding,
            vertices,
            morph_targets,
            bounding_box,
        } = self;

        let index = id.read_slab(index, slab);
        let index = material.read_slab(index, slab);
        let index = padding.read_slab(index, slab);
        let index = vertices.read_slab(index, slab);
        let index = morph_targets.read_slab(index, slab);
        bounding_box.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mesh {
    pub id: Id<Mesh>,
    pub primitives: slab::Array<Id<Primitive>>,
    pub weights: slab::Array<f32>,
}

impl FromSlab for Mesh {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            id,
            primitives,
            weights,
        } = self;
        let index = id.read_slab(index, slab);
        let index = primitives.read_slab(index, slab);
        weights.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
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
    pub children: slab::Array<Id<Node>>,
    // Weights of the instantiated morph target
    pub weights: slab::Array<f32>,
    // transform of this node
    pub transform: Mat4,
}

impl FromSlab for Node {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let Self {
            id,
            camera,
            light,
            skin,
            mesh,
            padding,
            children,
            weights,
            transform,
        } = self;
        let index = id.read_slab(index, slab);
        let index = camera.read_slab(index, slab);
        let index = light.read_slab(index, slab);
        let index = skin.read_slab(index, slab);
        let index = mesh.read_slab(index, slab);
        let index = padding.read_slab(index, slab);
        let index = children.read_slab(index, slab);
        let index = weights.read_slab(index, slab);
        transform.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Scene {
    pub id: Id<Scene>,
    // indices of contiguous nodes
    pub nodes: slab::Array<Id<Node>>,
}
