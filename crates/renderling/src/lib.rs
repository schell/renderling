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
//!   - [x] physically based shading atlas)
//! - [x] gltf support
//!   - [x] scenes, nodes
//!   - [x] cameras
//!   - [x] meshes
//!   - [x] materials
//!   - [x] textures, images, samplers
//!   - [x] skins
//!   - [x] animations
//! - [x] high definition rendering
//! - [x] image based lighting
//! - [ ] bloom
//! - [ ] ssao
//! - [ ] depth of field
//!
//! ## Raw shaders
//! You can also use the [shaders module](crate::shaders) without renderlings
//! and manage your own resources for maximum flexibility.

// TODO: Audit the API and make it more ergonomic/predictable.

mod atlas;
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
mod skybox;
mod stage;
mod state;
#[cfg(feature = "text")]
mod text;
mod texture;
mod tonemapping;
//mod tutorial;
mod ui;
mod uniform;

pub use atlas::*;
pub use buffer_array::*;
pub use camera::*;
pub use hdr::*;
use image::GenericImageView;
pub use renderer::*;
pub use skybox::*;
pub use stage::*;
pub use state::*;
#[cfg(feature = "text")]
pub use text::*;
pub use texture::*;
pub use tonemapping::*;
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

pub use crabslab::*;
pub use graph::{graph, Graph, GraphError, Move, View, ViewMut};
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
    use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
    use pretty_assertions::assert_eq;
    use renderling_shader::{
        gltf as gl,
        pbr::PbrMaterial,
        stage::{light::*, Camera, RenderUnit, Transform, Vertex},
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
        let node = stage.append(&gl::GltfNode {
            mesh: stage.append(&mesh),
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let _tri = stage.draw_unit(&RenderUnit {
            camera,
            node_path,
            vertex_count: 3,
            ..Default::default()
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
        let camera = stage.append(&Camera::new(projection, view));
        let mesh = stage
            .new_mesh()
            .with_primitive(right_tri_vertices(), [], Id::NONE)
            .build();
        let node = stage.append(&gl::GltfNode {
            mesh: stage.append(&mesh),
            ..Default::default()
        });
        let transform = stage.append(&Transform::default());
        let _tri = stage.draw_unit(&RenderUnit {
            camera,
            node_path: stage.append_array(&[node]),
            vertex_count: 3,
            transform,
            ..Default::default()
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
        let transform = Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        };
        let node = stage.append(&gl::GltfNode {
            mesh: stage.append(&mesh),
            ..Default::default()
        });
        let _cube = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count,
            node_path: stage.append_array(&[node]),
            transform: stage.append(&transform),
            ..Default::default()
        });
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube.png", img);
    }

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
        let node = stage.append(&gl::GltfNode {
            mesh: stage.append(&mesh),
            ..Default::default()
        });
        let mut render_unit = RenderUnit {
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
        let _cube_one = stage.draw_unit(&render_unit);
        render_unit.transform = stage.append(&Transform {
            translation: Vec3::new(4.5, 0.0, 0.0),
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4),
        });
        let cube_two = stage.draw_unit(&render_unit);

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
    // Tests the ability to specify indexed vertices, as well as the ability to
    // update a field within a struct stored on the slab by offset.
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
        let pyramid_vertices = pyramid_points().map(cmy_gpu_vertex);
        let pyramid_indices = pyramid_indices().map(|i| i as u32);
        let _pyramid_vertex_count = pyramid_indices.len();
        let pyramid_mesh = stage
            .new_mesh()
            .with_primitive(pyramid_vertices, pyramid_indices, Id::NONE)
            .build();
        let pyramid_node = stage.append(&gl::GltfNode {
            mesh: stage.append(&pyramid_mesh),
            ..Default::default()
        });
        let pyramid_node_path = stage.append_array(&[pyramid_node]);

        let cube_vertices = renderling_shader::math::UNIT_POINTS.map(cmy_gpu_vertex);
        let cube_indices = renderling_shader::math::UNIT_INDICES.map(|i| i as u32);
        let cube_vertex_count = cube_indices.len();
        let cube_mesh = stage
            .new_mesh()
            .with_primitive(cube_vertices, cube_indices, Id::NONE)
            .build();
        let cube_node = stage.append(&gl::GltfNode {
            mesh: stage.append(&cube_mesh),
            ..Default::default()
        });
        let cube = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count: cube_vertex_count as u32,
            node_path: stage.append_array(&[cube_node]),
            transform: stage.append(&Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            }),
            ..Default::default()
        });

        // we should see a cube
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("cmy_cube_remesh_before.png", img);

        // Update the cube mesh to a pyramid by overwriting the `.node_path` field
        // of `RenderUnit`
        stage
            .write(cube + RenderUnit::offset_of_node_path(), &pyramid_node_path)
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

    #[test]
    // Tests that updating the material actually updates the rendering of an unlit
    // mesh
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
        let sandstone = AtlasImage::from(image::open("../../img/sandstone.png").unwrap());
        let dirt = AtlasImage::from(image::open("../../img/dirt.jpg").unwrap());
        let textures = stage.set_images([sandstone, dirt]).unwrap();
        let sandstone_tex = textures[0];
        let dirt_tex = textures[1];
        let sandstone_tex_id = stage.append(&sandstone_tex);
        let material_id = stage.append(&PbrMaterial {
            albedo_texture: sandstone_tex_id,
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let vertices = gpu_uv_unit_cube();
        let vertex_count = vertices.len() as u32;
        let mesh = stage
            .new_mesh()
            .with_primitive(vertices, [], material_id)
            .build();
        let node = stage.append(&gl::GltfNode {
            mesh: stage.append(&mesh),
            ..Default::default()
        });
        let cube = stage.draw_unit(&RenderUnit {
            camera,
            node_path: stage.append_array(&[node]),
            transform: stage.append(&Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            }),
            vertex_count,
            ..Default::default()
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
        let textures = stage.append_array(&stage.set_images([img]).unwrap());
        let cheetah_material = stage.append(&PbrMaterial {
            albedo_texture: textures.at(0),
            lighting_model: LightingModel::NO_LIGHTING,
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
        let _unit = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count: cheetah_primitive.vertex_count,
            node_path: stage.append_array(&[stage.append(&gl::GltfNode {
                mesh: stage.append(&gl::GltfMesh {
                    primitives: stage.append_array(&[color_primitive]),
                    ..Default::default()
                }),
                ..Default::default()
            })]),
            ..Default::default()
        });
        let _cheetah_unit = stage.draw_unit(&RenderUnit {
            camera,
            transform: stage.append(&Transform {
                translation: Vec3::new(15.0, 35.0, 0.5),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            }),
            vertex_count: cheetah_primitive.vertex_count,
            node_path: stage.append_array(&[stage.append(&gl::GltfNode {
                mesh: stage.append(&gl::GltfMesh {
                    primitives: stage.append_array(&[cheetah_primitive]),
                    ..Default::default()
                }),
                ..Default::default()
            })]),
            ..Default::default()
        });

        let img = r.render_image().unwrap();

        img_diff::assert_img_eq("gpu_scene_sanity2.png", img);
    }

    #[test]
    /// Tests shading with directional light.
    fn scene_cube_directional() {
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

        let material = stage.append(&PbrMaterial::default());
        let verts = renderling_shader::math::unit_cube()
            .into_iter()
            .map(|(p, n)| Vertex {
                position: p.extend(1.0),
                normal: n.extend(0.0),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let vertex_count = verts.len() as u32;
        let mesh = stage.new_mesh().with_primitive(verts, [], material).build();
        let _cube = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count,
            node_path: stage.append_array(&[stage.append(&gl::GltfNode {
                mesh: stage.append(&mesh),
                ..Default::default()
            })]),
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

    #[test]
    // tests that nested children are transformed by their parent's transform
    fn scene_parent_sanity() {
        let mut r = Renderling::headless(100, 100);
        r.set_background_color(Vec4::splat(0.0));
        let stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera::new(projection, view));
        let size = 1.0;
        let cyan_material = stage.append(&PbrMaterial {
            albedo_factor: Vec4::new(0.0, 1.0, 1.0, 1.0),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let yellow_material = stage.append(&PbrMaterial {
            albedo_factor: Vec4::new(1.0, 1.0, 0.0, 1.0),
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        let red_material = stage.append(&PbrMaterial {
            albedo_factor: Vec4::new(1.0, 0.0, 0.0, 1.0),
            lighting_model: LightingModel::NO_LIGHTING,
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

        let root_node = stage.allocate::<gl::GltfNode>();
        let cyan_node = stage.allocate::<gl::GltfNode>();
        let yellow_node = stage.allocate::<gl::GltfNode>();
        let red_node = stage.allocate::<gl::GltfNode>();

        // Write the nodes now that we have references to them all
        stage
            .write(
                root_node,
                &gl::GltfNode {
                    children: stage.append_array(&[cyan_node]),
                    scale: Vec3::new(25.0, 25.0, 1.0),
                    ..Default::default()
                },
            )
            .unwrap();
        stage
            .write(
                cyan_node,
                &gl::GltfNode {
                    mesh: stage.append(&gl::GltfMesh {
                        primitives: stage.append_array(&[cyan_primitive]),
                        ..Default::default()
                    }),
                    children: stage.append_array(&[yellow_node]),
                    translation: Vec3::new(1.0, 1.0, 0.0),
                    ..Default::default()
                },
            )
            .unwrap();
        stage
            .write(
                yellow_node,
                &gl::GltfNode {
                    mesh: stage.append(&gl::GltfMesh {
                        primitives: stage.append_array(&[yellow_primitive]),
                        ..Default::default()
                    }),
                    children: stage.append_array(&[red_node]),
                    translation: Vec3::new(1.0, 1.0, 0.0),
                    ..Default::default()
                },
            )
            .unwrap();
        stage
            .write(
                red_node,
                &gl::GltfNode {
                    mesh: stage.append(&gl::GltfMesh {
                        primitives: stage.append_array(&[red_primitive]),
                        ..Default::default()
                    }),
                    translation: Vec3::new(1.0, 1.0, 0.0),
                    ..Default::default()
                },
            )
            .unwrap();

        let _cyan_unit = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count: 3,
            node_path: stage.append_array(&[root_node, cyan_node]),
            ..Default::default()
        });
        let _yellow_unit = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count: 3,
            node_path: stage.append_array(&[root_node, cyan_node, yellow_node]),
            ..Default::default()
        });
        let _red_unit = stage.draw_unit(&RenderUnit {
            camera,
            vertex_count: 3,
            node_path: stage.append_array(&[root_node, cyan_node, yellow_node, red_node]),
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
                prim.material = stage.append(&PbrMaterial {
                    albedo_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    metallic_factor: metallic,
                    roughness_factor: roughness,
                    ..Default::default()
                });
                let _entity = stage.draw_unit(&RenderUnit {
                    camera,
                    vertex_count: prim.vertex_count,
                    node_path: stage.append_array(&[stage.append(&gl::GltfNode {
                        mesh: stage.append(&gl::GltfMesh {
                            primitives: stage.append_array(&[prim]),
                            ..Default::default()
                        }),
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    })]),
                    ..Default::default()
                });
            }
        }

        let (device, queue) = r.get_device_and_queue_owned();
        let hdr_image = AtlasImage::from_hdr_path("../../img/hdr/resting_place.hdr").unwrap();
        let skybox = crate::skybox::Skybox::new(&device, &queue, hdr_image, camera);
        stage.set_skybox(skybox);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("pbr/metallic_roughness_spheres.png", img);
    }

    #[test]
    fn is_skin_sanity() {
        let info = GpuEntityInfo(2048);
        assert!(info.is_skin());
    }
}
