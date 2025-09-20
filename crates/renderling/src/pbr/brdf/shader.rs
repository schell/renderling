//! Shader side of BRDF stuff.

use glam::{Vec2, Vec3, Vec4Swizzles};

use crate::math::{IsSampler, IsVector, Sample2d};

pub fn sample_brdf<T: Sample2d<Sampler = S>, S: IsSampler>(
    brdf: &T,
    brdf_sampler: &S,
    // camera position in world space
    camera_pos: Vec3,
    // fragment position in world space
    in_pos: Vec3,
    // normal vector
    n: Vec3,
    roughness: f32,
) -> Vec2 {
    let v = (camera_pos - in_pos).alt_norm_or_zero();
    brdf.sample_by_lod(*brdf_sampler, Vec2::new(n.dot(v).max(0.0), roughness), 0.0)
        .xy()
}
