//! Cubemap utilities.
//!
//! Shaders, render pipelines and layouts for creating and sampling cubemaps.
use crabslab::{Array, Id, Slab};
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4};
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

/// Represents one side of a cubemap.
///
/// Assumes the camera is at the origin, inside the cube, with
/// a left-handed coordinate system (+Z going into the screen).
pub struct CubemapFaceDirection {
    /// Where is the camera
    pub eye: Vec3,
    /// Where is the camera looking
    pub dir: Vec3,
    /// Which direction is up
    pub up: Vec3,
}

impl CubemapFaceDirection {
    pub const X: Self = Self {
        eye: Vec3::ZERO,
        dir: Vec3::X,
        up: Vec3::Y,
    };
    pub const NEG_X: Self = Self {
        eye: Vec3::ZERO,
        dir: Vec3::NEG_X,
        up: Vec3::Y,
    };

    pub const Y: Self = Self {
        eye: Vec3::ZERO,
        dir: Vec3::Y,
        up: Vec3::NEG_Z,
    };
    pub const NEG_Y: Self = Self {
        eye: Vec3::ZERO,
        dir: Vec3::NEG_Y,
        up: Vec3::Z,
    };

    pub const Z: Self = Self {
        eye: Vec3::ZERO,
        dir: Vec3::Z,
        up: Vec3::Y,
    };
    pub const NEG_Z: Self = Self {
        eye: Vec3::ZERO,
        dir: Vec3::NEG_Z,
        up: Vec3::Y,
    };

    pub const FACES: [Self; 6] = [
        CubemapFaceDirection::X,
        CubemapFaceDirection::NEG_X,
        CubemapFaceDirection::Y,
        CubemapFaceDirection::NEG_Y,
        CubemapFaceDirection::Z,
        CubemapFaceDirection::NEG_Z,
    ];

    pub fn right(&self) -> Vec3 {
        -self.dir.cross(self.up)
    }

    /// The view from _inside_ the cube.
    pub fn view(&self) -> Mat4 {
        Mat4::look_at_lh(self.eye, self.eye + self.dir, self.up)
    }
}

pub struct CubemapDescriptor {
    atlas_descriptor_id: Id<AtlasDescriptor>,
    faces: Array<AtlasTexture>,
}

impl CubemapDescriptor {
    /// Return the face index and UV coordinates that can be used to sample
    /// a cubemap from the given directional coordinate.
    pub fn get_face_index_and_uv(coord: Vec3) -> (usize, Vec2) {
        // A plane is the normal of the plane and its distance from the origin
        const X_PLANE: Vec4 = Vec4::new(-1.0, 0.0, 0.0, 1.0);
        const NEG_X_PLANE: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
        const Y_PLANE: Vec4 = Vec4::new(0.0, -1.0, 0.0, 1.0);
        const NEG_Y_PLANE: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
        const Z_PLANE: Vec4 = Vec4::new(0.0, 0.0, -1.0, 1.0);
        const NEG_Z_PLANE: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);

        {
            let (intersects, intersection) =
                crate::math::intersect_plane_with_ray_from_origin(X_PLANE, coord);
            if intersects {
                let uv = (Vec2::new(-intersection.z, -intersection.y) + 1.0) / 2.0;
                return (0, uv);
            }
        }

        {
            let (intersects, intersection) =
                crate::math::intersect_plane_with_ray_from_origin(NEG_X_PLANE, coord);
            if intersects {
                let uv = (Vec2::new(intersection.z, -intersection.y) + 1.0) / 2.0;
                return (1, uv);
            }
        }

        {
            let (intersects, intersection) =
                crate::math::intersect_plane_with_ray_from_origin(Y_PLANE, coord);
            if intersects {
                let uv = (Vec2::new(intersection.x, intersection.z) + 1.0) / 2.0;
                return (2, uv);
            }
        }

        {
            let (intersects, intersection) =
                crate::math::intersect_plane_with_ray_from_origin(NEG_Y_PLANE, coord);
            if intersects {
                let uv = (Vec2::new(intersection.x, -intersection.z) + 1.0) / 2.0;
                return (3, uv);
            }
        }

        {
            let (intersects, intersection) =
                crate::math::intersect_plane_with_ray_from_origin(Z_PLANE, coord);
            if intersects {
                let uv = (Vec2::new(intersection.x, -intersection.y) + 1.0) / 2.0;
                return (4, uv);
            }
        }

        {
            let (intersects, intersection) =
                crate::math::intersect_plane_with_ray_from_origin(NEG_Z_PLANE, coord);
            if intersects {
                let uv = (Vec2::new(-intersection.x, -intersection.y) + 1.0) / 2.0;
                return (5, uv);
            }
        }

        (0, Vec2::new(0.5, 0.5))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cubemap_right() {
        assert_eq!(Vec3::NEG_Z, CubemapFaceDirection::X.right());
        assert_eq!(Vec3::Z, CubemapFaceDirection::NEG_X.right());
        assert_eq!(Vec3::X, CubemapFaceDirection::Y.right());
        assert_eq!(Vec3::X, CubemapFaceDirection::NEG_Y.right());
        assert_eq!(Vec3::X, CubemapFaceDirection::Z.right());
        assert_eq!(Vec3::NEG_X, CubemapFaceDirection::NEG_Z.right());

        assert_eq!(
            (1, Vec2::new(0.0, 1.0)),
            CubemapDescriptor::get_face_index_and_uv(Vec3::NEG_ONE)
        );
    }
}
