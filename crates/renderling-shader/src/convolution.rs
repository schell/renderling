//! Convolution shaders.
//!
//! These shaders convolve various functions to produce cached maps.
use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{num_traits::Zero, image::Cubemap, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::scene::GpuConstants;

fn radical_inverse_vdc(mut bits: u32) -> f32 {
    bits = (bits << 16u32) | (bits >> 16u32);
    bits = ((bits & 0x55555555u32) << 1u32) | ((bits & 0xAAAAAAAAu32) >> 1u32);
    bits = ((bits & 0x33333333u32) << 2u32) | ((bits & 0xCCCCCCCCu32) >> 2u32);
    bits = ((bits & 0x0F0F0F0Fu32) << 4u32) | ((bits & 0xF0F0F0F0u32) >> 4u32);
    bits = ((bits & 0x00FF00FFu32) << 8u32) | ((bits & 0xFF00FF00u32) >> 8u32);
    (bits as f32) * 2.3283064365386963e-10 // / 0x100000000
}

fn hammersley(i: u32, n: u32) -> Vec2 {
    Vec2::new(i as f32 / n as f32, radical_inverse_vdc(i))
}

fn importance_sample_ggx(xi: Vec2, n: Vec3, roughness: f32) -> Vec3 {
    let a = roughness * roughness;

    let phi = 2.0 * core::f32::consts::PI * xi.x;
    let cos_theta = f32::sqrt((1.0 - xi.y) / (1.0 + (a * a - 1.0) * xi.y));
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

    // Convert spherical to cartesian coordinates
    let h = Vec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta);

    // Convert tangent-space vector to world-space vector
    let up = if n.z.abs() < 0.999 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let tangent = up.cross(n).normalize_or_zero();
    let bitangent = n.cross(tangent);

    let result = tangent * h.x + bitangent * h.y + n * h.z;
    result.normalize_or_zero()
}

fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let r = roughness;
    let k = (r * r) / 2.0;

    let nom = n_dot_v;
    let denom = n_dot_v * (1.0 - k) + k;

    if denom.is_zero() {
        0.0
    } else {
        nom / denom
    }
}

fn geometry_smith(normal: Vec3, view_dir: Vec3, light_dir: Vec3, roughness: f32) -> f32 {
    let n_dot_v = normal.dot(view_dir).max(0.0);
    let n_dot_l = normal.dot(light_dir).max(0.0);
    let ggx1 = geometry_schlick_ggx(n_dot_v, roughness);
    let ggx2 = geometry_schlick_ggx(n_dot_l, roughness);

    ggx1 * ggx2
}

const SAMPLE_COUNT: u32 = 1024;

pub fn integrate_brdf(mut n_dot_v: f32, roughness: f32) -> Vec2 {
    n_dot_v = n_dot_v.max(f32::EPSILON);
    let v = Vec3::new(f32::sqrt(1.0 - n_dot_v * n_dot_v), 0.0, n_dot_v);

    let mut a = 0.0f32;
    let mut b = 0.0f32;

    let n = Vec3::Z;

    for i in 1..SAMPLE_COUNT {
        let xi = hammersley(i, SAMPLE_COUNT);
        let h = importance_sample_ggx(xi, n, roughness);
        let l = (2.0 * v.dot(h) * h - v).normalize_or_zero();

        let n_dot_l = l.z.max(0.0);
        let n_dot_h = h.z.max(0.0);
        let v_dot_h = v.dot(h).max(0.0);

        if n_dot_l > 0.0 {
            let g = geometry_smith(n, v, l, roughness);
            let denom = n_dot_h * n_dot_v;
            let g_vis = (g * v_dot_h) / denom;
            let f_c = (1.0 - v_dot_h).powf(5.0);

            a += (1.0 - f_c) * g_vis;
            b += f_c * g_vis;
        }
    }

    a /= SAMPLE_COUNT as f32;
    b /= SAMPLE_COUNT as f32;

    Vec2::new(a, b)
}

/// This function doesn't work on rust-gpu, presumably because of the loop.
pub fn integrate_brdf_doesnt_work(mut n_dot_v: f32, roughness: f32) -> Vec2 {
    n_dot_v = n_dot_v.max(f32::EPSILON);
    let v = Vec3::new(f32::sqrt(1.0 - n_dot_v * n_dot_v), 0.0, n_dot_v);

    let mut a = 0.0f32;
    let mut b = 0.0f32;

    let n = Vec3::Z;

    let mut i = 0u32;
    while i < SAMPLE_COUNT {
        i += 1;

        let xi = hammersley(i, SAMPLE_COUNT);
        let h = importance_sample_ggx(xi, n, roughness);
        let l = (2.0 * v.dot(h) * h - v).normalize_or_zero();

        let n_dot_l = l.z.max(0.0);
        let n_dot_h = h.z.max(0.0);
        let v_dot_h = v.dot(h).max(0.0);

        if n_dot_l > 0.0 {
            let g = geometry_smith(n, v, l, roughness);
            let denom = n_dot_h * n_dot_v;
            let g_vis = (g * v_dot_h) / denom;
            let f_c = (1.0 - v_dot_h).powf(5.0);

            a += (1.0 - f_c) * g_vis;
            b += f_c * g_vis;
        }
    }

    a /= SAMPLE_COUNT as f32;
    b /= SAMPLE_COUNT as f32;

    Vec2::new(a, b)
}

pub fn vertex_prefilter_environment_cubemap(
    constants: &GpuConstants,
    in_pos: Vec3,
    out_pos: &mut Vec3,
    gl_pos: &mut Vec4,
) {
    *out_pos = in_pos;
    *gl_pos = constants.camera_projection * constants.camera_view * in_pos.extend(1.0);
}

pub fn fragment_prefilter_environment_cubemap(
    roughness: &f32,
    environment_cubemap: &Cubemap,
    sampler: &Sampler,
    in_pos: Vec3,
    frag_color: &mut Vec4
) {
    let mut n = in_pos.normalize_or_zero();
    // TODO: ensure that flipping y is what we want (pretty sure it is as `wgpu` and vulkan's y coords
    // are flipped from opengl)
    n.y *= -1.0;
    // These moves are redundant but the names have connections to the PBR equations.
    let r = n;
    let v = r;

    let mut total_weight = 0.0f32;
    let mut prefiltered_color = Vec3::ZERO;

    for i in 0..SAMPLE_COUNT {
        let xi = hammersley(i, SAMPLE_COUNT);
        let h = importance_sample_ggx(xi, n, *roughness);
        let l = (2.0 * v.dot(h) * h - v).normalize_or_zero();

        let n_dot_l = n.dot(l).max(0.0);
        if n_dot_l > 0.0 {
            prefiltered_color += environment_cubemap.sample_by_lod(*sampler, l, 0.0).xyz() * n_dot_l;
            total_weight += n_dot_l;
        }
    }

    prefiltered_color /= total_weight;
    *frag_color = prefiltered_color.extend(1.0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integrate_brdf_sanity() {
        let points = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
        for (x, y) in points.into_iter() {
            assert!(!integrate_brdf(x, y).is_nan(), "brdf is NaN at {x},{y}"); //
        }
        let size = 32;
        let mut img = image::RgbaImage::new(size, size);
        for (x, y, image::Rgba([r, g, _, a])) in img.enumerate_pixels_mut() {
            let u = x as f32 / size as f32;
            let v = y as f32 / size as f32;
            let brdf = integrate_brdf(u, v);
            *r = (brdf.x * 255.0) as u8;
            *g = (brdf.y * 255.0) as u8;
            *a = 255;
        }
        img_diff::assert_img_eq("skybox/brdf_cpu.png", img);
    }
}
