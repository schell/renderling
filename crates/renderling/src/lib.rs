//! <div style="float: right; padding: 1em;">
//!    <img
//!       style="image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;"
//!       alt="renderling mascot" width="180"
//!       src="https://github.com/user-attachments/assets/83eafc47-287c-4b5b-8fd7-2063e56b2338"
//!    />
//! </div>
//!
//! `renderling` is a "GPU driven" renderer with a focus on simplicity and ease
//! of use, targeting WebGPU.
//!
//! Shaders are written in Rust using [`rust-gpu`](https://rust-gpu.github.io/).
//!
//! ## Hello triangle
//!
//! Here we'll run through the classic "hello triangle", which will
//! display a colored triangle.
//!
//! ### Context creation
//!
//! First you must create a [`Context`].
//! The `Context` holds the render target - either a native window, an HTML
//! canvas or a texture.
//!
//! ```
//! use renderling::prelude::*;
//!
//! // create a headless context with dimensions 100, 100.
//! let ctx = futures_lite::future::block_on(Context::headless(100, 100));
//! ```
//!
//! [`Context::headless`] creates a `Context` that renders to a texture.
//!
//! [`Context::from_winit_window`] creates a `Context` that renders to a native
//! window.
//!
//! [`Context::try_new_with_surface`] creates a `Context` that renders to any
//! [`wgpu::SurfaceTarget`].
//!
//! See the [`renderling::context`](context) module documentation for
//! more info.
//!
//! ### Staging
//!
//! We then create a "stage" to place the camera, geometry, materials and lights.
//!
//! ```
//! # use renderling::prelude::*;
//! # let ctx = futures_lite::future::block_on(Context::headless(100, 100));
//! let stage: Stage = ctx
//!     .new_stage()
//!     .with_background_color([1.0, 1.0, 1.0, 1.0])
//!     // For this demo we won't use lighting
//!     .with_lighting(false);
//! ```
//!
//! The [`Stage`](crate::stage::Stage) is neat in that it allows you to "stage" data
//! directly onto the GPU. Those values can be modified on the CPU and
//! synchronization will happen during
//! [`Stage::render`](crate::stage::Stage::render).
//!
//! Use one of the many `Stage::new_*` functions to stage data on the GPU:
//! * [`Stage::new_camera`]
//! * [`Stage::new_vertices`]
//! * [`Stage::new_material`]
//! * [`Stage::new_renderlet`]
//! * ...and more
//!
//! Many of these functions return [`Hybrid`](crate::prelude::Hybrid)s or
//! [`HybridArray`](crate::prelude::HybridArray)s in return.
//!
//! In order to render, we need to "stage" a
//! [`Renderlet`](crate::stage::Renderlet), which is a bundle of rendering
//! resources, roughly representing a singular mesh.
//!
//! But first we'll need a [`Camera`](crate::camera::Camera) so we can see
//! what's on the stage, and then we'll need a list
//! of [`Vertex`](crate::stage::Vertex) organized as triangles with
//! counter-clockwise winding. Here we'll use the builder pattern to create a
//! staged [`Renderlet`](crate::stage::Renderlet) using our vertices.
//!
//! ```
//! # use renderling::prelude::*;
//! # let ctx = futures_lite::future::block_on(Context::headless(100, 100));
//! # let stage: Stage = ctx.new_stage();
//!
//! let camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
//! let (vertices, triangle_renderlet) = stage
//!     .builder()
//!     .with_vertices([
//!         Vertex::default()
//!             .with_position([0.0, 0.0, 0.0])
//!             .with_color([0.0, 1.0, 1.0, 1.0]),
//!         Vertex::default()
//!             .with_position([0.0, 100.0, 0.0])
//!             .with_color([1.0, 1.0, 0.0, 1.0]),
//!         Vertex::default()
//!             .with_position([100.0, 0.0, 0.0])
//!             .with_color([1.0, 0.0, 1.0, 1.0]),
//!     ])
//!     .build();
//! ```
//!
//! The builder is of the type [`RenderletBuilder`](crate::stage::RenderletBuilder)
//! and after building, it leaves you with all the resources that have been staged,
//! including the `Renderlet`.
//! The return type of [`RenderletBuilder::build`](crate::stage::RenderletBuilder::build)
//! is special in that it depends on the new resources that have been staged.
//! The type will be a tuple of all the newly staged resources that have been added.
//! In this case it's our mesh data and the `Renderlet`.
//!
//! ### Rendering
//!
//! Finally, we get the next frame from the context with
//! [`Context::get_next_frame`], render to it using
//! [`Stage::render`](crate::stage::Stage::render) and then present the
//! frame with [`Frame::present`].
//!
//! ```
//! # use renderling::prelude::*;
//! # let ctx = futures_lite::future::block_on(Context::headless(100, 100));
//! # let stage = ctx.new_stage();
//! # let _camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
//! # let _rez = stage.builder().with_vertices([
//! #     Vertex::default()
//! #         .with_position([0.0, 0.0, 0.0])
//! #         .with_color([0.0, 1.0, 1.0, 1.0]),
//! #     Vertex::default()
//! #         .with_position([0.0, 100.0, 0.0])
//! #         .with_color([1.0, 1.0, 0.0, 1.0]),
//! #     Vertex::default()
//! #         .with_position([100.0, 0.0, 0.0])
//! #         .with_color([1.0, 0.0, 1.0, 1.0]),
//! # ]).build();
//!
//! let frame = ctx.get_next_frame().unwrap();
//! stage.render(&frame.view());
//! let img = futures_lite::future::block_on(frame.read_image()).unwrap();
//! frame.present();
//! ```
//!
//! Saving `img` should give us this:
//!
//! ![renderling hello triangle](https://github.com/schell/renderling/blob/main/test_img/cmy_triangle.png?raw=true)
//!
//! ### Modifying
//!
//! Later, if we want to modify any of the staged values, we can do so through
//! [`Hybrid`](crate::prelude::Hybrid) and [`HybridArray`](crate::prelude::HybridArray).
//! The changes made will be synchronized to the GPU at the beginning of the
//! [`Stage::render`](crate::prelude::Stage::render) function.
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
#[cfg(cpu)]
pub mod bindgroup;
pub mod bits;
pub mod bloom;
pub mod bvol;
pub mod camera;
pub mod color;
#[cfg(cpu)]
pub mod context;
pub mod convolution;
pub mod cubemap;
pub mod cull;
pub mod debug;
pub mod draw;
pub mod geometry;
pub mod ibl;
#[cfg(cpu)]
pub mod internal;
pub mod light;
#[cfg(cpu)]
pub mod linkage;
pub mod material;
pub mod math;
pub mod pbr;
pub mod sdf;
pub mod skybox;
pub mod stage;
pub mod sync;
#[cfg(cpu)]
pub mod texture;
pub mod tonemapping;
pub mod transform;
pub mod tuple;
pub mod tutorial;
#[cfg(feature = "ui")]
pub mod ui;

#[cfg(cpu)]
pub use context::*;

pub mod prelude {
    //! A prelude, meant to be glob-imported.

    pub extern crate glam;

    #[cfg(cpu)]
    pub use craballoc::prelude::*;
    pub use crabslab::{Array, Id};

    pub use crate::{
        camera::*, geometry::*, light::*, material::MaterialDescriptor, stage::*,
        transform::TransformDescriptor,
    };

    #[cfg(cpu)]
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
        atlas::AtlasImage, camera::Camera, geometry::Vertex, material::MaterialDescriptor,
        transform::TransformDescriptor,
    };

    use glam::{Mat3, Mat4, Quat, UVec2, Vec2, Vec3, Vec4};
    use img_diff::DiffCfg;
    use light::{AnalyticalLight, DirectionalLightDescriptor};
    use pretty_assertions::assert_eq;
    use stage::Stage;

    #[cfg_attr(not(target_arch = "wasm32"), ctor::ctor)]
    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
        log::info!("logging is on");
    }

    pub fn workspace_dir() -> std::path::PathBuf {
        std::path::PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"))
    }

    #[allow(dead_code)]
    pub fn test_output_dir() -> std::path::PathBuf {
        workspace_dir().join("test_output")
    }

    /// Marker trait to block on futures in synchronous code.
    ///
    /// This is a simple convenience.
    /// Many of the tests in this crate render something and then read a
    /// texture in order to perform a diff on the result using a known image.
    /// Since reading from the GPU is async, this trait helps cut down
    /// boilerplate.
    pub trait BlockOnFuture {
        type Output;

        /// Block on the future using [`futures_util::future::block_on`].
        fn block(self) -> Self::Output;
    }

    impl<T: std::future::Future> BlockOnFuture for T {
        type Output = <Self as std::future::Future>::Output;

        fn block(self) -> Self::Output {
            futures_lite::future::block_on(self)
        }
    }

    pub fn make_two_directional_light_setup(stage: &Stage) -> (AnalyticalLight, AnalyticalLight) {
        let sunlight_a = stage.new_analytical_light(DirectionalLightDescriptor {
            direction: Vec3::new(-0.8, -1.0, 0.5).normalize(),
            color: Vec4::ONE,
            intensity: 100.0,
        });
        let sunlight_b = stage.new_analytical_light(DirectionalLightDescriptor {
            direction: Vec3::new(1.0, 1.0, -0.1).normalize(),
            color: Vec4::ONE,
            intensity: 10.0,
        });
        (sunlight_a, sunlight_b)
    }

    #[allow(unused, reason = "Used in debugging on macos")]
    pub fn capture_gpu_frame<T>(
        ctx: &Context,
        path: impl AsRef<std::path::Path>,
        f: impl FnOnce() -> T,
    ) -> T {
        let path = workspace_dir().join("test_output").join(path);
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent).unwrap();

        #[cfg(target_os = "macos")]
        {
            if path.exists() {
                log::info!(
                    "deleting {} before writing gpu frame capture",
                    path.display()
                );
                std::fs::remove_dir_all(&path).unwrap();
            }

            if std::env::var("METAL_CAPTURE_ENABLED").is_err() {
                log::error!("Env var METAL_CAPTURE_ENABLED must be set");
                panic!("missing METAL_CAPTURE_ENABLED=1");
            }

            let m = metal::CaptureManager::shared();
            let desc = metal::CaptureDescriptor::new();

            desc.set_destination(metal::MTLCaptureDestination::GpuTraceDocument);
            desc.set_output_url(path);
            let maybe_metal_device = unsafe { ctx.get_device().as_hal::<wgpu_core::api::Metal>() };
            if let Some(metal_device) = maybe_metal_device {
                desc.set_capture_device(metal_device.raw_device().try_lock().unwrap().as_ref());
            } else {
                panic!("not a capturable device")
            }
            m.start_capture(&desc).unwrap();
            let t = f();
            m.stop_capture();
            t
        }
        #[cfg(not(target_os = "macos"))]
        {
            f()
        }
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let _camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
        let _rez = stage.builder().with_vertices(right_tri_vertices()).build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        frame.present();

        let depth_texture = stage.get_depth_texture();
        let depth_img = depth_texture.read_image().block().unwrap().unwrap();
        img_diff::assert_img_eq("cmy_triangle/depth.png", depth_img);

        let hdr_img = stage
            .hdr_texture
            .read()
            .unwrap()
            .read_hdr_image(&ctx)
            .block()
            .unwrap();
        img_diff::assert_img_eq("cmy_triangle/hdr.png", hdr_img);

        let bloom_mix = stage
            .bloom
            .get_mix_texture()
            .read_hdr_image(&ctx)
            .block()
            .unwrap();
        img_diff::assert_img_eq("cmy_triangle/bloom_mix.png", bloom_mix);
    }

    #[test]
    // This tests our ability to draw a CMYK triangle in the top left corner, using
    // CW geometry.
    fn cmy_triangle_backface() {
        use img_diff::DiffCfg;

        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let _camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
        let _rez = stage
            .builder()
            .with_vertices({
                let mut vs = right_tri_vertices();
                vs.reverse();
                vs
            })
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_linear_image().block().unwrap();
        img_diff::assert_img_eq_cfg(
            "cmy_triangle/hdr.png",
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let _camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
        let (_vertices, transform, _renderlet) = stage
            .builder()
            .with_vertices(right_tri_vertices())
            .with_transform(TransformDescriptor::default())
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());

        transform.set(TransformDescriptor {
            translation: Vec3::new(100.0, 0.0, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2),
            scale: Vec3::new(0.5, 0.5, 1.0),
        });

        stage.render(&frame.view());
        let img = frame.read_linear_image().block().unwrap();
        img_diff::assert_img_eq("cmy_triangle/update_transform.png", img);
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let _camera = stage.new_camera(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let _rez = stage
            .builder()
            .with_vertices(gpu_cube_vertices())
            .with_transform(TransformDescriptor {
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
                ..Default::default()
            })
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("cmy_cube/sanity.png", img);
    }

    #[test]
    // Tests our ability to draw a CMYK cube using indexed geometry.
    fn cmy_cube_indices() {
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let _camera = stage.new_camera(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let _rez = stage
            .builder()
            .with_vertices(math::UNIT_POINTS.map(cmy_gpu_vertex))
            .with_indices(math::UNIT_INDICES.map(|i| i as u32))
            .with_transform(TransformDescriptor {
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
                ..Default::default()
            })
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let _camera = stage.new_camera(Camera::new(projection, view));
        let (geometry, _cube_one_transform, _cube_one) = stage
            .builder()
            .with_vertices(gpu_cube_vertices())
            .with_transform(TransformDescriptor {
                translation: Vec3::new(-4.5, 0.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            })
            .build();

        let (_cube_two_transform, cube_two) = stage
            .builder()
            .with_vertices_array(geometry.array())
            .with_transform(TransformDescriptor {
                translation: Vec3::new(4.5, 0.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4),
            })
            .build();

        // we should see two colored cubes
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("cmy_cube/visible_before.png", img.clone());
        let img_before = img;
        frame.present();

        // update cube two making it invisible
        cube_two.modify(|r| r.visible = false);

        // we should see only one colored cube
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("cmy_cube/visible_after.png", img);
        frame.present();

        // update cube two making in visible again
        cube_two.modify(|r| r.visible = true);

        // we should see two colored cubes again
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_eq("cmy_cube/visible_before_again.png", img_before, img);
    }

    #[test]
    // Tests the ability to specify indexed vertices, as well as the ability to
    // update a field within a struct stored on the slab by using a `Hybrid`.
    fn cmy_cube_remesh() {
        let ctx = Context::headless(100, 100).block();
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_background_color(Vec4::splat(1.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let _camera = stage.new_camera(Camera::new(projection, view));
        let (_cube_geometry, _transform, cube) = stage
            .builder()
            .with_vertices(math::UNIT_INDICES.map(|i| cmy_gpu_vertex(math::UNIT_POINTS[i])))
            .with_transform(TransformDescriptor {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            })
            .build();

        // we should see a cube (in sRGB color space)
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("cmy_cube/remesh_before.png", img);
        frame.present();

        // Update the cube mesh to a pyramid by overwriting the `.vertices` field
        // of `Renderlet`
        let pyramid_points = pyramid_points();
        let pyramid_geometry = stage
            .new_vertices(pyramid_indices().map(|i| cmy_gpu_vertex(pyramid_points[i as usize])));
        cube.modify(|r| r.vertices_array = pyramid_geometry.array());

        // we should see a pyramid (in sRGB color space)
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let _camera = stage.new_camera(Camera::new(projection, view));

        let sandstone = AtlasImage::from(image::open("../../img/sandstone.png").unwrap());
        let dirt = AtlasImage::from(image::open("../../img/dirt.jpg").unwrap());
        let entries = stage.set_images([sandstone, dirt]).unwrap();

        let (material, _geometry, _transform, cube) = stage
            .builder()
            .with_material(MaterialDescriptor {
                albedo_texture_id: entries[0].id(),
                has_lighting: false,
                ..Default::default()
            })
            .with_vertices(gpu_uv_unit_cube())
            .with_transform(TransformDescriptor {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            })
            .build();
        println!("cube: {cube:?}");

        // we should see a cube with a stoney texture
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_before.png", img);
        frame.present();

        // update the material's texture on the GPU
        material.modify(|m| m.albedo_texture_id = entries[1].id());

        // we should see a cube with a dirty texture
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0));

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let _camera = stage.new_camera(Camera::new(projection, view));

        // now test the textures functionality
        let img = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let entries = stage.set_images([img]).unwrap();

        let (geometry, _color_prim) = stage
            .builder()
            .with_vertices([
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
            ])
            .build();

        let _rez = stage
            .builder()
            .with_vertices_array(geometry.array())
            .with_material(MaterialDescriptor {
                albedo_texture_id: entries[0].id(),
                has_lighting: false,
                ..Default::default()
            })
            .with_transform(TransformDescriptor {
                translation: Vec3::new(15.0, 35.0, 0.5),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            })
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("stage/shared_node_with_different_materials.png", img);
    }

    #[test]
    /// Tests shading with directional light.
    fn scene_cube_directional() {
        use crate::light::{DirectionalLightDescriptor, Light, LightStyle};

        let ctx = Context::headless(100, 100).block();
        let stage = ctx
            .new_stage()
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.0).extend(1.0));

        let (projection, _) = camera::default_perspective(100.0, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(1.8, 1.8, 1.8), Vec3::ZERO, Vec3::Y);
        let _camera = stage.new_camera(Camera::new(projection, view));

        let red = Vec3::X.extend(1.0);
        let green = Vec3::Y.extend(1.0);
        let blue = Vec3::Z.extend(1.0);
        let dir_red = stage.new_analytical_light(DirectionalLightDescriptor {
            direction: Vec3::NEG_Y,
            color: red,
            intensity: 10.0,
        });
        let _dir_green = stage.new_analytical_light(DirectionalLightDescriptor {
            direction: Vec3::NEG_X,
            color: green,
            intensity: 10.0,
        });
        let _dir_blue = stage.new_analytical_light(DirectionalLightDescriptor {
            direction: Vec3::NEG_Z,
            color: blue,
            intensity: 10.0,
        });
        assert_eq!(
            Light {
                light_type: LightStyle::Directional,
                index: dir_red
                    .light_details()
                    .as_directional()
                    .unwrap()
                    .id()
                    .inner(),
                ..Default::default()
            },
            Light::from(dir_red.light_details().as_directional().unwrap().id())
        );

        let _rez = stage
            .builder()
            .with_material(MaterialDescriptor::default())
            .with_vertices(
                math::unit_cube()
                    .into_iter()
                    .map(|(p, n)| Vertex {
                        position: p,
                        normal: n,
                        color: Vec4::ONE,
                        ..Default::default()
                    })
                    .collect::<Vec<_>>(),
            )
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        let depth_texture = stage.get_depth_texture();
        let depth_img = depth_texture.read_image().block().unwrap().unwrap();
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
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage().with_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let _camera = stage.new_camera(Camera::new(projection, view));

        let root_node = stage.new_nested_transform();
        root_node.set(TransformDescriptor {
            scale: Vec3::new(25.0, 25.0, 1.0),
            ..Default::default()
        });
        println!("root_node: {:#?}", root_node.get_global_transform());

        let offset = TransformDescriptor {
            translation: Vec3::new(1.0, 1.0, 0.0),
            ..Default::default()
        };

        let cyan_node = stage.new_nested_transform();
        cyan_node.set(offset);
        println!("cyan_node: {:#?}", cyan_node.get_global_transform());

        let yellow_node = stage.new_nested_transform();
        yellow_node.set(offset);
        println!("yellow_node: {:#?}", yellow_node.get_global_transform());

        let red_node = stage.new_nested_transform();
        red_node.set(offset);
        println!("red_node: {:#?}", red_node.get_global_transform());

        root_node.add_child(&cyan_node);
        println!("cyan_node: {:#?}", cyan_node.get_global_transform());
        cyan_node.add_child(&yellow_node);
        println!("yellow_node: {:#?}", yellow_node.get_global_transform());
        yellow_node.add_child(&red_node);
        println!("red_node: {:#?}", red_node.get_global_transform());

        let (geometry, _cyan_material, _cyan_primitive) = stage
            .builder()
            .with_vertices({
                let size = 1.0;
                [
                    Vertex::default().with_position([0.0, 0.0, 0.0]),
                    Vertex::default().with_position([size, size, 0.0]),
                    Vertex::default().with_position([size, 0.0, 0.0]),
                ]
            })
            .with_material(MaterialDescriptor {
                albedo_factor: Vec4::new(0.0, 1.0, 1.0, 1.0),
                has_lighting: false,
                ..Default::default()
            })
            .with_nested_transform(&cyan_node)
            .build();
        let _yellow = stage
            .builder()
            .with_vertices_array(geometry.array())
            .with_material(MaterialDescriptor {
                albedo_factor: Vec4::new(1.0, 1.0, 0.0, 1.0),
                has_lighting: false,
                ..Default::default()
            })
            .with_nested_transform(&yellow_node)
            .build();
        let _red = stage
            .builder()
            .with_vertices_array(geometry.array())
            .with_material(MaterialDescriptor {
                albedo_factor: Vec4::new(1.0, 0.0, 0.0, 1.0),
                has_lighting: false,
                ..Default::default()
            })
            .with_nested_transform(&red_node)
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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
        let mut ctx = Context::headless(size.x, size.y).block();
        let stage = ctx.new_stage();

        // create the CMY cube
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let _camera = stage.new_camera(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let _rez = stage
            .builder()
            .with_vertices(gpu_cube_vertices())
            .with_transform(TransformDescriptor {
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
                ..Default::default()
            })
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        assert_eq!(size, UVec2::new(img.width(), img.height()));
        img_diff::assert_img_eq("stage/resize_100.png", img);
        frame.present();

        let new_size = UVec2::new(200, 200);
        ctx.set_size(new_size);
        stage.set_size(new_size);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        assert_eq!(new_size, UVec2::new(img.width(), img.height()));
        img_diff::assert_img_eq("stage/resize_200.png", img);
        frame.present();
    }

    #[test]
    fn can_direct_draw_cube() {
        let size = UVec2::new(100, 100);
        let ctx = Context::headless(size.x, size.y)
            .block()
            .with_use_direct_draw(true);
        let stage = ctx.new_stage();

        // create the CMY cube
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let _camera = stage.new_camera(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let _rez = stage
            .builder()
            .with_vertices(gpu_cube_vertices())
            .with_transform(TransformDescriptor {
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
                ..Default::default()
            })
            .build();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        assert_eq!(size, UVec2::new(img.width(), img.height()));
        img_diff::assert_img_eq("stage/resize_100.png", img);
        frame.present();
    }
}
