//! Cubemap utilities.
//!
//! Shaders, render pipelines and layouts for creating and sampling cubemaps.
use crabslab::{Array, Id, Slab};
use glam::{Vec2, Vec3, Vec3Swizzles, Vec4};
use spirv_std::{num_traits::Zero, spirv};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

use crate::{
    atlas::{AtlasDescriptor, AtlasTexture},
    math::{IsSampler, Sample2dArray},
};

/// Vertex shader for testing cubemap sampling.
#[spirv(vertex)]
pub fn cubemap_sampling_test_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] uv: &Vec3,
    out_uv: &mut Vec3,
    #[spirv(position)] out_clip_coords: &mut Vec4,
) {
    let vertex_index = vertex_index as usize % 6;
    *out_clip_coords = crate::math::CLIP_SPACE_COORD_QUAD_CCW[vertex_index];
    *out_uv = *uv;
}

/// Vertex shader for testing cubemap sampling.
#[spirv(fragment)]
pub fn cubemap_sampling_test_fragment(
    #[spirv(descriptor_set = 0, binding = 1)] cubemap: &spirv_std::image::Cubemap,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &spirv_std::Sampler,
    in_uv: Vec3,
    frag_color: &mut Vec4,
) {
    *frag_color = cubemap.sample(*sampler, in_uv);
}

pub struct CubemapDescriptor {
    atlas_descriptor_id: Id<AtlasDescriptor>,
    faces: Array<AtlasTexture>,
}

impl CubemapDescriptor {
    /// Return the face index and UV coordinates that can be used to sample
    /// a cubemap from the given directional coordinate.
    ///
    /// `coord` must be normalized, and must **not be zero**.
    pub fn get_face_index_and_uv(coord: Vec3) -> (usize, Vec2) {
        // Unpack coordinate components
        let x = coord.x;
        let y = coord.y;
        let z = coord.z;

        // Determine the major direction
        let abs_x = x.abs();
        let abs_y = y.abs();
        let abs_z = z.abs();

        // Initialize face index and UV coordinates
        let (face_index, u, v);

        if abs_x >= abs_y && abs_x >= abs_z {
            // X-major direction
            if x > 0.0 {
                face_index = 0; // +X
                u = -z / x;
                v = -y / x;
            } else {
                face_index = 1; // -X
                u = -z / x;
                v = y / x;
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            // Y-major direction
            if y > 0.0 {
                face_index = 2; // +Y
                u = x / y;
                v = z / y;
            } else {
                face_index = 3; // -Y
                u = x / y;
                v = -z / y;
            }
        } else {
            // Z-major direction
            if z > 0.0 {
                face_index = 4; // +Z
                u = x / z;
                v = -y / z;
            } else {
                face_index = 5; // -Z
                u = -x / z;
                v = -y / z;
            }
        }

        // Convert from range [-1, 1] to [0, 1]
        let u = (u + 1.0) / 2.0;
        let v = (v + 1.0) / 2.0;

        (face_index, Vec2::new(u, v))
    }

    /// Sample the cubemap with a directional coordinate.
    pub fn sample<A, S>(&self, coord: Vec3, slab: &[u32], atlas: &A, sampler: &S) -> Vec4
    where
        A: Sample2dArray<Sampler = S>,
        S: IsSampler,
    {
        let coord = if coord.length().is_zero() {
            Vec3::X
        } else {
            coord.normalize()
        };
        let (face_index, uv) = Self::get_face_index_and_uv(coord);
        let atlas_image = slab.read_unchecked(self.faces.at(face_index));
        let atlas_desc = slab.read_unchecked(self.atlas_descriptor_id);
        let uv = atlas_image.uv(uv, atlas_desc.size.xy());
        atlas.sample_by_lod(*sampler, uv, 0.0)
    }
}
