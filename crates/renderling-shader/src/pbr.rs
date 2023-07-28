//! Physically based renderer shader code.
//!
//! ## References
//! * https://learnopengl.com/PBR/Theory
//! * https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/pbr.frag
//! * https://github.khronos.org/glTF-Sample-Viewer-Release/
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use glam::{Vec3, Vec4, Vec4Swizzles};

use crate::scene::{GpuLight, LightType};

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

fn fresnel_schlick_roughness(cos_theta: f32, f0: Vec3, roughness: f32) -> Vec3 {
    f0 + (Vec3::splat(1.0 - roughness).max(f0) - f0) * (1.0 - cos_theta).clamp(0.0, 1.0).powf(5.0)
}

fn outgoing_radiance(
    light_color: Vec4,
    albedo: Vec3,
    attenuation: f32,
    v: Vec3,
    l: Vec3,
    n: Vec3,
    metalness: f32,
    roughness: f32,
) -> Vec3 {
    let f0 = Vec3::splat(0.4).lerp(albedo, metalness);
    let radiance = light_color.xyz() * attenuation;
    let h = (v + l).normalize_or_zero();
    // cook-torrance brdf
    let ndf: f32 = normal_distribution_ggx(n, h, roughness);
    let g: f32 = geometry_smith(n, v, l, roughness);
    let f: Vec3 = fresnel_schlick(h.dot(v).max(0.0), f0);

    let k_s = f;
    let k_d = (Vec3::splat(1.0) - k_s) * (1.0 - metalness);

    let numerator: Vec3 = ndf * g * f;
    let n_dot_l = n.dot(l).max(0.0);
    let denominator: f32 = 4.0 * n.dot(v).max(0.0) * n_dot_l + 0.0001;
    let specular: Vec3 = numerator / denominator;

    (k_d * albedo / core::f32::consts::PI + specular) * radiance * n_dot_l
}

pub fn shade_fragment(
    // camera's position in world space
    camera_pos: Vec3,
    // normal of the fragment
    in_norm: Vec3,
    // position of the fragment in world space
    in_pos: Vec3,
    // base color of the fragment
    albedo: Vec3,
    metallic: f32,
    roughness: f32,
    ao: f32,
    irradiance: Vec3,

    lights: &[GpuLight],
) -> Vec4 {
    let n = in_norm.normalize_or_zero();
    let v = (camera_pos - in_pos).normalize_or_zero();

    // reflectance
    let mut lo = Vec3::ZERO;
    for i in 0..lights.len() {
        // calculate per-light radiance
        let light = lights[i];

        // determine the light ray and the radiance
        match light.light_type {
            LightType::END_OF_LIGHTS => {
                break;
            }
            LightType::POINT_LIGHT => {
                let frag_to_light = light.position.xyz() - in_pos;
                let distance = frag_to_light.length();
                if distance == 0.0 {
                    continue;
                }
                let l = frag_to_light.normalize_or_zero();
                let attenuation = light.intensity * 1.0 / (distance * distance);
                lo += outgoing_radiance(
                    light.color,
                    albedo,
                    attenuation,
                    v,
                    l,
                    n,
                    metallic,
                    roughness,
                );
            }

            LightType::SPOT_LIGHT => {
                let frag_to_light = light.position.xyz() - in_pos;
                let distance = frag_to_light.length();
                if distance == 0.0 {
                    continue;
                }
                let l = frag_to_light.normalize_or_zero();
                let theta: f32 = l.dot(light.direction.xyz().normalize_or_zero());
                let epsilon: f32 = light.inner_cutoff - light.outer_cutoff;
                let attenuation: f32 =
                    light.intensity * ((theta - light.outer_cutoff) / epsilon).clamp(0.0, 1.0);
                lo += outgoing_radiance(
                    light.color,
                    albedo,
                    attenuation,
                    v,
                    l,
                    n,
                    metallic,
                    roughness,
                );
            }

            LightType::DIRECTIONAL_LIGHT => {
                let l = -light.direction.xyz().normalize_or_zero();
                let attenuation = light.intensity;
                lo += outgoing_radiance(
                    light.color,
                    albedo,
                    attenuation,
                    v,
                    l,
                    n,
                    metallic,
                    roughness,
                );
            }
            _ => {}
        }
    }

    // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0
    // of 0.04 and if it's a metal, use the albedo color as F0 (metallic workflow)
    let f0: Vec3 = Vec3::splat(0.04).lerp(albedo, metallic);
    let ambient = {
        let cos_theta = n.dot(v).max(0.0);
        let ks = fresnel_schlick_roughness(cos_theta, f0, roughness);
        let kd = (1.0 - ks) * (1.0 - metallic);
        let diffuse = irradiance * albedo;
        kd * diffuse
    };
    let color = (lo + ambient) * ao;
    color.extend(1.0)
}
