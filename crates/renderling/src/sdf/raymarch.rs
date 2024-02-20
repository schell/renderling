//! Raymarching functions and shaders.
use crabslab::{Id, Slab, SlabItem};
use glam::{Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::spirv;

use crate::{
    math::{self, IsVector},
    Camera,
};

use super::Sdf;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;
#[cfg(not(target_arch = "spirv"))]
pub mod renderer;

#[derive(Clone, Copy, Default, SlabItem)]
pub struct Raymarch {
    pub screen_resolution: Vec2,
    pub pbr_config: Id<crate::pbr::PbrConfig>,
    pub default_material: Id<crate::pbr::Material>,
    pub scene: Id<crate::sdf::Scene>,
}

// TODO: Compute raymarching rays in the raymarching vertex shader
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

/// Converts a fragment from screen space to a normalized device space.
///
/// See [WGPU's docs on clip space](https://www.w3.org/TR/webgpu/#primitive-clipping).
///
/// ### tl;dr
/// - x: find `frag.x` where `[-1.0 (left), 1.0 (right)]`
/// - y: find `frag.y` where `[-1.0 (bottom), 1.0 (top)]`
/// - z: `frag.z`, where `[0.0, 1.0]` (which is near depends on the projection)
fn frag_coord_to_ndc(frag_coord: Vec3, screen_resolution: Vec2) -> Vec3 {
    let invert_y = Vec2::new(1.0, -1.0);
    ((2.0 * frag_coord.xy() / screen_resolution - 1.0) * invert_y).extend(frag_coord.z)
}

fn ndc_coord_to_world(clip_coord: Vec3, camera: &Camera) -> Vec3 {
    let p = (camera.projection * camera.view).inverse() * clip_coord.extend(1.0);
    p.xyz() / p.w
}

/// Convert the fragment coordinate to world space.
fn frag_coord_to_world(frag_coord: Vec3, screen_resolution: Vec2, camera: &Camera) -> Vec3 {
    // Convert fragment coordinates to clip
    let clip_coord = frag_coord_to_ndc(frag_coord, screen_resolution);
    // Transform point to world space
    let world_point = ndc_coord_to_world(clip_coord.xyz(), camera);
    world_point.xyz()
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct MarchResult {
    pub position: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub bitangent: Vec3,
    /// The total distance marched until finding a hit
    pub distance_marched: f32,
    /// The minimum step taken during the march
    pub min_step: f32,
    /// The total number of steps marched
    pub step_count: u32,
    pub hit: bool,
    pub id: Id<Sdf>,
}

impl Default for MarchResult {
    fn default() -> Self {
        MarchResult {
            position: Vec3::ZERO,
            normal: Vec3::Z,
            tangent: Vec3::X,
            bitangent: Vec3::Y,
            distance_marched: 0.0,
            min_step: f32::MAX,
            step_count: 0,
            hit: false,
            id: Id::NONE,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, SlabItem)]
pub struct Ray {
    /// Origin of the ray
    pub origin: Vec3,
    /// Direction of the ray
    pub direction: Vec3,
    /// Length of the ray (from near to far plane)
    pub length: f32,
}

impl Ray {
    /// Returns the ray origin and direction for the given fragment coordinate.
    pub fn from_fragment(frag_coord: Vec2, resolution: Vec2, camera: &Camera) -> Ray {
        // Assumes a right-handed projection matrix where 1.0 is near and 0.0 is far
        let frustum_near = frag_coord_to_world(frag_coord.extend(0.0), resolution, camera);
        let frustum_far = frag_coord_to_world(frag_coord.extend(1.0), resolution, camera);
        let vector = frustum_far - frustum_near;
        let direction = vector.alt_norm_or_zero();
        let length = vector.length();
        Ray {
            origin: frustum_near,
            direction,
            length,
        }
    }

    const MAX_STEPS: u32 = 2048;

    /// Perform raymarching.
    pub fn march(
        self,
        scene: &crate::sdf::Scene,
        slab: &[u32],
        ray_dx: Ray,
        ray_dy: Ray,
    ) -> MarchResult {
        let Ray {
            origin,
            direction,
            length: max_dist,
        } = self;

        let mut result = MarchResult::default();

        for i in 0..Self::MAX_STEPS {
            result.step_count = i + 1;

            result.position = origin + result.distance_marched * direction;
            let (current_dist, id) = (super::Sphere::distance(result.position, 1.0), Id::NONE); // scene.distance_estimate(result.position, slab);

            // variable minimum distance based on the pixel footprint
            let min_dist = result
                .position
                .distance(ray_dx.origin + result.distance_marched * ray_dx.direction)
                .min(
                    result
                        .position
                        .distance(ray_dy.origin + result.distance_marched * ray_dy.direction),
                );
            if current_dist < min_dist {
                result.normal = find_normal_by_gradient(result.position, min_dist, scene, slab);
                let (tangent, bitangent) = produce_tangent_and_bitangent(result.normal);
                result.tangent = tangent;
                result.bitangent = bitangent;
                result.hit = true;
                result.id = id;
                break;
            } else if current_dist >= max_dist {
                break;
            }

            result.distance_marched += current_dist;
            result.min_step = result.min_step.min(min_dist);
        }

        result
    }
}

pub fn find_normal_by_gradient(
    p: Vec3,
    epsilon: f32,
    scene: &crate::sdf::Scene,
    slab: &[u32],
) -> Vec3 {
    let dx = Vec3::new(epsilon, 0.0, 0.0);
    let dy = Vec3::new(0.0, epsilon, 0.0);
    let dz = Vec3::new(0.0, 0.0, epsilon);
    let mut distances = [0f32; 6];
    let sample_points = [p + dx, p - dx, p + dy, p - dy, p + dz, p - dz];
    for i in 0..6 {
        let point = sample_points[i];
        distances[i] = scene.distance_estimate(point, slab).0;
    }
    Vec3::new(
        distances[0] - distances[1],
        distances[2] - distances[3],
        distances[4] - distances[5],
    )
    .alt_norm_or_zero()
}

pub fn produce_tangent_and_bitangent(normal: Vec3) -> (Vec3, Vec3) {
    let tangent = normal.cross(Vec3::new(-normal.z, normal.x, normal.y));
    let bitangent = Vec3::cross(normal, tangent);
    (tangent, bitangent)
}

pub fn antialias_distance(_distance: f32) -> f32 {
    #[cfg(not(target_arch = "spirv"))]
    {
        1.0
    }
    #[cfg(target_arch = "spirv")]
    {
        let distance_change = spirv_std::arch::fwidth(_distance);
        let opacity = math::smoothstep(distance_change, -distance_change, _distance);
        opacity
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
    let scene = slab.read(cfg.scene);
    let camera = slab.read(scene.camera);
    let Ray {
        origin: _,
        direction,
        length: _,
    } = Ray::from_fragment(frag_coord.xy(), cfg.screen_resolution, &camera);
    *out_color = direction.extend(1.0);
}

#[spirv(fragment)]
pub fn raymarch_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(flat)] raymarch: Id<Raymarch>,
    out_color: &mut Vec4,
) {
    let cfg = slab.read(raymarch);
    let scene = slab.read(cfg.scene);
    let camera = slab.read(scene.camera);
    let resolution = cfg.screen_resolution;

    let fragment = frag_coord.xy() + 0.5;
    let fragment_dx = fragment + Vec2::new(0.5, 0.0);
    let fragment_dy = fragment + Vec2::new(0.0, 0.5);

    let ray = Ray::from_fragment(fragment, resolution, &camera);
    let ray_dx = Ray::from_fragment(fragment_dx, resolution, &camera);
    let ray_dy = Ray::from_fragment(fragment_dy, resolution, &camera);

    let result = ray.march(&scene, slab, ray_dx, ray_dy);
    let mut color = Vec4::new(1.0, 0.0, 0.0, 1.0);
    if result.hit {
        // TODO: use ray differentials (pixel footprint) for anti-aliasing
        let sdf = slab.read(result.id);
        let albedo = sdf.albedo(slab, result.position);
        //let material = shape.color(result.position, slab);
        //let material = slab.read(material);
        //color = material.albedo_factor;
        color = albedo.extend(1.0);
        // crate::pbr::fragment_impl(
        //     atlas,
        //     atlas_sampler,
        //     irradiance,
        //     irradiance_sampler,
        //     prefiltered,
        //     prefiltered_sampler,
        //     brdf,
        //     brdf_sampler,
        //     slab,
        //     cfg.pbr_config,
        //     scene.camera.inner(),
        //     material.inner(),
        //     Vec4::ONE,  // albedo color
        //     Vec2::ZERO, // uv0
        //     Vec2::ZERO, // uv1
        //     result.normal,
        //     result.tangent,
        //     result.bitangent,
        //     result.position,
        //     &mut color,
        // );
    } else {
        // let background = prefiltered
        //     .sample_by_lod(*prefiltered_sampler, ray.direction, 0.0)
        //     .xyz();
        // color = background.extend(1.0);
    }
    *out_color = color;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        sdf::{raymarch::renderer::RaymarchingRenderer, Sdf},
        AtlasImage, Skybox,
    };
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
    fn raymarch_sanity_can_determine_glam_matrix_is_orthographic() {
        let persp = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 16.0 / 9.0, 0.1, 100.0);
        let ortho = Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, 1.0, 0.0);

        fn is_ortho(m: &Mat4) -> bool {
            m.w_axis.w == 1.0
        }

        assert!(!is_ortho(&persp));
        assert!(is_ortho(&ortho));
    }

    #[test]
    fn raymarch_sanity_ray_from_fragment() {
        // obtain rays along the diagonal of the screen
        fn diagonal(resolution: Vec2, camera: &Camera) -> [Ray; 3] {
            [
                // top left
                Ray::from_fragment(Vec2::ZERO, resolution, camera),
                // center
                Ray::from_fragment(resolution * 0.5, resolution, camera),
                // bottom right
                Ray::from_fragment(resolution, resolution, camera),
            ]
        }

        // ensure perspective fragments and rays are as expected
        let resolution = Vec2::splat(10.0);
        let camera = Camera::new(
            Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(Vec3::NEG_Z, Vec3::ZERO, Vec3::Y),
        );
        let rays = diagonal(resolution, &camera);
        assert_eq!(
            Vec2::ZERO,
            rays[1].origin.xy(),
            "Center ray origin of perspective should be at XY 0"
        );
        assert!(
            rays[0].direction.x > 0.0,
            "Perspective top left direction looks along positive because of right handedness x: {:?}",
            rays[0]
        );
        assert_eq!(
            Vec3::Z,
            rays[1].direction,
            "Center ray direction of perspective should be at Z"
        );
        assert_eq!(
            rays[0].direction.xy(),
            rays[2].direction.xy() * -1.0,
            "Perspective top left direction xy is inverse of bottom left direction"
        );

        // ensure orthographic 2d fragments and rays are as expected
        let camera = Camera::new(
            Mat4::orthographic_rh(0.0, 10.0, 10.0, 0.0, 1.0, -1.0),
            Mat4::IDENTITY,
        );
        let rays = diagonal(resolution, &camera);
        assert!(
            rays.iter().all(|r| r.direction == Vec3::Z),
            "Orthographic rays are parallel and Z"
        );
        assert_eq!(
            Vec2::ZERO,
            rays[0].origin.xy(),
            "Orthographic top left is origin"
        );

        // Orthographic bottom right is 10,10
        assert_approx_eq::assert_approx_eq!(rays[2].origin.x, 10.0, 0.0001);
        assert_approx_eq::assert_approx_eq!(rays[2].origin.y, 10.0, 0.0001);

        // ensure orthographic 3d fragments and rays are as expected
        let width = 10.0;
        let height = 10.0;
        let projection = Mat4::orthographic_rh(
            -width * 0.5,
            width * 0.5,
            -height * 0.5,
            height * 0.5,
            -1.0,
            1.0,
        );
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let camera = Camera::new(projection, view);
        let rays = diagonal(resolution, &camera);
        assert!(rays.iter().all(|r| r.direction == Vec3::NEG_Z));
    }

    #[test]
    fn raymarch_sanity_frag_coord_to_ndc() {
        let resolution = Vec2::new(10.0, 10.0);
        let top_left = Vec3::ZERO;
        let center = resolution.extend(0.0) * 0.5;
        let bottom_right = resolution.extend(0.0);

        let ndc_left = -1.0;
        let ndc_right = 1.0;
        let ndc_top = 1.0;
        let ndc_bottom = -1.0;
        let ndc_near = 0.0;

        assert_eq!(
            Vec3::new(ndc_left, ndc_top, ndc_near),
            frag_coord_to_ndc(top_left, resolution),
            "Top left fragment should be ndc top left"
        );
        assert_eq!(
            Vec3::new(0.0, 0.0, 0.0),
            frag_coord_to_ndc(center, resolution),
            "Center fragment should be the origin"
        );
        assert_eq!(
            Vec3::new(ndc_right, ndc_bottom, ndc_near),
            frag_coord_to_ndc(bottom_right, resolution),
            "Bottom right fragment should be ndc bottom right"
        );
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
    fn raymarch_draw_perspective_rays() {
        let width = 16.0;
        let height = 9.0;
        let mut r = RaymarchingRenderer::headless(width as u32, height as u32);
        let camera = r.slab.append(&{
            let projection =
                Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
            let view = Mat4::look_at_rh(Vec3::NEG_Z, Vec3::ZERO, Vec3::Y);
            Camera::new(projection, view)
        });
        let scene = r.slab.append(&crate::sdf::Scene {
            camera,
            ..Default::default()
        });
        let raymarch = r.slab.append(&Raymarch {
            screen_resolution: Vec2::new(width, height),
            scene,
            ..Default::default()
        });
        let img = r.render_rays(raymarch);
        img_diff::assert_img_eq("raymarch/perspective_rays.png", img);
    }

    #[test]
    fn raymarch_draw_ortho_rays() {
        let width = 16.0;
        let height = 9.0;
        let mut r = RaymarchingRenderer::headless(width as u32, height as u32);
        let projection = Mat4::orthographic_rh(0.0, width, height, 0.0, 1.0, -1.0);
        let view = Mat4::IDENTITY;
        let camera = Camera::new(projection, view);
        let camera = r.slab.append(&camera);
        let scene = r.slab.append(&crate::sdf::Scene {
            camera,
            ..Default::default()
        });
        let raymarch = r.slab.append(&Raymarch {
            screen_resolution: Vec2::new(width, height),
            scene,
            ..Default::default()
        });
        let img = r.render_rays(raymarch);
        img_diff::assert_img_eq("raymarch/ortho_rays.png", img);
    }

    #[test]
    fn raymarch_sanity_sphere_hit() {
        let width = 16.0;
        let height = 9.0;
        let mut slab = crabslab::CpuSlab::new(vec![]);
        // The projection is:
        //      +Y  -Z
        //       ^ /
        // -X <- * -> +X
        //     / v
        //   +Z -Y
        let projection = Mat4::orthographic_rh(
            -width * 0.5,
            width * 0.5,
            -height * 0.5,
            height * 0.5,
            -height * 0.5,
            height * 0.5,
        );
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
        let camera = Camera::new(projection, view);

        let camera_id = slab.append(&camera);
        let _default_material = slab.append(&crate::pbr::Material {
            albedo_factor: Vec4::new(1.0, 0.0, 0.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });
        let _pbr_config = slab.append(&crate::pbr::PbrConfig::default());
        // red
        let albedo_color = slab.append(&Vec3::X);
        let distance = sdf_sphere(input_position(), 1.0);
        let sdf = crate::sdf::Sdf {
            distance: slab.append_array(distance.as_ref()),
            albedo: slab.append_array(var(albedo_color).as_ref()),
        };
        let sdfs = slab.append_array(&[sdf]);
        let scene = crate::sdf::Scene {
            camera: camera_id,
            sdfs,
            ..Default::default()
        };
        let _scene_id = slab.append(&scene);
        let resolution = Vec2::new(width, height);

        let middle_fragment = resolution * 0.5;

        let near_point = frag_coord_to_world(middle_fragment.extend(0.0), resolution, &camera);
        println!("near_point: {near_point}");
        let far_point = frag_coord_to_world(middle_fragment.extend(1.0), resolution, &camera);
        println!("far_point: {far_point}");

        let ray = Ray::from_fragment(middle_fragment, resolution, &camera);
        let ray_dx =
            Ray::from_fragment(middle_fragment + Vec2::new(0.001, 0.0), resolution, &camera);
        let ray_dy =
            Ray::from_fragment(middle_fragment + Vec2::new(0.0, 0.001), resolution, &camera);
        println!("ray: {ray:#?}");
        let result = ray.march(&scene, slab.as_ref(), ray_dx, ray_dy);
        pretty_assertions::assert_eq!(
            MarchResult {
                hit: true,
                position: Vec3::Z,
                normal: Vec3::Z,
                tangent: Vec3::NEG_Y,
                bitangent: Vec3::X,
                distance_marched: 6.5000005,
                min_step: 0.0009999275,
                step_count: 2,
                id: sdfs.at(0)
            },
            result
        );
    }

    #[test]
    fn raymarch_sphere() {
        let width = 256.0;
        let height = 128.0;
        let mut r = RaymarchingRenderer::headless(width as u32, height as u32);
        let projection =
            Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 3.0, 5.0), Vec3::ZERO, Vec3::Y);
        let camera = Camera::new(projection, view);
        let camera_id = r.slab.append(&camera);
        let (device, queue) = r.renderling.get_device_and_queue_owned();
        let hdr = AtlasImage::from_hdr_path("../../img/hdr/helipad.hdr")
            .unwrap_or_else(|e| panic!("could not load hdr: {e}\n{:?}", std::env::current_dir()));
        let skybox = Skybox::new(device, queue, hdr, camera_id);
        r = r.with_skybox(skybox);
        let default_material = r.slab.append(&crate::pbr::Material {
            albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
            metallic_factor: 0.7,
            roughness_factor: 0.3,
            has_lighting: true,
            ..Default::default()
        });
        let pbr_config = r.slab.append(&crate::pbr::PbrConfig::default());

        let mut sdfs = vec![];

        let sdf = Sdf {
            distance: r.slab.append_array(&sdf_sphere(input_position(), 1.0)),
            albedo: r.slab.append_array(&constant(Vec3::new(1.0, 1.0, 0.0))),
        };
        sdfs.push(sdf);

        let sdf = Sdf {
            distance: r
                .slab
                .append_array(&sdf_plane(input_position(), Vec3::Y, 0.0)),
            albedo: r.slab.append_array(&constant(Vec3::new(0.0, 1.0, 1.0))),
        };
        sdfs.push(sdf);

        let sdfs = r.slab.append_array(&sdfs);

        let scene = r.slab.append(&crate::sdf::Scene {
            camera: camera_id,
            sdfs,
            ..Default::default()
        });
        let raymarch = r.slab.append(&Raymarch {
            scene,
            screen_resolution: Vec2::new(width, height),
            pbr_config,
            default_material,
            ..Default::default()
        });
        let img = r.render_image(raymarch);
        img_diff::save("raymarch/sphere.png", img);
    }

    //#[test]
    // fn raymarch_translated_sphere() {
    //     let width = 64.0;
    //     let height = 36.0;
    //     let mut r = RaymarchingRenderer::new(width as u32, height as u32);
    //     let projection =
    //         Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
    //     let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    //     let camera = Camera::new(projection, view);
    //     let camera_id = r.slab.append(&camera);
    //     let (device, queue) = r.renderling.get_device_and_queue_owned();
    //     let hdr = AtlasImage::from_hdr_path("../../img/hdr/helipad.hdr")
    //         .unwrap_or_else(|e| panic!("could not load hdr: {e}\n{:?}", std::env::current_dir()));
    //     let skybox = Skybox::new(device, queue, hdr, camera_id);
    //     r = r.with_skybox(skybox);
    //     let default_material = r.slab.append(&crate::pbr::Material {
    //         albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
    //         metallic_factor: 0.7,
    //         roughness_factor: 0.3,
    //         has_lighting: true,
    //         ..Default::default()
    //     });
    //     let pbr_config = r.slab.append(&crate::pbr::PbrConfig::default());
    //     let sphere = r.slab.append(&crate::sdf::Sphere { radius: 1.0 });
    //     let sphere_shape = r.slab.append(&crate::sdf::SdfPrim::Sphere(sphere));
    //     let translated = r.slab.append(&crate::sdf::Transformed {
    //         shape: sphere_shape,
    //         transform: Transform {
    //             translation: Vec3::new(3.0, 0.0, 0.0),
    //             ..Default::default()
    //         },
    //     });
    //     let shapes = r
    //         .slab
    //         .append_array(&[crate::sdf::SdfPrim::Transformed(translated)]);
    //     let scene = r.slab.append(&crate::sdf::Scene {
    //         camera: camera_id,
    //         shapes,
    //         ..Default::default()
    //     });
    //     let raymarch = r.slab.append(&Raymarch {
    //         scene,
    //         screen_resolution: Vec2::new(width, height),
    //         pbr_config,
    //         default_material,
    //         ..Default::default()
    //     });
    //     let img = r.render_image(raymarch);
    //     img_diff::assert_img_eq("raymarch/translated_sphere.png", img);
    // }

    //#[test]
    // fn raymarch_scene() {
    //     let width = 512.0;
    //     let height = 288.0;
    //     let mut r = RaymarchingRenderer::new(width as u32, height as u32);
    //     let projection =
    //         Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
    //     let view = Mat4::look_at_rh(Vec3::new(0.0, 3.0, 5.0), Vec3::ZERO, Vec3::Y);
    //     let camera = Camera::new(projection, view);
    //     let camera_id = r.slab.append(&camera);
    //     let (device, queue) = r.renderling.get_device_and_queue_owned();
    //     let hdr = AtlasImage::from_hdr_path("../../img/hdr/helipad.hdr")
    //         .unwrap_or_else(|e| panic!("could not load hdr: {e}\n{:?}", std::env::current_dir()));
    //     let skybox = Skybox::new(device, queue, hdr, camera_id);
    //     r = r.with_skybox(skybox);
    //     let default_material = r.slab.append(&crate::pbr::Material {
    //         albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
    //         metallic_factor: 0.7,
    //         roughness_factor: 0.3,
    //         has_lighting: true,
    //         ..Default::default()
    //     });
    //     let [cyan, canary, magenta] = [
    //         Vec4::new(0.0, 1.0, 1.0, 1.0),
    //         Vec4::new(1.0, 1.0, 0.0, 1.0),
    //         Vec4::new(1.0, 0.0, 1.0, 0.0),
    //     ]
    //     .map(|albedo_factor| {
    //         r.slab.append(&crate::pbr::Material {
    //             albedo_factor,
    //             metallic_factor: 0.5,
    //             roughness_factor: 0.5,
    //             ..Default::default()
    //         })
    //     });
    //     let pbr_config = r.slab.append(&crate::pbr::PbrConfig::default());

    //     let sphere = r.slab.append(&crate::sdf::Sphere { radius: 1.0 });
    //     let sphere_shape = r.slab.append(&crate::sdf::SdfPrim::Sphere(sphere));
    //     let translated_sphere = r.slab.append(&crate::sdf::Transformed {
    //         shape: sphere_shape,
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 1.0, 0.0),
    //             ..Default::default()
    //         },
    //     });

    //     let cube = r.slab.append(&crate::sdf::Cuboid {
    //         size: Vec3::new(2.0, 0.75, 2.0),
    //     });
    //     let cube_shape = r.slab.append(&crate::sdf::SdfPrim::Cuboid(cube));
    //     let cube_material = r.slab.append(&crate::sdf::Materialized {
    //         shape: cube_shape,
    //         material: cyan,
    //     });
    //     let cube_mat = r
    //         .slab
    //         .append(&crate::sdf::SdfPrim::Materialized(cube_material));
    //     let transformed_cube = r.slab.append(&crate::sdf::Transformed {
    //         shape: cube_mat,
    //         transform: Transform {
    //             translation: Vec3::new(0.0, -1.0, 0.0),
    //             rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_4),
    //             ..Default::default()
    //         },
    //     });

    //     let shapes = r.slab.append_array(&[
    //         crate::sdf::SdfPrim::Transformed(translated_sphere),
    //         crate::sdf::SdfPrim::Transformed(transformed_cube),
    //     ]);
    //     let scene = r.slab.append(&crate::sdf::Scene {
    //         camera: camera_id,
    //         shapes,
    //         ..Default::default()
    //     });
    //     let raymarch = r.slab.append(&Raymarch {
    //         scene,
    //         screen_resolution: Vec2::new(width, height),
    //         pbr_config,
    //         default_material,
    //         ..Default::default()
    //     });
    //     let img = r.render_image(raymarch);
    //     img_diff::save("raymarch/scene.png", img);
    // }
}
