//! Shader side of IBL
use glam::{Vec3, Vec4, Vec4Swizzles};
#[cfg(gpu)]
use spirv_std::num_traits::Float;
use spirv_std::{image::Cubemap, spirv, Sampler};

use crate::math::IsVector;

/// Diffuse irradiance convolution.
#[spirv(fragment)]
pub fn di_convolution_fragment(
    #[spirv(descriptor_set = 0, binding = 1)] environment_texture: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 2)] environment_sampler: &Sampler,
    local_pos: Vec3,
    frag_color: &mut Vec4,
) {
    let normal = {
        let mut n = local_pos.alt_norm_or_zero();
        n.y *= -1.0;
        n
    };
    let mut irradiance = Vec3::ZERO;
    let right = Vec3::Y.cross(normal).alt_norm_or_zero();
    let up = normal.cross(right).alt_norm_or_zero();

    let sample_delta = 0.025;
    let mut nr_samples = 0.0;
    let mut phi = 0.0f32;
    while phi < 2.0 * core::f32::consts::PI {
        let mut theta = 0.0f32;
        while theta < core::f32::consts::FRAC_PI_2 {
            // spherical to cartisian tangent coords
            let tangent_sample = Vec3::new(
                theta.sin() * phi.cos(),
                theta.sin() * phi.sin(),
                theta.cos(),
            );
            // tangent to world coords
            let sample_vec =
                (tangent_sample.x * right + tangent_sample.y * up + tangent_sample.z * normal)
                    .alt_norm_or_zero();
            let sample = environment_texture.sample(*environment_sampler, sample_vec)
                * theta.cos()
                * theta.sin();
            irradiance += sample.xyz();
            nr_samples += 1.0;

            theta += sample_delta;
        }
        phi += sample_delta
    }

    *frag_color = (irradiance * (core::f32::consts::PI / nr_samples)).extend(1.0);
}
