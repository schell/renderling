use crabslab::{Array, Id, Slab, SlabItem};
use glam::Mat4;

use crate::{
    camera::shader::CameraDescriptor, geometry::Vertex, transform::shader::TransformDescriptor,
};

/// A vertex skin descriptor.
///
/// For more info on vertex skinning, see
/// <https://github.khronos.org/glTF-Tutorials/gltfTutorial/gltfTutorial_019_SimpleSkin.html>
#[derive(Clone, Copy, Default, SlabItem)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct SkinDescriptor {
    // Ids of the skeleton nodes' global transforms used as joints in this skin.
    pub joints_array: Array<Id<TransformDescriptor>>,
    // Contains the 4x4 inverse-bind matrices.
    //
    // When is none, each matrix is assumed to be the 4x4 identity matrix
    // which implies that the inverse-bind matrices were pre-applied.
    pub inverse_bind_matrices_array: Array<Mat4>,
}

impl SkinDescriptor {
    pub fn get_joint_matrix(&self, i: usize, vertex: Vertex, slab: &[u32]) -> Mat4 {
        let joint_index = vertex.joints[i] as usize;
        let joint_id = slab.read(self.joints_array.at(joint_index));
        let joint_transform = slab.read(joint_id);
        // First apply the inverse bind matrix to bring the vertex into the joint's
        // local space, then apply the joint's current transformation to move it
        // into world space.
        let inverse_bind_matrix = slab.read(self.inverse_bind_matrices_array.at(joint_index));
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

/// Holds configuration info for vertex and shading render passes of
/// geometry.
///
/// This descriptor lives at the root (index 0) of the geometry slab.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
#[offsets]
pub struct GeometryDescriptor {
    pub camera_id: Id<CameraDescriptor>,
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
