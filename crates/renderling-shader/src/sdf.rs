//! SDF shaders
use crabslab::{Id, Slab, SlabItem};
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::{
    pbr::Material,
    stage::{Camera, Transform, Vertex},
    IsSampler, IsVector, Sample2d, SampleCube,
};

#[derive(Debug, Clone, Copy, SlabItem)]
pub struct Circle {
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

impl Circle {
    pub fn signed_distance(&self, sample_position: Vec2) -> f32 {
        sample_position.length() - self.radius
    }

    /// Returns the vertex at the given index.
    ///
    /// This [`Vertex`] is one of the points of the quad that makes up a unit square.
    /// It should be transformed by the [`Transform`] of the [`Sdf`] that owns this [`Circle`].
    pub fn get_vertex(&self, index: usize) -> Vertex {
        Vertex::default().with_position(crate::math::CLIP_SPACE_COORD_QUAD_CCW[index % 6].xyz())
    }
}

#[derive(Default, Debug, Clone, Copy, SlabItem)]
pub enum SdfShape {
    #[default]
    None,
    Circle(Id<Circle>),
}

impl SdfShape {
    pub fn get_vertex(&self, index: usize, slab: &[u32]) -> Vertex {
        match self {
            Self::None => Vertex::default(),
            Self::Circle(id) => slab.read(*id).get_vertex(index),
        }
    }
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Sdf {
    pub shape: SdfShape,
    pub transform: Transform,
    pub material: Id<Material>,
    pub camera: Id<Camera>,
}

pub fn vertex(
    sdf_id: Id<Sdf>,
    // Which vertex within the render unit are we rendering
    vertex_index: u32,
    slab: &[u32],
    out_camera: &mut u32,
    out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,
    clip_pos: &mut Vec4,
) {
    let Sdf {
        shape,
        transform,
        material: material_id,
        camera: camera_id,
    } = slab.read(sdf_id);

    let vertex = shape.get_vertex(vertex_index as usize, slab);
    let model_matrix = Mat4::from_scale_rotation_translation(
        transform.scale,
        transform.rotation,
        transform.translation,
    );
    let scale2 = transform.scale * transform.scale;
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
    let camera = slab.read(camera_id);
    *out_camera = camera_id.into();
    *clip_pos = camera.projection * camera.view * view_pos;

    *out_material = material_id.into();
    *out_color = Vec4::ONE;
    *out_uv0 = Vec2::ZERO;
    *out_uv1 = Vec2::ZERO;
}

pub fn fragment<T, C, S>(
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
) where
    T: Sample2d<Sampler = S>,
    C: SampleCube<Sampler = S>,
    S: IsSampler,
{
    crate::pbr::fragment(
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
        in_color * Vec4::new(1.0, 0.0, 0.0, 1.0),
        in_uv0,
        in_uv1,
        in_norm,
        in_tangent,
        in_bitangent,
        in_pos,
        output,
    );
}
