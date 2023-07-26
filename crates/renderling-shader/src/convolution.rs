//! Convolution shaders.
//!
//! These shaders convolve various functions to produce cached maps.
use glam::{Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Cubemap, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

pub fn fragment_convolve_diffuse_irradiance(
    environment_texture: &Cubemap,
    sampler: &Sampler,
    local_pos: Vec3,
    frag_color: &mut Vec4
) {
    let normal = local_pos.normalize_or_zero();
    let mut irradiance = Vec3::ZERO;
    let right = Vec3::Y.cross(normal);
    let up = normal.cross(right);

    let sample_delta = 0.025;
    let mut nr_samples = 0.0;
    let mut phi = 0.0;
    while phi < core::f32::consts::PI * 2.0 {
        phi += sample_delta;
        let mut theta = 0.0;
        while theta < core::f32::consts::FRAC_2_PI {
            theta += sample_delta;
            // spherical to cartisian tangent coords
            let tangent_sample = Vec3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos());
            // tangent to world coords
            let sample_vec = tangent_sample.x * right + tangent_sample.y * up + tangent_sample.z * normal;

            let sample = environment_texture.sample_by_lod(*sampler, sample_vec, 0.0) * theta.cos() * theta.sin();
            irradiance += sample.xyz();
            nr_samples += 1.0;
        }
    }
    *frag_color = (core::f32::consts::PI * irradiance * (1.0/ nr_samples)).extend(1.0);
}
