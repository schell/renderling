//! Holds geometry on CPU and GPU.
use crabslab::{Array, Id, Slab, SlabItem};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;
use glam::{Mat4, Vec2, Vec3, Vec4};

use crate::{
    bvol::BoundingSphere, camera::Camera, math::IsVector, pbr::Material, transform::Transform,
};

/// A vertex skin.
///
/// For more info on vertex skinning, see
/// <https://github.khronos.org/glTF-Tutorials/gltfTutorial/gltfTutorial_019_SimpleSkin.html>
#[derive(Clone, Copy, Default, SlabItem)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct Skin {
    // Ids of the skeleton nodes' global transforms used as joints in this skin.
    pub joints: Array<Id<Transform>>,
    // Contains the 4x4 inverse-bind matrices.
    //
    // When is none, each matrix is assumed to be the 4x4 identity matrix
    // which implies that the inverse-bind matrices were pre-applied.
    pub inverse_bind_matrices: Array<Mat4>,
}

impl Skin {
    pub fn get_joint_matrix(&self, i: usize, vertex: Vertex, slab: &[u32]) -> Mat4 {
        let joint_index = vertex.joints[i] as usize;
        let joint_id = slab.read(self.joints.at(joint_index));
        let joint_transform = slab.read(joint_id);
        // First apply the inverse bind matrix to bring the vertex into the joint's
        // local space, then apply the joint's current transformation to move it
        // into world space.
        let inverse_bind_matrix = slab.read(self.inverse_bind_matrices.at(joint_index));
        Mat4::from(joint_transform) * inverse_bind_matrix
    }

    pub fn get_skinning_matrix(&self, vertex: Vertex, slab: &[u32]) -> Mat4 {
        let mut skinning_matrix = Mat4::ZERO;
        for i in 0..vertex.joints.len() {
            let joint_matrix = self.get_joint_matrix(i, vertex, slab);
            // Ensure weights are applied correctly to the joint matrix
            let weight = vertex.weights[i];
            skinning_matrix += weight * joint_matrix;
        }

        if skinning_matrix == Mat4::ZERO {
            Mat4::IDENTITY
        } else {
            skinning_matrix
        }
    }
}

/// A displacement target.
///
/// Use to displace vertices using weights defined on the mesh.
///
/// For more info on morph targets, see
/// <https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#morph-targets>
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
#[cfg_attr(cpu, derive(Debug))]
pub struct MorphTarget {
    pub position: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    // TODO: Extend MorphTargets to include UV and Color.
    // I think this would take a contribution to the `gltf` crate.
}

/// A vertex in a mesh.
#[derive(Clone, Copy, core::fmt::Debug, PartialEq, SlabItem)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
    pub uv0: Vec2,
    pub uv1: Vec2,
    pub normal: Vec3,
    pub tangent: Vec4,
    // Indices that point to this vertex's 'joint' transforms.
    pub joints: [u32; 4],
    // The weights of influence that each joint has over this vertex
    pub weights: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::ONE,
            uv0: Vec2::ZERO,
            uv1: Vec2::ZERO,
            normal: Vec3::Z,
            tangent: Vec4::Y,
            joints: [0; 4],
            weights: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn with_position(mut self, p: impl Into<Vec3>) -> Self {
        self.position = p.into();
        self
    }

    pub fn with_color(mut self, c: impl Into<Vec4>) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_uv0(mut self, uv: impl Into<Vec2>) -> Self {
        self.uv0 = uv.into();
        self
    }

    pub fn with_uv1(mut self, uv: impl Into<Vec2>) -> Self {
        self.uv1 = uv.into();
        self
    }

    pub fn with_normal(mut self, n: impl Into<Vec3>) -> Self {
        self.normal = n.into();
        self
    }

    pub fn with_tangent(mut self, t: impl Into<Vec4>) -> Self {
        self.tangent = t.into();
        self
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

    #[cfg(cpu)]
    /// A triangle list mesh of points.
    pub fn cube_mesh() -> [Vertex; 36] {
        let mut mesh = [Vertex::default(); 36];
        let unit_cube = crate::math::unit_cube();
        debug_assert_eq!(36, unit_cube.len());
        for (i, (position, normal)) in unit_cube.into_iter().enumerate() {
            mesh[i].position = position;
            mesh[i].normal = normal;
        }
        mesh
    }
}

/// Holds configuration info for vertex and shading render passes of
/// geometry.
///
/// This descriptor lives at the root (index 0) of the geometry slab.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
#[offsets]
pub struct GeometryDescriptor {
    pub camera_id: Id<Camera>,
    pub atlas_size: glam::UVec2,
    pub resolution: glam::UVec2,
    pub debug_channel: crate::pbr::debug::DebugChannel,
    pub has_lighting: bool,
    pub has_skinning: bool,
    pub perform_frustum_culling: bool,
    pub perform_occlusion_culling: bool,
}

impl Default for GeometryDescriptor {
    fn default() -> Self {
        Self {
            camera_id: Id::NONE,
            atlas_size: Default::default(),
            resolution: glam::UVec2::ONE,
            debug_channel: Default::default(),
            has_lighting: true,
            has_skinning: true,
            perform_frustum_culling: true,
            perform_occlusion_culling: false,
        }
    }
}
