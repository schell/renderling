//! A "GPU driven" renderer  with a focus on simplicity and ease of use.
//! Backed by WebGPU.
//! Shaders written in rust using `rust-gpu`.
//!
//! # WARNING
//! This is very much a work in progress.
//! YMMV.
//! PRs are very welcomed :)
//!
//! # renderlings 🍖
//!
//! Render graphs and all their resources are called "renderlings" for maximum
//! cuteness. Renderlings are configurable DAGs that draw something to a screen
//! or texture.
//!
//! ## Features
//!
//! - forward+ style pipeline, configurable lighting model per material
//!   - [ ] light tiling
//!   - [ ] occlusion culling
//!   - [x] physically based shading
//!   - [x] user interface "colored text" shading (uses opacity glyphs in an
//!     atlas)
//!   - [x] no shading
//! - [ ] gltf support
//!   - [ ] scenes, nodes
//!   - [x] cameras
//!   - [x] meshes
//!   - [x] materials
//!   - [x] textures, images, samplers
//!   - [x] skins
//!   - [x] animations
//! - [x] high definition rendering
//! - [x] image based lighting
//! - [x] bloom
//! - [ ] ssao
//! - [ ] depth of field
//!
//! ## Raw shaders
//! You can also use the [shaders module](crate::shaders) without renderlings
//! and manage your own resources for maximum flexibility.

// TODO: Audit the API and make it more ergonomic/predictable.

mod atlas;
pub mod bloom;
mod buffer_array;
mod camera;
pub mod cubemap;
pub mod frame;
mod hdr;
pub mod ibl;
mod linkage;
pub mod math;
pub mod mesh;
mod renderer;
mod scene;
mod skybox;
mod slab;
mod stage;
mod state;
#[cfg(feature = "text")]
mod text;
mod texture;
mod tutorial;
mod ui;
mod uniform;

pub use atlas::*;
pub use buffer_array::*;
pub use camera::*;
pub use hdr::*;
use image::GenericImageView;
pub use renderer::*;
pub use scene::*;
pub use skybox::*;
pub use slab::*;
pub use stage::*;
pub use state::*;
#[cfg(feature = "text")]
pub use text::*;
pub use texture::*;
pub use ui::*;
pub use uniform::*;

pub mod color;

pub mod debug {
    //! Re-exports of [`renderling_shader::debug`].

    pub use renderling_shader::debug::*;
}

pub mod graph {
    //! Re-exports of [`moongraph`].
    pub use moongraph::*;

    pub type RenderNode = Node<Function, TypeKey>;
}

pub use graph::{graph, Graph, GraphError, Move, View, ViewMut};
pub use renderling_shader::id::{Id, ID_NONE};
pub mod shader {
    //! Re-exports of [`renderling_shader`].
    pub use renderling_shader::*;
}

pub use renderling_shader::stage::{GpuEntityInfo, LightingModel};

/// A CPU-side texture sampler.
///
/// Provided primarily for testing purposes.
#[derive(Debug, Clone, Copy)]
pub struct CpuSampler;

impl crate::shader::IsSampler for CpuSampler {}

#[derive(Debug)]
pub struct CpuTexture2d {
    pub image: image::DynamicImage,
}

impl crate::shader::Sample2d for CpuTexture2d {
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
pub struct CpuCubemap;

impl crate::shader::SampleCube for CpuCubemap {
    type Sampler = CpuSampler;

    fn sample_by_lod(&self, _sampler: Self::Sampler, _uv: glam::Vec3, _lod: f32) -> glam::Vec4 {
        // TODO: implement CPU-side cubemap sampling
        glam::Vec4::ONE
    }
}

/// Set up the render graph, including:
/// * 3d scene objects
/// * skybox
/// * bloom filter
/// * hdr tonemapping
/// * UI
///
/// This is mostly for internal use. See [`Renderling::setup_render_graph`].
pub fn setup_render_graph(
    r: &mut Renderling,
    scene: Scene,
    ui_scene: UiScene,
    ui_objects: impl IntoIterator<Item = UiDrawObject>,
    with_screen_capture: bool,
    with_bloom: bool,
) {
    // add resources
    let ui_objects = UiDrawObjects(ui_objects.into_iter().collect::<Vec<_>>());
    r.graph.add_resource(ui_scene);
    r.graph.add_resource(ui_objects);
    r.graph.add_resource(scene);
    let ui_pipeline = UiRenderPipeline(
        r.graph
            .visit(|(device, target): (View<Device>, View<RenderTarget>)| {
                create_ui_pipeline(&device, target.format())
            })
            .unwrap(),
    );
    r.graph.add_resource(ui_pipeline);

    let (hdr_surface,) = r
        .graph
        .visit(hdr::create_hdr_render_surface)
        .unwrap()
        .unwrap();
    let device = r.get_device();
    let queue = r.get_queue();
    let hdr_texture_format = hdr_surface.hdr_texture.texture.format();
    let size = hdr_surface.hdr_texture.texture.size();
    let scene_render_pipeline =
        SceneRenderPipeline(create_scene_render_pipeline(device, hdr_texture_format));
    let compute_cull_pipeline =
        SceneComputeCullPipeline(create_scene_compute_cull_pipeline(device));
    let skybox_pipeline = crate::skybox::create_skybox_render_pipeline(device, hdr_texture_format);
    let mut bloom = crate::bloom::BloomFilter::new(device, queue, size.width, size.height);
    bloom.on = with_bloom;
    r.graph.add_resource(scene_render_pipeline);
    r.graph.add_resource(hdr_surface);
    r.graph.add_resource(compute_cull_pipeline);
    r.graph.add_resource(skybox_pipeline);
    r.graph.add_resource(bloom);

    // pre-render subgraph
    use frame::{clear_depth, create_frame, present};

    #[cfg(not(target_arch = "wasm32"))]
    let scene_cull = scene_cull_gpu;
    #[cfg(target_arch = "wasm32")]
    let scene_cull = scene_cull_cpu;
    r.graph
        .add_subgraph(graph!(
            create_frame,
            clear_surface_hdr_and_depth,
            hdr_surface_update,
            scene_update < scene_cull,
            ui_scene_update
        ))
        .add_barrier();

    // render subgraph
    use crate::bloom::bloom_filter;
    r.graph
        .add_subgraph(graph!(
            scene_render
                < skybox_render
                < bloom_filter
                < tonemapping
                < clear_depth
                < ui_scene_render
        ))
        .add_barrier();

    // post-render subgraph
    r.graph.add_subgraph(if with_screen_capture {
        use crate::frame::copy_frame_to_post;
        graph!(copy_frame_to_post < present)
    } else {
        graph!(present)
    });
}

#[cfg(test)]
#[ctor::ctor]
fn init_logging() {
    let _ = env_logger::builder()
        .is_test(true)
        //.filter_level(log::LevelFilter::Trace)
        .filter_module("moongraph", log::LevelFilter::Trace)
        .filter_module("renderling", log::LevelFilter::Trace)
        //.filter_module("naga", log::LevelFilter::Debug)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .filter_module("wgpu_hal", log::LevelFilter::Warn)
        .try_init();
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::{Mat3, Mat4, Quat, UVec2, Vec2, Vec3, Vec4, Vec4Swizzles};
    use pretty_assertions::assert_eq;
    use renderling_shader::stage::{
        light::*, new_stage_vertex, Camera, GpuEntity, NativeVertexData, RenderUnit, Transform,
        Vertex, VertexData,
    };

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

    #[test]
    // This tests our ability to draw a CMYK triangle in the top left corner.
    fn cmy_triangle_sanity() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let vertices = stage.append_array(&right_tri_vertices());
        let native_vertex_data = stage.append(&NativeVertexData {
            vertices,
            ..Default::default()
        });
        let vertex_data = VertexData::new_native(native_vertex_data);
        let transform = stage.append(&Transform::default());
        let _tri = stage.draw_unit(&RenderUnit {
            vertex_data,
            camera,
            vertex_count: vertices.len() as u32,
            transform,
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_triangle.png", img);
    }

    #[test]
    // This tests our ability to update the transform of a `RenderUnit` after it
    // has already been sent to the GPU.
    // We do this by writing over the previous transform in the stage.
    fn cmy_triangle_update_transform() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let vertices = stage.append_array(&right_tri_vertices());
        let native_vertex_data = stage.append(&NativeVertexData {
            vertices,
            ..Default::default()
        });
        let vertex_data = VertexData::new_native(native_vertex_data);
        let transform = stage.append(&Transform::default());
        let _tri = stage.draw_unit(&RenderUnit {
            vertex_data,
            camera,
            vertex_count: vertices.len() as u32,
            transform,
        });

        let _ = r.render_image().unwrap();

        stage
            .write(
                transform,
                &Transform {
                    translation: Vec3::new(100.0, 0.0, 0.0),
                    rotation: Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2),
                    scale: Vec3::new(0.5, 0.5, 1.0),
                },
            )
            .unwrap();

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
        renderling_shader::math::UNIT_INDICES
            .iter()
            .map(|i| cmy_gpu_vertex(renderling_shader::math::UNIT_POINTS[*i as usize]))
            .collect()
    }

    #[test]
    // Tests our ability to draw a CMYK cube.
    fn cmy_cube_sanity() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let camera_position = Vec3::new(0.0, 12.0, 20.0);
        let camera = stage.append(&Camera {
            projection: Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            view: Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
            position: camera_position,
        });
        let vertices = stage.append_array(&gpu_cube_vertices());
        let native_vertex_data = stage.append(&NativeVertexData {
            vertices,
            ..Default::default()
        });
        let transform = Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        };
        let _cube = stage.draw_unit(&RenderUnit {
            vertex_data: VertexData::new_native(native_vertex_data),
            camera,
            vertex_count: vertices.len() as u32,
            transform: stage.append(&transform),
        });
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube.png", img);
    }

    #[test]
    // Test our ability to create two cubes and toggle the visibility of one of
    // them.
    fn cmy_cube_visible() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let vertices = stage.append_array(&gpu_cube_vertices());
        let native_vertex_data = stage.append(&NativeVertexData {
            vertices,
            ..Default::default()
        });
        let _cube_one = stage.draw_unit(&RenderUnit {
            vertex_data: VertexData::new_native(native_vertex_data),
            camera,
            vertex_count: vertices.len() as u32,
            transform: stage.append(&Transform {
                translation: Vec3::new(-4.5, 0.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            }),
        });
        let cube_two = stage.draw_unit(&RenderUnit {
            vertex_data: VertexData::new_native(native_vertex_data),
            camera,
            vertex_count: vertices.len() as u32,
            transform: stage.append(&Transform {
                translation: Vec3::new(4.5, 0.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 6.0),
                rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4),
            }),
        });

        // we should see two colored cubes
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube_visible_before.png", img.clone());
        let img_before = img;

        // update cube two making it invisible
        stage.hide_unit(cube_two);

        // we should see only one colored cube
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube_visible_after.png", img);

        // update cube two making in visible again
        stage.show_unit(cube_two);

        // we should see two colored cubes again
        let img = r.render_image().unwrap();
        img_diff::assert_eq("cmy_cube_visible_before_again.png", img_before, img);
    }

    #[test]
    // Tests the ability to specify indexed vertices, as well as the ability to update
    // the vertex data of a RenderUnit.
    fn cmy_cube_remesh() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let stage = r.new_stage().with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        let pyramid_vertices = stage.append_array(&pyramid_points().map(cmy_gpu_vertex));
        let pyramid_indices = stage.append_array(&pyramid_indices().map(|i| i as u32));
        let pyramid_vertex_data = stage.append(&NativeVertexData {
            vertices: pyramid_vertices,
            indices: pyramid_indices,
            ..Default::default()
        });
        let cube_vertices =
            stage.append_array(&renderling_shader::math::UNIT_POINTS.map(cmy_gpu_vertex));
        let cube_indices =
            stage.append_array(&renderling_shader::math::UNIT_INDICES.map(|i| i as u32));
        let cube_vertex_data = stage.append(&NativeVertexData {
            vertices: cube_vertices,
            indices: cube_indices,
            ..Default::default()
        });
        let transform = stage.append(&Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        });
        let cube = stage.draw_unit(&RenderUnit {
            vertex_data: VertexData::new_native(cube_vertex_data),
            camera,
            transform,
            vertex_count: cube_indices.len() as u32,
        });

        // we should see a cube
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube_remesh_before.png", img);

        // update the cube mesh to a pyramid by writing over the `RenderUnit`
        stage
            .write(
                cube,
                &RenderUnit {
                    vertex_data: VertexData::new_native(pyramid_vertex_data),
                    vertex_count: pyramid_indices.len() as u32,
                    camera,
                    transform,
                },
            )
            .unwrap();

        // we should see a pyramid
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube_remesh_after.png", img);
    }

    fn gpu_uv_unit_cube() -> Vec<Vertex> {
        let p: [Vec3; 8] = renderling_shader::math::UNIT_POINTS;
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

    /// A helper struct that contains all outputs of the vertex shader.
    #[allow(unused)]
    #[derive(Debug, Default)]
    pub struct VertexInvocation {
        pub instance_index: u32,
        pub vertex_index: u32,
        pub render_unit_id: Id<RenderUnit>,
        pub render_unit: RenderUnit,
        pub out_camera: u32,
        pub out_material: u32,
        pub out_color: Vec4,
        pub out_uv0: Vec2,
        pub out_uv1: Vec2,
        pub out_norm: Vec3,
        pub out_tangent: Vec3,
        pub out_bitangent: Vec3,
        pub out_pos: Vec3,
        // output clip coordinates
        pub clip_pos: Vec4,
        // output normalized device coordinates
        pub ndc_pos: Vec3,
    }

    impl VertexInvocation {
        pub fn invoke(instance_index: u32, vertex_index: u32, slab: &[u32]) -> Self {
            let mut v = Self {
                instance_index,
                vertex_index,
                ..Default::default()
            };
            v.render_unit_id = Id::from(v.instance_index);
            v.render_unit = slab.read(v.render_unit_id);
            new_stage_vertex(
                v.instance_index,
                v.vertex_index,
                slab,
                &mut v.out_camera,
                &mut v.out_material,
                &mut v.out_color,
                &mut v.out_uv0,
                &mut v.out_uv1,
                &mut v.out_norm,
                &mut v.out_tangent,
                &mut v.out_bitangent,
                &mut v.out_pos,
                &mut v.clip_pos,
            );
            v.ndc_pos = v.clip_pos.xyz() / v.clip_pos.w;
            v
        }
    }

    #[test]
    // Tests that updating the material actually updates the rendering of an unlit mesh
    fn unlit_textured_cube_material() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(0.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let sandstone = SceneImage::from(image::open("../../img/sandstone.png").unwrap());
        let dirt = SceneImage::from(image::open("../../img/dirt.jpg").unwrap());
        let textures = stage.set_images([sandstone, dirt]).unwrap();
        let sandstone_tex = textures[0];
        let dirt_tex = textures[1];
        let sandstone_tex_id = stage.append(&sandstone_tex);
        let material_id = stage.append(&PbrMaterial {
            albedo_texture: sandstone_tex_id,
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let vertices = stage.append_array(&gpu_uv_unit_cube());
        let vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: material_id,
            ..Default::default()
        });
        let cube = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(vertex_data),
            transform: stage.append(&Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            }),
            vertex_count: vertices.len() as u32,
        });
        println!("cube: {cube:?}");

        // we should see a cube with a stoney texture
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_before.png", img);

        // update the material's texture on the GPU
        stage.write(sandstone_tex_id, &dirt_tex).unwrap();

        // we should see a cube with a dirty texture
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("unlit_textured_cube_material_after.png", img);
    }

    #[test]
    // Tests setting up a 2d scene with one CMY triangle.
    fn gpu_scene_sanity1() {
        let mut r =
            Renderling::headless(100, 100).with_background_color(Vec3::splat(0.0).extend(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        let vertices = stage.append_array(&vec![
            Vertex {
                position: Vec4::new(0.0, 0.0, 0.0, 1.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(100.0, 100.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(100.0, 0.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ]);
        let vertex_data = stage.append(&NativeVertexData {
            vertices,
            ..Default::default()
        });
        let _unit = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(vertex_data),
            vertex_count: vertices.len() as u32,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gpu_scene_sanity.png", img);
    }

    #[test]
    fn gpu_scene_sanity2() {
        let mut r =
            Renderling::headless(100, 100).with_background_color(Vec3::splat(0.0).extend(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        // now test the textures functionality
        let img = SceneImage::from_path("../../img/cheetah.jpg").unwrap();
        let textures = stage.append_array(&stage.set_images([img]).unwrap());
        let material = stage.append(&PbrMaterial {
            albedo_texture: textures.at(0),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });

        let vertices = stage.append_array(&vec![
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
        ]);
        let cheetah_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material,
            ..Default::default()
        });
        let transform = stage.append(&Transform {
            translation: Vec3::new(15.0, 35.0, 0.5),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        });
        let _cheetah_unit = stage.draw_unit(&RenderUnit {
            camera,
            transform,
            vertex_data: VertexData::new_native(cheetah_vertex_data),
            vertex_count: vertices.len() as u32,
        });
        let vertex_data = stage.append(&NativeVertexData {
            vertices,
            ..Default::default()
        });
        let _unit = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count: vertices.len() as u32,
            vertex_data: VertexData::new_native(vertex_data),
            ..Default::default()
        });

        let img = r.render_image().unwrap();

        img_diff::assert_img_eq("gpu_scene_sanity2.png", img);
    }

    #[test]
    fn atlas_uv_mapping() {
        let mut r =
            Renderling::headless(32, 32).with_background_color(Vec3::splat(0.0).extend(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_ortho2d(32.0, 32.0);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });
        let dirt = SceneImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = SceneImage::from_path("../../img/sandstone.png").unwrap();
        let texels = SceneImage::from_path("../../test_img/atlas_uv_mapping.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();
        let mut texels_tex = textures[2];
        texels_tex
            .modes
            .set_wrap_s(TextureAddressMode::CLAMP_TO_EDGE);
        texels_tex
            .modes
            .set_wrap_t(TextureAddressMode::CLAMP_TO_EDGE);
        let texels_tex_id = stage.append(&texels_tex);
        let material_id = stage.append(&PbrMaterial {
            albedo_texture: texels_tex_id,
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let vertices = stage.append_array(&{
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(1.0, 0.0, 0.0))
                .with_uv0(Vec2::new(1.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, 1.0, 0.0))
                .with_uv0(Vec2::new(0.0, 1.0));
            let br = Vertex::default()
                .with_position(Vec3::new(1.0, 1.0, 0.0))
                .with_uv0(Vec2::splat(1.0));
            vec![tl, bl, br, tl, br, tr]
        });
        let vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: material_id,
            ..Default::default()
        });
        let transform = stage.append(&Transform {
            scale: Vec3::new(32.0, 32.0, 1.0),
            ..Default::default()
        });
        let _unit = stage.draw_unit(&RenderUnit {
            camera,
            transform,
            vertex_data: VertexData::new_native(vertex_data),
            vertex_count: vertices.len() as u32,
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("atlas_uv_mapping.png", img);
    }

    #[test]
    fn uv_wrapping() {
        let icon_w = 32;
        let icon_h = 41;
        let sheet_w = icon_w * 3;
        let sheet_h = icon_h * 3;
        let w = sheet_w * 3 + 2;
        let h = sheet_h;
        let mut r = Renderling::headless(w, h).with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, view) = camera::default_ortho2d(w as f32, h as f32);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        let dirt = SceneImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = SceneImage::from_path("../../img/sandstone.png").unwrap();
        let texels = SceneImage::from_path("../../img/happy_mac.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();
        let texel_tex = textures[2];
        let mut clamp_tex = texel_tex;
        clamp_tex
            .modes
            .set_wrap_s(TextureAddressMode::CLAMP_TO_EDGE);
        clamp_tex
            .modes
            .set_wrap_t(TextureAddressMode::CLAMP_TO_EDGE);
        let mut repeat_tex = texel_tex;
        repeat_tex.modes.set_wrap_s(TextureAddressMode::REPEAT);
        repeat_tex.modes.set_wrap_t(TextureAddressMode::REPEAT);
        let mut mirror_tex = texel_tex;
        mirror_tex
            .modes
            .set_wrap_s(TextureAddressMode::MIRRORED_REPEAT);
        mirror_tex
            .modes
            .set_wrap_t(TextureAddressMode::MIRRORED_REPEAT);

        let clamp_material_id = stage.append(&PbrMaterial {
            albedo_texture: stage.append(&clamp_tex),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let repeat_material_id = stage.append(&PbrMaterial {
            albedo_texture: stage.append(&repeat_tex),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let mirror_material_id = stage.append(&PbrMaterial {
            albedo_texture: stage.append(&mirror_tex),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;
        let vertices = stage.append_array(&{
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(sheet_w, 0.0, 0.0))
                .with_uv0(Vec2::new(3.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, sheet_h, 0.0))
                .with_uv0(Vec2::new(0.0, 3.0));
            let br = Vertex::default()
                .with_position(Vec3::new(sheet_w, sheet_h, 0.0))
                .with_uv0(Vec2::splat(3.0));
            vec![tl, bl, br, tl, br, tr]
        });
        let clamp_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: clamp_material_id,
            ..Default::default()
        });
        let repeat_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: repeat_material_id,
            ..Default::default()
        });
        let mirror_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: mirror_material_id,
            ..Default::default()
        });

        let _clamp = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(clamp_vertex_data),
            vertex_count: vertices.len() as u32,
            ..Default::default()
        });
        let _repeat = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(repeat_vertex_data),
            vertex_count: vertices.len() as u32,
            transform: stage.append(&Transform {
                translation: Vec3::new(sheet_w + 1.0, 0.0, 0.0),
                ..Default::default()
            }),
        });
        let _mirror = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(mirror_vertex_data),
            vertex_count: vertices.len() as u32,
            transform: stage.append(&Transform {
                translation: Vec3::new(sheet_w as f32 * 2.0 + 2.0, 0.0, 0.0),
                ..Default::default()
            }),
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("uv_wrapping.png", img);
    }

    #[test]
    fn negative_uv_wrapping() {
        let icon_w = 32;
        let icon_h = 41;
        let sheet_w = icon_w * 3;
        let sheet_h = icon_h * 3;
        let w = sheet_w * 3 + 2;
        let h = sheet_h;
        let mut r = Renderling::headless(w, h).with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, view) = camera::default_ortho2d(w as f32, h as f32);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        let dirt = SceneImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = SceneImage::from_path("../../img/sandstone.png").unwrap();
        let texels = SceneImage::from_path("../../img/happy_mac.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();

        let texel_tex = textures[2];
        let mut clamp_tex = texel_tex;
        clamp_tex
            .modes
            .set_wrap_s(TextureAddressMode::CLAMP_TO_EDGE);
        clamp_tex
            .modes
            .set_wrap_t(TextureAddressMode::CLAMP_TO_EDGE);
        let mut repeat_tex = texel_tex;
        repeat_tex.modes.set_wrap_s(TextureAddressMode::REPEAT);
        repeat_tex.modes.set_wrap_t(TextureAddressMode::REPEAT);
        let mut mirror_tex = texel_tex;
        mirror_tex
            .modes
            .set_wrap_s(TextureAddressMode::MIRRORED_REPEAT);
        mirror_tex
            .modes
            .set_wrap_t(TextureAddressMode::MIRRORED_REPEAT);

        let clamp_material_id = stage.append(&PbrMaterial {
            albedo_texture: stage.append(&clamp_tex),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let repeat_material_id = stage.append(&PbrMaterial {
            albedo_texture: stage.append(&repeat_tex),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let mirror_material_id = stage.append(&PbrMaterial {
            albedo_texture: stage.append(&mirror_tex),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;

        let vertices = stage.append_array(&{
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(sheet_w, 0.0, 0.0))
                .with_uv0(Vec2::new(-3.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, sheet_h, 0.0))
                .with_uv0(Vec2::new(0.0, -3.0));
            let br = Vertex::default()
                .with_position(Vec3::new(sheet_w, sheet_h, 0.0))
                .with_uv0(Vec2::splat(-3.0));
            vec![tl, bl, br, tl, br, tr]
        });

        let clamp_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: clamp_material_id,
            ..Default::default()
        });
        let repeat_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: repeat_material_id,
            ..Default::default()
        });
        let mirror_vertex_data = stage.append(&NativeVertexData {
            vertices,
            material: mirror_material_id,
            ..Default::default()
        });

        let _clamp = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(clamp_vertex_data),
            vertex_count: vertices.len() as u32,
            ..Default::default()
        });
        let _repeat = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(repeat_vertex_data),
            vertex_count: vertices.len() as u32,
            transform: stage.append(&Transform {
                translation: Vec3::new(sheet_w + 1.0, 0.0, 0.0),
                ..Default::default()
            }),
        });
        let _mirror = stage.draw_unit(&RenderUnit {
            camera,
            vertex_data: VertexData::new_native(mirror_vertex_data),
            vertex_count: vertices.len() as u32,
            transform: stage.append(&Transform {
                translation: Vec3::new(sheet_w as f32 * 2.0 + 2.0, 0.0, 0.0),
                ..Default::default()
            }),
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("negative_uv_wrapping.png", img);
    }

    #[test]
    fn transform_uvs_for_atlas() {
        let mut tex = GpuTexture {
            offset_px: UVec2::ZERO,
            size_px: UVec2::ONE,
            modes: {
                let mut modes = TextureModes::default();
                modes.set_wrap_s(TextureAddressMode::CLAMP_TO_EDGE);
                modes.set_wrap_t(TextureAddressMode::CLAMP_TO_EDGE);
                modes
            },
            ..Default::default()
        };
        assert_eq!(Vec2::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(100)));
        assert_eq!(Vec2::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(1)));
        assert_eq!(Vec2::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(256)));
        tex.offset_px = UVec2::splat(10);
        assert_eq!(Vec2::splat(0.1), tex.uv(Vec2::ZERO, UVec2::splat(100)));
    }

    #[test]
    /// Tests shading with directional light.
    fn old_scene_cube_directional() {
        let mut r =
            Renderling::headless(100, 100).with_background_color(Vec3::splat(0.0).extend(1.0));

        let mut builder = r.new_scene();
        let red = Vec3::X.extend(1.0);
        let green = Vec3::Y.extend(1.0);
        let blue = Vec3::Z.extend(1.0);
        let _dir_red = builder
            .new_directional_light()
            .with_direction(Vec3::NEG_Y)
            .with_color(red)
            .with_intensity(10.0)
            .build();
        let _dir_green = builder
            .new_directional_light()
            .with_direction(Vec3::NEG_X)
            .with_color(green)
            .with_intensity(10.0)
            .build();
        let _dir_blue = builder
            .new_directional_light()
            .with_direction(Vec3::NEG_Z)
            .with_color(blue)
            .with_intensity(10.0)
            .build();

        let material = builder.add_material(PbrMaterial::default());

        let _cube = builder
            .new_entity()
            .with_meshlet(
                renderling_shader::math::unit_cube()
                    .into_iter()
                    .map(|(p, n)| Vertex {
                        position: p.extend(1.0),
                        normal: n.extend(0.0),
                        ..Default::default()
                    }),
            )
            .with_material(material)
            .build();

        let mut scene = builder.build().unwrap();

        let (projection, _) = camera::default_perspective(100.0, 100.0);
        let view = Mat4::look_at_rh(
            Vec3::new(1.8, 1.8, 1.8),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );
        scene.set_camera(projection, view);

        r.setup_render_graph(RenderGraphConfig {
            scene: Some(scene),
            with_screen_capture: true,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("scene_cube_directional.png", img);
    }

    #[test]
    /// Tests shading with directional light.
    fn scene_cube_directional() {
        let mut r =
            Renderling::headless(100, 100).with_background_color(Vec3::splat(0.0).extend(1.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, _) = camera::default_perspective(100.0, 100.0);
        let view = Mat4::look_at_rh(
            Vec3::new(1.8, 1.8, 1.8),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

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

        let material = stage.append(&PbrMaterial::default());
        let vertices = stage.append_array(
            &renderling_shader::math::unit_cube()
                .into_iter()
                .map(|(p, n)| Vertex {
                    position: p.extend(1.0),
                    normal: n.extend(0.0),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        );
        let vertex_data = stage.append(&NativeVertexData {
            vertices,
            material,
            ..Default::default()
        });

        let cube = stage.draw_unit(&RenderUnit {
            vertex_data: VertexData::new_native(vertex_data),
            vertex_count: vertices.len() as u32,
            camera,
            ..Default::default()
        });

        let data = stage.read_all_raw().unwrap();
        let invocation = VertexInvocation::invoke(cube.inner(), 0, &data);
        println!("vertex invocation: {:#?}", invocation);

        let atlas: image::DynamicImage = stage.atlas.read().unwrap().atlas_img().into();
        let atlas = CpuTexture2d { image: atlas };
        let _ = crate::shader::stage_fragment_impl(&atlas, &CpuSampler);

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

    #[test]
    // tests that nested children are transformed by their parent's transform
    fn scene_parent_sanity() {
        let mut r = Renderling::headless(100, 100);
        r.set_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(projection, view);
        let size = 1.0;
        let root = builder.new_entity().with_scale([25.0, 25.0, 1.0]).build();
        let cyan_tri = builder
            .new_entity()
            .with_meshlet(vec![
                Vertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(size, size, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(size, 0.0, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
            ])
            .with_position([1.0, 1.0, 0.0])
            .with_parent(root)
            .build();
        let yellow_tri = builder //
            .new_entity()
            .with_meshlet(vec![
                Vertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(size, size, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(size, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
            ])
            .with_position([1.0, 1.0, 0.0])
            .with_parent(&cyan_tri)
            .build();
        let _red_tri = builder
            .new_entity()
            .with_meshlet(vec![
                Vertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(size, size, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(size, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
            ])
            .with_position([1.0, 1.0, 0.0])
            .with_parent(&yellow_tri)
            .build();

        assert_eq!(
            vec![
                GpuEntity {
                    id: Id::new(0),
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    scale: Vec4::new(25.0, 25.0, 1.0, 1.0),
                    ..Default::default()
                },
                GpuEntity {
                    id: Id::new(1),
                    parent: Id::new(0),
                    position: Vec4::new(1.0, 1.0, 0.0, 0.0),
                    scale: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    mesh_first_vertex: 0,
                    mesh_vertex_count: 3,
                    ..Default::default()
                },
                GpuEntity {
                    id: Id::new(2),
                    parent: Id::new(1),
                    position: Vec4::new(1.0, 1.0, 0.0, 0.0),
                    scale: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    mesh_first_vertex: 3,
                    mesh_vertex_count: 3,
                    ..Default::default()
                },
                GpuEntity {
                    id: Id::new(3),
                    parent: Id::new(2),
                    position: Vec4::new(1.0, 1.0, 0.0, 0.0),
                    scale: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    mesh_first_vertex: 6,
                    mesh_vertex_count: 3,
                    ..Default::default()
                }
            ],
            builder.entities
        );
        let tfrm = yellow_tri.get_world_transform(&builder.entities);
        assert_eq!(
            (
                Vec3::new(50.0, 50.0, 0.0),
                Quat::IDENTITY,
                Vec3::new(25.0, 25.0, 1.0),
            ),
            tfrm
        );
        // while we're at it let's also check that skinning doesn't affect
        // entities/vertices that aren't skins
        let vertex = &builder.vertices[yellow_tri.mesh_first_vertex as usize];
        let skin_matrix = vertex.get_skin_matrix(&yellow_tri.skin_joint_ids, &builder.entities);
        assert_eq!(Mat4::IDENTITY, skin_matrix);

        let entities = builder.entities.clone();
        let scene = builder.build().unwrap();
        r.setup_render_graph(RenderGraphConfig {
            scene: Some(scene),
            with_screen_capture: true,
            with_bloom: false,
            ..Default::default()
        });

        let gpu_entities = futures_lite::future::block_on(
            r.graph
                .get_resource::<Scene>()
                .unwrap()
                .unwrap()
                .entities
                .read_gpu(r.get_device(), r.get_queue(), 0, entities.len()),
        )
        .unwrap();
        assert_eq!(entities, gpu_entities);

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

    #[test]
    // Tests the initial implementation of pbr metallic roughness on an array of
    // spheres with different metallic roughnesses lit by an environment map.
    //
    // see https://learnopengl.com/PBR/Lighting
    fn pbr_metallic_roughness_spheres() {
        let ss = 600;
        let mut r =
            Renderling::headless(ss, ss).with_background_color(Vec3::splat(0.0).extend(1.0));

        let radius = 0.5;
        let mut icosphere = icosahedron::Polyhedron::new_isocahedron(radius, 5);
        icosphere.compute_triangle_normals();
        let icosahedron::Polyhedron {
            positions,
            normals,
            cells,
            ..
        } = icosphere;
        log::info!("icosphere created");

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

        let mut builder = r
            .new_scene()
            .with_camera(projection, view)
            .with_skybox_image_from_path("../../img/hdr/resting_place.hdr");
        let (start, count) = builder.add_meshlet(sphere_vertices);

        for i in 0..k {
            let roughness = i as f32 / (k - 1) as f32;
            let x = (diameter + spacing) * i as f32;
            for j in 0..k {
                let metallic = j as f32 / (k - 1) as f32;
                let y = (diameter + spacing) * j as f32;
                let material_id = builder.add_material(PbrMaterial {
                    albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    metallic_factor: metallic,
                    roughness_factor: roughness,
                    ..Default::default()
                });
                let _entity = builder
                    .new_entity()
                    .with_starting_vertex_and_count(start, count)
                    .with_material(material_id)
                    .with_position([x, y, 0.0])
                    .build();
            }
        }

        let scene = builder.build().unwrap();
        r.setup_render_graph(RenderGraphConfig {
            scene: Some(scene),
            with_screen_capture: true,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("pbr_metallic_roughness_spheres.png", img);
    }

    #[test]
    fn is_skin_sanity() {
        let info = GpuEntityInfo(2048);
        assert!(info.is_skin());
    }
}
