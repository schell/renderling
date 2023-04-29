//! Physically based renderer shader code.
//!
//! Thanks to https://learnopengl.com/PBR/Theory
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::{scene::GpuLight, math::Vec3ColorSwizzles};

/// Trowbridge-Reitz GGX normal distribution function (NDF).
///
/// The normal distribution function D statistically approximates the relative
/// surface area of microfacets exactly aligned to the (halfway) vector h.
fn normal_distribution_ggx(n: Vec3, h: Vec3, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let ndot_h = n.dot(h).max(0.0);
    let ndot_h2 = ndot_h * ndot_h;

    let num = a2;
    let denom = (ndot_h2 * (a2 - 1.0) + 1.0).powf(2.0) * core::f32::consts::PI;

    num / denom
}

fn geometry_schlick_ggx(ndot_v: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    let num = ndot_v;
    let denom = ndot_v * (1.0 - k) + k;

    num / denom
}

/// The geometry function statistically approximates the relative surface area
/// where its micro surface-details overshadow each other, causing light rays to
/// be occluded.
fn geometry_smith(n: Vec3, v: Vec3, l: Vec3, roughness: f32) -> f32 {
    let ndot_v = n.dot(v).max(0.0);
    let ndot_l = n.dot(l).max(0.0);
    let ggx1 = geometry_schlick_ggx(ndot_v, roughness);
    let ggx2 = geometry_schlick_ggx(ndot_l, roughness);

    ggx1 * ggx2
}

/// Fresnel-Schlick approximation function.
///
/// The Fresnel equation describes the ratio of light that gets reflected over
/// the light that gets refracted, which varies over the angle we're looking at
/// a surface. The moment light hits a surface, based on the surface-to-view
/// angle, the Fresnel equation tells us the percentage of light that gets
/// reflected. From this ratio of reflection and the energy conservation
/// principle we can directly obtain the refracted portion of light.
fn fresnel_schlick(
    // dot product result between the surface's normal n and the halfway h (or view v) direction.
    cos_theta: f32,
    // surface reflection at zero incidence (how much the surface reflects if looking directly at
    // the surface)
    f0: Vec3,
) -> Vec3 {
    f0 + (1.0 - f0) * (1.0 - cos_theta).clamp(0.0, 1.0).powf(5.0)
}

pub fn shade_fragment(
    // world transform of the camera
    camera_view: &Mat4,
    // normal of the fragment
    in_norm: Vec3,
    // position of the fragment (world space)
    in_pos: Vec3,
    // base color of the fragment
    albedo: Vec3,
    metalness: f32,
    roughness: f32,
    ao: f32,

    lights: &[GpuLight],
) -> Vec4 {
    if lights.is_empty() || lights[0].light_type == GpuLight::END_OF_LIGHTS {
        // the scene is unlit, so we should provide some default
        let desaturated_norm = in_norm.abs().dot(Vec3::new(0.2126, 0.7152, 0.0722));
        return (albedo * desaturated_norm).extend(1.0);
    }

    let camera_pos = camera_view.transform_point3(Vec3::ZERO);
    let n = in_norm.normalize_or_zero();
    let v = (camera_pos - in_pos).normalize_or_zero();

    let f0 = Vec3::splat(0.4).lerp(albedo, metalness);

    // reflectance
    let mut lo = Vec3::ZERO;
    for i in 0..lights.len() {
        // calculate per-light radiance
        let light = lights[i];
        match light.light_type {
            GpuLight::END_OF_LIGHTS => {
                break;
            }
            GpuLight::POINT_LIGHT => {
                let frag_to_light = light.position.xyz() - in_pos;
                let l = frag_to_light.normalize_or_zero();
                let h = (v + l).normalize_or_zero();
                let distance = frag_to_light.length();
                if distance == 0.0 {
                    continue;
                }
                let attenuation = 1.0 / (distance * distance);
                let radiance = light.diffuse_color.rgb() * attenuation;

                // cook-torrance brdf
                let ndf: f32 = normal_distribution_ggx(n, h, roughness);
                let g: f32 = geometry_smith(n, v, l, roughness);
                let f: Vec3 = fresnel_schlick(h.dot(v).max(0.0), f0);

                let k_s = f;
                let k_d = (Vec3::splat(1.0) - k_s) * (1.0 - metalness);

                let numerator: Vec3 = ndf * g * f;
                let denominator: f32 = 4.0 * n.dot(v).max(0.0) * n.dot(l).max(0.0) + 0.0001;
                let specular: Vec3 = numerator / denominator;

                // add to outgoing radiance Lo
                let ndot_l = n.dot(l).max(0.0);
                lo += (k_d * albedo / core::f32::consts::PI + specular) * radiance * ndot_l;
            }
            _ => {}
        }
    }

    let ambient = Vec3::splat(0.03) * albedo * ao;
    let color = ambient + lo;
    let color = color / (color + Vec3::ONE);
    let color = color.powf(1.0 / 2.2);

    color.extend(1.0)
}
