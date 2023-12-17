//! Physically based renderer shader code.
//!
//! ## References
//! * https://learnopengl.com/PBR/Theory
//! * https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/pbr.frag
//! * https://github.khronos.org/glTF-Sample-Viewer-Release/

use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use renderling_derive::Slabbed;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{
    self as renderling_shader,
    array::Array,
    id::Id,
    math,
    slab::Slab,
    stage::{light::LightStyle, GpuLight, LightType, LightingModel},
    texture::GpuTexture,
    IsSampler, IsVector, Sample2d, SampleCube,
};

/// Represents a material on the GPU.
///
/// `PbrMaterial` is capable of representing many material types.
/// Use the appropriate builder for your material type from
/// [`SceneBuilder`](crate::SceneBuilder).
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct PbrMaterial {
    // x, y, z is emissive factor, default [0.0, 0.0, 0.0]
    // w is emissive strength multiplier (gltf's KHR_materials_emissive_strength extension),
    // default 1.0
    pub emissive_factor: Vec4,
    pub albedo_factor: Vec4,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub factor_padding: [f32; 2],

    pub albedo_texture: Id<GpuTexture>,
    pub metallic_roughness_texture: Id<GpuTexture>,
    pub normal_texture: Id<GpuTexture>,
    pub ao_texture: Id<GpuTexture>,
    pub emissive_texture: Id<GpuTexture>,

    pub albedo_tex_coord: u32,
    pub metallic_roughness_tex_coord: u32,
    pub normal_tex_coord: u32,
    pub ao_tex_coord: u32,
    pub emissive_tex_coord: u32,

    pub lighting_model: LightingModel,
    pub ao_strength: f32,
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            emissive_factor: Vec3::ZERO.extend(1.0),
            albedo_factor: Vec4::ONE,
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            factor_padding: [0.0; 2],
            albedo_texture: Id::NONE,
            metallic_roughness_texture: Id::NONE,
            normal_texture: Id::NONE,
            ao_texture: Id::NONE,
            albedo_tex_coord: 0,
            metallic_roughness_tex_coord: 0,
            normal_tex_coord: 0,
            ao_tex_coord: 0,
            lighting_model: LightingModel::PBR_LIGHTING,
            ao_strength: 0.0,
            emissive_texture: Id::NONE,
            emissive_tex_coord: 0,
        }
    }
}

/// Trowbridge-Reitz GGX normal distribution function (NDF).
///
/// The normal distribution function D statistically approximates the relative
/// surface area of microfacets exactly aligned to the (halfway) vector h.
pub fn normal_distribution_ggx(n: Vec3, h: Vec3, roughness: f32) -> f32 {
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

#[allow(clippy::too_many_arguments)]
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
    crate::println!("outgoing_radiance");
    crate::println!("    light_color: {light_color:?}");
    crate::println!("    albedo: {albedo:?}");
    crate::println!("    attenuation: {attenuation:?}");
    crate::println!("    v: {v:?}");
    crate::println!("    l: {l:?}");
    crate::println!("    n: {n:?}");
    crate::println!("    metalness: {metalness:?}");
    crate::println!("    roughness: {roughness:?}");

    let f0 = Vec3::splat(0.4).lerp(albedo, metalness);
    crate::println!("    f0: {f0:?}");
    let radiance = light_color.xyz() * attenuation;
    crate::println!("    radiance: {radiance:?}");
    let h = (v + l).alt_norm_or_zero();
    crate::println!("    h: {h:?}");
    // cook-torrance brdf
    let ndf: f32 = normal_distribution_ggx(n, h, roughness);
    crate::println!("    ndf: {ndf:?}");
    let g: f32 = geometry_smith(n, v, l, roughness);
    crate::println!("    g: {g:?}");
    let f: Vec3 = fresnel_schlick(h.dot(v).max(0.0), f0);
    crate::println!("    f: {f:?}");

    let k_s = f;
    let k_d = (Vec3::splat(1.0) - k_s) * (1.0 - metalness);
    crate::println!("    k_s: {k_s:?}");

    let numerator: Vec3 = ndf * g * f;
    crate::println!("    numerator: {numerator:?}");
    let n_dot_l = n.dot(l).max(0.0);
    crate::println!("    n_dot_l: {n_dot_l:?}");
    let denominator: f32 = 4.0 * n.dot(v).max(0.0) * n_dot_l + 0.0001;
    crate::println!("    denominator: {denominator:?}");
    let specular: Vec3 = numerator / denominator;
    crate::println!("    specular: {specular:?}");

    (k_d * albedo / core::f32::consts::PI + specular) * radiance * n_dot_l
}

pub fn sample_irradiance<T: SampleCube<Sampler = S>, S: IsSampler>(
    irradiance: &T,
    irradiance_sampler: &S,
    // Normal vector
    n: Vec3,
) -> Vec3 {
    irradiance.sample_by_lod(*irradiance_sampler, n, 0.0).xyz()
}

pub fn sample_specular_reflection<T: SampleCube<Sampler = S>, S: IsSampler>(
    prefiltered: &T,
    prefiltered_sampler: &S,
    // camera position in world space
    camera_pos: Vec3,
    // fragment position in world space
    in_pos: Vec3,
    // normal vector
    n: Vec3,
    roughness: f32,
) -> Vec3 {
    let v = (camera_pos - in_pos).alt_norm_or_zero();
    let reflect_dir = math::reflect(-v, n);
    prefiltered
        .sample_by_lod(*prefiltered_sampler, reflect_dir, roughness * 4.0)
        .xyz()
}

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

#[allow(clippy::too_many_arguments)]
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
    emissive: Vec3,
    irradiance: Vec3,
    prefiltered: Vec3,
    brdf: Vec2,

    lights: &[GpuLight],
) -> Vec4 {
    let n = in_norm.alt_norm_or_zero();
    let v = (camera_pos - in_pos).alt_norm_or_zero();

    // reflectance
    let mut lo = Vec3::ZERO;
    #[allow(clippy::needless_range_loop)]
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
                let l = frag_to_light.alt_norm_or_zero();
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
                let l = frag_to_light.alt_norm_or_zero();
                let theta: f32 = l.dot(light.direction.xyz().alt_norm_or_zero());
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
                let l = -light.direction.xyz().alt_norm_or_zero();
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

    // calculate reflectance at normal incidence; if dia-electric (like plastic) use
    // F0 of 0.04 and if it's a metal, use the albedo color as F0 (metallic
    // workflow)
    let f0: Vec3 = Vec3::splat(0.04).lerp(albedo, metallic);
    let cos_theta = n.dot(v).max(0.0);
    let fresnel = fresnel_schlick_roughness(cos_theta, f0, roughness);
    let ks = fresnel;
    let kd = (1.0 - ks) * (1.0 - metallic);
    let diffuse = irradiance * albedo;
    let specular = prefiltered * (fresnel * brdf.x + brdf.y);
    let color = (kd * diffuse + specular) * ao + lo + emissive;
    color.extend(1.0)
}

#[allow(clippy::too_many_arguments)]
pub fn stage_shade_fragment(
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
    emissive: Vec3,
    irradiance: Vec3,
    prefiltered: Vec3,
    brdf: Vec2,

    lights: Array<crate::stage::light::Light>,
    slab: &[u32],
) -> Vec4 {
    let n = in_norm.alt_norm_or_zero();
    let v = (camera_pos - in_pos).alt_norm_or_zero();
    crate::println!("lights: {lights:?}");
    crate::println!("n: {n:?}");
    crate::println!("v: {v:?}");
    // reflectance
    let mut lo = Vec3::ZERO;
    for i in 0..lights.len() {
        // calculate per-light radiance
        let light = slab.read(lights.at(i));

        // determine the light ray and the radiance
        match light.light_type {
            LightStyle::Point => {
                let point_light = slab.read(light.into_point_id());
                let frag_to_light = point_light.position - in_pos;
                let distance = frag_to_light.length();
                if distance == 0.0 {
                    continue;
                }
                let l = frag_to_light.alt_norm_or_zero();
                let attenuation = point_light.intensity * 1.0 / (distance * distance);
                lo += outgoing_radiance(
                    point_light.color,
                    albedo,
                    attenuation,
                    v,
                    l,
                    n,
                    metallic,
                    roughness,
                );
            }

            LightStyle::Spot => {
                let spot_light = slab.read(light.into_spot_id());
                let frag_to_light = spot_light.position - in_pos;
                let distance = frag_to_light.length();
                if distance == 0.0 {
                    continue;
                }
                let l = frag_to_light.alt_norm_or_zero();
                let theta: f32 = l.dot(spot_light.direction.alt_norm_or_zero());
                let epsilon: f32 = spot_light.inner_cutoff - spot_light.outer_cutoff;
                let attenuation: f32 = spot_light.intensity
                    * ((theta - spot_light.outer_cutoff) / epsilon).clamp(0.0, 1.0);
                lo += outgoing_radiance(
                    spot_light.color,
                    albedo,
                    attenuation,
                    v,
                    l,
                    n,
                    metallic,
                    roughness,
                );
            }

            LightStyle::Directional => {
                let dir_light = slab.read(light.into_directional_id());
                let l = -dir_light.direction.alt_norm_or_zero();
                let attenuation = dir_light.intensity;
                crate::println!("dir_light: {dir_light:?}");
                crate::println!("l: {l:?}");
                crate::println!("attenuation: {attenuation:?}");
                let radiance = outgoing_radiance(
                    dir_light.color,
                    albedo,
                    attenuation,
                    v,
                    l,
                    n,
                    metallic,
                    roughness,
                );
                crate::println!("radiance: {radiance:?}");
                lo += radiance;
            }
        }
    }

    crate::println!("lo: {lo:?}");
    // calculate reflectance at normal incidence; if dia-electric (like plastic) use
    // F0 of 0.04 and if it's a metal, use the albedo color as F0 (metallic
    // workflow)
    let f0: Vec3 = Vec3::splat(0.04).lerp(albedo, metallic);
    let cos_theta = n.dot(v).max(0.0);
    let fresnel = fresnel_schlick_roughness(cos_theta, f0, roughness);
    let ks = fresnel;
    let kd = (1.0 - ks) * (1.0 - metallic);
    let diffuse = irradiance * albedo;
    let specular = prefiltered * (fresnel * brdf.x + brdf.y);
    let color = (kd * diffuse + specular) * ao + lo + emissive;
    color.extend(1.0)
}
