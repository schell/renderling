//! Types used to store and update an entire 3d scene on the GPU.
//!
//! This is roughly what the [vulkan guide](https://vkguide.dev/docs/gpudriven)
//! calls "gpu driven rendering".
//!
//! To read more about the technique, check out these resources:
//! * https://stackoverflow.com/questions/59686151/what-is-gpu-driven-rendering
use glam::{mat3, Mat4, Quat, UVec2, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::*;

use crate::{
    bits::{bits, pack, unpack},
    debug::*,
    math::Vec3ColorSwizzles,
    pbr, GpuToggles,
};

mod id;
pub use id::*;

mod texture;
pub use texture::*;

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4,
    // Ids that point to this vertex's joints by indexing into the [GpuEntity] buffer
    pub joints: [Id<GpuEntity>; 4],
    // The weights of influence that each joint has over this vertex
    pub weights: [f32; 4],
}

impl Default for GpuVertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
            tangent: Vec4::Y,
            joints: [Id::NONE; 4],
            weights: [0.0; 4],
        }
    }
}

impl GpuVertex {
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
    pub fn get_joint_matrix(&self, i: usize, entities: &[GpuEntity]) -> Mat4 {
        if i > self.joints.len() - 1 {
            return Mat4::IDENTITY;
        }
        let joint_id = self.joints[i];
        if joint_id.is_none() {
            return Mat4::IDENTITY;
        }
        let entity_index = joint_id.index();
        if entity_index > entities.len() - 1 {
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
    pub fn get_skin_matrix(&self, entities: &[GpuEntity]) -> Mat4 {
        let mut mat = Mat4::ZERO;
        for i in 0..self.joints.len() {
            mat += self.weights[i] * self.get_joint_matrix(i, entities);
        }
        if mat == Mat4::ZERO {
            return Mat4::IDENTITY;
        }
        mat
    }
}

#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightType(u32);

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Display for LightType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            &Self::END_OF_LIGHTS => "end of lights",
            &Self::POINT_LIGHT => "point light",
            &Self::SPOT_LIGHT => "spot light",
            &Self::DIRECTIONAL_LIGHT => "directional light",
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
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
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
    Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug, bytemuck::Pod, bytemuck::Zeroable,
)]
pub struct LightingModel(u32);

impl LightingModel {
    pub const NO_LIGHTING: Self = LightingModel(0);
    pub const PBR_LIGHTING: Self = LightingModel(1);
}

/// Represents a material on the GPU.
///
/// `GpuMaterial` is capable of representing many material types.
/// Use the appropriate builder for your material type from
/// [`SceneBuilder`](crate::SceneBuilder).
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuMaterial {
    pub factor0: Vec4,
    pub factor1: Vec4,

    pub texture0: Id<GpuTexture>,
    pub texture1: Id<GpuTexture>,
    pub texture2: Id<GpuTexture>,
    pub texture3: Id<GpuTexture>,

    pub texture0_tex_coord: u32,
    pub texture1_tex_coord: u32,
    pub texture2_tex_coord: u32,
    pub texture3_tex_coord: u32,

    pub lighting_model: LightingModel,
    pub padding: [u32; 3],
}

impl Default for GpuMaterial {
    fn default() -> Self {
        Self {
            factor0: Vec4::ONE,
            factor1: Vec4::ONE,
            texture0: Id::NONE,
            texture1: Id::NONE,
            texture2: Id::NONE,
            texture3: Id::NONE,
            texture0_tex_coord: 0,
            texture1_tex_coord: 0,
            texture2_tex_coord: 0,
            texture3_tex_coord: 0,
            lighting_model: LightingModel::NO_LIGHTING,
            padding: [0; 3],
        }
    }
}

/// Provides information about what attributes are provided by a morph target.
///
/// In `renderling` morph targets' geometry are meshlets - they are assumed to
/// come directly after the base mesh in the `GpuVertex` buffer, that way they
/// can be indexed from the base mesh using only the `GpuEntity` struct (through
/// `MorphTargetsInfo::num_targets`). See [`GpuEntity::get_vertex`] for more
/// info.
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MorphTargetsInfo(u32);

impl MorphTargetsInfo {
    const BITS_NUM_TARGETS: (u32, u32) = bits(0..=7);
    const BITS_HAS_POSITIONS: (u32, u32) = bits(8..=8);
    const BITS_HAS_NORMALS: (u32, u32) = bits(9..=9);
    const BITS_HAS_TANGENTS: (u32, u32) = bits(10..=10);

    pub fn num_targets(&self) -> u32 {
        unpack(self.0, Self::BITS_NUM_TARGETS)
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn set_num_targets(&mut self, num_targets: u8) {
        pack(&mut self.0, Self::BITS_NUM_TARGETS, num_targets as u32)
    }

    pub fn has_positions(&self) -> bool {
        unpack(self.0, Self::BITS_HAS_POSITIONS) == 1
    }

    pub fn set_has_positions(&mut self, has: bool) {
        pack(
            &mut self.0,
            Self::BITS_HAS_POSITIONS,
            if has { 1 } else { 0 },
        )
    }

    pub fn has_normals(&self) -> bool {
        unpack(self.0, Self::BITS_HAS_NORMALS) == 1
    }

    pub fn set_has_normals(&mut self, has: bool) {
        pack(&mut self.0, Self::BITS_HAS_NORMALS, if has { 1 } else { 0 })
    }

    pub fn has_tangents(&self) -> bool {
        unpack(self.0, Self::BITS_HAS_TANGENTS) == 1
    }

    pub fn set_has_tangents(&mut self, has: bool) {
        pack(
            &mut self.0,
            Self::BITS_HAS_TANGENTS,
            if has { 1 } else { 0 },
        )
    }
}

/// A bundle of GPU components.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuEntity {
    // The id of this entity. `Id::NONE` means this entity is not in use.
    pub id: Id<GpuEntity>,
    // The index of the first vertex in this entity's mesh.
    pub mesh_first_vertex: u32,
    // The number of vertices in this entity's mesh.
    pub mesh_vertex_count: u32,
    // Info about mesh morph targets
    pub morph_targets_info: MorphTargetsInfo,
    pub morph_targets_weights: [f32; 8],
    //
    pub padding: u32,
    // The index/id of this entity's material in the material buffer.
    pub material: Id<GpuMaterial>,
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
}

impl Default for GpuEntity {
    fn default() -> Self {
        Self {
            id: Id::NONE,
            mesh_first_vertex: 0,
            mesh_vertex_count: 0,
            morph_targets_info: MorphTargetsInfo::default(),
            morph_targets_weights: [0.0; 8],
            padding: 0,
            material: Id::NONE,
            position: Vec4::ZERO,
            scale: Vec4::ONE,
            rotation: Quat::IDENTITY,
            visible: 1,
            parent: Id::NONE,
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
        let mut position = Vec3::ZERO;
        let mut scale = Vec3::ONE;
        let mut rotation = Quat::IDENTITY;
        let mut id = self.id;
        loop {
            let entity = entities[id.index()];
            position += entity.position.xyz();
            scale *= entity.scale.xyz();
            rotation = entity.rotation * rotation;
            id = entity.parent;
            if id.index() >= entities.len() {
                break;
            }
        }
        (position, rotation, scale)
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
    /// reference.
    pub fn get_vertex(&self, vertex_index: u32, entities: &[GpuEntity], vertices: &[GpuVertex]) -> GpuVertex {
        let index = vertex_index as usize;
        let mut vertex = vertices[index];

        // Apply target morphing
        let targets = self.morph_targets_info.num_targets() as usize;
        let vertex_count = self.mesh_vertex_count as usize;
        for i in 1..=targets {
            let target_weight = self.morph_targets_weights[i - 1];
            let target_index = index + (i * vertex_count);
            let target = vertices[target_index];
            vertex.position += (target_weight * target.position.xyz()).extend(0.0);
            vertex.normal += (target_weight * target.normal.xyz()).extend(0.0);
            vertex.tangent += (target_weight * target.tangent.xyz()).extend(0.0);
        }

        // Apply skinning without assuming anything about the `w` components
        let skin_matrix = vertex.get_skin_matrix(entities);
        let position = skin_matrix.transform_point3(vertex.position.xyz());
        vertex.position.x = position.x;
        vertex.position.y = position.y;
        vertex.position.z = position.z;
        let normal = skin_matrix.transform_point3(vertex.normal.xyz());
        vertex.normal.x = normal.x;
        vertex.normal.y = normal.y;
        vertex.normal.z = normal.z;
        let tangent = skin_matrix.transform_point3(vertex.tangent.xyz());
        vertex.tangent.x = tangent.x;
        vertex.tangent.y = tangent.y;
        vertex.tangent.z = tangent.z;

        vertex
    }
}

/// Unforms/constants for a scene's worth of rendering.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuConstants {
    pub camera_projection: Mat4,
    pub camera_view: Mat4,
    pub camera_pos: Vec4,
    pub atlas_size: UVec2,
    pub debug_mode: DebugMode,
    pub toggles: GpuToggles,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DrawIndirect {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

fn texture_color(
    texture_id: Id<GpuTexture>,
    uv: Vec2,
    atlas: &Image2d,
    sampler: &Sampler,
    atlas_size: UVec2,
    textures: &[GpuTexture],
) -> Vec4 {
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

/// Scene vertex shader.
pub fn main_vertex_scene(
    // which entity are we drawing
    instance_index: u32,
    // which vertex are we drawing
    vertex_index: u32,

    constants: &GpuConstants,
    vertices: &[GpuVertex],
    entities: &[GpuEntity],

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
    let entity = entities[instance_index as usize];
    let vertex = entity.get_vertex(vertex_index, entities, vertices);
    let (position, rotation, scale) = entity.get_world_transform(entities);
    let model_matrix =
        Mat4::from_translation(position) * Mat4::from_quat(rotation) * Mat4::from_scale(scale);
    *out_material = entity.material.into();
    *out_color = vertex.color;
    *out_uv0 = vertex.uv.xy();
    *out_uv1 = vertex.uv.zw();

    let scale2 = scale * scale;
    let normal = vertex.normal.xyz().normalize_or_zero();
    let tangent = vertex.tangent.xyz().normalize_or_zero();
    let normal_w = (model_matrix * (normal / scale2).extend(0.0))
        .xyz()
        .normalize_or_zero();
    let tangent_w = (model_matrix * tangent.extend(0.0))
        .xyz()
        .normalize_or_zero();
    let bitangent_w = normal_w.cross(tangent_w) * vertex.tangent.w.signum();
    *out_tangent = tangent_w;
    *out_bitangent = bitangent_w;
    *out_norm = normal_w;

    let view_pos = model_matrix * vertex.position.xyz().extend(1.0);
    *out_pos = view_pos.xyz();
    *gl_pos = constants.camera_projection * constants.camera_view * view_pos;
}

/// Scene fragment shader.
pub fn main_fragment_scene(
    atlas: &Image2d,
    sampler: &Sampler,

    constants: &GpuConstants,
    lights: &[GpuLight],
    materials: &[GpuMaterial],
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
) {
    let material = if in_material == ID_NONE {
        GpuMaterial::default()
    } else {
        materials[in_material as usize]
    };

    let texture0_uv = if material.texture0_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let tex_color0 = texture_color(
        material.texture0,
        texture0_uv,
        atlas,
        sampler,
        constants.atlas_size,
        textures,
    );

    let texture1_uv = if material.texture1_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let tex_color1 = texture_color(
        material.texture1,
        texture1_uv,
        atlas,
        sampler,
        constants.atlas_size,
        textures,
    );

    let texture2_uv = if material.texture2_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let tex_color2 = texture_color(
        material.texture2,
        texture2_uv,
        atlas,
        sampler,
        constants.atlas_size,
        textures,
    );

    let (norm, uv_norm) = if material.texture2.is_none() {
        // there is no normal map, use the normal normal ;)
        (in_norm, Vec3::ZERO)
    } else {
        // convert the normal from color coords to tangent space -1,1
        let sampled_norm = (tex_color2.xyz() * 2.0 - Vec3::splat(1.0)).normalize_or_zero();
        let tbn = mat3(
            in_tangent.normalize_or_zero(),
            in_bitangent.normalize_or_zero(),
            in_norm.normalize_or_zero(),
        );
        // convert the normal from tangent space to world space
        let norm = (tbn * sampled_norm).normalize_or_zero();
        (norm, sampled_norm)
    };

    fn colorize(u: Vec3) -> Vec4 {
        ((u.normalize_or_zero() + Vec3::splat(1.0)) / 2.0).extend(1.0)
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
    }

    *output = match material.lighting_model {
        LightingModel::PBR_LIGHTING => {
            let albedo = tex_color0 * material.factor0 * in_color;
            let metallic = tex_color1.y * material.factor1.y;
            let roughness = tex_color1.z * material.factor1.z;
            let ao = 1.0;
            pbr::shade_fragment(
                constants.camera_pos.xyz(),
                norm,
                in_pos,
                albedo.rgb(),
                metallic,
                roughness,
                ao,
                lights,
            )
        }
        _unlit => in_color * tex_color0 * material.factor0 * tex_color1,
    };
}

/// Compute the draw calls for this frame.
///
/// This should be called with `groupcount = (entities.len() / threads) + 1`.
pub fn compute_cull_entities(entities: &[GpuEntity], draws: &mut [DrawIndirect], global_id: UVec3) {
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
