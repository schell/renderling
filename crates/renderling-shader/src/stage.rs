//! Types used to store and update an entire scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{mat3, Mat4, Quat, UVec2, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

use crate::{
    self as renderling_shader,
    array::Array,
    bits::{bits, extract, insert},
    debug::*,
    gltf::{GltfMesh, GltfNode},
    id::{Id, ID_NONE},
    pbr::{self, PbrMaterial},
    slab::{Slab, Slabbed},
    texture::GpuTexture,
    IsMatrix, IsSampler, IsVector, Sample2d, SampleCube,
};

pub mod light;

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4,
    // Indices that point to this vertex's joints by indexing into an array of Id<GpuEntity>
    // provided by the GpuEntity that is using this vertex
    pub joints: [u32; 4],
    // The weights of influence that each joint has over this vertex
    pub weights: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
            tangent: Vec4::Y,
            joints: [0; 4],
            weights: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn with_position(mut self, p: impl Into<Vec3>) -> Self {
        self.position = p.into().extend(0.0);
        self
    }

    pub fn with_color(mut self, c: impl Into<Vec4>) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_uv0(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uv.x = uv.x;
        self.uv.y = uv.y;
        self
    }

    pub fn with_uv1(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uv.z = uv.x;
        self.uv.w = uv.y;
        self
    }

    pub fn with_normal(mut self, n: impl Into<Vec3>) -> Self {
        self.normal = n.into().extend(0.0);
        self
    }

    pub fn with_tangent(mut self, t: impl Into<Vec4>) -> Self {
        self.tangent = t.into();
        self
    }

    /// Return the matrix needed to bring vertices into the coordinate space of
    /// the joint node.
    pub fn get_joint_matrix(
        &self,
        i: usize,
        joint_ids: &[Id<GpuEntity>; 32],
        entities: &[GpuEntity],
    ) -> Mat4 {
        if i >= self.joints.len() {
            return Mat4::IDENTITY;
        }
        let joint_index = self.joints[i];
        let joint_id = if joint_index as usize >= joint_ids.len() {
            Id::NONE
        } else {
            joint_ids[joint_index as usize]
        };
        if joint_id.is_none() {
            return Mat4::IDENTITY;
        }
        let entity_index = joint_id.index();
        if entity_index >= entities.len() {
            return Mat4::IDENTITY;
        }
        let joint_entity = &entities[entity_index];
        let (t, r, s) = joint_entity.get_world_transform(entities);
        let trs = Mat4::from_scale_rotation_translation(s, r, t);
        trs * joint_entity.inverse_bind_matrix
    }

    /// Return the result of adding all joint matrices multiplied by their
    /// weights for the given vertex.
    // See the [khronos gltf viewer reference](https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/47a191931461a6f2e14de48d6da0f0eb6ec2d147/source/Renderer/shaders/animation.glsl#L47)
    pub fn get_skin_matrix(&self, joint_ids: &[Id<GpuEntity>; 32], entities: &[GpuEntity]) -> Mat4 {
        let mut mat = Mat4::ZERO;
        for i in 0..self.joints.len() {
            mat += self.weights[i] * self.get_joint_matrix(i, joint_ids, entities);
        }
        if mat == Mat4::ZERO {
            return Mat4::IDENTITY;
        }
        mat
    }

    pub fn generate_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        let ab = a - b;
        let ac = a - c;
        ab.cross(ac).normalize()
    }

    pub fn generate_tangent(a: Vec3, a_uv: Vec2, b: Vec3, b_uv: Vec2, c: Vec3, c_uv: Vec2) -> Vec4 {
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac);
        let d_uv1 = b_uv - a_uv;
        let d_uv2 = c_uv - a_uv;
        let denom = d_uv1.x * d_uv2.y - d_uv2.x * d_uv1.y;
        let denom_sign = if denom >= 0.0 { 1.0 } else { -1.0 };
        let denom = denom.abs().max(f32::EPSILON) * denom_sign;
        let f = 1.0 / denom;
        let s = f * Vec3::new(
            d_uv2.y * ab.x - d_uv1.y * ac.x,
            d_uv2.y * ab.y - d_uv1.y * ac.y,
            d_uv2.y * ab.z - d_uv1.y * ac.z,
        );
        let t = f * Vec3::new(
            d_uv1.x * ac.x - d_uv2.x * ab.x,
            d_uv1.x * ac.y - d_uv2.x * ab.y,
            d_uv1.x * ac.z - d_uv2.x * ab.z,
        );
        let n_cross_t_dot_s_sign = if n.cross(t).dot(s) >= 0.0 { 1.0 } else { -1.0 };
        (s - s.dot(n) * n)
            .alt_norm_or_zero()
            .extend(n_cross_t_dot_s_sign)
    }
}

#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct LightType(u32);

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Display for LightType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match *self {
            Self::END_OF_LIGHTS => "end of lights",
            Self::POINT_LIGHT => "point light",
            Self::SPOT_LIGHT => "spot light",
            Self::DIRECTIONAL_LIGHT => "directional light",
            _ => "unsupported light",
        };
        f.write_str(s)
    }
}

impl LightType {
    pub const END_OF_LIGHTS: Self = Self(0);
    pub const POINT_LIGHT: Self = Self(1);
    pub const SPOT_LIGHT: Self = Self(2);
    pub const DIRECTIONAL_LIGHT: Self = Self(3);
}

/// A light capable of representing a directional, point or spotlight.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct GpuLight {
    pub position: Vec4,
    pub direction: Vec4,
    pub attenuation: Vec4,
    pub color: Vec4,
    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub intensity: f32,
    pub light_type: LightType,
}

/// Determines the lighting to use in an ubershader.
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    bytemuck::Pod,
    bytemuck::Zeroable,
    Slabbed,
)]
pub struct LightingModel(u32);

impl LightingModel {
    pub const NO_LIGHTING: Self = LightingModel(0);
    pub const PBR_LIGHTING: Self = LightingModel(1);
}

/// Provides information about an entity.
///
/// ### Provides what attributes are provided by a morph target.
///
/// In `renderling` morph targets' geometry are meshlets - they are assumed to
/// come directly after the base mesh in the `GpuVertex` buffer, that way they
/// can be indexed from the base mesh using only the `GpuEntity` struct (through
/// `MorphTargetsInfo::num_targets`). See [`GpuEntity::get_vertex`] for more
/// info.
///
/// ### Provides info about if the entity is a skin.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct GpuEntityInfo(pub u32);

impl GpuEntityInfo {
    const BITS_NUM_TARGETS: (u32, u32) = bits(0..=7);
    const BITS_HAS_POSITIONS: (u32, u32) = bits(8..=8);
    const BITS_HAS_NORMALS: (u32, u32) = bits(9..=9);
    const BITS_HAS_TANGENTS: (u32, u32) = bits(10..=10);
    const BITS_IS_SKIN: (u32, u32) = bits(11..=11);

    pub fn num_morph_targets(&self) -> u32 {
        extract(self.0, Self::BITS_NUM_TARGETS)
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn set_num_morph_targets(&mut self, num_targets: u8) {
        insert(&mut self.0, Self::BITS_NUM_TARGETS, num_targets as u32)
    }

    pub fn morph_targets_have_positions(&self) -> bool {
        extract(self.0, Self::BITS_HAS_POSITIONS) == 1
    }

    pub fn set_morph_targets_have_positions(&mut self, has: bool) {
        insert(
            &mut self.0,
            Self::BITS_HAS_POSITIONS,
            if has { 1 } else { 0 },
        )
    }

    pub fn morph_targets_have_normals(&self) -> bool {
        extract(self.0, Self::BITS_HAS_NORMALS) == 1
    }

    pub fn set_morph_targets_have_normals(&mut self, has: bool) {
        insert(&mut self.0, Self::BITS_HAS_NORMALS, if has { 1 } else { 0 })
    }

    pub fn morph_targets_have_tangents(&self) -> bool {
        extract(self.0, Self::BITS_HAS_TANGENTS) == 1
    }

    pub fn set_morph_targets_have_tangents(&mut self, has: bool) {
        insert(
            &mut self.0,
            Self::BITS_HAS_TANGENTS,
            if has { 1 } else { 0 },
        )
    }

    pub fn is_skin(&self) -> bool {
        extract(self.0, Self::BITS_IS_SKIN) == 1
    }

    pub fn set_is_skin(&mut self, is_skin: bool) {
        insert(&mut self.0, Self::BITS_IS_SKIN, if is_skin { 1 } else { 0 })
    }
}

/// A bundle of GPU components.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct GpuEntity {
    // The id of this entity. `Id::NONE` means this entity is not in use.
    pub id: Id<GpuEntity>,
    // Info about the entity.
    pub info: GpuEntityInfo,
    // The index of the first vertex in this entity's mesh.
    pub mesh_first_vertex: u32,
    // The number of vertices in this entity's mesh.
    pub mesh_vertex_count: u32,
    // Nothing to see here
    pub padding: u32,
    // The index/id of this entity's material in the material buffer.
    pub material: Id<pbr::PbrMaterial>,
    // The id of this entity's parent, if it exists. `Id::NONE` means "no parent".
    pub parent: Id<GpuEntity>,
    // Whether this entity is visible. `0` is "not visible", any other value is "visible".
    pub visible: u32,
    // The local translation of this entity
    pub position: Vec4,
    // The local scale of this entity
    pub scale: Vec4,
    // The local rotation of this entity
    pub rotation: Quat,
    // A joint inverse-bind-matrix
    // TODO: restructure the buffers so we don't need this
    pub inverse_bind_matrix: Mat4,
    pub skin_joint_ids: [Id<GpuEntity>; 32],
    // Morph target weights for this entity, if it has morph targets
    pub morph_targets_weights: [f32; 8],
}

impl Default for GpuEntity {
    fn default() -> Self {
        Self {
            id: Id::NONE,
            mesh_first_vertex: 0,
            mesh_vertex_count: 0,
            info: GpuEntityInfo::default(),
            morph_targets_weights: [0.0; 8],
            padding: 0,
            material: Id::NONE,
            position: Vec4::ZERO,
            scale: Vec4::ONE,
            rotation: Quat::IDENTITY,
            visible: 1,
            parent: Id::NONE,
            skin_joint_ids: [Id::NONE; 32],
            inverse_bind_matrix: Mat4::IDENTITY,
        }
    }
}

impl From<&GpuEntity> for Id<GpuEntity> {
    fn from(value: &GpuEntity) -> Self {
        value.id
    }
}

impl From<GpuEntity> for Id<GpuEntity> {
    fn from(value: GpuEntity) -> Self {
        value.id
    }
}

impl GpuEntity {
    pub fn is_alive(&self) -> bool {
        !self.id.is_none()
    }

    /// Return the position, rotation and scale that describe this entity's
    /// transform in world space.
    pub fn get_world_transform(&self, entities: &[GpuEntity]) -> (Vec3, Quat, Vec3) {
        let mut mat = Mat4::IDENTITY;
        let mut id = self.id;
        loop {
            let entity = entities[id.index()];
            mat = Mat4::from_scale_rotation_translation(
                entity.scale.xyz(),
                entity.rotation,
                entity.position.xyz(),
            ) * mat;
            id = entity.parent;
            if id.index() >= entities.len() {
                break;
            }
        }
        let (s, r, t) = mat.to_scale_rotation_translation_or_id();
        (t, r, s)
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn set_morph_target_weights(&mut self, weights: impl IntoIterator<Item = f32>) {
        for (i, weight) in weights
            .into_iter()
            .take(self.morph_targets_weights.len())
            .enumerate()
        {
            self.morph_targets_weights[i] = weight;
        }
    }

    /// Return the `GpuVertex` at the given index.
    ///
    /// This takes into consideration any morph targets the base mesh may
    /// reference as well as any joints.
    pub fn get_vertex(&self, vertex_index: u32, vertices: &[Vertex]) -> Vertex {
        let index = vertex_index as usize;
        let mut vertex = vertices[index];

        // Apply target morphing
        let targets = self.info.num_morph_targets() as usize;
        let vertex_count = self.mesh_vertex_count as usize;
        for i in 1..=targets {
            let target_weight = self.morph_targets_weights[i - 1];
            let target_index = index + (i * vertex_count);
            let target = vertices[target_index];
            vertex.position += (target_weight * target.position.xyz()).extend(0.0);
            vertex.normal += (target_weight * target.normal.xyz()).extend(0.0);
            vertex.tangent += (target_weight * target.tangent.xyz()).extend(0.0);
        }

        vertex
    }

    #[cfg(not(target_arch = "spirv"))]
    /// Return all the vertices of this entity.
    ///
    /// This takes into consideration any morph targets the base mesh may
    /// reference as well as any joints.
    pub fn get_vertices(&self, vertices: &[Vertex]) -> Vec<Vertex> {
        let mut mesh_vertices = vec![];
        for vertex_index in self.mesh_first_vertex..self.mesh_first_vertex + self.mesh_vertex_count
        {
            mesh_vertices.push(self.get_vertex(vertex_index, vertices));
        }
        mesh_vertices
    }
}

/// Boolean toggles that cause the renderer to turn on/off certain features.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct GpuToggles(pub u32);

impl GpuToggles {
    const BITS_HAS_SKYBOX: (u32, u32) = bits(0..=0);
    const BITS_USE_LIGHTING: (u32, u32) = bits(1..=1);

    pub fn get_has_skybox(&self) -> bool {
        extract(self.0, Self::BITS_HAS_SKYBOX) == 1
    }

    pub fn set_has_skybox(&mut self, has: bool) {
        insert(&mut self.0, Self::BITS_HAS_SKYBOX, if has { 1 } else { 0 })
    }

    pub fn get_use_lighting(&self) -> bool {
        extract(self.0, Self::BITS_USE_LIGHTING) == 1
    }

    /// Setting this to `false` causes all models to be rendered "unlit", as
    /// if each used a material with `lighting_model = LightModel::NO_LIGHTING`.
    pub fn set_use_lighting(&mut self, use_lighting: bool) {
        insert(
            &mut self.0,
            Self::BITS_USE_LIGHTING,
            if use_lighting { 1 } else { 0 },
        )
    }
}

/// Unforms/constants for a scene's worth of rendering.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct GpuConstants {
    pub camera_projection: Mat4,
    pub camera_view: Mat4,
    pub camera_pos: Vec4,
    pub atlas_size: UVec2,
    pub debug_mode: DebugMode,
    pub toggles: GpuToggles,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

fn texture_color<T, S>(
    texture_id: Id<GpuTexture>,
    uv: Vec2,
    atlas: &T,
    sampler: &S,
    atlas_size: UVec2,
    textures: &[GpuTexture],
) -> Vec4
where
    T: Sample2d<Sampler = S>,
    S: IsSampler,
{
    let texture = if texture_id.is_none() {
        GpuTexture::default()
    } else {
        textures[texture_id.index()]
    };

    let uv = texture.uv(uv, atlas_size);
    let mut color: Vec4 = atlas.sample_by_lod(*sampler, uv, 0.0);
    if texture_id.is_none() {
        color = Vec4::splat(1.0);
    }
    color
}

fn stage_texture_color<T: Sample2d<Sampler = S>, S: IsSampler>(
    texture_id: Id<GpuTexture>,
    uv: Vec2,
    atlas: &T,
    sampler: &S,
    atlas_size: UVec2,
    slab: &[u32],
) -> Vec4 {
    let texture = slab.read(texture_id);
    let uv = texture.uv(uv, atlas_size);
    let mut color: Vec4 = atlas.sample_by_lod(*sampler, uv, 0.0);
    if texture_id.is_none() {
        color = Vec4::splat(1.0);
    }
    color
}

#[allow(clippy::too_many_arguments)]
#[spirv(vertex)]
/// Scene vertex shader.
pub fn main_vertex_scene(
    // which entity are we drawing
    #[spirv(instance_index)] instance_index: u32,
    // which vertex are we drawing
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &GpuConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] vertices: &[Vertex],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities: &[GpuEntity],

    #[spirv(flat)] out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,

    #[spirv(position)] gl_pos: &mut Vec4,
) {
    let entity = entities[instance_index as usize];
    let vertex = entity.get_vertex(vertex_index, vertices);

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

#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
/// Scene fragment shader.
pub fn main_fragment_scene(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 0, binding = 5)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 6)] irradiance_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 7)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 8)] prefiltered_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 9)] brdf: &Image2d,
    #[spirv(descriptor_set = 0, binding = 10)] brdf_sampler: &Sampler,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &GpuConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] lights: &[GpuLight],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] materials: &[pbr::PbrMaterial],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 2)] textures: &[GpuTexture],

    #[spirv(flat)] in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
    brigtness: &mut Vec4,
) {
    main_fragment_impl(
        atlas,
        atlas_sampler,
        irradiance,
        irradiance_sampler,
        prefiltered,
        prefiltered_sampler,
        brdf,
        brdf_sampler,
        constants,
        lights,
        materials,
        textures,
        in_material,
        in_color,
        in_uv0,
        in_uv1,
        in_norm,
        in_tangent,
        in_bitangent,
        in_pos,
        output,
        brigtness,
    );
}

/// Scene fragment shader, callable from the CPU or GPU.
pub fn main_fragment_impl<T, C, S>(
    atlas: &T,
    atlas_sampler: &S,

    irradiance: &C,
    irradiance_sampler: &S,
    prefiltered: &C,
    prefiltered_sampler: &S,
    brdf: &T,
    brdf_sampler: &S,

    constants: &GpuConstants,
    lights: &[GpuLight],
    materials: &[pbr::PbrMaterial],
    textures: &[GpuTexture],

    in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
    brigtness: &mut Vec4,
) where
    T: Sample2d<Sampler = S>,
    C: SampleCube<Sampler = S>,
    S: IsSampler,
{
    let material = if in_material == ID_NONE || !constants.toggles.get_use_lighting() {
        // without an explicit material (or if the entire render has no lighting)
        // the entity will not participate in any lighting calculations
        pbr::PbrMaterial {
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        }
    } else {
        materials[in_material as usize]
    };

    let albedo_tex_uv = if material.albedo_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let albedo_tex_color = texture_color(
        material.albedo_texture,
        albedo_tex_uv,
        atlas,
        atlas_sampler,
        constants.atlas_size,
        textures,
    );

    let metallic_roughness_uv = if material.metallic_roughness_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let metallic_roughness_tex_color = texture_color(
        material.metallic_roughness_texture,
        metallic_roughness_uv,
        atlas,
        atlas_sampler,
        constants.atlas_size,
        textures,
    );

    let normal_tex_uv = if material.normal_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let normal_tex_color = texture_color(
        material.normal_texture,
        normal_tex_uv,
        atlas,
        atlas_sampler,
        constants.atlas_size,
        textures,
    );

    let ao_tex_uv = if material.ao_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let ao_tex_color = texture_color(
        material.ao_texture,
        ao_tex_uv,
        atlas,
        atlas_sampler,
        constants.atlas_size,
        textures,
    );

    let emissive_tex_uv = if material.emissive_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let emissive_tex_color = texture_color(
        material.emissive_texture,
        emissive_tex_uv,
        atlas,
        atlas_sampler,
        constants.atlas_size,
        textures,
    );

    let (norm, uv_norm) = if material.normal_texture.is_none() {
        // there is no normal map, use the normal normal ;)
        (in_norm, Vec3::ZERO)
    } else {
        // convert the normal from color coords to tangent space -1,1
        let sampled_norm = (normal_tex_color.xyz() * 2.0 - Vec3::splat(1.0)).alt_norm_or_zero();
        let tbn = mat3(
            in_tangent.alt_norm_or_zero(),
            in_bitangent.alt_norm_or_zero(),
            in_norm.alt_norm_or_zero(),
        );
        // convert the normal from tangent space to world space
        let norm = (tbn * sampled_norm).alt_norm_or_zero();
        (norm, sampled_norm)
    };

    let n = norm;
    let albedo = albedo_tex_color * material.albedo_factor * in_color;
    let roughness = metallic_roughness_tex_color.y * material.roughness_factor;
    let metallic = metallic_roughness_tex_color.z * material.metallic_factor;
    let ao = 1.0 + material.ao_strength * (ao_tex_color.x - 1.0);
    let emissive =
        emissive_tex_color.xyz() * material.emissive_factor.xyz() * material.emissive_factor.w;
    let irradiance = pbr::sample_irradiance(irradiance, irradiance_sampler, n);
    let specular = pbr::sample_specular_reflection(
        prefiltered,
        prefiltered_sampler,
        constants.camera_pos.xyz(),
        in_pos,
        n,
        roughness,
    );
    let brdf = pbr::sample_brdf(
        brdf,
        brdf_sampler,
        constants.camera_pos.xyz(),
        in_pos,
        n,
        roughness,
    );

    fn colorize(u: Vec3) -> Vec4 {
        ((u.alt_norm_or_zero() + Vec3::splat(1.0)) / 2.0).extend(1.0)
    }

    match constants.debug_mode.into() {
        DebugChannel::None => {}
        DebugChannel::UvCoords0 => {
            *output = colorize(Vec3::new(in_uv0.x, in_uv0.y, 0.0));
            return;
        }
        DebugChannel::UvCoords1 => {
            *output = colorize(Vec3::new(in_uv1.x, in_uv1.y, 0.0));
            return;
        }
        DebugChannel::Normals => {
            *output = colorize(norm);
            return;
        }
        DebugChannel::VertexColor => {
            *output = in_color;
            return;
        }
        DebugChannel::VertexNormals => {
            *output = colorize(in_norm);
            return;
        }
        DebugChannel::UvNormals => {
            *output = colorize(uv_norm);
            return;
        }
        DebugChannel::Tangents => {
            *output = colorize(in_tangent);
            return;
        }
        DebugChannel::Bitangents => {
            *output = colorize(in_bitangent);
            return;
        }
        DebugChannel::DiffuseIrradiance => {
            *output = irradiance.extend(1.0);
            return;
        }
        DebugChannel::SpecularReflection => {
            *output = specular.extend(1.0);
            return;
        }
        DebugChannel::Brdf => {
            *output = brdf.extend(1.0).extend(1.0);
            return;
        }
        DebugChannel::Roughness => {
            *output = Vec3::splat(roughness).extend(1.0);
            return;
        }
        DebugChannel::Metallic => {
            *output = Vec3::splat(metallic).extend(1.0);
            return;
        }
        DebugChannel::Albedo => {
            *output = albedo;
            return;
        }
        DebugChannel::Occlusion => {
            *output = Vec3::splat(ao).extend(1.0);
            return;
        }
        DebugChannel::Emissive => {
            *output = emissive.extend(1.0);
            return;
        }
        DebugChannel::UvEmissive => {
            *output = emissive_tex_color.xyz().extend(1.0);
            return;
        }
        DebugChannel::EmissiveFactor => {
            *output = material.emissive_factor.xyz().extend(1.0);
            return;
        }
        DebugChannel::EmissiveStrength => {
            *output = Vec3::splat(material.emissive_factor.w).extend(1.0);
            return;
        }
    }

    *output = match material.lighting_model {
        LightingModel::PBR_LIGHTING => pbr::shade_fragment(
            constants.camera_pos.xyz(),
            n,
            in_pos,
            albedo.xyz(),
            metallic,
            roughness,
            ao,
            emissive,
            irradiance,
            specular,
            brdf,
            lights,
        ),
        _unlit => in_color * albedo_tex_color * material.albedo_factor,
    };

    // write the brightest colors for the bloom effect
    let brightness_value = output.xyz().dot(Vec3::new(0.2126, 0.7152, 0.0722));
    *brigtness = if brightness_value > 1.0 {
        output.xyz()
    } else {
        Vec3::ZERO
    }
    .extend(1.0);
}

/// A camera used for transforming the stage during rendering.
///
/// Use `Camera::new(projection, view)` to create a new camera.
/// Or use `Camera::default` followed by `Camera::with_projection_and_view`
/// to set the projection and view matrices. Using the `with_*` or `set_*`
/// methods is preferred over setting the fields directly because they will
/// also update the camera's position.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Slabbed)]
pub struct Camera {
    pub projection: Mat4,
    pub view: Mat4,
    pub position: Vec3,
}

impl Camera {
    pub fn new(projection: Mat4, view: Mat4) -> Self {
        Camera::default().with_projection_and_view(projection, view)
    }

    pub fn set_projection_and_view(&mut self, projection: Mat4, view: Mat4) {
        self.projection = projection;
        self.view = view;
        self.position = view.inverse().transform_point3(Vec3::ZERO);
    }

    pub fn with_projection_and_view(mut self, projection: Mat4, view: Mat4) -> Self {
        self.set_projection_and_view(projection, view);
        self
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.set_projection_and_view(projection, self.view);
    }

    pub fn with_projection(mut self, projection: Mat4) -> Self {
        self.set_projection(projection);
        self
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.set_projection_and_view(self.projection, view);
    }

    pub fn with_view(mut self, view: Mat4) -> Self {
        self.set_view(view);
        self
    }
}

/// Holds important info about the stage.
///
/// This should be the first struct in the stage's slab.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Slabbed)]
pub struct StageLegend {
    pub atlas_size: UVec2,
    pub debug_mode: DebugMode,
    pub has_skybox: bool,
    pub has_lighting: bool,
    pub light_array: Array<light::Light>,
}

impl Default for StageLegend {
    fn default() -> Self {
        Self {
            atlas_size: Default::default(),
            debug_mode: Default::default(),
            has_skybox: Default::default(),
            has_lighting: true,
            light_array: Default::default(),
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Slabbed)]
pub struct GltfVertexData {
    // A path of node ids that leads to the node that contains the mesh.
    pub parent_node_path: Array<Id<GltfNode>>,
    // Points to a `GltfMesh` in the stage's slab.
    pub mesh: Id<GltfMesh>,
    // The index of the primitive within the mesh that this unit draws.
    pub primitive_index: u32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Slabbed)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

/// A rendering "command" that draws a single mesh from a top-level node.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Slabbed)]
pub struct RenderUnit {
    // Which node are we rendering, and what is the path through its
    // ancestors to get to it.
    pub node_path: Array<Id<GltfNode>>,
    // Index of the mesh within the child node that we're rendering.
    pub mesh_index: u32,
    // Index of the primitive within the mesh that we're rendering.
    pub primitive_index: u32,
    // Points to a `Camera` in the stage's slab.
    pub camera: Id<Camera>,
    // Points to a top-level `Transform` in the stage's slab.
    //
    // This is used to transform your GLTF models.
    pub transform: Id<Transform>,
    // Number of vertices to draw for this unit.
    //
    // This is a cache for convenience on CPU.
    pub vertex_count: u32,
}

impl RenderUnit {
    pub fn get_vertex_details(
        &self,
        vertex_index: u32,
        slab: &[u32],
    ) -> (Vertex, Transform, Id<PbrMaterial>) {
        let t = slab.read(self.transform);
        crate::println!("t: {t:#?}");
        let mut model = Mat4::from_scale_rotation_translation(t.scale, t.rotation, t.translation);
        crate::println!("model: {model:#?}");
        let mut node = GltfNode::default();
        for id_id in self.node_path.iter() {
            let node_id = slab.read(id_id);
            crate::println!("  node_id: {node_id:?}");
            node = slab.read(node_id);
            crate::println!("  node.scale: {:?}", node.scale);
            crate::println!("  node.rotation: {:?}", node.rotation);
            crate::println!("  node.translation: {:?}", node.translation);
            let node_transform =
                Mat4::from_scale_rotation_translation(node.scale, node.rotation, node.translation);
            model = model * node_transform;
        }

        crate::println!("model(after): {model:#?}");
        // TODO: check nodes for skinning
        let mesh = slab.read(node.mesh);
        let primitive_id = mesh.primitives.at(self.primitive_index as usize);
        let primitive = slab.read(primitive_id);
        let material = primitive.material;
        let vertex = primitive.get_vertex(vertex_index as usize, slab);
        let (s, r, t) = model.to_scale_rotation_translation_or_id();
        let transform = Transform {
            translation: t,
            rotation: r,
            scale: s,
        };
        (vertex, transform, material)
    }
}

#[spirv(vertex)]
pub fn gltf_vertex(
    // Which render unit are we rendering
    #[spirv(instance_index)] instance_index: u32,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] out_camera: &mut u32,
    #[spirv(flat)] out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let unit_id: Id<RenderUnit> = Id::from(instance_index);
    let unit = slab.read(unit_id);
    let (vertex, tfrm, material) = unit.get_vertex_details(vertex_index, slab);
    let model_matrix =
        Mat4::from_scale_rotation_translation(tfrm.scale, tfrm.rotation, tfrm.translation);
    *out_material = material.into();
    *out_color = vertex.color;
    *out_uv0 = vertex.uv.xy();
    *out_uv1 = vertex.uv.zw();
    let scale2 = tfrm.scale * tfrm.scale;
    let normal = vertex.normal.xyz().alt_norm_or_zero();
    let tangent = vertex.tangent.xyz().alt_norm_or_zero();
    let normal_w: Vec3 = (model_matrix * (normal / scale2).extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    let tangent_w: Vec3 = (model_matrix * tangent.extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    let bitangent_w = normal_w.cross(tangent_w) * if vertex.tangent.w >= 0.0 { 1.0 } else { -1.0 };
    *out_tangent = tangent_w;
    *out_bitangent = bitangent_w;
    *out_norm = normal_w;
    let view_pos = model_matrix * vertex.position.xyz().extend(1.0);
    *out_pos = view_pos.xyz();
    let camera = slab.read(unit.camera);
    *out_camera = unit.camera.into();
    *clip_pos = camera.projection * camera.view * view_pos;
}

/// Returns the `StageLegend` from the stage's slab.
///
/// The `StageLegend` should be the first struct in the slab, always.
pub fn get_stage_legend(slab: &[u32]) -> StageLegend {
    slab.read(Id::new(0))
}

/// Returns the `PbrMaterial` from the stage's slab.
pub fn get_material(material_index: u32, has_lighting: bool, slab: &[u32]) -> pbr::PbrMaterial {
    if material_index == ID_NONE {
        // without an explicit material (or if the entire render has no lighting)
        // the entity will not participate in any lighting calculations
        pbr::PbrMaterial {
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        }
    } else {
        let mut material = slab.read(Id::<PbrMaterial>::new(material_index));
        if !has_lighting {
            material.lighting_model = LightingModel::NO_LIGHTING;
        }
        material
    }
}

#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
/// Scene fragment shader.
pub fn gltf_fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] in_camera: u32,
    #[spirv(flat)] in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
    brigtness: &mut Vec4,
) {
    gltf_fragment_impl(
        atlas,
        atlas_sampler,
        irradiance,
        irradiance_sampler,
        prefiltered,
        prefiltered_sampler,
        brdf,
        brdf_sampler,
        slab,
        in_camera,
        in_material,
        in_color,
        in_uv0,
        in_uv1,
        in_norm,
        in_tangent,
        in_bitangent,
        in_pos,
        output,
        brigtness,
    );
}

#[allow(clippy::too_many_arguments)]
/// Scene fragment shader.
pub fn gltf_fragment_impl<T, C, S>(
    atlas: &T,
    atlas_sampler: &S,
    irradiance: &C,
    irradiance_sampler: &S,
    prefiltered: &C,
    prefiltered_sampler: &S,
    brdf: &T,
    brdf_sampler: &S,

    slab: &[u32],

    in_camera: u32,
    in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
    brigtness: &mut Vec4,
) where
    T: Sample2d<Sampler = S>,
    C: SampleCube<Sampler = S>,
    S: IsSampler,
{
    let legend = get_stage_legend(slab);
    crate::println!("legend: {:?}", legend);
    let StageLegend {
        atlas_size,
        debug_mode,
        has_skybox: _,
        has_lighting,
        light_array,
    } = legend;

    let material = get_material(in_material, has_lighting, slab);
    crate::println!("material: {:?}", material);

    let albedo_tex_uv = if material.albedo_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let albedo_tex_color = stage_texture_color(
        material.albedo_texture,
        albedo_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );
    crate::println!("albedo_tex_color: {:?}", albedo_tex_color);

    let metallic_roughness_uv = if material.metallic_roughness_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let metallic_roughness_tex_color = stage_texture_color(
        material.metallic_roughness_texture,
        metallic_roughness_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );
    crate::println!(
        "metallic_roughness_tex_color: {:?}",
        metallic_roughness_tex_color
    );

    let normal_tex_uv = if material.normal_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let normal_tex_color = stage_texture_color(
        material.normal_texture,
        normal_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );
    crate::println!("normal_tex_color: {:?}", normal_tex_color);

    let ao_tex_uv = if material.ao_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let ao_tex_color = stage_texture_color(
        material.ao_texture,
        ao_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );

    let emissive_tex_uv = if material.emissive_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let emissive_tex_color = stage_texture_color(
        material.emissive_texture,
        emissive_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );

    let (norm, uv_norm) = if material.normal_texture.is_none() {
        // there is no normal map, use the normal normal ;)
        (in_norm, Vec3::ZERO)
    } else {
        // convert the normal from color coords to tangent space -1,1
        let sampled_norm = (normal_tex_color.xyz() * 2.0 - Vec3::splat(1.0)).alt_norm_or_zero();
        let tbn = mat3(
            in_tangent.alt_norm_or_zero(),
            in_bitangent.alt_norm_or_zero(),
            in_norm.alt_norm_or_zero(),
        );
        // convert the normal from tangent space to world space
        let norm = (tbn * sampled_norm).alt_norm_or_zero();
        (norm, sampled_norm)
    };

    let n = norm;
    let albedo = albedo_tex_color * material.albedo_factor * in_color;
    let roughness = metallic_roughness_tex_color.y * material.roughness_factor;
    let metallic = metallic_roughness_tex_color.z * material.metallic_factor;
    let ao = 1.0 + material.ao_strength * (ao_tex_color.x - 1.0);
    let emissive =
        emissive_tex_color.xyz() * material.emissive_factor.xyz() * material.emissive_factor.w;
    let irradiance = pbr::sample_irradiance(irradiance, irradiance_sampler, n);
    let camera = slab.read(Id::<Camera>::new(in_camera));
    let specular = pbr::sample_specular_reflection(
        prefiltered,
        prefiltered_sampler,
        camera.position,
        in_pos,
        n,
        roughness,
    );
    let brdf = pbr::sample_brdf(brdf, brdf_sampler, camera.position, in_pos, n, roughness);

    fn colorize(u: Vec3) -> Vec4 {
        ((u.alt_norm_or_zero() + Vec3::splat(1.0)) / 2.0).extend(1.0)
    }

    match debug_mode.into() {
        DebugChannel::None => {}
        DebugChannel::UvCoords0 => {
            *output = colorize(Vec3::new(in_uv0.x, in_uv0.y, 0.0));
            return;
        }
        DebugChannel::UvCoords1 => {
            *output = colorize(Vec3::new(in_uv1.x, in_uv1.y, 0.0));
            return;
        }
        DebugChannel::Normals => {
            *output = colorize(norm);
            return;
        }
        DebugChannel::VertexColor => {
            *output = in_color;
            return;
        }
        DebugChannel::VertexNormals => {
            *output = colorize(in_norm);
            return;
        }
        DebugChannel::UvNormals => {
            *output = colorize(uv_norm);
            return;
        }
        DebugChannel::Tangents => {
            *output = colorize(in_tangent);
            return;
        }
        DebugChannel::Bitangents => {
            *output = colorize(in_bitangent);
            return;
        }
        DebugChannel::DiffuseIrradiance => {
            *output = irradiance.extend(1.0);
            return;
        }
        DebugChannel::SpecularReflection => {
            *output = specular.extend(1.0);
            return;
        }
        DebugChannel::Brdf => {
            *output = brdf.extend(1.0).extend(1.0);
            return;
        }
        DebugChannel::Roughness => {
            *output = Vec3::splat(roughness).extend(1.0);
            return;
        }
        DebugChannel::Metallic => {
            *output = Vec3::splat(metallic).extend(1.0);
            return;
        }
        DebugChannel::Albedo => {
            *output = albedo;
            return;
        }
        DebugChannel::Occlusion => {
            *output = Vec3::splat(ao).extend(1.0);
            return;
        }
        DebugChannel::Emissive => {
            *output = emissive.extend(1.0);
            return;
        }
        DebugChannel::UvEmissive => {
            *output = emissive_tex_color.xyz().extend(1.0);
            return;
        }
        DebugChannel::EmissiveFactor => {
            *output = material.emissive_factor.xyz().extend(1.0);
            return;
        }
        DebugChannel::EmissiveStrength => {
            *output = Vec3::splat(material.emissive_factor.w).extend(1.0);
            return;
        }
    }

    *output = match material.lighting_model {
        LightingModel::PBR_LIGHTING => pbr::stage_shade_fragment(
            camera.position,
            n,
            in_pos,
            albedo.xyz(),
            metallic,
            roughness,
            ao,
            emissive,
            irradiance,
            specular,
            brdf,
            light_array,
            slab,
        ),
        _unlit => in_color * albedo_tex_color * material.albedo_factor,
    };

    // write the brightest colors for the bloom effect
    let brightness_value = output.xyz().dot(Vec3::new(0.2126, 0.7152, 0.0722));
    *brigtness = if brightness_value > 1.0 {
        output.xyz()
    } else {
        Vec3::ZERO
    }
    .extend(1.0);
}

#[spirv(compute(threads(32)))]
/// Compute the draw calls for this frame.
///
/// This should be called with `groupcount = (entities.len() / threads) + 1`.
pub fn compute_cull_entities(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities: &[GpuEntity],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] draws: &mut [DrawIndirect],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let i = global_id.x as usize;

    if i > entities.len() {
        return;
    }

    // when the vertex count and/or instance count is 0, it effectively filters
    // the draw call
    let mut call = DrawIndirect {
        vertex_count: 0,
        instance_count: 0,
        base_vertex: 0,
        base_instance: i as u32,
    };
    let entity = &entities[i];
    let is_visible = entity.visible != 0;
    if entity.is_alive() && is_visible {
        //// once naga supports atomics we can use this to compact the array
        // let index = unsafe {
        //    spirv_std::arch::atomic_i_increment::<
        //        u32,
        //        { spirv_std::memory::Scope::Device as u32 },
        //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
        //    >(count)
        //};
        call.instance_count = 1;
        call.base_vertex = entity.mesh_first_vertex;
        call.vertex_count = entity.mesh_vertex_count;
    }
    draws[i] = call;
}

#[spirv(compute(threads(32)))]
/// A shader to ensure that we can extract i8 and i16 values from a storage buffer.
pub fn test_i8_i16_extraction(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &mut [u32],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let index = global_id.x as usize;
    let (value, _, _) = crate::bits::extract_i8(index, 2, slab);
    if value > 0 {
        slab[index] = value as u32;
    }
    let (value, _, _) = crate::bits::extract_i16(index, 2, slab);
    if value > 0 {
        slab[index] = value as u32;
    }
}

#[cfg(test)]
mod test {
    use crate::{self as renderling_shader, id::Id, slab::Slab};
    use renderling_shader::slab::Slabbed;

    #[derive(Default, Debug, PartialEq, Slabbed)]
    struct TheType {
        a: glam::Vec3,
        b: glam::Vec2,
        c: glam::Vec4,
    }

    #[test]
    fn slabbed_writeread() {
        let mut slab = [0u32; 100];
        let the = TheType {
            a: glam::Vec3::new(0.0, 1.0, 2.0),
            b: glam::Vec2::new(3.0, 4.0),
            c: glam::Vec4::new(5.0, 6.0, 7.0, 8.0),
        };
        let index = slab.write(&the, 0);
        assert_eq!(9, index);
        let the2 = slab.read(Id::<TheType>::new(0));
        assert_eq!(the, the2);
    }
}
