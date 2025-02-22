//! "Physically based" types and functions.
//!
//! ## References
//! * <https://learnopengl.com/PBR/Theory>
//! * <https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/5b1b7f48a8cb2b7aaef00d08fdba18ccc8dd331b/source/Renderer/shaders/pbr.frag>
//! * <https://github.khronos.org/glTF-Sample-Viewer-Release/>
use crabslab::{Id, Slab, SlabItem};
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

#[allow(unused)]
use spirv_std::num_traits::{Float, Zero};

use crate::{
    atlas::AtlasTexture,
    light::{
        DirectionalLightDescriptor, LightStyle, LightingDescriptor, PointLightDescriptor,
        ShadowCalculation, SpotLightCalculation,
    },
    math::{self, IsSampler, IsVector, Sample2d, Sample2dArray, SampleCube},
    println as my_println,
    stage::Renderlet,
};

pub mod debug;
use debug::DebugChannel;

/// Represents a material on the GPU.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Material {
    pub emissive_factor: Vec3,
    pub emissive_strength_multiplier: f32,
    pub albedo_factor: Vec4,
    pub metallic_factor: f32,
    pub roughness_factor: f32,

    pub albedo_texture_id: Id<AtlasTexture>,
    pub metallic_roughness_texture_id: Id<AtlasTexture>,
    pub normal_texture_id: Id<AtlasTexture>,
    pub ao_texture_id: Id<AtlasTexture>,
    pub emissive_texture_id: Id<AtlasTexture>,

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
            albedo_texture_id: Id::NONE,
            metallic_roughness_texture_id: Id::NONE,
            normal_texture_id: Id::NONE,
            ao_texture_id: Id::NONE,
            albedo_tex_coord: 0,
            metallic_roughness_tex_coord: 0,
            normal_tex_coord: 0,
            ao_tex_coord: 0,
            has_lighting: true,
            ao_strength: 0.0,
            emissive_texture_id: Id::NONE,
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
#[offsets]
pub struct PbrConfig {
    pub atlas_size: glam::UVec2,
    pub resolution: glam::UVec2,
    pub debug_channel: debug::DebugChannel,
    pub has_lighting: bool,
    pub has_skinning: bool,
    pub perform_frustum_culling: bool,
    pub perform_occlusion_culling: bool,
}

impl Default for PbrConfig {
    fn default() -> Self {
        Self {
            atlas_size: Default::default(),
            resolution: glam::UVec2::ONE,
            debug_channel: Default::default(),
            has_lighting: true,
            has_skinning: true,
            perform_frustum_culling: true,
            perform_occlusion_culling: false,
        }
    }
}

/// Returns the `Material` from the stage's slab.
pub fn get_material(material_id: Id<Material>, has_lighting: bool, slab: &[u32]) -> Material {
    if material_id.is_none() {
        // without an explicit material (or if the entire render has no lighting)
        // the entity will not participate in any lighting calculations
        Material {
            has_lighting: false,
            ..Default::default()
        }
    } else {
        let mut material = slab.read(material_id);
        material.has_lighting &= has_lighting;
        material
    }
}

pub fn texture_color<A: Sample2dArray<Sampler = S>, S: IsSampler>(
    texture_id: Id<AtlasTexture>,
    uv: Vec2,
    atlas: &A,
    sampler: &S,
    atlas_size: glam::UVec2,
    slab: &[u32],
) -> Vec4 {
    let texture = slab.read(texture_id);
    // uv is [0, 0] when texture_id is Id::NONE
    let uv = texture.uv(uv, atlas_size);
    crate::println!("uv: {uv}");
    let mut color: Vec4 = atlas.sample_by_lod(*sampler, uv, 0.0);
    if texture_id.is_none() {
        color = Vec4::splat(1.0);
    }
    color
}

/// PBR fragment shader capable of being run on CPU or GPU.
#[allow(clippy::too_many_arguments)]
pub fn fragment_impl<A, T, DtA, C, S>(
    atlas: &A,
    atlas_sampler: &S,
    irradiance: &C,
    irradiance_sampler: &S,
    prefiltered: &C,
    prefiltered_sampler: &S,
    brdf: &T,
    brdf_sampler: &S,
    shadow_map: &DtA,
    shadow_map_sampler: &S,

    slab: &[u32],
    lighting_slab: &[u32],

    renderlet_id: Id<Renderlet>,

    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    in_pos: Vec3,

    output: &mut Vec4,
) where
    A: Sample2dArray<Sampler = S>,
    T: Sample2d<Sampler = S>,
    DtA: Sample2dArray<Sampler = S>,
    C: SampleCube<Sampler = S>,
    S: IsSampler,
{
    let renderlet = slab.read_unchecked(renderlet_id);
    // TODO: rename `PbrConfig` to `PbrShaderDescriptor`
    let pbr_desc = slab.read_unchecked(renderlet.pbr_config_id);
    crate::println!("pbr_desc_id: {:?}", renderlet.pbr_config_id);
    crate::println!("pbr_desc: {pbr_desc:#?}");
    let PbrConfig {
        atlas_size,
        resolution: _,
        debug_channel,
        has_lighting,
        has_skinning: _,
        perform_frustum_culling: _,
        perform_occlusion_culling: _,
    } = pbr_desc;

    let material = get_material(renderlet.material_id, has_lighting, slab);
    crate::println!("material: {:#?}", material);

    let albedo_tex_uv = if material.albedo_tex_coord == 0 {
        in_uv0
    } else {
        in_uv1
    };
    let albedo_tex_color = texture_color(
        material.albedo_texture_id,
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
        material.metallic_roughness_texture_id,
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
        material.normal_texture_id,
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
        material.ao_texture_id,
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
        material.emissive_texture_id,
        emissive_tex_uv,
        atlas,
        atlas_sampler,
        atlas_size,
        slab,
    );

    let (norm, uv_norm) = if material.normal_texture_id.is_none() {
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
    let camera = slab.read(renderlet.camera_id);
    let specular = sample_specular_reflection(
        prefiltered,
        prefiltered_sampler,
        camera.position(),
        in_pos,
        n,
        roughness,
    );
    let brdf = sample_brdf(brdf, brdf_sampler, camera.position(), in_pos, n, roughness);

    fn colorize(u: Vec3) -> Vec4 {
        ((u.alt_norm_or_zero() + Vec3::splat(1.0)) / 2.0).extend(1.0)
    }

    crate::println!("debug_mode: {debug_channel:?}");
    match debug_channel {
        DebugChannel::None => {}
        DebugChannel::UvCoords0 => {
            *output = colorize(Vec3::new(in_uv0.x, in_uv0.y, 0.0));
            return;
        }
        DebugChannel::UvCoords1 => {
            *output = colorize(Vec3::new(in_uv1.x, in_uv1.y, 0.0));
            return;
        }
        DebugChannel::Normals => {
            *output = colorize(norm);
            return;
        }
        DebugChannel::VertexColor => {
            *output = in_color;
            return;
        }
        DebugChannel::VertexNormals => {
            *output = colorize(in_norm);
            return;
        }
        DebugChannel::UvNormals => {
            *output = colorize(uv_norm);
            return;
        }
        DebugChannel::Tangents => {
            *output = colorize(in_tangent);
            return;
        }
        DebugChannel::Bitangents => {
            *output = colorize(in_bitangent);
            return;
        }
        DebugChannel::DiffuseIrradiance => {
            *output = irradiance.extend(1.0);
            return;
        }
        DebugChannel::SpecularReflection => {
            *output = specular.extend(1.0);
            return;
        }
        DebugChannel::Brdf => {
            *output = brdf.extend(1.0).extend(1.0);
            return;
        }
        DebugChannel::Roughness => {
            *output = Vec3::splat(roughness).extend(1.0);
            return;
        }
        DebugChannel::Metallic => {
            *output = Vec3::splat(metallic).extend(1.0);
            return;
        }
        DebugChannel::Albedo => {
            *output = albedo;
            return;
        }
        DebugChannel::Occlusion => {
            *output = Vec3::splat(ao).extend(1.0);
            return;
        }
        DebugChannel::Emissive => {
            *output = emissive.extend(1.0);
            return;
        }
        DebugChannel::UvEmissive => {
            *output = emissive_tex_color.xyz().extend(1.0);
            return;
        }
        DebugChannel::EmissiveFactor => {
            *output = material.emissive_factor.extend(1.0);
            return;
        }
        DebugChannel::EmissiveStrength => {
            *output = Vec3::splat(material.emissive_strength_multiplier).extend(1.0);
            return;
        }
    }

    *output = if material.has_lighting {
        shade_fragment(
            shadow_map,
            shadow_map_sampler,
            camera.position(),
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
            lighting_slab,
        )
    } else {
        crate::println!("no shading!");
        in_color * albedo_tex_color * material.albedo_factor
    };
}

#[allow(clippy::too_many_arguments)]
pub fn shade_fragment<S, T>(
    shadow_map: &T,
    shadow_map_sampler: &S,
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

    light_slab: &[u32],
) -> Vec4
where
    S: IsSampler,
    T: Sample2dArray<Sampler = S>,
{
    let n = in_norm.alt_norm_or_zero();
    let v = (camera_pos - in_pos).alt_norm_or_zero();
    // There is always a `LightingDescriptor` stored at index `0` of the
    // light slab.
    let lighting_desc = light_slab.read_unchecked(Id::<LightingDescriptor>::new(0));
    let analytical_lights_array = lighting_desc.analytical_lights_array;
    my_println!("lights: {analytical_lights_array:?}");
    my_println!("n: {n:?}");
    my_println!("v: {v:?}");

    // accumulated outgoing radiance
    let mut lo = Vec3::ZERO;
    for i in 0..analytical_lights_array.len() {
        // calculate per-light radiance
        let light_id = light_slab.read(analytical_lights_array.at(i));
        let light = light_slab.read(light_id);
        let transform = light_slab.read(light.transform_id);
        let transform = Mat4::from(transform);

        // determine the light ray and the radiance
        match light.light_type {
            LightStyle::Point => {
                let PointLightDescriptor {
                    position,
                    color,
                    intensity,
                } = light_slab.read(light.into_point_id());
                let position = transform.transform_point3(position);
                let frag_to_light = position - in_pos;
                let distance = frag_to_light.length();
                if distance == 0.0 {
                    continue;
                }
                let l = frag_to_light.alt_norm_or_zero();
                let attenuation = intensity * 1.0 / (distance * distance);
                lo += outgoing_radiance(color, albedo, attenuation, v, l, n, metallic, roughness);
            }

            LightStyle::Spot => {
                let spot_light_descriptor = light_slab.read(light.into_spot_id());
                let calculation =
                    SpotLightCalculation::new(spot_light_descriptor, transform, in_pos);
                if calculation.frag_to_light_distance == 0.0 {
                    continue;
                }
                let attenuation: f32 = spot_light_descriptor.intensity * calculation.contribution;
                let radiance = outgoing_radiance(
                    spot_light_descriptor.color,
                    albedo,
                    attenuation,
                    v,
                    calculation.frag_to_light,
                    n,
                    metallic,
                    roughness,
                );
                let shadow = if light.shadow_map_desc_id.is_some() {
                    // Shadow is 1.0 when the fragment is in the shadow of this light,
                    // and 0.0 in darkness
                    ShadowCalculation::new(light_slab, light, in_pos, n, calculation.frag_to_light)
                        .run(shadow_map, shadow_map_sampler)
                } else {
                    0.0
                };
                lo += radiance * (1.0 - shadow);
            }

            LightStyle::Directional => {
                let DirectionalLightDescriptor {
                    direction,
                    color,
                    intensity,
                } = light_slab.read(light.into_directional_id());
                let direction = transform.transform_vector3(direction);
                let l = -direction.alt_norm_or_zero();
                let attenuation = intensity;
                let radiance =
                    outgoing_radiance(color, albedo, attenuation, v, l, n, metallic, roughness);
                my_println!("radiance: {radiance:?}");

                let shadow = if light.shadow_map_desc_id.is_some() {
                    // Shadow is 1.0 when the fragment is in the shadow of this light,
                    // and 0.0 in darkness
                    ShadowCalculation::new(light_slab, light, in_pos, n, l)
                        .run(shadow_map, shadow_map_sampler)
                } else {
                    0.0
                };
                lo += radiance * (1.0 - shadow);
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

#[cfg(test)]
mod test {
    use crate::{
        atlas::AtlasImage,
        camera::Camera,
        math::{Vec3, Vec4},
        pbr::Material,
        stage::{Renderlet, Vertex},
        transform::Transform,
    };

    #[test]
    // Tests the initial implementation of pbr metallic roughness on an array of
    // spheres with different metallic roughnesses lit by an environment map.
    //
    // see https://learnopengl.com/PBR/Lighting
    fn pbr_metallic_roughness_spheres() {
        let ss = 600;
        let ctx = crate::Context::headless(ss, ss);
        let stage = ctx.new_stage();

        let radius = 0.5;
        let ss = ss as f32;
        let projection = crate::camera::perspective(ss, ss);
        let k = 7;
        let diameter = 2.0 * radius;
        let spacing = radius * 0.25;
        let len = (k - 1) as f32 * (diameter + spacing);
        let half = len / 2.0;
        let view = crate::camera::look_at(
            Vec3::new(half, half, 1.6 * len),
            Vec3::new(half, half, 0.0),
            Vec3::Y,
        );
        let camera = stage.new_value(Camera::new(projection, view));

        let geometry = stage.new_array({
            let mut icosphere = icosahedron::Polyhedron::new_isocahedron(radius, 5);
            icosphere.compute_triangle_normals();
            let icosahedron::Polyhedron {
                positions,
                normals,
                cells,
                ..
            } = icosphere;
            log::info!("icosphere created on CPU");

            let to_vertex = |ndx: &usize| -> Vertex {
                let p: [f32; 3] = positions[*ndx].0.into();
                let n: [f32; 3] = normals[*ndx].0.into();
                Vertex::default().with_position(p).with_normal(n)
            };
            cells
                .iter()
                .flat_map(|icosahedron::Triangle { a, b, c }| {
                    let p0 = to_vertex(a);
                    let p1 = to_vertex(b);
                    let p2 = to_vertex(c);
                    vec![p0, p1, p2]
                })
                .collect::<Vec<_>>()
        });
        let mut spheres = vec![];
        for i in 0..k {
            let roughness = i as f32 / (k - 1) as f32;
            let x = (diameter + spacing) * i as f32;
            for j in 0..k {
                let metallic = j as f32 / (k - 1) as f32;
                let y = (diameter + spacing) * j as f32;

                let material = stage.new_value(Material {
                    albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    metallic_factor: metallic,
                    roughness_factor: roughness,
                    ..Default::default()
                });
                let transform = stage.new_value(Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                });
                let sphere = stage.new_value(Renderlet {
                    camera_id: camera.id(),
                    vertices_array: geometry.array(),
                    transform_id: transform.id(),
                    material_id: material.id(),
                    ..Default::default()
                });
                stage.add_renderlet(&sphere);
                spheres.push((sphere, material, transform));
            }
        }

        let hdr_image = AtlasImage::from_hdr_path("../../img/hdr/resting_place.hdr").unwrap();
        let skybox = crate::skybox::Skybox::new(&ctx, hdr_image, camera.id());
        stage.set_skybox(skybox);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("pbr/metallic_roughness_spheres.png", img);
    }
}
