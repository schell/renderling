//! Raymarching functions and shaders.
use crabslab::{Id, Slab, SlabItem};
use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

use crate::{
    math::{self, IsVector},
    Camera,
};

#[derive(Clone, Copy, Default, SlabItem)]
pub struct Raymarch {
    pub screen_resolution: Vec2,
    pub camera: Id<Camera>,
    pub pbr_config: Id<crate::pbr::PbrConfig>,
    pub default_material: Id<crate::pbr::Material>,
}

#[spirv(vertex)]
pub fn raymarch_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(instance_index)] raymarch: Id<Raymarch>,
    #[spirv(flat)] out_raymarch: &mut Id<Raymarch>,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    *out_raymarch = raymarch;
    let clip_pos = math::CLIP_QUAD_CCW[(vertex_index % 6) as usize];
    *out_clip_pos = clip_pos.extend(1.0);
}

/// Converts from a fragment coordinate to a clip coordinate.
///
/// See [WGPU's docs on clip space](https://www.w3.org/TR/webgpu/#primitive-clipping).
///
/// ### tl;dr
/// - x: find `frag.x` where `[-1.0 (left), 1.0 (right)]`
/// - y: find `frag.y` where `[-1.0 (bottom), 1.0 (top)]`
/// - z: `depth`, where `[0.0, 1.0]` (which is near depends on the projection)
/// - w: `1.0`
fn frag_coord_to_near_clip(frag_coord: Vec2, screen_resolution: Vec2, depth: f32) -> Vec4 {
    ((2.0 * frag_coord / screen_resolution - 1.0) * Vec2::new(-1.0, -1.0))
        .extend(depth)
        .extend(1.0)
}

/// Convert the fragment coordinate to world space.
fn frag_coord_to_world(frag_coord: Vec2, screen_resolution: Vec2, camera: &Camera) -> Vec3 {
    // Convert fragment coordinates to clip
    let clip_coord = frag_coord_to_near_clip(frag_coord, screen_resolution, 1.0);
    // Transform point to camera space
    let world_point = (camera.projection * camera.view).inverse() * clip_coord;
    world_point.xyz()
}

fn raymarching_ray_direction(
    camera: &Camera,
    fragment_coord: Vec2,
    screen_resolution: Vec2,
) -> Vec3 {
    let world_point = frag_coord_to_world(fragment_coord, screen_resolution, camera);
    // Ray direction is the normalized vector from the camera position
    // to the fragment's world position
    let ray_direction = (world_point - camera.position).alt_norm_or_zero();
    ray_direction
}

fn sphere_distance(center: Vec3, radius: f32, point: Vec3) -> f32 {
    (point - center).length() - radius
}

#[derive(Clone, Copy, Default)]
struct MarchResult {
    position: Vec3,
    normal: Vec3,
    tangent: Vec3,
    bitangent: Vec3,
    total_distance: f32,
    min_distance: f32,
    hit: bool,
}

const SPHERE_CENTER: Vec3 = Vec3::new(0.0, 0.0, 6.0);
const SPHERE_RADIUS: f32 = 1.0;
const NORMAL_EPSILON: f32 = 0.0001;

fn find_normal_by_gradient(p: Vec3) -> Vec3 {
    let dx = Vec3::new(NORMAL_EPSILON, 0.0, 0.0);
    let dy = Vec3::new(0.0, NORMAL_EPSILON, 0.0);
    let dz = Vec3::new(0.0, 0.0, NORMAL_EPSILON);
    Vec3::new(
        sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, p + dx)
            - sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, p - dx),
        sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, p + dy)
            - sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, p - dy),
        sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, p + dz)
            - sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, p - dz),
    )
    .alt_norm_or_zero()
}

fn produce_tangent_and_bitangent(normal: Vec3) -> (Vec3, Vec3) {
    let tangent = normal.cross(Vec3::new(-normal.z, normal.x, normal.y));
    let bitangent = Vec3::cross(normal, tangent);
    (tangent, bitangent)
}

fn march(ray_origin: Vec3, ray_direction: Vec3) -> MarchResult {
    const MAX_DIST: f32 = 100.0;
    const MIN_DIST: f32 = 0.001;
    const NUM_STEPS: u32 = 32;

    let mut ray_dist: f32 = 0.0;
    let mut min_dist: f32 = MIN_DIST;
    for _ in 0..NUM_STEPS {
        let current_pos = ray_origin + ray_dist * ray_direction;
        let current_dist = sphere_distance(SPHERE_CENTER, SPHERE_RADIUS, current_pos);
        min_dist = min_dist.min(current_dist);
        if current_dist <= MIN_DIST {
            let normal = find_normal_by_gradient(current_pos);
            let (tangent, bitangent) = produce_tangent_and_bitangent(normal);
            return MarchResult {
                position: current_pos,
                normal,
                tangent,
                bitangent,
                total_distance: ray_dist,
                min_distance: min_dist,
                hit: true,
            };
        } else if current_dist >= MAX_DIST {
            break;
        }
        ray_dist += current_dist;
    }
    MarchResult {
        position: ray_origin + ray_dist * ray_direction,
        normal: Vec3::Z,
        tangent: Vec3::X,
        bitangent: Vec3::Y,
        total_distance: MAX_DIST,
        min_distance: min_dist,
        hit: false,
    }
}

#[spirv(fragment)]
pub fn raymarch_rays(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] raymarch: Id<Raymarch>,
    out_color: &mut Vec4,
) {
    let cfg = slab.read(raymarch);
    let camera = slab.read(cfg.camera);
    let ray_dir = raymarching_ray_direction(&camera, frag_coord.xy(), cfg.screen_resolution);
    *out_color = ray_dir.extend(1.0);
}

#[spirv(fragment)]
pub fn raymarch_fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(flat)] raymarch: Id<Raymarch>,
    out_color: &mut Vec4,
) {
    let cfg = slab.read(raymarch);
    let camera = slab.read(cfg.camera);
    let ray_dir = raymarching_ray_direction(&camera, frag_coord.xy(), cfg.screen_resolution);
    let result = march(camera.position, ray_dir);
    if result.hit {
        crate::pbr::fragment_impl(
            atlas,
            atlas_sampler,
            irradiance,
            irradiance_sampler,
            prefiltered,
            prefiltered_sampler,
            brdf,
            brdf_sampler,
            slab,
            cfg.pbr_config,
            cfg.camera.inner(),
            cfg.default_material.inner(),
            Vec4::ONE,  // albedo color
            Vec2::ZERO, // uv0
            Vec2::ZERO, // uv1
            result.normal,
            result.tangent,
            result.bitangent,
            result.position,
            out_color,
        );
    } else {
        let color = prefiltered.sample(*prefiltered_sampler, ray_dir).xyz();
        *out_color = color.extend(1.0);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sdf::helper::RaymarchingRenderer;
    use assert_approx_eq::assert_approx_eq;
    use crabslab::GrowableSlab;
    use glam::{Mat4, Vec3Swizzles};

    fn camera(width: f32, height: f32) -> Camera {
        let aspect = width / height;
        let fovy = std::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
        let view = Mat4::look_at_rh(Vec3::ZERO, Vec3::Z, Vec3::Y);
        Camera::new(projection, view)
    }

    #[test]
    // This test is to make sure that the inverse of the projection and view
    // matrices are the same as the inverse of the multiplied matrices.
    //
    // Just a sanity check for me to make sure I'm not doing something stupid.
    fn raymarch_sanity_inverse_proj_view_matrix() {
        // Inverting causes values of `-0.0`, which != `0.0`
        fn remove_neg_zeros(elements: &mut [f32]) {
            for e in elements {
                if *e == -0.0 {
                    *e = 0.0;
                }
            }
        }

        let camera = camera(256.0, 192.0);

        let mut inverse_multiplied = camera.view.inverse() * camera.projection.inverse();
        remove_neg_zeros(inverse_multiplied.as_mut());
        let mut multiplied_inverse = (camera.projection * camera.view).inverse();
        remove_neg_zeros(multiplied_inverse.as_mut());

        for (a, b) in inverse_multiplied
            .as_ref()
            .iter()
            .zip(multiplied_inverse.as_ref().iter())
        {
            assert!(
                (a - b).abs() < 0.0001,
                "inverse_multiplied: {:?}, multiplied_inverse: {:?}",
                inverse_multiplied,
                multiplied_inverse,
            );
        }
    }

    #[test]
    fn raymarch_sanity_normal_tangent_bitangent() {
        let normal = Vec3::new(24.0, -8.0, 3.5).normalize();
        let tangent = normal.cross(Vec3::new(-normal.z, normal.x, normal.y));
        let bitangent = Vec3::cross(normal, tangent);
        assert_approx_eq!(0.0, normal.dot(tangent), f32::EPSILON);
        assert_approx_eq!(0.0, normal.dot(bitangent), f32::EPSILON);
        assert_approx_eq!(0.0, tangent.dot(bitangent), f32::EPSILON);
    }

    #[test]
    fn raymarch_check_rays() {
        let width = 256.0;
        let height = 192.0;
        let camera = camera(width, height);
        let resolution = Vec2::new(width, height);
        let top_left_frag_coord = Vec2::ZERO;
        let top_left_ray = raymarching_ray_direction(&camera, top_left_frag_coord, resolution);
        let center_frag_coord = resolution / 2.0;
        let center_ray = raymarching_ray_direction(&camera, center_frag_coord, resolution);
        let bottom_right_frag_coord = resolution;
        let bottom_right_ray =
            raymarching_ray_direction(&camera, bottom_right_frag_coord, resolution);
        println!("top_left: {top_left_ray:?}");
        println!("center: {center_ray:?}");
        println!("bottom_right: {bottom_right_ray:?}");
        assert_eq!(
            Vec2::ZERO.extend(1.0),
            center_ray,
            "Center ray should always shoot straight forward"
        );
        assert_eq!(
            top_left_ray.xy(),
            -bottom_right_ray.xy(),
            "Top left ray should be the opposite of bottom right ray"
        );
    }

    #[test]
    fn raymarch_rays() {
        let width = 256.0;
        let height = 192.0;
        let mut r = RaymarchingRenderer::new(width as u32, height as u32);
        let camera = r.slab.append(&camera(width, height));
        let raymarch = r.slab.append(&Raymarch {
            camera,
            screen_resolution: Vec2::new(width, height),
            ..Default::default()
        });
        let img = r.render_rays(raymarch);
        img_diff::save("raymarch/rays.png", img);
    }

    #[test]
    fn raymarch_sphere() {
        let width = 256.0;
        let height = 192.0;
        let mut r = RaymarchingRenderer::new(width as u32, height as u32);
        let (device, queue) = r.renderling.get_device_and_queue_owned();
        let camera = r.slab.append(&camera(width, height));
        let hdr =
            crate::AtlasImage::from_hdr_path("../../img/hdr/helipad.hdr").unwrap_or_else(|e| {
                panic!(
                    "Failed to load HDR image: {}\ncwd: {}",
                    e,
                    std::env::current_dir().unwrap().display()
                )
            });
        let skybox = crate::Skybox::new(device, queue, hdr, camera);
        r = r.with_skybox(skybox);
        let default_material = r.slab.append(&crate::pbr::Material {
            metallic_factor: 1.0,
            roughness_factor: 0.0,
            ..Default::default()
        });
        let pbr_config = r.slab.append(&crate::pbr::PbrConfig::default());
        let raymarch = r.slab.append(&Raymarch {
            camera,
            screen_resolution: Vec2::new(width, height),
            pbr_config,
            default_material,
            ..Default::default()
        });
        let img = r.render_image(raymarch);
        img_diff::save("raymarch/sphere.png", img);
    }
}
