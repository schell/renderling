//! A few "GPU driven" renderers  with a focus on simplicity and ease of use.
//! Backed by WebGPU.
//! Shaders written in rust using `rust-gpu` (mostly).
//!
//! # WARNING
//! This is very much a work in progress.
//! YMMV.
//! PRs are very welcomed :)
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

// TODO: Audit the API and make it more ergonomic/predictable.

mod atlas;
pub mod bits;
mod camera;
pub mod convolution;
#[cfg(not(target_arch = "spirv"))]
pub mod cubemap;
#[cfg(not(target_arch = "spirv"))]
pub mod frame;
#[cfg(feature = "gltf")]
pub mod gltf;
#[cfg(not(target_arch = "spirv"))]
mod hdr;
#[cfg(not(target_arch = "spirv"))]
pub mod ibl;
#[cfg(not(target_arch = "spirv"))]
pub mod linkage;
pub mod math;
#[cfg(not(target_arch = "spirv"))]
pub mod mesh;
pub mod pbr;
#[cfg(not(target_arch = "spirv"))]
mod renderer;
pub mod sdf;
pub mod skybox;
mod stage;
#[cfg(not(target_arch = "spirv"))]
mod state;
//#[cfg(feature = "text")]
//mod text;
#[cfg(not(target_arch = "spirv"))]
mod texture;
pub mod tonemapping;
mod transform;
//mod tutorial;
//mod ui;

#[cfg(not(target_arch = "spirv"))]
mod uniform;

pub use atlas::*;
pub mod color;
pub use camera::*;
use glam::Vec3;
#[cfg(not(target_arch = "spirv"))]
pub use hdr::*;
#[cfg(not(target_arch = "spirv"))]
use image::GenericImageView;
#[cfg(not(target_arch = "spirv"))]
pub use renderer::*;
#[cfg(not(target_arch = "spirv"))]
pub use skybox::Skybox;
pub use stage::*;
#[cfg(not(target_arch = "spirv"))]
pub use state::*;
//#[cfg(feature = "text")]
//pub use text::*;
#[cfg(not(target_arch = "spirv"))]
pub use texture::*;
pub use transform::*;
//pub use ui::*;
#[cfg(not(target_arch = "spirv"))]
pub use uniform::*;

#[cfg(not(target_arch = "spirv"))]
pub mod graph {
    //! Re-exports of [`moongraph`].
    //!
    //! ## Note
    //! Only available on CPU. Not available in shaders.

    pub use moongraph::*;

    pub type RenderNode = Node<Function, TypeKey>;
}

pub use crabslab::*;
// TODO: Remove some re-exports of `graph`
#[cfg(not(target_arch = "spirv"))]
pub use graph::{graph, Graph, GraphError, Move, View, ViewMut};
use spirv_std::{
    image::{Cubemap, Image2d},
    Sampler,
};

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        #[cfg(not(target_arch = "spirv"))]
        {
            std::println!($($arg)*);
        }
    }
}

pub trait IsSampler: Copy + Clone {}

impl IsSampler for Sampler {}

pub trait Sample2d {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec2, lod: f32) -> glam::Vec4;
}

impl Sample2d for Image2d {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: glam::Vec2, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

pub trait SampleCube {
    type Sampler: IsSampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: Vec3, lod: f32) -> glam::Vec4;
}

impl SampleCube for Cubemap {
    type Sampler = Sampler;

    fn sample_by_lod(&self, sampler: Self::Sampler, uv: Vec3, lod: f32) -> glam::Vec4 {
        self.sample_by_lod(sampler, uv, lod)
    }
}

#[cfg(not(target_arch = "spirv"))]
mod cpu {
    use super::*;

    /// A CPU-side texture sampler.
    ///
    /// Provided primarily for testing purposes.
    #[derive(Debug, Clone, Copy)]
    pub struct CpuSampler;

    impl IsSampler for CpuSampler {}

    #[derive(Debug)]
    pub struct CpuTexture2d {
        pub image: image::DynamicImage,
    }

    impl Sample2d for CpuTexture2d {
        type Sampler = CpuSampler;

        fn sample_by_lod(&self, _sampler: Self::Sampler, uv: glam::Vec2, _lod: f32) -> glam::Vec4 {
            // TODO: lerp the CPU texture sampling
            let image::Rgba([r, g, b, a]) = self.image.get_pixel(uv.x as u32, uv.y as u32);
            glam::Vec4::new(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            )
        }
    }

    /// A CPU-side cubemap texture.
    ///
    /// Provided primarily for testing purposes.
    pub struct CpuCubemap {
        pub images: [image::DynamicImage; 6],
    }

    impl SampleCube for CpuCubemap {
        type Sampler = CpuSampler;

        fn sample_by_lod(
            &self,
            _sampler: Self::Sampler,
            direction: glam::Vec3,
            _lod: f32,
        ) -> glam::Vec4 {
            // Take the absolute value of the direction vector components
            let abs_direction = direction.abs();
            let (max_dim, u, v): (usize, f32, f32);

            // Determine which face of the cubemap the direction vector is pointing towards
            // by finding the largest component of the vector.
            // The u and v texture coordinates within that face are calculated by dividing
            // the other two components of the direction vector by the largest component.
            if abs_direction.x >= abs_direction.y && abs_direction.x >= abs_direction.z {
                max_dim = if direction.x >= 0.0 { 0 } else { 1 };
                u = -direction.z / abs_direction.x;
                v = -direction.y / abs_direction.x;
            } else if abs_direction.y >= abs_direction.x && abs_direction.y >= abs_direction.z {
                max_dim = if direction.y >= 0.0 { 2 } else { 3 };
                u = direction.x / abs_direction.y;
                v = -direction.z / abs_direction.y;
            } else {
                max_dim = if direction.z >= 0.0 { 4 } else { 5 };
                u = direction.x / abs_direction.z;
                v = direction.y / abs_direction.z;
            }

            // Get the dimensions of the cubemap image
            let (width, height) = self.images[max_dim].dimensions();
            // Convert the u and v coordinates from [-1, 1] to [0, width/height]
            let tex_u = ((u + 1.0) * 0.5 * (width as f32 - 1.0)).round() as u32;
            let tex_v = ((1.0 - v) * 0.5 * (height as f32 - 1.0)).round() as u32;

            // Sample and return the color from the appropriate image in the cubemap
            let pixel = self.images[max_dim].get_pixel(tex_u, tex_v);
            glam::Vec4::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
                pixel[3] as f32 / 255.0,
            )
        }
    }

    #[cfg(test)]
    #[ctor::ctor]
    fn init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Warn)
            .filter_module("moongraph", log::LevelFilter::Trace)
            .filter_module("renderling", log::LevelFilter::Trace)
            .try_init();
    }
}

#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

pub mod shader_test;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{pbr::Material, stage::Vertex};

    use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
    use pretty_assertions::assert_eq;

    #[test]
    fn sanity_transmute() {
        let zerof32 = 0f32;
        let zerof32asu32: u32 = unsafe { std::mem::transmute(zerof32) };
        assert_eq!(0, zerof32asu32);

        let foure_45 = 4e-45f32;
        let in_u32: u32 = unsafe { std::mem::transmute(foure_45) };
        assert_eq!(3, in_u32);

        let u32max = u32::MAX;
        let f32nan: f32 = unsafe { std::mem::transmute(u32max) };
        assert!(f32nan.is_nan());

        let u32max: u32 = unsafe { std::mem::transmute(f32nan) };
        assert_eq!(u32::MAX, u32max);
    }

    fn right_tri_vertices() -> Vec<Vertex> {
        vec![
            Vertex::default()
                .with_position([0.0, 0.0, 0.5])
                .with_color([0.0, 1.0, 1.0, 1.0]),
            Vertex::default()
                .with_position([0.0, 100.0, 0.5])
                .with_color([1.0, 1.0, 0.0, 1.0]),
            Vertex::default()
                .with_position([100.0, 0.0, 0.5])
                .with_color([1.0, 0.0, 1.0, 1.0]),
        ]
    }

    #[cfg(feature = "gltf")]
    #[test]
    // This tests our ability to draw a CMYK triangle in the top left corner.
    fn cmy_triangle_sanity() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let mesh = stage
            .new_mesh()
            .with_primitive(right_tri_vertices(), [], Id::NONE)
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let _tri = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            node_path,
            vertex_count: 3,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_triangle.png", img);
    }

    #[cfg(feature = "gltf")]
    #[test]
    // This tests our ability to draw a CMYK triangle in the top left corner, using
    // CW geometry.
    fn cmy_triangle_backface() {
        use img_diff::DiffCfg;

        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let mesh = stage
            .new_mesh()
            .with_primitive(
                {
                    let mut vs = right_tri_vertices();
                    vs.reverse();
                    vs
                },
                [],
                Id::NONE,
            )
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let _tri = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            node_path,
            vertex_count: 3,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "cmy_triangle.png",
            img,
            DiffCfg {
                test_name: Some("cmy_triangle_backface.png"),
                ..Default::default()
            },
        );
    }

    #[cfg(feature = "gltf")]
    #[test]
    // This tests our ability to update the transform of a `RenderUnit` after it
    // has already been sent to the GPU.
    // We do this by writing over the previous transform in the stage.
    fn cmy_triangle_update_transform() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera::new(projection, view));
        let mesh = stage
            .new_mesh()
            .with_primitive(right_tri_vertices(), [], Id::NONE)
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let transform = stage.append(&Transform::default());
        let node_path = stage.append_array(&[node]);
        let _tri = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            node_path,
            vertex_count: 3,
            transform,
            ..Default::default()
        });

        let _ = r.render_image().unwrap();

        stage.write(
            transform,
            &Transform {
                translation: Vec3::new(100.0, 0.0, 0.0),
                rotation: Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2),
                scale: Vec3::new(0.5, 0.5, 1.0),
            },
        );

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_triangle_update_transform.png", img);
    }

    /// Points around a pyramid height=1 with the base around the origin.
    ///
    ///    yb
    ///    |               *top
    ///    |___x       tl_____tr
    ///   /    g        /    /
    /// z/r          bl/____/br
    fn pyramid_points() -> [Vec3; 5] {
        let tl = Vec3::new(-0.5, -0.5, -0.5);
        let tr = Vec3::new(0.5, -0.5, -0.5);
        let br = Vec3::new(0.5, -0.5, 0.5);
        let bl = Vec3::new(-0.5, -0.5, 0.5);
        let top = Vec3::new(0.0, 0.5, 0.0);
        [tl, tr, br, bl, top]
    }

    fn pyramid_indices() -> [u16; 18] {
        let (tl, tr, br, bl, top) = (0, 1, 2, 3, 4);
        [
            tl, br, bl, tl, tr, br, br, top, bl, bl, top, tl, tl, top, tr, tr, top, br,
        ]
    }

    fn cmy_gpu_vertex(p: Vec3) -> Vertex {
        let r: f32 = p.z + 0.5;
        let g: f32 = p.x + 0.5;
        let b: f32 = p.y + 0.5;
        Vertex::default()
            .with_position([p.x.min(1.0), p.y.min(1.0), p.z.min(1.0)])
            .with_color([r, g, b, 1.0])
    }

    fn gpu_cube_vertices() -> Vec<Vertex> {
        math::UNIT_INDICES
            .iter()
            .map(|i| cmy_gpu_vertex(math::UNIT_POINTS[*i as usize]))
            .collect()
    }

    #[cfg(feature = "gltf")]
    #[test]
    // Tests our ability to draw a CMYK cube.
    fn cmy_cube_sanity() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let camera = stage.append(&Camera {
            projection: Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            view: Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
            position: camera_position,
        });
        let vertices = gpu_cube_vertices();
        let vertex_count = vertices.len() as u32;
        let mesh = stage
            .new_mesh()
            .with_primitive(vertices, [], Id::NONE)
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let transform = Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        };
        let transform = stage.append(&transform);
        let _cube = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            vertex_count,
            node_path,
            transform,
            ..Default::default()
        });
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/sanity.png", img);
    }

    #[cfg(feature = "gltf")]
    #[test]
    // Test our ability to create two cubes and toggle the visibility of one of
    // them.
    fn cmy_cube_visible() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let vertices = gpu_cube_vertices();
        let vertex_count = vertices.len() as u32;
        let mesh = stage
            .new_mesh()
            .with_primitive(vertices, [], Id::NONE)
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let mut render_unit = crate::gltf::GltfRendering {
            camera,
            vertex_count,
            node_path: stage.append_array(&[node]),
            transform: stage.append(&Transform {
                translation: Vec3::new(-4.5, 0.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            }),
            ..Default::default()
        };
        let _cube_one = stage.draw_gltf_rendering(&render_unit);
        render_unit.transform = stage.append(&Transform {
            translation: Vec3::new(4.5, 0.0, 0.0),
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4),
        });
        let (_, cube_two_rendering) = stage.draw_gltf_rendering(&render_unit);

        // we should see two colored cubes
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/visible_before.png", img.clone());
        let img_before = img;

        // update cube two making it invisible
        stage.hide_unit(cube_two_rendering);

        // we should see only one colored cube
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/visible_after.png", img);

        // update cube two making in visible again
        stage.show_unit(cube_two_rendering);

        // we should see two colored cubes again
        let img = r.render_image().unwrap();
        img_diff::assert_eq("cmy_cube/visible_before_again.png", img_before, img);
    }

    #[cfg(feature = "gltf")]
    #[test]
    // Tests the ability to specify indexed vertices, as well as the ability to
    // update a field within a struct stored on the slab by offset.
    fn cmy_cube_remesh() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage().with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let pyramid_vertices = pyramid_points().map(cmy_gpu_vertex);
        let pyramid_indices = pyramid_indices().map(|i| i as u32);
        let _pyramid_vertex_count = pyramid_indices.len();
        let pyramid_mesh = stage
            .new_mesh()
            .with_primitive(pyramid_vertices, pyramid_indices, Id::NONE)
            .build();
        let mesh = stage.append(&pyramid_mesh);
        let pyramid_node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let pyramid_node_path = stage.append_array(&[pyramid_node]);

        let cube_vertices = math::UNIT_POINTS.map(cmy_gpu_vertex);
        let cube_indices = math::UNIT_INDICES.map(|i| i as u32);
        let cube_vertex_count = cube_indices.len();
        let cube_mesh = stage
            .new_mesh()
            .with_primitive(cube_vertices, cube_indices, Id::NONE)
            .build();
        let mesh = stage.append(&cube_mesh);
        let cube_node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[cube_node]);
        let transform = stage.append(&Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        });

        let (cube, _) = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            vertex_count: cube_vertex_count as u32,
            node_path,
            transform,
            ..Default::default()
        });

        // we should see a cube (in sRGB color space)
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/remesh_before.png", img);

        // Update the cube mesh to a pyramid by overwriting the `.node_path` field
        // of `RenderUnit`
        stage.write(
            cube + crate::gltf::GltfRendering::offset_of_node_path(),
            &pyramid_node_path,
        );

        // we should see a pyramid (in sRGB color space)
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/remesh_after.png", img);
    }

    fn gpu_uv_unit_cube() -> Vec<Vertex> {
        let p: [Vec3; 8] = math::UNIT_POINTS;
        let tl = Vec2::new(0.0, 0.0);
        let tr = Vec2::new(1.0, 0.0);
        let bl = Vec2::new(0.0, 1.0);
        let br = Vec2::new(1.0, 1.0);

        vec![
            // top
            Vertex::default().with_position(p[0]).with_uv0(bl),
            Vertex::default().with_position(p[2]).with_uv0(tr),
            Vertex::default().with_position(p[1]).with_uv0(tl),
            Vertex::default().with_position(p[0]).with_uv0(bl),
            Vertex::default().with_position(p[3]).with_uv0(br),
            Vertex::default().with_position(p[2]).with_uv0(tr),
            // bottom
            Vertex::default().with_position(p[4]).with_uv0(bl),
            Vertex::default().with_position(p[6]).with_uv0(tr),
            Vertex::default().with_position(p[5]).with_uv0(tl),
            Vertex::default().with_position(p[4]).with_uv0(bl),
            Vertex::default().with_position(p[7]).with_uv0(br),
            Vertex::default().with_position(p[6]).with_uv0(tr),
            // left
            Vertex::default().with_position(p[7]).with_uv0(bl),
            Vertex::default().with_position(p[0]).with_uv0(tr),
            Vertex::default().with_position(p[1]).with_uv0(tl),
            Vertex::default().with_position(p[7]).with_uv0(bl),
            Vertex::default().with_position(p[4]).with_uv0(br),
            Vertex::default().with_position(p[0]).with_uv0(tr),
            // right
            Vertex::default().with_position(p[5]).with_uv0(bl),
            Vertex::default().with_position(p[2]).with_uv0(tr),
            Vertex::default().with_position(p[3]).with_uv0(tl),
            Vertex::default().with_position(p[5]).with_uv0(bl),
            Vertex::default().with_position(p[6]).with_uv0(br),
            Vertex::default().with_position(p[2]).with_uv0(tr),
            // front
            Vertex::default().with_position(p[4]).with_uv0(bl),
            Vertex::default().with_position(p[3]).with_uv0(tr),
            Vertex::default().with_position(p[0]).with_uv0(tl),
            Vertex::default().with_position(p[4]).with_uv0(bl),
            Vertex::default().with_position(p[5]).with_uv0(br),
            Vertex::default().with_position(p[3]).with_uv0(tr),
        ]
    }

    #[cfg(feature = "gltf")]
    #[test]
    // Tests that updating the material actually updates the rendering of an unlit
    // mesh
    fn unlit_textured_cube_material() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(0.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let sandstone = AtlasImage::from(image::open("../../img/sandstone.png").unwrap());
        let dirt = AtlasImage::from(image::open("../../img/dirt.jpg").unwrap());
        let textures = stage.set_images([sandstone, dirt]).unwrap();
        let sandstone_tex = textures[0];
        let dirt_tex = textures[1];
        let sandstone_tex_id = stage.append(&sandstone_tex);
        let material_id = stage.append(&Material {
            albedo_texture: sandstone_tex_id,
            has_lighting: false,
            ..Default::default()
        });
        let vertices = gpu_uv_unit_cube();
        let vertex_count = vertices.len() as u32;
        let mesh = stage
            .new_mesh()
            .with_primitive(vertices, [], material_id)
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let transform = stage.append(&Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        });
        let cube = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            node_path,
            transform,
            vertex_count,
            ..Default::default()
        });
        println!("cube: {cube:?}");

        // we should see a cube with a stoney texture
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_before.png", img);

        // update the material's texture on the GPU
        stage.write(sandstone_tex_id, &dirt_tex);

        // we should see a cube with a dirty texture
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_after.png", img);
    }

    #[cfg(feature = "gltf")]
    #[test]
    // Ensures that we can render multiple nodes with mesh primitives
    // that share the same buffer and geometry but have different materials.
    fn multi_node_scene() {
        let mut r =
            Renderling::headless(100, 100).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        // now test the textures functionality
        let img = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let textures = stage.set_images([img]).unwrap();
        let textures = stage.append_array(&textures);
        let cheetah_material = stage.append(&Material {
            albedo_texture: textures.at(0),
            has_lighting: false,
            ..Default::default()
        });

        let cheetah_primitive = stage.new_primitive(
            vec![
                Vertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    uv: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(100.0, 100.0, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    uv: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(100.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                    uv: Vec4::new(1.0, 0.0, 1.0, 0.0),
                    ..Default::default()
                },
            ],
            [],
            cheetah_material,
        );
        let color_primitive = {
            let mut prim = cheetah_primitive;
            prim.material = Id::NONE;
            prim
        };
        let _unit = {
            let primitives = stage.append_array(&[color_primitive]);
            let mesh = stage.append(&crate::gltf::GltfMesh {
                primitives,
                ..Default::default()
            });
            let node = stage.append(&crate::gltf::GltfNode {
                mesh,
                ..Default::default()
            });
            let node_path = stage.append_array(&[node]);

            stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
                camera,
                vertex_count: cheetah_primitive.vertex_count,
                node_path,
                ..Default::default()
            })
        };
        let _cheetah_unit = {
            let primitives = stage.append_array(&[cheetah_primitive]);
            let mesh = stage.append(&crate::gltf::GltfMesh {
                primitives,
                ..Default::default()
            });
            let node = stage.append(&crate::gltf::GltfNode {
                mesh,
                ..Default::default()
            });
            let node_path = stage.append_array(&[node]);
            let transform = stage.append(&Transform {
                translation: Vec3::new(15.0, 35.0, 0.5),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            });
            stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
                camera,
                transform,
                vertex_count: cheetah_primitive.vertex_count,
                node_path,
                ..Default::default()
            })
        };

        let img = r.render_image().unwrap();

        img_diff::assert_img_eq("gpu_scene_sanity2.png", img);
    }

    #[cfg(feature = "gltf")]
    #[test]
    /// Tests shading with directional light.
    fn scene_cube_directional() {
        use crate::pbr::light::{DirectionalLight, Light, LightStyle};

        let mut r =
            Renderling::headless(100, 100).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, _) = camera::default_perspective(100.0, 100.0);
        let view = Mat4::look_at_rh(
            Vec3::new(1.8, 1.8, 1.8),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );
        let camera = stage.append(&Camera::default().with_projection_and_view(projection, view));

        let red = Vec3::X.extend(1.0);
        let green = Vec3::Y.extend(1.0);
        let blue = Vec3::Z.extend(1.0);
        let dir_red = stage.append(&DirectionalLight {
            direction: Vec3::NEG_Y,
            color: red,
            intensity: 10.0,
        });
        let dir_green = stage.append(&DirectionalLight {
            direction: Vec3::NEG_X,
            color: green,
            intensity: 10.0,
        });
        let dir_blue = stage.append(&DirectionalLight {
            direction: Vec3::NEG_Z,
            color: blue,
            intensity: 10.0,
        });
        assert_eq!(
            Light {
                light_type: LightStyle::Directional,
                index: dir_red.inner()
            },
            dir_red.into()
        );
        let lights = stage.append_array(&[dir_red.into(), dir_green.into(), dir_blue.into()]);
        stage.set_lights(lights);

        let material = stage.append(&Material::default());
        let verts = math::unit_cube()
            .into_iter()
            .map(|(p, n)| Vertex {
                position: p.extend(1.0),
                normal: n.extend(0.0),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let vertex_count = verts.len() as u32;
        let mesh = stage.new_mesh().with_primitive(verts, [], material).build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&crate::gltf::GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let _cube = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            vertex_count,
            node_path,
            ..Default::default()
        });
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("scene_cube_directional.png", img);
    }

    #[test]
    // Test to make sure that we can reconstruct a normal matrix without using the
    // inverse transpose of a model matrix, so long as we have the T R S
    // transformation components (we really only need the scale).
    //
    // see Eric's comment here https://computergraphics.stackexchange.com/questions/1502/why-is-the-transposed-inverse-of-the-model-view-matrix-used-to-transform-the-nor?newreg=ffeabc7602da4fa2bc15fb9c84179dff
    // see Eric's blog post here https://lxjk.github.io/2017/10/01/Stop-Using-Normal-Matrix.html
    // squaring a vector https://math.stackexchange.com/questions/1419887/squaring-a-vector#1419889
    // more convo wrt shaders https://github.com/mrdoob/three.js/issues/18497
    fn square_scale_norm_check() {
        let quat = Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_4);
        let scale = Vec3::new(10.0, 20.0, 1.0);
        let model_matrix = Mat4::from_translation(Vec3::new(10.0, 10.0, 20.0))
            * Mat4::from_quat(quat)
            * Mat4::from_scale(scale);
        let normal_matrix = model_matrix.inverse().transpose();
        let scale2 = scale * scale;

        for i in 0..9 {
            for j in 0..9 {
                for k in 0..9 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }
                    let norm = Vec3::new(i as f32, j as f32, k as f32).normalize();
                    let model = Mat3::from_mat4(model_matrix);
                    let norm_a = (Mat3::from_mat4(normal_matrix) * norm).normalize();
                    let norm_b = (model * (norm / scale2)).normalize();
                    assert!(
                        norm_a.abs_diff_eq(norm_b, f32::EPSILON),
                        "norm:{norm}, scale2:{scale2}"
                    );
                }
            }
        }
    }

    #[cfg(feature = "gltf")]
    #[test]
    // tests that nested children are transformed by their parent's transform
    fn scene_parent_sanity() {
        let mut r = Renderling::headless(100, 100);
        r.set_background_color(Vec4::splat(0.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera::new(projection, view));
        let size = 1.0;
        let cyan_material = stage.append(&Material {
            albedo_factor: Vec4::new(0.0, 1.0, 1.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });
        let yellow_material = stage.append(&Material {
            albedo_factor: Vec4::new(1.0, 1.0, 0.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });
        let red_material = stage.append(&Material {
            albedo_factor: Vec4::new(1.0, 0.0, 0.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });

        let cyan_primitive = stage.new_primitive(
            [
                Vertex::default().with_position([0.0, 0.0, 0.0]),
                Vertex::default().with_position([size, size, 0.0]),
                Vertex::default().with_position([size, 0.0, 0.0]),
            ],
            [],
            cyan_material,
        );
        let yellow_primitive = {
            let mut p = cyan_primitive;
            p.material = yellow_material;
            p
        };
        let red_primitive = {
            let mut p = cyan_primitive;
            p.material = red_material;
            p
        };

        let root_node = stage.allocate::<crate::gltf::GltfNode>();
        let cyan_node = stage.allocate::<crate::gltf::GltfNode>();
        let yellow_node = stage.allocate::<crate::gltf::GltfNode>();
        let red_node = stage.allocate::<crate::gltf::GltfNode>();

        // Write the nodes now that we have references to them all
        let children = stage.append_array(&[cyan_node]);
        stage.write(
            root_node,
            &crate::gltf::GltfNode {
                children,
                scale: Vec3::new(25.0, 25.0, 1.0),
                ..Default::default()
            },
        );

        let primitives = stage.append_array(&[cyan_primitive]);
        let children = stage.append_array(&[yellow_node]);
        let mesh = stage.append(&crate::gltf::GltfMesh {
            primitives,
            ..Default::default()
        });
        stage.write(
            cyan_node,
            &crate::gltf::GltfNode {
                mesh,
                children,
                translation: Vec3::new(1.0, 1.0, 0.0),
                ..Default::default()
            },
        );

        let primitives = stage.append_array(&[yellow_primitive]);
        let children = stage.append_array(&[red_node]);
        let mesh = stage.append(&crate::gltf::GltfMesh {
            primitives,
            ..Default::default()
        });
        stage.write(
            yellow_node,
            &crate::gltf::GltfNode {
                mesh,
                children,
                translation: Vec3::new(1.0, 1.0, 0.0),
                ..Default::default()
            },
        );

        let primitives = stage.append_array(&[red_primitive]);
        let mesh = stage.append(&crate::gltf::GltfMesh {
            primitives,
            ..Default::default()
        });
        stage.write(
            red_node,
            &crate::gltf::GltfNode {
                mesh,
                translation: Vec3::new(1.0, 1.0, 0.0),
                ..Default::default()
            },
        );

        let node_path = stage.append_array(&[root_node, cyan_node]);
        let _cyan_unit = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            vertex_count: 3,
            node_path,
            ..Default::default()
        });

        let node_path = stage.append_array(&[root_node, cyan_node, yellow_node]);
        let _yellow_unit = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            vertex_count: 3,
            node_path,
            ..Default::default()
        });

        let node_path = stage.append_array(&[root_node, cyan_node, yellow_node, red_node]);
        let _red_unit = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
            camera,
            vertex_count: 3,
            node_path,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("scene_parent_sanity.png", img);
    }

    #[test]
    // sanity tests that we can extract the position of the camera using the
    // camera's view transform
    fn camera_position_from_view_matrix() {
        let position = Vec3::new(1.0, 2.0, 12.0);
        let view = Mat4::look_at_rh(position, Vec3::new(1.0, 2.0, 0.0), Vec3::Y);
        let extracted_position = view.inverse().transform_point3(Vec3::ZERO);
        assert_eq!(position, extracted_position);
    }

    #[cfg(feature = "gltf")]
    #[test]
    // Tests the initial implementation of pbr metallic roughness on an array of
    // spheres with different metallic roughnesses lit by an environment map.
    //
    // see https://learnopengl.com/PBR/Lighting
    fn pbr_metallic_roughness_spheres() {
        let ss = 600;
        let mut r =
            Renderling::headless(ss, ss).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let radius = 0.5;
        let ss = ss as f32;
        let projection = camera::perspective(ss, ss);
        let k = 7;
        let diameter = 2.0 * radius;
        let spacing = radius * 0.25;
        let len = (k - 1) as f32 * (diameter + spacing);
        let half = len / 2.0;
        let view = camera::look_at(
            Vec3::new(half, half, 1.6 * len),
            Vec3::new(half, half, 0.0),
            Vec3::Y,
        );
        let camera = stage.append(&Camera::new(projection, view));

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
        let sphere_vertices = cells.iter().flat_map(|icosahedron::Triangle { a, b, c }| {
            let p0 = to_vertex(&a);
            let p1 = to_vertex(&b);
            let p2 = to_vertex(&c);
            vec![p0, p1, p2]
        });
        let sphere_primitive = stage.new_primitive(sphere_vertices, [], Id::NONE);
        for i in 0..k {
            let roughness = i as f32 / (k - 1) as f32;
            let x = (diameter + spacing) * i as f32;
            for j in 0..k {
                let metallic = j as f32 / (k - 1) as f32;
                let y = (diameter + spacing) * j as f32;
                let mut prim = sphere_primitive;
                prim.material = stage.append(&Material {
                    albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    metallic_factor: metallic,
                    roughness_factor: roughness,
                    ..Default::default()
                });
                let primitives = stage.append_array(&[prim]);
                let mesh = stage.append(&crate::gltf::GltfMesh {
                    primitives,
                    ..Default::default()
                });
                let node = stage.append(&crate::gltf::GltfNode {
                    mesh,
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                });
                let node_path = stage.append_array(&[node]);
                let _entity = stage.draw_gltf_rendering(&crate::gltf::GltfRendering {
                    camera,
                    vertex_count: prim.vertex_count,
                    node_path,
                    ..Default::default()
                });
            }
        }

        let (device, queue) = r.get_device_and_queue_owned();
        let hdr_image = AtlasImage::from_hdr_path("../../img/hdr/resting_place.hdr").unwrap();
        let skybox = crate::skybox::Skybox::new(device, queue, hdr_image, camera);
        stage.set_skybox(skybox);

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("pbr/metallic_roughness_spheres.png", img);
    }
}
