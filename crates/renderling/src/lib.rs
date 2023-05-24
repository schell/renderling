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
//! cuteness. Renderlings are configurable DAGs.
//!
//! ## Features
//!
//! - forward+ style pipeline, configurable lighting model per material
//!   - [ ] physically based shading
//!   - [x] blinn-phong shading
//!   - [x] user interface "colored text" shading (uses opacity glyphs in an
//!     atlas)
//!   - [x] no shading
//! - [ ] gltf support
//!   - [ ] scenes, nodes
//!   - [x] cameras
//!   - [x] meshes
//!   - [x] materials
//!   - [x] textures, images, samplers
//!   - [ ] skins
//!   - [ ] animations
//! - [ ] high definition rendering
//! - [ ] bloom
//! - [ ] image based lighting
//! - [ ] ssao
//! - [ ] depth of field
//!
//! ## Raw shaders
//! You can also use the [shaders module](crate::shaders) without renderlings
//! and manage your own resources for maximum flexibility.

mod atlas;
// mod bank;
mod buffer_array;
mod camera;
mod uniform;
pub mod node;
mod renderer;
mod scene;
mod state;
#[cfg(feature = "text")]
mod text;
mod texture;
mod ui;

pub use atlas::*;
pub use buffer_array::*;
pub use camera::*;
pub use uniform::*;
use moongraph::IsGraphNode;
pub use renderer::*;
pub use scene::*;
pub use state::*;
#[cfg(feature = "text")]
pub use text::*;
pub use texture::*;
pub use ui::*;

pub mod math;

pub mod graph {
    //! Re-exports of [`moongraph`].

    pub use moongraph::*;

    pub type RenderNode = Node<Function, TypeKey>;
}

pub use graph::{Graph, Move, Read, Write};

#[cfg(test)]
mod img_diff;

pub fn setup_ui_and_scene_render_graph(
    r: &mut Renderling,
    ui_scene: UiScene,
    ui_objects: impl IntoIterator<Item = UiDrawObject>,
    scene: Scene,
    with_screen_capture: bool,
) {
    let ui_objects = UiDrawObjects(ui_objects.into_iter().collect::<Vec<_>>());
    r.graph.add_resource(ui_scene);
    r.graph.add_resource(ui_objects);
    r.graph.add_resource(scene);
    let ui_pipeline = UiRenderPipeline(
        r.graph
            .visit(|(device, target): (Read<Device>, Read<RenderTarget>)| {
                create_ui_pipeline(&device, target.format())
            })
            .unwrap(),
    );
    r.graph.add_resource(ui_pipeline);

    let (hdr_surface,) = r
        .graph
        .visit(node::create_hdr_render_surface)
        .unwrap()
        .unwrap();
    let pipeline = SceneRenderPipeline({
        let device = r.get_device();
        create_scene_render_pipeline(&device, hdr_surface.texture.texture.format())
    });
    r.graph.add_resource(pipeline);
    let scene_cull_pipeline = SceneComputeCullPipeline(
        r.graph
            .visit(|device: Read<Device>| create_scene_compute_cull_pipeline(&device))
            .unwrap(),
    );
    r.graph.add_resource(scene_cull_pipeline);
    let scene_pipeline = SceneRenderPipeline(
        r.graph
            .visit(|device: Read<Device>| {
                create_scene_render_pipeline(&device, hdr_surface.texture.texture.format())
            })
            .unwrap(),
    );
    r.graph.add_resource(scene_pipeline);
    r.graph.add_resource(hdr_surface);
    r.graph.add_node(
        crate::node::create_frame
            .into_node()
            .with_name("create_frame"),
    );
    r.graph.add_node(
        node::clear_surface_hdr_and_depth
            .into_node()
            .with_name("clear_hdr_frame_and_depth"),
    );

    r.graph.add_node(
        node::hdr_surface_update
            .into_node()
            .with_name("hdr_surface_update"),
    );
    r.graph
        .add_node(scene_update.into_node().with_name("scene_update"));
    r.graph.add_node(
        scene_cull
            .into_node()
            .with_name("scene_cull")
            .run_after("scene_update"),
    );
    r.graph
        .add_node(ui_scene_update.into_node().with_name("ui_scene_update"));
    r.graph.add_barrier();

    r.graph
        .add_node(scene_render.into_node().with_name("scene_render"));
    r.graph.add_node(
        scene_tonemapping
            .into_node()
            .with_name("scene_tonemapping")
            .run_after("scene_render")
            .run_before("present"),
    );
    r.graph.add_node(
        crate::node::clear_depth
            .into_node()
            .with_name("clear_depth")
            .run_after("scene_tonemapping"),
    );
    r.graph.add_node(
        ui_scene_render
            .into_node()
            .with_name("ui_scene_render")
            .run_after("clear_depth"),
    );

    r.graph.add_barrier();

    if with_screen_capture {
        r.graph.add_node(
            crate::node::PostRenderBufferCreate::create
                .into_node()
                .with_name("copy_frame_to_post")
                .run_before("present"),
        );
    }

    r.graph
        .add_node(crate::node::present.into_node().with_name("present"));
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::{Mat3, Mat4, Quat, UVec2, Vec2, Vec3, Vec4};
    use moongraph::Read;
    use renderling_shader::scene::{DrawIndirect, GpuEntity, GpuVertex};

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

    fn right_tri_vertices() -> Vec<GpuVertex> {
        vec![
            GpuVertex::default()
                .with_position([0.0, 0.0, 0.5])
                .with_color([0.0, 1.0, 1.0, 1.0]),
            GpuVertex::default()
                .with_position([0.0, 100.0, 0.5])
                .with_color([1.0, 1.0, 0.0, 1.0]),
            GpuVertex::default()
                .with_position([100.0, 0.0, 0.5])
                .with_color([1.0, 0.0, 1.0, 1.0]),
        ]
    }

    pub fn _init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            //.filter_level(log::LevelFilter::Trace)
            .filter_module("renderling", log::LevelFilter::Trace)
            //.filter_module("naga", log::LevelFilter::Debug)
            //.filter_module("wgpu", log::LevelFilter::Debug)
            //.filter_module("wgpu_hal", log::LevelFilter::Warn)
            .try_init();
    }

    struct CmyTri {
        ui: Renderling,
        tri: GpuEntity,
    }

    fn cmy_triangle_setup() -> CmyTri {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(1.0));
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(projection, view);
        let tri = builder
            .new_entity()
            .with_meshlet(right_tri_vertices())
            .build();
        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        CmyTri { ui: r, tri }
    }

    #[test]
    fn cmy_triangle_sanity() {
        let mut c = cmy_triangle_setup();
        let img = c.ui.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_triangle.png", img);
    }

    #[test]
    fn cmy_triangle_update_transform() {
        let mut c = cmy_triangle_setup();
        let _ = c.ui.render_image().unwrap();

        let mut tri = c.tri;
        tri.position = Vec4::new(100.0, 0.0, 0.0, 0.0);
        tri.rotation = Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2);
        tri.scale = Vec4::new(0.5, 0.5, 1.0, 0.0);
        c.ui.graph
            .visit(|mut scene: Write<Scene>| {
                scene.update_entity(tri).unwrap();
            })
            .unwrap();

        let img = c.ui.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_triangle_update_transform.png", img);
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

    fn cmy_gpu_vertex(p: Vec3) -> GpuVertex {
        let r: f32 = p.z + 0.5;
        let g: f32 = p.x + 0.5;
        let b: f32 = p.y + 0.5;
        GpuVertex::default()
            .with_position([p.x.min(1.0), p.y.min(1.0), p.z.min(1.0)])
            .with_color([r, g, b, 1.0])
    }

    fn gpu_cube_vertices() -> Vec<GpuVertex> {
        let vertices = crate::math::unit_points();
        let indices: [u16; 12 * 3] = [
            0, 2, 1, 0, 3, 2, // top
            0, 4, 3, 4, 5, 3, // front
            3, 6, 2, 3, 5, 6, // right
            1, 7, 0, 7, 4, 0, // left
            4, 6, 5, 4, 7, 6, // bottom
            2, 7, 1, 2, 6, 7, // back
        ];
        indices
            .iter()
            .map(|i| cmy_gpu_vertex(vertices[*i as usize]))
            .collect()
    }

    fn gpu_pyramid_vertices() -> Vec<GpuVertex> {
        let vertices = pyramid_points();
        let indices = pyramid_indices();
        indices
            .into_iter()
            .map(|i| cmy_gpu_vertex(vertices[i as usize]))
            .collect()
    }

    #[test]
    fn cmy_cube_sanity() {
        _init_logging();
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(1.0));
        let mut builder = r.new_scene().with_camera(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0),
            Mat4::look_at_rh(Vec3::new(0.0, 12.0, 20.0), Vec3::ZERO, Vec3::Y),
        );

        let _cube = builder
            .new_entity()
            .with_meshlet(gpu_cube_vertices())
            .with_scale(Vec3::new(6.0, 6.0, 6.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4))
            .build();
        let scene = builder.build().unwrap();

        crate::setup_scene_render_graph(scene, &mut r, true);
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube.png", img);
    }

    #[test]
    fn cmy_cube_visible() {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(1.0));

        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(projection, view);

        let _cube_one = builder
            .new_entity()
            .with_meshlet(gpu_cube_vertices())
            .with_position(Vec3::new(-4.5, 0.0, 0.0))
            .with_scale(Vec3::new(6.0, 6.0, 6.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4))
            .build();

        let mut cube_two = builder
            .new_entity()
            .with_meshlet(gpu_cube_vertices())
            .with_position(Vec3::new(4.5, 0.0, 0.0))
            .with_scale(Vec3::new(6.0, 6.0, 6.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4))
            .build();

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        // we should see two colored cubes
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube_visible_before.png", img);

        // update cube two making in invisible
        r.graph
            .visit(|mut scene: Write<Scene>| {
                cube_two.visible = 0;
                scene.update_entity(cube_two).unwrap();
            })
            .unwrap();

        // we should see one colored cube
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube_visible_after.png", img);

        // update cube two making in visible again
        r.graph
            .visit(|mut scene: Write<Scene>| {
                cube_two.visible = 1;
                scene.update_entity(cube_two).unwrap();
            })
            .unwrap();

        // we should see two colored cubes again
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_with_testname(
            "cmy_cube_visible_before_again",
            "cmy_cube_visible_before.png",
            img,
        );
    }

    #[test]
    fn cmy_cube_remesh() {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(projection, view);

        let (pyramid_start, pyramid_count) = builder.add_meshlet(gpu_pyramid_vertices());

        let mut cube = builder
            .new_entity()
            .with_meshlet(gpu_cube_vertices())
            .with_scale(Vec3::new(10.0, 10.0, 10.0))
            .build();

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        // we should see a cube
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube_remesh_before.png", img);

        // update the cube mesh to a pyramid
        r.graph
            .visit(|mut scene: Write<Scene>| {
                cube.mesh_first_vertex = pyramid_start;
                cube.mesh_vertex_count = pyramid_count;
                scene.update_entity(cube).unwrap();
            })
            .unwrap();

        // we should see a pyramid
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube_remesh_after.png", img);
    }

    fn gpu_uv_unit_cube() -> Vec<GpuVertex> {
        let p: [Vec3; 8] = crate::math::unit_points();
        let tl = Vec2::new(0.0, 0.0);
        let tr = Vec2::new(1.0, 0.0);
        let bl = Vec2::new(0.0, 1.0);
        let br = Vec2::new(1.0, 1.0);

        vec![
            // top
            GpuVertex::default().with_position(p[0]).with_uv0(bl),
            GpuVertex::default().with_position(p[2]).with_uv0(tr),
            GpuVertex::default().with_position(p[1]).with_uv0(tl),
            GpuVertex::default().with_position(p[0]).with_uv0(bl),
            GpuVertex::default().with_position(p[3]).with_uv0(br),
            GpuVertex::default().with_position(p[2]).with_uv0(tr),
            // bottom
            GpuVertex::default().with_position(p[4]).with_uv0(bl),
            GpuVertex::default().with_position(p[6]).with_uv0(tr),
            GpuVertex::default().with_position(p[5]).with_uv0(tl),
            GpuVertex::default().with_position(p[4]).with_uv0(bl),
            GpuVertex::default().with_position(p[7]).with_uv0(br),
            GpuVertex::default().with_position(p[6]).with_uv0(tr),
            // left
            GpuVertex::default().with_position(p[7]).with_uv0(bl),
            GpuVertex::default().with_position(p[0]).with_uv0(tr),
            GpuVertex::default().with_position(p[1]).with_uv0(tl),
            GpuVertex::default().with_position(p[7]).with_uv0(bl),
            GpuVertex::default().with_position(p[4]).with_uv0(br),
            GpuVertex::default().with_position(p[0]).with_uv0(tr),
            // right
            GpuVertex::default().with_position(p[5]).with_uv0(bl),
            GpuVertex::default().with_position(p[2]).with_uv0(tr),
            GpuVertex::default().with_position(p[3]).with_uv0(tl),
            GpuVertex::default().with_position(p[5]).with_uv0(bl),
            GpuVertex::default().with_position(p[6]).with_uv0(br),
            GpuVertex::default().with_position(p[2]).with_uv0(tr),
            // front
            GpuVertex::default().with_position(p[4]).with_uv0(bl),
            GpuVertex::default().with_position(p[3]).with_uv0(tr),
            GpuVertex::default().with_position(p[0]).with_uv0(tl),
            GpuVertex::default().with_position(p[4]).with_uv0(bl),
            GpuVertex::default().with_position(p[5]).with_uv0(br),
            GpuVertex::default().with_position(p[3]).with_uv0(tr),
        ]
    }

    #[test]
    // tests that updating the material actually updates the rendering of an unlit
    // mesh
    fn unlit_textured_cube_material() {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec4::splat(0.0));
        let (proj, view) = camera::default_perspective(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(proj, view);
        let sandstone = SceneImage::from(image::open("../../img/sandstone.png").unwrap());
        let sandstone_id = builder.add_image_texture(sandstone);
        let dirt = SceneImage::from(image::open("../../img/dirt.jpg").unwrap());
        let dirt_id = builder.add_image_texture(dirt);

        let material_id = builder
            .new_unlit_material()
            .with_texture0(sandstone_id)
            .build();
        let mut material = builder.materials.get(material_id.index()).copied().unwrap();
        let _cube = builder
            .new_entity()
            .with_material(material_id)
            .with_meshlet(gpu_uv_unit_cube())
            .with_scale(Vec3::new(10.0, 10.0, 10.0))
            .build();
        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);
        // we should see a cube with a stoney texture
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("unlit_textured_cube_material_before.png", img);

        // update the material's texture on the GPU
        r.graph
            .visit(|mut scene: Write<Scene>| {
                material.texture0 = dirt_id;
                let _ = scene
                    .materials
                    .overwrite(material_id.index(), vec![material])
                    .unwrap();
            })
            .unwrap();

        // we should see a cube with a dirty texture
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("unlit_textured_cube_material_after.png", img);
    }

    #[test]
    fn gpu_array_update() {
        _init_logging();
        let (device, queue, _) = futures_lite::future::block_on(
            crate::state::new_device_queue_and_target(100, 100, None as Option<CreateSurfaceFn>),
        );

        let points = vec![
            Vec4::new(0.0, 0.0, 0.0, 0.0),
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(1.0, 1.0, 0.0, 0.0),
        ];
        let mut array = GpuArray::new(
            &device,
            &points,
            6,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );

        // send them to the GPU
        array.update(&queue);
        // read them back
        let verts = array.read(&device, &queue, 0, 3).unwrap();

        println!("{verts:#?}");
        assert_eq!(points, verts);

        let additions = vec![Vec4::splat(1.0), Vec4::splat(2.0)];
        let (start_index, len) = array.overwrite(2, additions.clone()).unwrap();
        assert_eq!((2, 2), (start_index, len));

        array.update(&queue);
        let verts = array.read(&device, &queue, 0, 4).unwrap();
        let all_points = points[0..2]
            .into_iter()
            .copied()
            .chain(additions)
            .collect::<Vec<_>>();
        assert_eq!(all_points, verts);

        let (start, len) = array.extend(vec![Vec4::Y, Vec4::Z]).unwrap();
        assert_eq!((4, 2), (start, len));
    }

    #[test]
    fn gpu_scene_sanity1() {
        _init_logging();
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut builder = r.new_scene();

        let verts = vec![
            GpuVertex {
                position: Vec4::new(0.0, 0.0, 0.0, 1.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 100.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 0.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];

        let ent = builder.new_entity().with_meshlet(verts.clone()).build();

        let mut scene = builder.build().unwrap();

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        scene.set_camera(projection, view);

        scene::setup_scene_render_graph(scene, &mut r, true);

        r.graph.visit(scene::scene_update).unwrap().unwrap();
        r.graph.visit(scene::scene_cull).unwrap().unwrap();

        let (constants, gpu_verts, ents, indirect) = r
            .graph
            .visit(
                |(scene, device, queue): (Read<Scene>, Read<Device>, Read<Queue>)| {
                    let constants = crate::read_buffer::<GpuConstants>(
                        &device,
                        &queue,
                        &scene.constants_buffer,
                        0,
                        1,
                    )
                    .unwrap();
                    let vertices = scene.vertices.read(&device, &queue, 0, 3).unwrap();
                    let entities = scene
                        .entities
                        .read(&device, &queue, 0, scene.entities.capacity())
                        .unwrap();
                    let indirect = if scene.entities.capacity() > 0 {
                        scene
                            .indirect_draws
                            .read(&device, &queue, 0, scene.entities.capacity())
                            .unwrap()
                    } else {
                        vec![]
                    };
                    (constants[0], vertices, entities, indirect)
                },
            )
            .unwrap();
        assert_eq!(constants.camera_projection, projection);
        assert_eq!(constants.camera_view, view);
        assert_eq!(verts, gpu_verts);
        assert_eq!(vec![ent], ents);
        assert_eq!(
            vec![DrawIndirect {
                vertex_count: 3,
                instance_count: 1,
                base_vertex: 0,
                base_instance: 0
            },],
            indirect
        );

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("gpu_scene_sanity.png", img);
    }

    #[test]
    fn gpu_scene_sanity2() {
        _init_logging();

        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(projection, view);
        // now test the textures functionality
        let img = image::io::Reader::open("../../img/cheetah.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let tex_id = builder.add_image_texture(img);
        assert_eq!(Id::new(0), tex_id);
        let material = builder.new_unlit_material().with_texture0(tex_id).build();

        let verts = vec![
            GpuVertex {
                position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                uv: Vec4::new(0.0, 0.0, 0.0, 0.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 100.0, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                uv: Vec4::new(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                uv: Vec4::new(1.0, 0.0, 1.0, 0.0),
                ..Default::default()
            },
        ];
        let ent = builder
            .new_entity()
            .with_meshlet(verts.clone())
            .with_material(material)
            .with_position(Vec3::new(15.0, 35.0, 0.5))
            .with_scale(Vec3::new(0.5, 0.5, 1.0))
            .build();

        assert_eq!(Id::new(0), ent.id);
        assert_eq!(
            GpuEntity {
                id: Id::new(0),
                mesh_first_vertex: 0,
                mesh_vertex_count: 3,
                material: Id::new(0),
                position: Vec4::new(15.0, 35.0, 0.5, 0.0),
                scale: Vec4::new(0.5, 0.5, 1.0, 0.0),
                ..Default::default()
            },
            ent
        );

        let ent = builder.new_entity().with_meshlet(verts.clone()).build();
        assert_eq!(Id::new(1), ent.id);

        let scene = builder.build().unwrap();
        assert_eq!(2, scene.entities.len());

        let textures = scene.atlas().frames().collect::<Vec<_>>();
        assert_eq!(1, textures.len());
        assert_eq!(0, textures[0].0);
        assert_eq!(UVec2::splat(170), textures[0].1 .1);

        scene::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();

        let scene = r.graph.get_resource::<Scene>().unwrap().unwrap();
        let draws = scene
            .indirect_draws
            .read(r.get_device(), r.get_queue(), 0, 2)
            .unwrap();
        assert_eq!(
            vec![
                DrawIndirect {
                    vertex_count: 3,
                    instance_count: 1,
                    base_vertex: 0,
                    base_instance: 0
                },
                DrawIndirect {
                    vertex_count: 3,
                    instance_count: 1,
                    base_vertex: 3,
                    base_instance: 1
                }
            ],
            draws
        );
        let constants: GpuConstants =
            read_buffer(r.get_device(), r.get_queue(), &scene.constants_buffer, 0, 1).unwrap()[0];
        assert_eq!(UVec2::splat(256), constants.atlas_size);

        let materials = scene
            .materials
            .read(r.get_device(), r.get_queue(), 0, 1)
            .unwrap();
        assert_eq!(
            vec![GpuMaterial {
                texture0: Id::new(0),
                ..Default::default()
            },],
            materials
        );

        crate::img_diff::assert_img_eq("gpu_scene_sanity2.png", img);
    }

    #[test]
    fn atlas_uv_mapping() {
        let mut r = Renderling::headless(32, 32)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let (projection, view) = camera::default_ortho2d(32.0, 32.0);
        let mut builder = r.new_scene().with_camera(projection, view);
        let dirt = image::open("../../img/dirt.jpg").unwrap();
        let dirt = builder.add_image(dirt);
        println!("dirt: {dirt}");
        let sandstone = image::open("../../img/sandstone.png").unwrap();
        let sandstone = builder.add_image(sandstone);
        println!("sandstone: {sandstone}");
        let texels = image::open("../../test_img/atlas_uv_mapping.png")
            .unwrap();
        let texels_index = builder.add_image(texels);
        println!("atlas_uv_mapping: {texels_index}");
        let texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::CLAMP_TO_EDGE,
            mode_t: TextureAddressMode::CLAMP_TO_EDGE,
        });
        let material_id = builder
            .new_unlit_material()
            .with_texture0(texture_id)
            .build();
        let _ = builder
            .new_entity()
            .with_material(material_id)
            .with_meshlet({
                let tl = GpuVertex::default()
                    .with_position(Vec3::ZERO)
                    .with_uv0(Vec2::ZERO);
                let tr = GpuVertex::default()
                    .with_position(Vec3::new(1.0, 0.0, 0.0))
                    .with_uv0(Vec2::new(1.0, 0.0));
                let bl = GpuVertex::default()
                    .with_position(Vec3::new(0.0, 1.0, 0.0))
                    .with_uv0(Vec2::new(0.0, 1.0));
                let br = GpuVertex::default()
                    .with_position(Vec3::new(1.0, 1.0, 0.0))
                    .with_uv0(Vec2::splat(1.0));
                vec![tl, bl, br, tl, br, tr]
            })
            .with_scale([32.0, 32.0, 1.0])
            .build();
        let scene = builder.build().unwrap();
        //let atlas_img = scene.atlas.texture.read(
        //    r.get_device(),
        //    r.get_queue(),
        //    scene.atlas.size.x as usize,
        //    scene.atlas.size.y as usize,
        //    4,
        //    1,
        //);
        //let atlas_img = atlas_img.into_rgba(r.get_device()).unwrap();
        //crate::img_diff::save("atlas.png", atlas_img);
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("atlas_uv_mapping.png", img);
    }

    #[test]
    fn uv_wrapping() {
        let icon_w = 32;
        let icon_h = 41;
        let sheet_w = icon_w * 3;
        let sheet_h = icon_h * 3;
        let w = sheet_w * 3 + 2;
        let h = sheet_h;
        let mut r = Renderling::headless(w, h)
            .unwrap()
            .with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let (projection, view) = camera::default_ortho2d(w as f32, h as f32);
        let mut builder = r.new_scene().with_camera(projection, view);
        let dirt = image::open("../../img/dirt.jpg").unwrap();
        builder.add_image(dirt);
        let sandstone = image::open("../../img/sandstone.png").unwrap();
        builder.add_image(sandstone);
        let texels = image::open("../../img/happy_mac.png").unwrap();
        let texels_index = builder.add_image(texels);
        let clamp_texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::CLAMP_TO_EDGE,
            mode_t: TextureAddressMode::CLAMP_TO_EDGE,
        });
        let repeat_texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::REPEAT,
            mode_t: TextureAddressMode::REPEAT,
        });
        let mirror_texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::MIRRORED_REPEAT,
            mode_t: TextureAddressMode::MIRRORED_REPEAT,
        });

        let clamp_material_id = builder
            .new_unlit_material()
            .with_texture0(clamp_texture_id)
            .build();
        let repeat_material_id = builder
            .new_unlit_material()
            .with_texture0(repeat_texture_id)
            .build();
        let mirror_material_id = builder
            .new_unlit_material()
            .with_texture0(mirror_texture_id)
            .build();

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;

        let (start, count) = builder.add_meshlet({
            let tl = GpuVertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = GpuVertex::default()
                .with_position(Vec3::new(sheet_w, 0.0, 0.0))
                .with_uv0(Vec2::new(3.0, 0.0));
            let bl = GpuVertex::default()
                .with_position(Vec3::new(0.0, sheet_h, 0.0))
                .with_uv0(Vec2::new(0.0, 3.0));
            let br = GpuVertex::default()
                .with_position(Vec3::new(sheet_w, sheet_h, 0.0))
                .with_uv0(Vec2::splat(3.0));
            vec![tl, bl, br, tl, br, tr]
        });

        let _clamp = builder
            .new_entity()
            .with_material(clamp_material_id)
            .with_starting_vertex_and_count(start, count)
            .build();
        let _repeat = builder
            .new_entity()
            .with_material(repeat_material_id)
            .with_starting_vertex_and_count(start, count)
            .with_position([sheet_w as f32 + 1.0, 0.0, 0.0])
            .build();
        let _mirror = builder
            .new_entity()
            .with_material(mirror_material_id)
            .with_starting_vertex_and_count(start, count)
            .with_position([sheet_w as f32 * 2.0 + 2.0, 0.0, 0.0])
            .build();

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("uv_wrapping.png", img);
    }

    #[test]
    fn negative_uv_wrapping() {
        let icon_w = 32;
        let icon_h = 41;
        let sheet_w = icon_w * 3;
        let sheet_h = icon_h * 3;
        let w = sheet_w * 3 + 2;
        let h = sheet_h;
        let mut r = Renderling::headless(w, h)
            .unwrap()
            .with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let (projection, view) = camera::default_ortho2d(w as f32, h as f32);
        let mut builder = r.new_scene().with_camera(projection, view);
        let dirt = image::open("../../img/dirt.jpg").unwrap();
        builder.add_image(dirt);
        let sandstone = image::open("../../img/sandstone.png").unwrap();
        builder.add_image(sandstone);
        let texels = image::open("../../img/happy_mac.png").unwrap();
        let texels_index = builder.add_image(texels);
        let clamp_texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::CLAMP_TO_EDGE,
            mode_t: TextureAddressMode::CLAMP_TO_EDGE,
        });
        let repeat_texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::REPEAT,
            mode_t: TextureAddressMode::REPEAT,
        });
        let mirror_texture_id = builder.add_texture(TextureParams {
            image_index: texels_index,
            mode_s: TextureAddressMode::MIRRORED_REPEAT,
            mode_t: TextureAddressMode::MIRRORED_REPEAT,
        });

        let clamp_material_id = builder
            .new_unlit_material()
            .with_texture0(clamp_texture_id)
            .build();
        let repeat_material_id = builder
            .new_unlit_material()
            .with_texture0(repeat_texture_id)
            .build();
        let mirror_material_id = builder
            .new_unlit_material()
            .with_texture0(mirror_texture_id)
            .build();

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;

        let (start, count) = builder.add_meshlet({
            let tl = GpuVertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = GpuVertex::default()
                .with_position(Vec3::new(sheet_w, 0.0, 0.0))
                .with_uv0(Vec2::new(-3.0, 0.0));
            let bl = GpuVertex::default()
                .with_position(Vec3::new(0.0, sheet_h, 0.0))
                .with_uv0(Vec2::new(0.0, -3.0));
            let br = GpuVertex::default()
                .with_position(Vec3::new(sheet_w, sheet_h, 0.0))
                .with_uv0(Vec2::splat(-3.0));
            vec![tl, bl, br, tl, br, tr]
        });

        let _clamp = builder
            .new_entity()
            .with_material(clamp_material_id)
            .with_starting_vertex_and_count(start, count)
            .build();
        let _repeat = builder
            .new_entity()
            .with_material(repeat_material_id)
            .with_starting_vertex_and_count(start, count)
            .with_position([sheet_w as f32 + 1.0, 0.0, 0.0])
            .build();
        let _mirror = builder
            .new_entity()
            .with_material(mirror_material_id)
            .with_starting_vertex_and_count(start, count)
            .with_position([sheet_w as f32 * 2.0 + 2.0, 0.0, 0.0])
            .build();

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("negative_uv_wrapping.png", img);
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
    /// Ensures that the directional light coloring works.
    fn scene_cube_directional() {
        _init_logging();
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));

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

        let material = builder.new_pbr_material().build();

        let _cube = builder
            .new_entity()
            .with_meshlet(
                crate::math::unit_cube()
                    .into_iter()
                    .map(|(p, n)| GpuVertex {
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

        setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("scene_cube_directional.png", img);
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
        let mut r = Renderling::headless(100, 100).unwrap();
        r.set_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        let mut builder = r.new_scene().with_camera(projection, view);
        let size = 1.0;
        let cyan_tri = builder
            .new_entity()
            .with_meshlet(vec![
                GpuVertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                GpuVertex {
                    position: Vec4::new(size, size, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                GpuVertex {
                    position: Vec4::new(size, 0.0, 0.0, 0.0),
                    color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
            ])
            .with_position(Vec3::new(25.0, 25.0, 0.0))
            .with_scale(Vec3::new(25.0, 25.0, 1.0))
            .build();
        let yellow_tri = builder
            .new_entity()
            .with_meshlet(vec![
                GpuVertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
                GpuVertex {
                    position: Vec4::new(size, size, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
                GpuVertex {
                    position: Vec4::new(size, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
            ])
            .with_position(Vec3::new(25.0, 25.0, 0.1))
            .with_parent(&cyan_tri)
            .build();
        let _red_tri = builder
            .new_entity()
            .with_meshlet(vec![
                GpuVertex {
                    position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
                GpuVertex {
                    position: Vec4::new(size, size, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
                GpuVertex {
                    position: Vec4::new(size, 0.0, 0.0, 0.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
            ])
            .with_position(Vec3::new(25.0, 25.0, 0.1))
            .with_parent(&yellow_tri)
            .build();

        assert_eq!(
            vec![
                GpuEntity {
                    id: Id::new(0),
                    position: Vec4::new(25.0, 25.0, 0.0, 0.0),
                    scale: Vec4::new(25.0, 25.0, 1.0, 0.0),
                    mesh_first_vertex: 0,
                    mesh_vertex_count: 3,
                    ..Default::default()
                },
                GpuEntity {
                    id: Id::new(1),
                    parent: Id::new(0),
                    position: Vec4::new(25.0, 25.0, 0.1, 0.0),
                    scale: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    mesh_first_vertex: 3,
                    mesh_vertex_count: 3,
                    ..Default::default()
                },
                GpuEntity {
                    id: Id::new(2),
                    parent: Id::new(1),
                    position: Vec4::new(25.0, 25.0, 0.1, 0.0),
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
                Vec3::new(50.0, 50.0, 0.1),
                Quat::IDENTITY,
                Vec3::new(25.0, 25.0, 1.0),
            ),
            tfrm
        );

        let entities = builder.entities.clone();
        let scene = builder.build().unwrap();
        setup_scene_render_graph(scene, &mut r, true);

        let gpu_entities = r
            .graph
            .get_resource::<Scene>()
            .unwrap()
            .unwrap()
            .entities
            .read(r.get_device(), r.get_queue(), 0, entities.len())
            .unwrap();
        assert_eq!(entities, gpu_entities);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("scene_parent_sanity.png", img);
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
    // spheres with different metallic roughnesses lit by 4 point lights
    //
    // see https://learnopengl.com/PBR/Lighting
    fn pbr_point_lights_metallic_roughness_spheres() {
        _init_logging();
        let ss = 600;
        let mut r = Renderling::headless(ss, ss)
            .unwrap()
            .with_background_color(Vec3::splat(0.0).extend(1.0));

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

        let to_vertex = |ndx: &usize| -> GpuVertex {
            let p: [f32; 3] = positions[*ndx].0.into();
            let n: [f32; 3] = normals[*ndx].0.into();
            GpuVertex::default().with_position(p).with_normal(n)
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

        let mut builder = r.new_scene().with_camera(projection, view);
        let (start, count) = builder.add_meshlet(sphere_vertices);

        let light_z = 3.0 * radius;
        let light_positions = [
            [0.0, 0.0, light_z],
            [len, 0.0, light_z],
            [0.0, len, light_z],
            [len, len, light_z],
        ];
        for position in light_positions.into_iter() {
            let _light = builder
                .new_point_light()
                .with_position(position)
                .with_color(Vec4::ONE)
                .build();
            let light_material_id = builder
                .new_unlit_material()
                .with_base_color(Vec4::splat(1.0))
                .build();
            let _light_entity = builder
                .new_entity()
                .with_starting_vertex_and_count(start, count)
                .with_position(position)
                .with_scale(Vec3::splat(0.5))
                .with_material(light_material_id)
                .build();
        }

        for i in 0..k {
            let roughness = i as f32 / (k - 1) as f32;
            let x = (diameter + spacing) * i as f32;
            for j in 0..k {
                let metallic = j as f32 / (k - 1) as f32;
                let y = (diameter + spacing) * j as f32;
                let material_id = builder
                    .new_pbr_material()
                    .with_base_color_factor(Vec4::new(1.0, 0.0, 0.0, 1.0))
                    .with_metallic_factor(metallic)
                    .with_roughness_factor(roughness)
                    .build();
                let _entity = builder
                    .new_entity()
                    .with_starting_vertex_and_count(start, count)
                    .with_material(material_id)
                    .with_position([x, y, 0.0])
                    .build();
            }
        }

        let scene = builder.build().unwrap();
        crate::setup_scene_render_graph(scene, &mut r, true);

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("pbr_point_lights_metallic_roughness.png", img);

        let view = camera::look_at([-len, len, len], [half, half, 0.0], Vec3::Y);
        r.graph
            .visit(|mut scene: Write<Scene>| scene.set_camera(projection, view))
            .unwrap();

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("pbr_point_lights_metallic_roughness_side.png", img);
    }

    //#[cfg(feature = "gltf")]
    //#[test]
    // fn gltf_load_scene() {
    //    _init_logging();
    //    let (mut r, cam) = forward_renderling(177, 100);
    //    cam.destroy();

    //    let mut loader = r.new_gltf_loader();
    //    let (document, buffers, images) =
    // gltf::import("../../gltf/cheetah_cone.glb").unwrap();    loader.load(&mut
    // r, &document, &buffers, &images).unwrap();    let cam = loader
    //        .cameras()
    //        .next()
    //        .unwrap()
    //        .variant
    //        .as_camera()
    //        .unwrap()
    //        .clone();
    //    r.graph.add_resource(ForwardRenderCamera(cam.id));

    //    let img = r.render_image().unwrap();
    //    crate::img_diff::assert_img_eq_save(
    //        Save::No,
    //        "gltf_load_scene",
    //        "gltf_load_scene.png",
    //        img,
    //    )
    //    .unwrap();
    //}

    //#[cfg(feature = "gltf")]
    //#[test]
    // fn gltf_box_animated() {
    //    _init_logging();

    //    let (mut r, cam) = forward_renderling_with(
    //        100,
    //        100,
    //        Some(wgpu::PrimitiveState {
    //            topology: wgpu::PrimitiveTopology::TriangleList,
    //            strip_index_format: None,
    //            front_face: wgpu::FrontFace::Ccw,
    //            cull_mode: Some(wgpu::Face::Back),
    //            polygon_mode: wgpu::PolygonMode::Fill,
    //            conservative: false,
    //            unclipped_depth: false,
    //        }),
    //    );
    //    r.set_background_color(Vec4::splat(1.0));
    //    cam.set_view(Mat4::look_at_rh(
    //        Vec3::new(0.0, 2.0, 2.0),
    //        Vec3::ZERO,
    //        Vec3::Y,
    //    ));

    //    let mut loader = r.new_gltf_loader();
    //    let (document, buffers, images) =
    // gltf::import("../../gltf/box_animated.glb").unwrap();    loader.load(&mut
    // r, &document, &buffers, &images).unwrap();

    //    let img = r.render_image().unwrap();
    //    crate::img_diff::assert_img_eq_save(
    //        Save::No,
    //        "gltf_box_animated",
    //        "gltf_box_animated.png",
    //        img,
    //    )
    //    .unwrap();

    //    loader.load_animations(&document, &buffers).unwrap();
    //    assert_eq!(1, loader.animations().count());

    //    let anime = loader.get_animation(0).unwrap();
    //    println!("anime: {:?}", anime);
    //    assert_eq!(3.70833, anime.length_in_seconds());
    //    assert_eq!(2, anime.tweens[0].target_node_index);
    //    assert_eq!(0, anime.tweens[1].target_node_index);
    //}

    // fn big_scene_cube_builder() -> MeshBuilder<UiVertex> {
    //    let vertices = crate::math::unit_points();
    //    let indices: [([u16; 3], [u16; 3], Vec4); 6] = [
    //        ([0, 1, 2], [0, 2, 3], Vec4::Y),     // top
    //        ([0, 3, 4], [4, 3, 5], Vec4::Z),     // front
    //        ([3, 2, 6], [3, 6, 5], Vec4::X),     // right
    //        ([1, 0, 7], [7, 0, 4], Vec4::NEG_X), // left
    //        ([4, 5, 6], [4, 6, 7], Vec4::NEG_Y), // bottom
    //        ([2, 1, 7], [2, 7, 6], Vec4::NEG_Z), // back
    //    ];
    //    MeshBuilder::default()
    //        .with_vertices(indices.flat_map(|ui_vert| GpuVertex {
    //            position: Vec3::from_array(ui_vert.position).extend(0.0),
    //            color: Vec4::from_array(ui_vert.color),
    //            uv: Vec4::ZERO,
    //            norm: Vec4::ZERO,
    //        }))
    //        .with_indices(indices)
    //}

    ////#[cfg(feature = "gltf")]
    ////#[test]
    //// fn gltf_simple_morph_triangle() {
    ////    let (document, buffers, images) =
    //// gltf::import("../../gltf/simple_morph_triangle.gltf").unwrap();
    ////    let mesh = document.meshes().next().unwrap();
    ////    let primitive = mesh.primitives().next().unwrap();
    ////    let reader = primitive.reader(|buffer|
    //// Some(&buffers[buffer.index()]));    let positions: Vec<_> =
    //// reader.read_positions().unwrap().collect();    let morphs: Vec<(_, _,
    //// _)> = reader.read_morph_targets().collect();    println!("positions.
    //// len(): {}", positions.len());    println!("morphs.len(): {}",
    //// morphs.len());    for (i, (ps, ns, ts)) in
    //// morphs.into_iter().enumerate() {        println!("{i}");
    ////        println!("ps: {:?}", ps.map(|vs| vs.collect::<Vec<_>>()));
    ////        println!("ns: {:?}", ns.map(|vs| vs.collect::<Vec<_>>()));
    ////        println!("ts: {:?}", ts.map(|vs| vs.collect::<Vec<_>>()));
    ////    }
    ////    panic!("blah")
    //// }
}
