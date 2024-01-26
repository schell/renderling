//! Convolution shaders.
//!
//! These shaders convolve various functions to produce cached maps.
use crabslab::{Id, Slab, SlabItem};
use glam::{UVec2, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    num_traits::Zero,
    spirv, Sampler,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{math::IsVector, Camera};

fn radical_inverse_vdc(mut bits: u32) -> f32 {
    bits = (bits << 16u32) | (bits >> 16u32);
    bits = ((bits & 0x55555555u32) << 1u32) | ((bits & 0xAAAAAAAAu32) >> 1u32);
    bits = ((bits & 0x33333333u32) << 2u32) | ((bits & 0xCCCCCCCCu32) >> 2u32);
    bits = ((bits & 0x0F0F0F0Fu32) << 4u32) | ((bits & 0xF0F0F0F0u32) >> 4u32);
    bits = ((bits & 0x00FF00FFu32) << 8u32) | ((bits & 0xFF00FF00u32) >> 8u32);
    (bits as f32) * 2.328_306_4e-10 // / 0x100000000
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
    let tangent = up.cross(n).alt_norm_or_zero();
    let bitangent = n.cross(tangent);

    let result = tangent * h.x + bitangent * h.y + n * h.z;
    result.alt_norm_or_zero()
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
        let l = (2.0 * v.dot(h) * h - v).alt_norm_or_zero();

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
        let l = (2.0 * v.dot(h) * h - v).alt_norm_or_zero();

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

/// Used by [`vertex_prefilter_environment_cubemap`] to read the camera and
/// roughness values from the slab.
#[derive(Default, SlabItem)]
pub struct VertexPrefilterEnvironmentCubemapIds {
    pub camera: Id<Camera>,
    pub roughness: Id<f32>,
}

/// Uses the `instance_index` as the [`Id`] of a [`PrefilterEnvironmentIds`].
/// roughness value.
#[spirv(vertex)]
pub fn vertex_prefilter_environment_cubemap(
    #[spirv(instance_index)] instance_index: u32,
    #[spirv(vertex_index)] vertex_id: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    out_pos: &mut Vec3,
    out_roughness: &mut f32,
    #[spirv(position)] gl_pos: &mut Vec4,
) {
    let in_pos = crate::math::CUBE[vertex_id as usize];
    let VertexPrefilterEnvironmentCubemapIds { camera, roughness } =
        slab.read(Id::new(instance_index));
    let camera = slab.read(camera);
    *out_roughness = slab.read(roughness);
    *out_pos = in_pos;
    *gl_pos = camera.projection * camera.view * in_pos.extend(1.0);
}

/// Lambertian prefilter.
#[spirv(fragment)]
pub fn fragment_prefilter_environment_cubemap(
    #[spirv(descriptor_set = 0, binding = 1)] environment_cubemap: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_pos: Vec3,
    in_roughness: f32,
    frag_color: &mut Vec4,
) {
    let mut n = in_pos.alt_norm_or_zero();
    // `wgpu` and vulkan's y coords are flipped from opengl
    n.y *= -1.0;
    let r = n;
    let v = r;

    let mut total_weight = 0.0f32;
    let mut prefiltered_color = Vec3::ZERO;

    for i in 0..SAMPLE_COUNT {
        let xi = hammersley(i, SAMPLE_COUNT);
        let h = importance_sample_ggx(xi, n, in_roughness);
        let l = (2.0 * v.dot(h) * h - v).alt_norm_or_zero();

        let n_dot_l = n.dot(l).max(0.0);
        if n_dot_l > 0.0 {
            let mip_level = if in_roughness == 0.0 {
                0.0
            } else {
                calc_lod(n_dot_l)
            };
            prefiltered_color += environment_cubemap
                .sample_by_lod(*sampler, l, mip_level)
                .xyz()
                * n_dot_l;
            total_weight += n_dot_l;
        }
    }

    prefiltered_color /= total_weight;
    *frag_color = prefiltered_color.extend(1.0);
}

pub fn calc_lod_old(n: Vec3, v: Vec3, h: Vec3, roughness: f32) -> f32 {
    // sample from the environment's mip level based on roughness/pdf
    let d = crate::pbr::normal_distribution_ggx(n, h, roughness);
    let n_dot_h = n.dot(h).max(0.0);
    let h_dot_v = h.dot(v).max(0.0);
    let pdf = (d * n_dot_h / (4.0 * h_dot_v)).max(core::f32::EPSILON);

    let resolution = 512.0; // resolution of source cubemap (per face)
    let sa_texel = 4.0 * core::f32::consts::PI / (6.0 * resolution * resolution);
    let sa_sample = 1.0 / (SAMPLE_COUNT as f32 * pdf + core::f32::EPSILON);

    0.5 * (sa_sample / sa_texel).log2()
}

pub fn calc_lod(n_dot_l: f32) -> f32 {
    let cube_width = 512.0;
    let pdf = (n_dot_l * core::f32::consts::FRAC_1_PI).max(0.0);
    0.5 * (6.0 * cube_width * cube_width / (SAMPLE_COUNT as f32 * pdf).max(core::f32::EPSILON))
        .log2()
}

#[spirv(vertex)]
pub fn vertex_generate_mipmap(
    #[spirv(vertex_index)] vertex_id: u32,
    out_uv: &mut Vec2,
    #[spirv(position)] gl_pos: &mut Vec4,
) {
    let i = vertex_id as usize;
    *out_uv = crate::math::UV_COORD_QUAD_CCW[i];
    *gl_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
}

#[spirv(fragment)]
pub fn fragment_generate_mipmap(
    #[spirv(descriptor_set = 0, binding = 0)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    in_uv: Vec2,
    frag_color: &mut Vec4,
) {
    *frag_color = texture.sample(*sampler, in_uv);
}

#[spirv(fragment)]
pub fn fragment_bloom(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] horizontal: &u32,
    #[spirv(uniform, descriptor_set = 0, binding = 1)] UVec2 { x, y }: &UVec2,
    #[spirv(descriptor_set = 0, binding = 2)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    in_uv: Vec2,
    frag_color: &mut Vec4,
) {
    let horizontal = *horizontal != 0;
    let weight = [0.227027f32, 0.1945946, 0.1216216, 0.054054, 0.016216];
    let texel_offset = 1.0 / Vec2::new(*x as f32, *y as f32);
    let mut result = texture.sample(*sampler, in_uv).xyz() * weight[0];
    let sample_offset = if horizontal {
        Vec2::new(1.0, 0.0)
    } else {
        Vec2::new(0.0, 1.0)
    };

    #[allow(clippy::needless_range_loop)]
    for i in 1..5 {
        let offset = sample_offset * texel_offset * i as f32;
        result += texture.sample_by_lod(*sampler, in_uv + offset, 0.0).xyz() * weight[i];
        result += texture.sample_by_lod(*sampler, in_uv - offset, 0.0).xyz() * weight[i];
    }
    *frag_color = result.extend(1.0);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vert {
    pos: [f32; 3],
    uv: [f32; 2],
}

/// A screen-space quad.
const BRDF_VERTS: [Vert; 6] = {
    let bl = Vert {
        pos: [-1.0, -1.0, 0.0],
        uv: [0.0, 1.0],
    };
    let br = Vert {
        pos: [1.0, -1.0, 0.0],
        uv: [1.0, 1.0],
    };
    let tl = Vert {
        pos: [-1.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    };
    let tr = Vert {
        pos: [1.0, 1.0, 0.0],
        uv: [1.0, 0.0],
    };

    [bl, br, tr, bl, tr, tl]
};

#[spirv(vertex)]
pub fn vertex_brdf_lut_convolution(
    #[spirv(vertex_index)] vertex_id: u32,
    out_uv: &mut glam::Vec2,
    #[spirv(position)] gl_pos: &mut glam::Vec4,
) {
    let Vert { pos, uv } = BRDF_VERTS[vertex_id as usize];
    *out_uv = Vec2::from(uv);
    *gl_pos = Vec3::from(pos).extend(1.0);
}

#[spirv(fragment)]
pub fn fragment_brdf_lut_convolution(in_uv: glam::Vec2, out_color: &mut glam::Vec2) {
    *out_color = integrate_brdf(in_uv.x, in_uv.y);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integrate_brdf_sanity() {
        let points = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
        for (x, y) in points.into_iter() {
            assert!(!integrate_brdf(x, y).is_nan(), "brdf is NaN at {x},{y}");
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
