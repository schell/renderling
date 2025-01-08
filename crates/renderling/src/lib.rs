//! A "GPU driven" renderer with a focus on simplicity and ease of use.
//! Backed by WebGPU.
//!
//! Shaders are written in Rust using `rust-gpu`.
//!
//! All data is staged on the GPU using a [slab allocator](crate::slab).
//!
//! ## Hello triangle
//!
//! First you must create a [`crate::context::Context`].
//!
//! ```
//! use renderling::Context;
//!
//! // create a headless context with dimensions 100, 100.
//! let ctx = Context::headless(100, 100);
//! ```
//!
//! Then create a stage to place your camera, geometry, materials and lights.
//!
//! ```
//! # use renderling::Context;
//! use renderling::stage::Stage;
//! # let ctx = Context::headless(100, 100);
//! let stage: Stage = ctx
//!     .new_stage()
//!     .with_background_color([1.0, 1.0, 1.0, 1.0])
//!     // For this demo we won't use lighting
//!     .with_lighting(false);
//! ```
//!
//! The stage is neat in that it allows you to place values and arrays of values
//! directly onto the GPU. Those values can be modified on the CPU and
//! synchronization will happen during
//! [`Stage::render`](crate::stage::Stage::render). These values are called
//! [`Hybrid`](crate::slab::Hybrid)s and
//! [`HybridArray`](crate::slab::HybridArray)s.
//!
//! ```
//! # use renderling::{Context, stage::Stage};
//! # let ctx = Context::headless(100, 100);
//! # let stage: Stage = ctx.new_stage();
//! use renderling::slab::{Hybrid, HybridArray};
//!
//! let an_f32: Hybrid<f32> = stage.new_value(1337.0);
//!
//! let an_array_of_tuples: HybridArray<(f32, u32, bool)> =
//!     stage.new_array([(0.0, 0, false), (1.0, 1, true)]);
//! ```
//!
//! In order to render, we need to "stage" a
//! [`Renderlet`](crate::stage::Renderlet), which is a bundle of rendering
//! resources. We do this in the same way we "staged" `an_f32` above.
//!
//! But first we'll need a [`Camera`](crate::camera::Camera) and some vertices
//! of [`Vertex`](crate::stage::Vertex) organized as triangles with
//! counter-clockwise winding.
//!
//! ```
//! # use renderling::{Context, stage::Stage};
//! # let ctx = Context::headless(100, 100);
//! # let stage: Stage = ctx.new_stage();
//! use renderling::{
//!     camera::Camera,
//!     stage::{Renderlet, Vertex},
//! };
//! let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
//! let vertices = stage.new_array([
//!     Vertex::default()
//!         .with_position([0.0, 0.0, 0.0])
//!         .with_color([0.0, 1.0, 1.0, 1.0]),
//!     Vertex::default()
//!         .with_position([0.0, 100.0, 0.0])
//!         .with_color([1.0, 1.0, 0.0, 1.0]),
//!     Vertex::default()
//!         .with_position([100.0, 0.0, 0.0])
//!         .with_color([1.0, 0.0, 1.0, 1.0]),
//! ]);
//! let triangle = stage.new_value(Renderlet {
//!     camera_id: camera.id(),
//!     vertices_array: vertices.array(),
//!     ..Default::default()
//! });
//! ```
//!
//! This gives us `triangle`, which is a `Hybrid<Renderlet>`. We can now pass
//! `triangle` to the stage to draw each frame using
//! [`Stage::add_renderlet`](crate::stage::Stage::add_renderlet).
//!
//! Finally, we get the next frame from the context with
//! [`Context::get_next_frame`], render to it using
//! [`Stage::render`](crate::stage::Stage::render) and then present the
//! frame with [`Frame::present`].
//!
//! ```
//! # use renderling::{
//! #     Context,
//! #     camera::Camera,
//! #     stage::{
//! #         Vertex,
//! #         Renderlet,
//! #     }
//! # };
//! # let ctx = Context::headless(100, 100);
//! # let stage = ctx.new_stage();
//! # let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
//! # let vertices = stage.new_array([
//! #     Vertex::default()
//! #         .with_position([0.0, 0.0, 0.0])
//! #         .with_color([0.0, 1.0, 1.0, 1.0]),
//! #     Vertex::default()
//! #         .with_position([0.0, 100.0, 0.0])
//! #         .with_color([1.0, 1.0, 0.0, 1.0]),
//! #     Vertex::default()
//! #         .with_position([100.0, 0.0, 0.0])
//! #         .with_color([1.0, 0.0, 1.0, 1.0]),
//! # ]);
//! # let triangle = stage.new_value(Renderlet {
//! #     camera_id: camera.id(),
//! #     vertices_array: vertices.array(),
//! #     ..Default::default()
//! # });
//! stage.add_renderlet(&triangle);
//!
//! let frame = ctx.get_next_frame().unwrap();
//! stage.render(&frame.view());
//! let img = frame.read_image().unwrap();
//! frame.present();
//! ```
//!
//! Saving `img` should give us this:
//!
//! ![renderling hello triangle](https://github.com/schell/renderling/blob/main/test_img/cmy_triangle.png?raw=true)
//!
//! # WARNING
//!
//! This is very much a work in progress.
//!
//! Your mileage may vary, but I hope you get good use out of this library.
//!
//! PRs, criticisms and ideas are all very much welcomed [at the repo](https://github.com/schell/renderling).
//!
//! 😀☕
#![allow(unexpected_cfgs)]
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

pub mod atlas;
pub mod bits;
pub mod bloom;
pub mod bvol;
pub mod camera;
pub mod color;
#[cfg(not(target_arch = "spirv"))]
mod context;
pub mod convolution;
#[cfg(not(target_arch = "spirv"))]
pub mod cubemap;
pub mod cull;
pub mod debug;
pub mod draw;
pub mod ibl;
#[cfg(not(target_arch = "spirv"))]
mod linkage;
pub mod math;
pub mod pbr;
pub mod sdf;
pub mod skybox;
pub mod stage;
#[cfg(not(target_arch = "spirv"))]
pub mod texture;
pub mod tonemapping;
pub mod transform;
#[cfg(feature = "tutorial")]
pub mod tutorial;

#[cfg(not(target_arch = "spirv"))]
pub use context::*;

pub mod prelude {
    //! A prelude, meant to be glob-imported.

    pub extern crate craballoc;
    pub extern crate glam;

    pub use craballoc::prelude::*;
    pub use crabslab::{Array, Id};

    pub use crate::{
        camera::*,
        pbr::{light::*, Material},
        stage::*,
        transform::Transform,
    };

    #[cfg(not(target_arch = "spirv"))]
    pub use crate::context::*;
}

#[macro_export]
/// A wrapper around `std::println` that is a noop on the GPU.
macro_rules! println {
    ($($arg:tt)*) => {
        #[cfg(not(target_arch = "spirv"))]
        {
            std::println!($($arg)*);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        atlas::AtlasImage,
        camera::Camera,
        pbr::Material,
        stage::{NestedTransform, Renderlet, Vertex},
        transform::Transform,
    };

    use craballoc::value::Hybrid;
    use glam::{Mat3, Mat4, Quat, UVec2, Vec2, Vec3, Vec4};
    use img_diff::DiffCfg;
    use pretty_assertions::assert_eq;

    #[ctor::ctor]
    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sanity_transmute() {
        let zerof32 = 0f32;
        let zerof32asu32: u32 = zerof32.to_bits();
        assert_eq!(0, zerof32asu32);

        let foure_45 = 4e-45f32;
        let in_u32: u32 = foure_45.to_bits();
        assert_eq!(3, in_u32);

        let u32max = u32::MAX;
        let f32nan: f32 = f32::from_bits(u32max);
        assert!(f32nan.is_nan());

        let u32max: u32 = f32nan.to_bits();
        assert_eq!(u32::MAX, u32max);
    }

    pub fn right_tri_vertices() -> Vec<Vertex> {
        vec![
            Vertex::default()
                .with_position([0.0, 0.0, 0.0])
                .with_color([0.0, 1.0, 1.0, 1.0]),
            Vertex::default()
                .with_position([0.0, 100.0, 0.0])
                .with_color([1.0, 1.0, 0.0, 1.0]),
            Vertex::default()
                .with_position([100.0, 0.0, 0.0])
                .with_color([1.0, 0.0, 1.0, 1.0]),
        ]
    }

    #[test]
    // This tests our ability to draw a CMYK triangle in the top left corner.
    fn cmy_triangle_sanity() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
        let geometry = stage.new_array(right_tri_vertices());
        let tri = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            ..Default::default()
        });
        stage.add_renderlet(&tri);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        frame.present();

        let depth_texture = stage.get_depth_texture();
        let depth_img = depth_texture.read_image().unwrap();
        img_diff::assert_img_eq("cmy_triangle_depth.png", depth_img.clone());
        img_diff::save("cmy_triangle/depth.png", depth_img);

        let hdr_img = stage
            .hdr_texture
            .read()
            .unwrap()
            .read_hdr_image(&ctx)
            .unwrap();
        //let hdr_img: RgbaImage = hdr_img.convert();
        img_diff::save("cmy_triangle/hdr.png", hdr_img);

        let bloom_mix = stage.bloom.get_mix_texture().read_hdr_image(&ctx).unwrap();
        img_diff::save("cmy_triangle/bloom_mix.png", bloom_mix);

        img_diff::assert_img_eq("cmy_triangle.png", img);
    }

    #[test]
    // This tests our ability to draw a CMYK triangle in the top left corner, using
    // CW geometry.
    fn cmy_triangle_backface() {
        use img_diff::DiffCfg;

        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
        let geometry = stage.new_array({
            let mut vs = right_tri_vertices();
            vs.reverse();
            vs
        });
        let tri = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            ..Default::default()
        });
        stage.add_renderlet(&tri);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "cmy_triangle.png",
            img,
            DiffCfg {
                test_name: Some("cmy_triangle_backface.png"),
                ..Default::default()
            },
        );
    }

    #[test]
    // This tests our ability to update the transform of a `Renderlet` after it
    // has already been sent to the GPU.
    // We do this by writing over the previous transform in the stage.
    fn cmy_triangle_update_transform() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
        let geometry = stage.new_array(right_tri_vertices());
        let transform = stage.new_value(Transform::default());
        let tri = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&tri);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());

        transform.set(Transform {
            translation: Vec3::new(100.0, 0.0, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2),
            scale: Vec3::new(0.5, 0.5, 1.0),
        });

        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
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

    pub fn gpu_cube_vertices() -> Vec<Vertex> {
        math::UNIT_INDICES
            .iter()
            .map(|i| cmy_gpu_vertex(math::UNIT_POINTS[*i]))
            .collect()
    }

    #[test]
    // Tests our ability to draw a CMYK cube.
    fn cmy_cube_sanity() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let camera = stage.new_value(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let geometry = stage.new_array(gpu_cube_vertices());
        let transform = stage.new_value(Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        });
        let cube = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/sanity.png", img);
    }

    #[test]
    // Tests our ability to draw a CMYK cube using indexed geometry.
    fn cmy_cube_indices() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let camera = stage.new_value(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let vertices = stage.new_array(math::UNIT_POINTS.map(cmy_gpu_vertex));
        let indices = stage.new_array(math::UNIT_INDICES.map(|i| i as u32));
        let transform = stage.new_value(Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        });
        let cube = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "cmy_cube/sanity.png",
            img,
            DiffCfg {
                test_name: Some("cmy_cube/indices"),
                ..Default::default()
            },
        );
    }

    #[test]
    // Test our ability to create two cubes and toggle the visibility of one of
    // them.
    fn cmy_cube_visible() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let geometry = stage.new_array(gpu_cube_vertices());
        let cube_one_transform = stage.new_value(Transform {
            translation: Vec3::new(-4.5, 0.0, 0.0),
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
        });
        let mut renderlet = Renderlet {
            vertices_array: geometry.array(),
            camera_id: camera.id(),
            transform_id: cube_one_transform.id(),
            ..Default::default()
        };
        let cube_one = stage.new_value(renderlet);
        stage.add_renderlet(&cube_one);

        let cube_two_transform = stage.new_value(Transform {
            translation: Vec3::new(4.5, 0.0, 0.0),
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4),
        });
        renderlet.transform_id = cube_two_transform.id();
        let cube_two = stage.new_value(renderlet);
        stage.add_renderlet(&cube_two);

        // we should see two colored cubes
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/visible_before.png", img.clone());
        let img_before = img;
        frame.present();

        // update cube two making it invisible
        cube_two.modify(|r| r.visible = false);

        // we should see only one colored cube
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/visible_after.png", img);
        frame.present();

        // update cube two making in visible again
        cube_two.modify(|r| r.visible = true);

        // we should see two colored cubes again
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_eq("cmy_cube/visible_before_again.png", img_before, img);
    }

    #[test]
    // Tests the ability to specify indexed vertices, as well as the ability to
    // update a field within a struct stored on the slab by using a `Hybrid`.
    fn cmy_cube_remesh() {
        let ctx = Context::headless(100, 100);
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_background_color(Vec4::splat(1.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let cube_geometry =
            stage.new_array(math::UNIT_INDICES.map(|i| cmy_gpu_vertex(math::UNIT_POINTS[i])));
        let pyramid_points = pyramid_points();
        let pyramid_geometry =
            stage.new_array(pyramid_indices().map(|i| cmy_gpu_vertex(pyramid_points[i as usize])));

        let transform = stage.new_value(Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        });

        let cube: Hybrid<Renderlet> = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: cube_geometry.array(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);

        // we should see a cube (in sRGB color space)
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("cmy_cube/remesh_before.png", img);
        frame.present();

        // Update the cube mesh to a pyramid by overwriting the `.vertices` field
        // of `Renderlet`
        cube.modify(|r| r.vertices_array = pyramid_geometry.array());

        // we should see a pyramid (in sRGB color space)
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
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

    #[test]
    // Tests that updating the material actually updates the rendering of an unlit
    // mesh
    fn unlit_textured_cube_material() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.new_value(Camera::new(projection, view));

        let sandstone = AtlasImage::from(image::open("../../img/sandstone.png").unwrap());
        let dirt = AtlasImage::from(image::open("../../img/dirt.jpg").unwrap());
        let entries = stage.set_images([sandstone, dirt]).unwrap();

        let material = stage.new_value(Material {
            albedo_texture_id: entries[0].id(),
            has_lighting: false,
            ..Default::default()
        });
        let geometry = stage.new_array(gpu_uv_unit_cube());
        let transform = stage.new_value(Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        });
        let cube = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: material.id(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);
        println!("cube: {cube:?}");

        // we should see a cube with a stoney texture
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_before.png", img);
        frame.present();

        // update the material's texture on the GPU
        material.modify(|m| m.albedo_texture_id = entries[1].id());

        // we should see a cube with a dirty texture
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_after.png", img);

        // let size = stage.atlas.get_size();
        // for i in 0..size.depth_or_array_layers {
        //     let atlas_img = stage.atlas.atlas_img(&ctx, i);
        //     img_diff::save(
        //         &format!("unlit_texture_cube_atlas_layer_{i}.png"),
        //         atlas_img,
        //     );
        // }
    }

    #[test]
    // Ensures that we can render multiple nodes with mesh primitives
    // that share the same geometry, but have different materials.
    fn multi_node_scene() {
        let ctx = Context::headless(100, 100);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0));

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.new_value(Camera::new(projection, view));

        // now test the textures functionality
        let img = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let entries = stage.set_images([img]).unwrap();

        let cheetah_material = stage.new_value(Material {
            albedo_texture_id: entries[0].id(),
            has_lighting: false,
            ..Default::default()
        });

        let geometry = stage.new_array([
            Vertex {
                position: Vec3::new(0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                uv0: Vec2::new(0.0, 0.0),
                uv1: Vec2::new(0.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(100.0, 100.0, 0.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                uv0: Vec2::new(1.0, 1.0),
                uv1: Vec2::new(1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(100.0, 0.0, 0.0),
                color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                uv0: Vec2::new(1.0, 0.0),
                uv1: Vec2::new(1.0, 0.0),
                ..Default::default()
            },
        ]);

        let color_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            ..Default::default()
        });
        stage.add_renderlet(&color_prim);

        let cheetah_transform = stage.new_value(Transform {
            translation: Vec3::new(15.0, 35.0, 0.5),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        });
        let cheetah_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            transform_id: cheetah_transform.id(),
            material_id: cheetah_material.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cheetah_prim);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("stage/shared_node_with_different_materials.png", img);
    }

    #[test]
    /// Tests shading with directional light.
    fn scene_cube_directional() {
        use crate::pbr::light::{DirectionalLight, Light, LightStyle};

        let ctx = Context::headless(100, 100);
        let stage = ctx
            .new_stage()
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.0).extend(1.0));

        let (projection, _) = camera::default_perspective(100.0, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(1.8, 1.8, 1.8), Vec3::ZERO, Vec3::Y);
        let camera = stage.new_value(Camera::new(projection, view));

        let red = Vec3::X.extend(1.0);
        let green = Vec3::Y.extend(1.0);
        let blue = Vec3::Z.extend(1.0);
        let dir_red = stage.new_value(DirectionalLight {
            direction: Vec3::NEG_Y,
            color: red,
            intensity: 10.0,
        });
        let dir_green = stage.new_value(DirectionalLight {
            direction: Vec3::NEG_X,
            color: green,
            intensity: 10.0,
        });
        let dir_blue = stage.new_value(DirectionalLight {
            direction: Vec3::NEG_Z,
            color: blue,
            intensity: 10.0,
        });
        assert_eq!(
            Light {
                light_type: LightStyle::Directional,
                index: dir_red.id().inner(),
                ..Default::default()
            },
            dir_red.id().into()
        );
        let dir_red = stage.new_value(Light::from(dir_red.id()));
        let dir_green = stage.new_value(Light::from(dir_green.id()));
        let dir_blue = stage.new_value(Light::from(dir_blue.id()));
        stage.set_lights(vec![dir_red.id(), dir_green.id(), dir_blue.id()]);

        let material = stage.new_value(Material::default());
        let geometry = stage.new_array(
            math::unit_cube()
                .into_iter()
                .map(|(p, n)| Vertex {
                    position: p,
                    normal: n,
                    color: Vec4::ONE,
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        );
        let cube = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: material.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        let depth_texture = stage.get_depth_texture();
        let depth_img = depth_texture.read_image().unwrap();
        img_diff::assert_img_eq("stage/cube_directional_depth.png", depth_img);
        img_diff::assert_img_eq("stage/cube_directional.png", img);
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

    #[test]
    // shows how to "nest" children to make them appear transformed by their
    // parent's transform
    fn scene_parent_sanity() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let size = 1.0;
        let cyan_material = stage.new_value(Material {
            albedo_factor: Vec4::new(0.0, 1.0, 1.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });
        let yellow_material = stage.new_value(Material {
            albedo_factor: Vec4::new(1.0, 1.0, 0.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });
        let red_material = stage.new_value(Material {
            albedo_factor: Vec4::new(1.0, 0.0, 0.0, 1.0),
            has_lighting: false,
            ..Default::default()
        });

        let geometry = stage.new_array([
            Vertex::default().with_position([0.0, 0.0, 0.0]),
            Vertex::default().with_position([size, size, 0.0]),
            Vertex::default().with_position([size, 0.0, 0.0]),
        ]);

        let root_node = stage.new_nested_transform();
        root_node.set(Transform {
            scale: Vec3::new(25.0, 25.0, 1.0),
            ..Default::default()
        });
        println!("root_node: {:#?}", root_node.get_global_transform());

        let offset = Transform {
            translation: Vec3::new(1.0, 1.0, 0.0),
            ..Default::default()
        };

        let cyan_node = NestedTransform::new(&stage);
        cyan_node.set(offset);
        println!("cyan_node: {:#?}", cyan_node.get_global_transform());

        let yellow_node = NestedTransform::new(&stage);
        yellow_node.set(offset);
        println!("yellow_node: {:#?}", yellow_node.get_global_transform());

        let red_node = NestedTransform::new(&stage);
        red_node.set(offset);
        println!("red_node: {:#?}", red_node.get_global_transform());

        root_node.add_child(&cyan_node);
        println!("cyan_node: {:#?}", cyan_node.get_global_transform());
        cyan_node.add_child(&yellow_node);
        println!("yellow_node: {:#?}", yellow_node.get_global_transform());
        yellow_node.add_child(&red_node);
        println!("red_node: {:#?}", red_node.get_global_transform());

        let cyan_primitive = stage.new_value(Renderlet {
            vertices_array: geometry.array(),
            camera_id: camera.id(),
            material_id: cyan_material.id(),
            transform_id: cyan_node.global_transform_id(),
            ..Default::default()
        });
        stage.add_renderlet(&cyan_primitive);
        let yellow_primitive = stage.new_value(Renderlet {
            vertices_array: geometry.array(),
            camera_id: camera.id(),
            material_id: yellow_material.id(),
            transform_id: yellow_node.global_transform_id(),
            ..Default::default()
        });
        stage.add_renderlet(&yellow_primitive);
        let red_primitive = stage.new_value(Renderlet {
            vertices_array: geometry.array(),
            camera_id: camera.id(),
            material_id: red_material.id(),
            transform_id: red_node.global_transform_id(),
            ..Default::default()
        });
        stage.add_renderlet(&red_primitive);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
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

    #[test]
    fn can_resize_context_and_stage() {
        let size = UVec2::new(100, 100);
        let mut ctx = Context::headless(size.x, size.y);
        let stage = ctx.new_stage();

        // create the CMY cube
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let camera = stage.new_value(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let geometry = stage.new_array(gpu_cube_vertices());
        let transform = stage.new_value(Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        });
        let cube = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        assert_eq!(size, UVec2::new(img.width(), img.height()));
        img_diff::save("stage/resize_100.png", img);
        frame.present();

        let new_size = UVec2::new(200, 200);
        ctx.set_size(new_size);
        stage.set_size(new_size);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        assert_eq!(new_size, UVec2::new(img.width(), img.height()));
        img_diff::save("stage/resize_200.png", img);
        frame.present();
    }
}
