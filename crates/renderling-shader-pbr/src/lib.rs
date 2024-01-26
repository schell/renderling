//! Physically-based shading code.
//!
//! ## References
//! * https://learnopengl.com/PBR/Theory
//! * https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/pbr.frag
//! * https://github.khronos.org/glTF-Sample-Viewer-Release/
#![cfg_attr(target_arch = "spirv", no_std)]

use crabslab::{Array, Id, Slab, SlabItem, ID_NONE};
use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use light::Light;
use renderling_shader_core::{
    math::{self, IsVector},
    println as my_println,
    texture::GpuTexture,
    Camera, IsSampler, Sample2d, SampleCube,
};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{debug::DebugMode, light::LightStyle};

pub mod debug;
pub mod light;

/// Represents a material on the GPU.
///
/// `Material` is capable of representing many material types.
/// Use the appropriate builder for your material type from
/// [`SceneBuilder`](crate::SceneBuilder).
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Material {
    pub emissive_factor: Vec3,
    pub emissive_strength_multiplier: f32,
    pub albedo_factor: Vec4,
    pub metallic_factor: f32,
    pub roughness_factor: f32,

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

    pub has_lighting: bool,
    pub ao_strength: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            emissive_factor: Vec3::ZERO,
            emissive_strength_multiplier: 1.0,
            albedo_factor: Vec4::ONE,
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            albedo_texture: Id::NONE,
            metallic_roughness_texture: Id::NONE,
            normal_texture: Id::NONE,
            ao_texture: Id::NONE,
            albedo_tex_coord: 0,
            metallic_roughness_tex_coord: 0,
            normal_tex_coord: 0,
            ao_tex_coord: 0,
            has_lighting: true,
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
    my_println!("outgoing_radiance");
    my_println!("    light_color: {light_color:?}");
    my_println!("    albedo: {albedo:?}");
    my_println!("    attenuation: {attenuation:?}");
    my_println!("    v: {v:?}");
    my_println!("    l: {l:?}");
    my_println!("    n: {n:?}");
    my_println!("    metalness: {metalness:?}");
    my_println!("    roughness: {roughness:?}");

    let f0 = Vec3::splat(0.4).lerp(albedo, metalness);
    my_println!("    f0: {f0:?}");
    let radiance = light_color.xyz() * attenuation;
    my_println!("    radiance: {radiance:?}");
    let h = (v + l).alt_norm_or_zero();
    my_println!("    h: {h:?}");
    // cook-torrance brdf
    let ndf: f32 = normal_distribution_ggx(n, h, roughness);
    my_println!("    ndf: {ndf:?}");
    let g: f32 = geometry_smith(n, v, l, roughness);
    my_println!("    g: {g:?}");
    let f: Vec3 = fresnel_schlick(h.dot(v).max(0.0), f0);
    my_println!("    f: {f:?}");

    let k_s = f;
    let k_d = (Vec3::splat(1.0) - k_s) * (1.0 - metalness);
    my_println!("    k_s: {k_s:?}");

    let numerator: Vec3 = ndf * g * f;
    my_println!("    numerator: {numerator:?}");
    let n_dot_l = n.dot(l).max(0.0);
    my_println!("    n_dot_l: {n_dot_l:?}");
    let denominator: f32 = 4.0 * n.dot(v).max(0.0) * n_dot_l + 0.0001;
    my_println!("    denominator: {denominator:?}");
    let specular: Vec3 = numerator / denominator;
    my_println!("    specular: {specular:?}");

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

/// Holds PBR configuration info.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct PbrConfig {
    pub atlas_size: glam::UVec2,
    pub debug_mode: debug::DebugMode,
    pub has_lighting: bool,
    pub light_array: Array<light::Light>,
}

impl Default for PbrConfig {
    fn default() -> Self {
        Self {
            atlas_size: Default::default(),
            debug_mode: Default::default(),
            has_lighting: true,
            light_array: Default::default(),
        }
    }
}

/// Returns the `Material` from the stage's slab.
pub fn get_material(material_index: u32, has_lighting: bool, slab: &[u32]) -> Material {
    if material_index == ID_NONE {
        // without an explicit material (or if the entire render has no lighting)
        // the entity will not participate in any lighting calculations
        Material {
            has_lighting: false,
            ..Default::default()
        }
    } else {
        let mut material = slab.read(Id::<Material>::new(material_index));
        if !has_lighting {
            material.has_lighting = false;
        }
        material
    }
}

pub fn texture_color<T: Sample2d<Sampler = S>, S: IsSampler>(
    texture_id: Id<GpuTexture>,
    uv: Vec2,
    atlas: &T,
    sampler: &S,
    atlas_size: glam::UVec2,
    slab: &[u32],
) -> Vec4 {
    let texture = slab.read(texture_id);
    let uv = texture.uv(uv, atlas_size);
    let mut color: Vec4 = atlas.sample_by_lod(*sampler, uv, 0.0);
    if texture_id.is_none() {
        color = Vec4::splat(1.0);
    }
    color
}

/// PBR fragment shader.
#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
pub fn pbr_fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] in_pbr_config: Id<PbrConfig>,
    #[spirv(flat)] in_camera: u32,
    #[spirv(flat)] in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
) {
    fragment_impl(
        atlas,
        atlas_sampler,
        irradiance,
        irradiance_sampler,
        prefiltered,
        prefiltered_sampler,
        brdf,
        brdf_sampler,
        slab,
        in_pbr_config,
        in_camera,
        in_material,
        in_color,
        in_uv0,
        in_uv1,
        in_norm,
        in_tangent,
        in_bitangent,
        in_pos,
        output,
    )
}

/// PBR fragment shader capable of being run on CPU or GPU.
#[allow(clippy::too_many_arguments)]
pub fn fragment_impl<T, C, S>(
    atlas: &T,
    atlas_sampler: &S,

    irradiance: &C,
    irradiance_sampler: &S,

    prefiltered: &C,
    prefiltered_sampler: &S,

    brdf: &T,
    brdf_sampler: &S,

    slab: &[u32],

    in_pbr_config: Id<PbrConfig>,
    in_camera: u32,
    in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
) where
    T: Sample2d<Sampler = S>,
    C: SampleCube<Sampler = S>,
    S: IsSampler,
{
    let pbr = slab.read(in_pbr_config);
    my_println!("pbr: {:?}", pbr);
    let PbrConfig {
        atlas_size,

        debug_mode,
        has_lighting,
        light_array,
    } = pbr;

    let material = get_material(in_material, has_lighting, slab);
    my_println!("material: {:?}", material);

    let albedo_tex_uv = if material.albedo_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let albedo_tex_color = texture_color(
        material.albedo_texture,
        albedo_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );
    my_println!("albedo_tex_color: {:?}", albedo_tex_color);

    let metallic_roughness_uv = if material.metallic_roughness_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let metallic_roughness_tex_color = texture_color(
        material.metallic_roughness_texture,
        metallic_roughness_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );
    my_println!(
        "metallic_roughness_tex_color: {:?}",
        metallic_roughness_tex_color
    );

    let normal_tex_uv = if material.normal_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let normal_tex_color = texture_color(
        material.normal_texture,
        normal_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );
    my_println!("normal_tex_color: {:?}", normal_tex_color);

    let ao_tex_uv = if material.ao_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let ao_tex_color = texture_color(
        material.ao_texture,
        ao_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );

    let emissive_tex_uv = if material.emissive_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let emissive_tex_color = texture_color(
        material.emissive_texture,
        emissive_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );

    let (norm, uv_norm) = if material.normal_texture.is_none() {
        // there is no normal map, use the normal normal ;)
        (in_norm, Vec3::ZERO)
    } else {
        // convert the normal from color coords to tangent space -1,1
        let sampled_norm = (normal_tex_color.xyz() * 2.0 - Vec3::splat(1.0)).alt_norm_or_zero();
        let tbn = glam::mat3(
            in_tangent.alt_norm_or_zero(),
            in_bitangent.alt_norm_or_zero(),
            in_norm.alt_norm_or_zero(),
        );
        // convert the normal from tangent space to world space
        let norm = (tbn * sampled_norm).alt_norm_or_zero();
        (norm, sampled_norm)
    };

    let n = norm;
    let albedo = albedo_tex_color * material.albedo_factor * in_color;
    let roughness = metallic_roughness_tex_color.y * material.roughness_factor;
    let metallic = metallic_roughness_tex_color.z * material.metallic_factor;
    let ao = 1.0 + material.ao_strength * (ao_tex_color.x - 1.0);
    let emissive =
        emissive_tex_color.xyz() * material.emissive_factor * material.emissive_strength_multiplier;
    let irradiance = sample_irradiance(irradiance, irradiance_sampler, n);
    let camera = slab.read(Id::<Camera>::new(in_camera));
    let specular = sample_specular_reflection(
        prefiltered,
        prefiltered_sampler,
        camera.position,
        in_pos,
        n,
        roughness,
    );
    let brdf = sample_brdf(brdf, brdf_sampler, camera.position, in_pos, n, roughness);

    fn colorize(u: Vec3) -> Vec4 {
        ((u.alt_norm_or_zero() + Vec3::splat(1.0)) / 2.0).extend(1.0)
    }

    match debug_mode {
        DebugMode::None => {}
        DebugMode::UvCoords0 => {
            *output = colorize(Vec3::new(in_uv0.x, in_uv0.y, 0.0));
            return;
        }
        DebugMode::UvCoords1 => {
            *output = colorize(Vec3::new(in_uv1.x, in_uv1.y, 0.0));
            return;
        }
        DebugMode::Normals => {
            *output = colorize(norm);
            return;
        }
        DebugMode::VertexColor => {
            *output = in_color;
            return;
        }
        DebugMode::VertexNormals => {
            *output = colorize(in_norm);
            return;
        }
        DebugMode::UvNormals => {
            *output = colorize(uv_norm);
            return;
        }
        DebugMode::Tangents => {
            *output = colorize(in_tangent);
            return;
        }
        DebugMode::Bitangents => {
            *output = colorize(in_bitangent);
            return;
        }
        DebugMode::DiffuseIrradiance => {
            *output = irradiance.extend(1.0);
            return;
        }
        DebugMode::SpecularReflection => {
            *output = specular.extend(1.0);
            return;
        }
        DebugMode::Brdf => {
            *output = brdf.extend(1.0).extend(1.0);
            return;
        }
        DebugMode::Roughness => {
            *output = Vec3::splat(roughness).extend(1.0);
            return;
        }
        DebugMode::Metallic => {
            *output = Vec3::splat(metallic).extend(1.0);
            return;
        }
        DebugMode::Albedo => {
            *output = albedo;
            return;
        }
        DebugMode::Occlusion => {
            *output = Vec3::splat(ao).extend(1.0);
            return;
        }
        DebugMode::Emissive => {
            *output = emissive.extend(1.0);
            return;
        }
        DebugMode::UvEmissive => {
            *output = emissive_tex_color.xyz().extend(1.0);
            return;
        }
        DebugMode::EmissiveFactor => {
            *output = material.emissive_factor.extend(1.0);
            return;
        }
        DebugMode::EmissiveStrength => {
            *output = Vec3::splat(material.emissive_strength_multiplier).extend(1.0);
            return;
        }
    }

    *output = if material.has_lighting {
        shade_fragment(
            camera.position,
            n,
            in_pos,
            albedo.xyz(),
            metallic,
            roughness,
            ao,
            emissive,
            irradiance,
            specular,
            brdf,
            light_array,
            slab,
        )
    } else {
        in_color * albedo_tex_color * material.albedo_factor
    };
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

    lights: Array<Light>,
    slab: &[u32],
) -> Vec4 {
    let n = in_norm.alt_norm_or_zero();
    let v = (camera_pos - in_pos).alt_norm_or_zero();
    my_println!("lights: {lights:?}");
    my_println!("n: {n:?}");
    my_println!("v: {v:?}");
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
                my_println!("dir_light: {dir_light:?}");
                my_println!("l: {l:?}");
                my_println!("attenuation: {attenuation:?}");
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
                my_println!("radiance: {radiance:?}");
                lo += radiance;
            }
        }
    }

    my_println!("lo: {lo:?}");
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
