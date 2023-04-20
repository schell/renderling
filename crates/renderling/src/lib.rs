//! A collection of renderers (callend "renderlings") with a focus on simplicity
//! and ease of use. Backed by WebGPU render pipelines and simple types for
//! marshalling data to the GPU.
//!
//! # WARNING
//! This is very much a work in progress.
//! YMMV.
//! PRs are very welcomed :)
//!
//! # renderlings 🍖
//! Individual renderers are called "renderlings" for maximum cuteness.
//! Renderlings manage their own resources and come in a couple flavors
//! depending on the shader used.
//!
//! ## Features
//! Features are used to enable specific renderlings, by default all renderlings
//! are enabled.
//!
//! * **ui**
//!   - simple simple diffuse material
//!   - colored or textured mesh attributes
//!   - mostly for rendering user interfaces
//! * **forward**
//!   - blinn-phong material
//!   - textured mesh attribute
//!   - maximum 64 point, 32 spot and 8 directional lights
//!
//! ## Raw shaders
//! You can also use the [shaders module](crate::shaders) without renderlings
//! and manage your own resources for maximum flexibility.
#[cfg(feature = "forward")]
mod forward;
#[cfg(feature = "forward")]
pub use forward::*;

#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use ui::*;

pub mod linkage;

mod bank;
mod camera;
mod gltf_support;
mod light;
mod material;
mod mesh;
pub mod node;
mod object;
mod pipeline;
mod renderer;
mod resources;
mod scene;
mod state;
mod texture;
mod transform;

pub use camera::*;
#[cfg(feature = "gltf")]
pub use gltf_support::*;
pub use light::*;
pub use material::*;
pub use mesh::*;
pub use object::*;
pub use pipeline::*;
pub use renderer::*;
pub use resources::*;
pub use scene::*;
pub use state::*;
pub use texture::*;
pub use transform::*;

pub mod math;

pub mod graph {
    //! Re-exports of [`moongraph`].

    pub use moongraph::*;
}

pub use graph::{Move, Read, Write};

#[cfg(test)]
mod img_diff;

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{
        img_diff::Save,
        node::{ForwardRenderCamera, UiRenderCamera},
    };

    use super::*;
    use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
    use moongraph::{DaggaError, Function, GraphError, Node, Read, TypeKey};
    use renderling_shader::scene::{DrawIndirect, GpuEntity, GpuVertex};

    #[test]
    fn init() {
        let r = Renderling::headless(100, 100).unwrap();
        assert_eq!(0, r.get_cameras().iter().count());
    }

    fn right_tri_builder() -> MeshBuilder<UiVertex> {
        MeshBuilder::default().with_vertices(vec![
            UiVertex::default()
                .with_position(0.0, 0.0, 0.5)
                .with_color(0.0, 1.0, 1.0, 1.0),
            UiVertex::default()
                .with_position(100.0, 0.0, 0.5)
                .with_color(1.0, 0.0, 1.0, 1.0),
            UiVertex::default()
                .with_position(0.0, 100.0, 0.5)
                .with_color(1.0, 1.0, 0.0, 1.0),
        ])
    }

    fn _init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            //.filter_level(log::LevelFilter::Trace)
            .filter_module("renderling", log::LevelFilter::Trace)
            .filter_module("naga", log::LevelFilter::Warn)
            .filter_module("wgpu", log::LevelFilter::Debug)
            .filter_module("wgpu_hal", log::LevelFilter::Warn)
            .try_init();
    }

    fn ui_renderling() -> (Renderling, Camera) {
        // set up our rendering graph
        let mut ui = Renderling::headless(100, 100)
            .unwrap()
            .with_default_ui_render_graph()
            .with_node(
                crate::node::PostRenderBufferCreate::create
                    .into_node()
                    .with_name("copy_frame_to_post")
                    .run_after("ui_render")
                    .run_before("present_frame"),
            );
        let cam = ui.new_camera().with_projection_ortho2d().build();
        ui.add_resource(UiRenderCamera(cam.id));
        (ui, cam)
    }

    struct CmyTri {
        ui: Renderling,
        _cam: Camera,
        tri: Object,
    }

    fn cmy_triangle_setup() -> CmyTri {
        let (mut ui, cam) = ui_renderling();
        let tri = ui
            .new_object()
            .with_mesh_builder(right_tri_builder())
            .build()
            .unwrap();
        CmyTri { _cam: cam, ui, tri }
    }

    #[test]
    fn cmy_triangle_sanity() {
        _init_logging();
        let mut c = cmy_triangle_setup();
        let img = c.ui.render_image().unwrap();
        c.ui.graph.save_graph_dot("cmy_triangle_renderer.dot");
        crate::img_diff::assert_img_eq("cmy_triangle", "cmy_triangle.png", img).unwrap();
    }

    #[test]
    fn cmy_triangle_update_transform() {
        _init_logging();
        let mut c = cmy_triangle_setup();
        let _ = c.ui.render_image().unwrap();
        c.tri.set_transform(
            Transform::default()
                .with_position(Vec3::new(100.0, 0.0, 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
        );
        let img = c.ui.render_image().unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_triangle_update_transform",
            "cmy_triangle_update_transform.png",
            img,
        )
        .unwrap();
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
            // bottom
            tl, bl, br, tl, br, tr, // front
            br, bl, top, // left
            bl, tl, top, // back
            tl, tr, top, // right
            tr, br, top,
        ]
    }

    fn cmy_vertex(p: Vec3) -> UiVertex {
        let r: f32 = p.z + 0.5;
        let g: f32 = p.x + 0.5;
        let b: f32 = p.y + 0.5;
        UiVertex::default()
            .with_position(p.x.min(1.0), p.y.min(1.0), p.z.min(1.0))
            .with_color(r, g, b, 1.0)
    }

    fn cube_builder() -> MeshBuilder<UiVertex> {
        let vertices = crate::math::unit_points();
        let indices: [u16; 12 * 3] = [
            0, 1, 2, 0, 2, 3, // top
            0, 3, 4, 4, 3, 5, // front
            3, 2, 6, 3, 6, 5, // right
            1, 0, 7, 7, 0, 4, // left
            4, 5, 6, 4, 6, 7, // bottom
            2, 1, 7, 2, 7, 6, // back
        ];
        MeshBuilder::default()
            .with_vertices(vertices.map(cmy_vertex))
            .with_indices(indices)
    }

    fn pyramid_builder() -> MeshBuilder<UiVertex> {
        let vertices = pyramid_points();
        let indices = pyramid_indices();
        MeshBuilder::default()
            .with_vertices(vertices.map(cmy_vertex))
            .with_indices(indices)
    }

    #[test]
    fn cmy_cube_sanity() {
        _init_logging();
        let (mut ui, cam) = ui_renderling();
        assert_eq!(1, ui.cameras().count());

        cam.look_at(Vec3::new(0.0, 12.0, 20.0), Vec3::ZERO, Vec3::Y);
        cam.set_projection(Mat4::perspective_rh(
            std::f32::consts::PI / 4.0,
            1.0,
            0.1,
            100.0,
        ));

        let _cube = ui
            .new_object()
            .with_mesh_builder(cube_builder())
            .with_scale(Vec3::new(6.0, 6.0, 6.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4))
            .build()
            .unwrap();

        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube", "cmy_cube.png", img).unwrap();
    }

    #[test]
    fn cmy_cube_visible() {
        _init_logging();
        let (mut ui, cam) = ui_renderling();

        let (projection, view) = camera::default_perspective(100.0, 100.0);
        cam.set_projection(projection);
        cam.set_view(view);

        let _cube_one = ui
            .new_object()
            .with_mesh_builder(cube_builder())
            .with_position(Vec3::new(-4.0, 0.0, 0.0))
            .with_scale(Vec3::new(6.0, 6.0, 6.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4))
            .build()
            .unwrap();

        let cube_two = ui
            .new_object()
            .with_mesh_builder(cube_builder())
            .with_position(Vec3::new(4.0, 0.0, 0.0))
            .with_scale(Vec3::new(6.0, 6.0, 6.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4))
            .build()
            .unwrap();

        // we should see two colored cubes
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "cmy_cube_visible_before",
            "cmy_cube_visible_before.png",
            img,
        )
        .unwrap();

        // we should see one colored cube
        cube_two.set_visible(false);
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "cmy_cube_visible_after",
            "cmy_cube_visible_after.png",
            img,
        )
        .unwrap();

        // we should see two colored cubes again
        cube_two.set_visible(true);
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_before_again",
            "cmy_cube_visible_before.png",
            img,
        )
        .unwrap();
    }

    #[test]
    fn cmy_cube_remesh() {
        _init_logging();
        let (mut ui, cam) = ui_renderling();
        // transparent background
        ui.set_background_color(Vec4::splat(0.0));
        let (projection, view) = camera::default_perspective(100.0, 100.0);
        cam.set_projection(projection);
        cam.set_view(view);
        let cube = ui
            .new_object()
            .with_mesh_builder(cube_builder())
            .with_scale(Vec3::new(10.0, 10.0, 10.0))
            .build()
            .unwrap();
        // we should see a cube
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "cmy_cube_remesh_before",
            "cmy_cube_remesh_before.png",
            img,
        )
        .unwrap();

        // we should see a pyramid
        let pyramid_mesh = pyramid_builder().build(Some("pyramid mesh"), &ui.get_device());
        cube.set_mesh(Arc::new(pyramid_mesh));
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "cmy_cube_remesh_after",
            "cmy_cube_remesh_after.png",
            img,
        )
        .unwrap();
    }

    fn uv_unit_cube() -> MeshBuilder<UiVertex> {
        MeshBuilder::default().with_vertices({
            let p: [Vec3; 8] = crate::math::unit_points();
            let tl = Vec2::from([0.0, 0.0]);
            let tr = Vec2::from([1.0, 0.0]);
            let bl = Vec2::from([0.0, 1.0]);
            let br = Vec2::from([1.0, 1.0]);

            vec![
                // top
                UiVertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[1].x, p[1].y, p[1].z)
                    .with_uv(tl.x, tl.y),
                UiVertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(br.x, br.y),
                // bottom
                UiVertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(tl.x, tl.y),
                UiVertex::default()
                    .with_position(p[6].x, p[6].y, p[6].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[6].x, p[6].y, p[6].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[7].x, p[7].y, p[7].z)
                    .with_uv(br.x, br.y),
                // left
                UiVertex::default()
                    .with_position(p[7].x, p[7].y, p[7].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[1].x, p[1].y, p[1].z)
                    .with_uv(tl.x, tl.y),
                UiVertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[7].x, p[7].y, p[7].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(br.x, br.y),
                // right
                UiVertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(tl.x, tl.y),
                UiVertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[6].x, p[6].y, p[6].z)
                    .with_uv(br.x, br.y),
                // front
                UiVertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(tl.x, tl.y),
                UiVertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                UiVertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(tr.x, tr.y),
                UiVertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(br.x, br.y),
            ]
        })
    }

    #[test]
    fn cmy_cube_material() {
        let (mut ui, cam) = ui_renderling();
        ui.set_background_color(Vec4::splat(0.0));
        let (proj, view) = camera::default_perspective(100.0, 100.0);
        cam.set_projection(proj);
        cam.set_view(view);

        let png = image::open("../../img/sandstone.png").unwrap();
        let tex = ui
            .create_texture(Some("sandstone_material"), &png.to_rgba8())
            .unwrap();
        let material = UiMaterial {
            diffuse_texture: tex,
            color_blend: UiColorBlend::UvOnly,
        };
        let builder = uv_unit_cube();
        let cube = ui
            .new_object()
            .with_material(material)
            .with_mesh_builder(builder)
            .with_scale(Vec3::new(10.0, 10.0, 10.0))
            .build()
            .unwrap();
        // we should see a cube with a stoney texture
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "cmy_cube_material_before",
            "cmy_cube_material_before.png",
            img,
        )
        .unwrap();

        let png = image::open("../../img/dirt.jpg").unwrap();
        let tex = ui
            .create_texture(Some("dirt_material"), &png.to_rgba8())
            .unwrap();
        let material = UiMaterial {
            diffuse_texture: tex,
            color_blend: UiColorBlend::UvOnly,
        };
        cube.set_material(material);
        // we should see a cube with a dirty texture
        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "cmy_cube_material_after",
            "cmy_cube_material_after.png",
            img,
        )
        .unwrap();
    }

    fn forward_renderling_with(
        width: u32,
        height: u32,
        prim: Option<wgpu::PrimitiveState>,
    ) -> (Renderling, Camera) {
        // set up our rendering graph
        let mut r = Renderling::headless(width, height)
            .unwrap()
            .with_forward_render_graph(if let Some(prim) = prim {
                Box::new(ForwardPipelineCreator::create_with_prim(prim))
                    as Box<dyn FnOnce(ForwardPipelineCreator) -> ForwardPipeline>
            } else {
                Box::new(ForwardPipelineCreator::create)
                    as Box<dyn FnOnce(ForwardPipelineCreator) -> ForwardPipeline>
            })
            .with_background_color(Vec4::splat(0.0))
            .with_node(
                crate::node::PostRenderBufferCreate::create
                    .into_node()
                    .with_name("copy_frame_to_post")
                    .run_after("forward_render")
                    .run_before("present_frame"),
            );
        let cam = r.new_camera().with_projection_perspective().build();
        r.add_resource(ForwardRenderCamera(cam.id));
        (r, cam)
    }

    fn forward_renderling(width: u32, height: u32) -> (Renderling, Camera) {
        forward_renderling_with(width, height, None)
    }

    #[test]
    /// Ensures that the directional light coloring works.
    fn forward_cube_directional() {
        _init_logging();
        let (mut r, cam) = forward_renderling(100, 100);
        r.set_background_color(Vec3::splat(0.0).extend(1.0));

        let (proj, _) = camera::default_perspective(100.0, 100.0);
        cam.set_projection(proj);
        cam.look_at(
            Vec3::new(1.8, 1.8, 1.8),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );

        let white = Vec4::splat(1.0);
        let red = Vec3::X.extend(1.0);
        let green = Vec3::Y.extend(1.0);
        let blue = Vec3::Z.extend(1.0);
        let transparent = Vec4::ZERO;
        let _dir_red = r
            .new_directional_light()
            .with_direction(Vec3::NEG_Y)
            .with_diffuse_color(red)
            .with_specular_color(red)
            .with_ambient_color(transparent)
            .build();
        let _dir_green = r
            .new_directional_light()
            .with_direction(Vec3::NEG_X)
            .with_diffuse_color(green)
            .with_specular_color(green)
            .with_ambient_color(transparent)
            .build();
        let _dir_blue = r
            .new_directional_light()
            .with_direction(Vec3::NEG_Z)
            .with_diffuse_color(blue)
            .with_specular_color(blue)
            .with_ambient_color(transparent)
            .build();

        let material = BlinnPhongMaterial::from_colors(&r, white, white, 16.0);
        let _cube = r
            .new_object()
            .with_material(material)
            .with_mesh_builder(MeshBuilder::default().with_vertices(
                crate::math::unit_cube().into_iter().map(|(p, n)| {
                    ForwardVertex::default()
                        .with_position(p.x, p.y, p.z)
                        .with_normal(n.x, n.y, n.z)
                }),
            ))
            .with_generate_normal_matrix(true)
            .build()
            .unwrap();

        let res = r.render_image();
        let img = match res {
            Ok(img) => img,
            Err(RenderlingError::Graph {
                source: GraphError::Scheduling { source },
            }) => match &source {
                DaggaError::CannotSolve { constraint } => {
                    // println!("{}", source);
                    fn print_node(name: &str, node: &Node<Function, TypeKey>) {
                        println!("{name}: {}", node.name());
                        println!("  barrier: {}", node.get_barrier());
                        println!(
                            "  runs_after: {:?}",
                            node.get_runs_after().collect::<Vec<_>>()
                        );
                        println!(
                            "  runs_before: {:?}",
                            node.get_runs_before().collect::<Vec<_>>()
                        );
                        println!(
                            "  reads: {:?}",
                            node.get_reads().map(|k| k.name()).collect::<Vec<_>>()
                        );
                        println!(
                            "  writes: {:?}",
                            node.get_writes().map(|k| k.name()).collect::<Vec<_>>()
                        );
                        println!(
                            "  moves: {:?}",
                            node.get_moves().map(|k| k.name()).collect::<Vec<_>>()
                        );
                        println!(
                            "  results: {:?}",
                            node.get_results().map(|k| k.name()).collect::<Vec<_>>()
                        );
                    }
                    assert!(r.graph.nodes().count() > 0);
                    println!("{source}");
                    let lhs = r.graph.get_node(&constraint.lhs).unwrap();
                    print_node("lhs", lhs);
                    let rhs = r.graph.get_node(&constraint.rhs).unwrap();
                    print_node("rhs", rhs);
                    panic!("bah!");
                }
                other => panic!("{}", other),
            },
            Err(other) => panic!("{}", other),
        };
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "forward_cube_directional",
            "forward_cube_directional.png",
            img,
        )
        .unwrap();
    }

    #[test]
    fn ui_text() {
        use crate::ui;

        let (mut r, cam) = ui_renderling();
        r.set_background_color(Vec4::splat(0.0));
        r.resize(100, 50);
        // after resizing we also have to adjust the camera
        let (proj, view) = camera::default_ortho2d(100.0, 50.0);
        cam.set_projection(proj);
        cam.set_view(view);

        let bytes: Vec<u8> =
            std::fs::read("../../fonts/Font Awesome 6 Free-Regular-400.otf").unwrap();

        let font = ui::FontArc::try_from_vec(bytes).unwrap();
        let mut glyph_cache = GlyphCache::new(&r, vec![font]);
        glyph_cache.queue(
            ui::Section::default()
                .add_text(
                    ui::Text::new("")
                        .with_color([1.0, 1.0, 0.0, 1.0])
                        .with_scale(32.0),
                )
                .add_text(
                    ui::Text::new("")
                        .with_color([1.0, 0.0, 1.0, 1.0])
                        .with_scale(32.0),
                )
                .add_text(
                    ui::Text::new("")
                        .with_color([0.0, 1.0, 1.0, 1.0])
                        .with_scale(32.0),
                ),
        );
        let (material, mesh) = glyph_cache.get_updated();
        let material = Arc::new(material.unwrap());
        let mesh = Arc::new(mesh.unwrap());
        let _obj_a = r
            .new_object()
            .with_material::<UiMaterial>(material.clone())
            .with_mesh(mesh.clone())
            .build()
            .unwrap();
        let _obj_b = r
            .new_object()
            .with_material::<UiMaterial>(material.clone())
            .with_mesh(mesh.clone())
            .with_position(Vec3::new(15.0, 15.0, 0.5))
            .build()
            .unwrap();
        // we should see three different colored check icons
        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(Save::No, "ui_text", "ui_text.png", img).unwrap();
    }

    #[test]
    // tests that nested children are transformed by their parent's transform
    fn parent_sanity() {
        let (mut ui, _cam) = ui_renderling();
        ui.set_background_color(Vec4::splat(0.0));
        let size = 1.0;
        let yellow_tri = ui
            .new_object()
            .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                            UiVertex::default()
                                .with_position(0.0, 0.0, 0.0)
                                .with_color(1.0, 1.0, 0.0, 1.0),
                            UiVertex::default()
                                .with_position(size, 0.0, 0.0)
                                .with_color(1.0, 1.0, 0.0, 1.0),
                            UiVertex::default()
                                .with_position(size, size, 0.0)
                                .with_color(1.0, 1.0, 0.0, 1.0),
                        ]))
            .with_position(Vec3::new(25.0, 25.0, 0.0))
            .build()
            .unwrap();
        let _cyan_tri = ui
            .new_object()
            .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                            UiVertex::default()
                                .with_position(0.0, 0.0, 0.0)
                                .with_color(0.0, 1.0, 1.0, 1.0),
                            UiVertex::default()
                                .with_position(size, 0.0, 0.0)
                                .with_color(0.0, 1.0, 1.0, 1.0),
                            UiVertex::default()
                                .with_position(size, size, 0.0)
                                .with_color(0.0, 1.0, 1.0, 1.0),
                        ]))
            .with_position(Vec3::new(25.0, 25.0, 0.0))
            .with_scale(Vec3::new(25.0, 25.0, 1.0))
            .with_child(&yellow_tri)
            .build()
            .unwrap();

        assert_eq!(
            WorldTransform::default()
                .with_position(Vec3::new(50.0, 50.0, 0.0))
                .with_scale(Vec3::new(25.0, 25.0, 1.0)),
            yellow_tri.get_world_transform()
        );

        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(Save::No, "parent_sanity", "parent_sanity.png", img)
            .unwrap();
    }

    #[cfg(feature = "gltf")]
    #[test]
    // tests importing a gltf file and rendering the first image as a 2d object
    fn gltf_images() {
        let (mut ui, _cam) = ui_renderling();
        let mut loader = ui.new_gltf_loader();
        let (document, _buffers, images) = gltf::import("../../gltf/cheetah_cone.glb").unwrap();
        loader.load_materials(&document, &images).unwrap();
        let _img = ui
            .new_object()
            .with_mesh_builder(
                MeshBuilder::default()
                    .with_vertices(vec![
                        UiVertex::default()
                            .with_position(0.0, 0.0, 0.0)
                            .with_uv(0.0, 0.0),
                        UiVertex::default()
                            .with_position(1.0, 0.0, 0.0)
                            .with_uv(1.0, 0.0),
                        UiVertex::default()
                            .with_position(1.0, 1.0, 0.0)
                            .with_uv(1.0, 1.0),
                        UiVertex::default()
                            .with_position(0.0, 1.0, 0.0)
                            .with_uv(0.0, 1.0),
                    ])
                    .with_indices(vec![0, 1, 2, 0, 2, 3]),
            )
            .with_material(UiMaterial {
                diffuse_texture: loader.textures().next().unwrap().clone(),
                color_blend: UiColorBlend::UvOnly,
            })
            .with_scale(Vec3::new(100.0, 100.0, 1.0))
            .build()
            .unwrap();

        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(Save::No, "gltf_images", "gltf_images.png", img)
            .unwrap();
    }

    #[cfg(feature = "gltf")]
    #[test]
    fn gltf_load_scene() {
        _init_logging();
        let (mut r, cam) = forward_renderling(177, 100);
        cam.destroy();

        let mut loader = r.new_gltf_loader();
        let (document, buffers, images) = gltf::import("../../gltf/cheetah_cone.glb").unwrap();
        loader.load(&mut r, &document, &buffers, &images).unwrap();
        let cam = loader
            .cameras()
            .next()
            .unwrap()
            .variant
            .as_camera()
            .unwrap()
            .clone();
        r.graph.add_resource(ForwardRenderCamera(cam.id));

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "gltf_load_scene",
            "gltf_load_scene.png",
            img,
        )
        .unwrap();
    }

    #[cfg(feature = "gltf")]
    #[test]
    fn gltf_simple_animation() {
        use moongraph::Read;

        use crate::node::FrameTextureView;

        _init_logging();
        let (mut r, cam) = forward_renderling(50, 50);
        r.set_background_color(Vec4::splat(1.0));

        let (proj, view) = camera::default_ortho2d(50.0, 50.0);
        cam.set_projection(proj);
        cam.set_view(view);
        // render once to get the right background color
        r.render().unwrap();

        // after this point we don't want to clear the frame before every rendering
        // because we're going to composite different frames of an animation into one,
        // so we'll replace the clear_frame_and_depth node with our own node
        // that only clears the depth.
        let clear_frame_and_depth_node = r.graph.remove_node("clear_frame_and_depth").unwrap();
        pub fn clear_only_depth(
            (device, queue, _frame_view, depth, color): (
                Read<Device>,
                Read<Queue>,
                Read<FrameTextureView>,
                Read<DepthTexture>,
                Read<BackgroundColor>,
            ),
        ) -> Result<(), WgpuStateError> {
            let depth_view = &depth.view;
            let [r, g, b, a] = color.0.to_array();
            let color = wgpu::Color {
                r: r.into(),
                g: g.into(),
                b: b.into(),
                a: a.into(),
            };
            crate::linkage::conduct_clear_pass(
                &device,
                &queue,
                Some("clear_only_depth"),
                None,
                Some(&depth_view),
                color,
            );
            Ok(())
        }
        let clear_only_depth_node = clear_only_depth
            .into_node()
            .with_name("clear_only_depth_node")
            .runs_after_barrier(clear_frame_and_depth_node.get_barrier());
        r.graph.add_node(clear_only_depth_node);

        let mut loader = r.new_gltf_loader();
        let (document, buffers, images) =
            gltf::import("../../gltf/animated_triangle.gltf").unwrap();
        loader.load(&mut r, &document, &buffers, &images).unwrap();

        let tri_node = loader.get_node(0).unwrap();
        let tri = tri_node.variant.as_object().unwrap();
        tri.set_scale(Vec3::splat(25.0));
        tri.set_position(Vec3::new(25.0, 25.0, 0.0));

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "gltf_simple_animation",
            "gltf_simple_animation.png",
            img,
        )
        .unwrap();

        loader.load_animations(&document, &buffers).unwrap();

        assert_eq!(1, loader.animations().count());

        let anime = loader.get_animation(0).unwrap();
        println!("anime: {:?}", anime);
        assert_eq!(1.0, anime.tweens[0].length_in_seconds());

        let num = 8;
        for i in 0..num {
            let t = i as f32 / num as f32;
            for tween in anime.tweens.iter() {
                let property = tween.interpolate(t).unwrap().unwrap();
                let node = loader.get_node(tween.target_node_index).unwrap();
                node.set_tween_property(property);
            }
            r.render().unwrap();
        }

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "gltf_simple_animation_after",
            "gltf_simple_animation_after.png",
            img,
        )
        .unwrap();
    }

    #[cfg(feature = "gltf")]
    #[test]
    fn gltf_box_animated() {
        _init_logging();

        let (mut r, cam) = forward_renderling_with(
            100,
            100,
            Some(wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                unclipped_depth: false,
            }),
        );
        r.set_background_color(Vec4::splat(1.0));
        cam.set_view(Mat4::look_at_rh(
            Vec3::new(0.0, 2.0, 2.0),
            Vec3::ZERO,
            Vec3::Y,
        ));

        let mut loader = r.new_gltf_loader();
        let (document, buffers, images) = gltf::import("../../gltf/box_animated.glb").unwrap();
        loader.load(&mut r, &document, &buffers, &images).unwrap();

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "gltf_box_animated",
            "gltf_box_animated.png",
            img,
        )
        .unwrap();

        loader.load_animations(&document, &buffers).unwrap();
        assert_eq!(1, loader.animations().count());

        let anime = loader.get_animation(0).unwrap();
        println!("anime: {:?}", anime);
        assert_eq!(3.70833, anime.length_in_seconds());
        assert_eq!(2, anime.tweens[0].target_node_index);
        assert_eq!(0, anime.tweens[1].target_node_index);
    }

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
                position: Vec4::new(100.0, 0.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 100.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
        ];

        let ent = builder
            .new_entity()
            .with_meshlet(verts.clone())
            .with_transform(Mat4::IDENTITY)
            .build();

        let mut scene = builder.build();

        let (projection, view) = camera::default_ortho2d(100.0, 100.0);
        scene.set_camera(projection, view);

        scene::setup_scene_render_graph(scene, &mut r);

        r.graph.visit(scene::scene_update).unwrap().unwrap();
        r.graph.visit(scene::scene_cull).unwrap().unwrap();

        let (constants, gpu_verts, ents, transforms, indirect) = r
            .graph
            .visit(
                |(scene, device, queue): (Read<Scene>, Read<Device>, Read<Queue>)| {
                    let constants =
                        scene::read_buffer::<GpuConstants>(&device, &queue, &scene.constants, 0, 1)
                            .unwrap();
                    let vertices = scene.vertices.read(&device, &queue, 0, 3).unwrap();
                    let entities = scene
                        .entities
                        .read(&device, &queue, 0, scene.entities.capacity())
                        .unwrap();
                    let transforms = scene.transforms.read(&device, &queue, 0, 1).unwrap();
                    let indirect = if scene.entities.capacity() > 0 {
                        scene
                            .indirect_draws
                            .read(&device, &queue, 0, scene.entities.capacity())
                            .unwrap()
                    } else {
                        vec![]
                    };
                    (constants[0], vertices, entities, transforms, indirect)
                },
            )
            .unwrap();
        assert_eq!(constants.camera_projection, projection);
        assert_eq!(constants.camera_view, view);
        assert_eq!(verts, gpu_verts);
        assert_eq!(vec![ent], ents);
        assert_eq!(vec![Mat4::IDENTITY], transforms);
        assert_eq!(
            vec![DrawIndirect {
                vertex_count: 3,
                instance_count: 1,
                base_vertex: 0,
                base_instance: 0
            },],
            indirect
        );

        r.graph.add_node(
            crate::node::PostRenderBufferCreate::create
                .into_node()
                .with_name("copy_frame_to_post")
                .run_after("scene_render")
                .run_before("present"),
        );

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(
            Save::No,
            "gpu_scene_sanity",
            "gpu_scene_sanity.png",
            img,
        )
        .unwrap();
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
        let img = img.to_rgba8();
        let tex_id = builder.add_image(img);
        assert_eq!(1, tex_id);

        let verts = vec![
            GpuVertex {
                position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                uv: Vec4::new(0.0, 0.0, 0.0, 0.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 0.0, 1.0, 1.0),
                uv: Vec4::new(1.0, 0.0, 1.0, 0.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(100.0, 100.0, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                uv: Vec4::new(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let ent = builder
            .new_entity()
            .with_meshlet(verts.clone())
            .with_texture_ids(Some(tex_id), None)
            .with_transform(
                Mat4::from_translation(Vec3::new(15.0, 35.0, 0.5))
                    * Mat4::from_scale(Vec3::new(0.5, 0.5, 1.0)),
            )
            .build();

        assert_eq!(0, ent.id);
        assert_eq!(
            GpuEntity {
                id: 0,
                mesh_first_vertex: 0,
                mesh_vertex_count: 3,
                model_matrix: 0,
                normal_matrix: 1,
                texture0: 1,
                texture1: 0,
                lighting: LightingModel::NO_LIGHTING,
                parent: u32::MAX,
            },
            ent
        );

        let ent = builder.new_entity().with_meshlet(verts.clone()).build();
        assert_eq!(1, ent.id);

        let scene = builder.build();
        assert_eq!(2, scene.entities.len());

        let rects = scene.atlas().images();
        assert_eq!(2, rects.len());
        assert_eq!(0, rects[0].0);
        assert_eq!(1, rects[0].1.w);
        assert_eq!(1, rects[0].1.h);

        scene::setup_scene_render_graph(scene, &mut r);
        r.graph.add_node(
            crate::node::PostRenderBufferCreate::create
                .into_node()
                .with_name("copy_frame_to_post")
                .run_after("scene_render")
                .run_before("present"),
        );

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
            read_buffer(r.get_device(), r.get_queue(), &scene.constants, 0, 1).unwrap()[0];
        assert_eq!(Vec2::splat(256.0), constants.atlas_size);

        let textures = scene
            .textures
            .read(r.get_device(), r.get_queue(), 0, 2)
            .unwrap();
        assert_eq!(
            vec![
                GpuTexture {
                    offset_px: Vec2::new(170.0, 0.0,),
                    size_px: Vec2::new(1.0, 1.0,),
                },
                GpuTexture {
                    offset_px: Vec2::new(0.0, 0.0,),
                    size_px: Vec2::new(170.0, 170.0,),
                },
            ],
            textures
        );

        crate::img_diff::assert_img_eq("gpu_scene_sanity2", "gpu_scene_sanity2.png", img).unwrap();
    }

    #[test]
    fn transform_uvs_for_atlas() {
        let rect = crunch::Rect {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        };
        assert_eq!(
            Vec2::ZERO,
            Atlas::transform_uvs(Vec2::ZERO, rect, Vec2::splat(100.0))
        );
        assert_eq!(
            Vec2::ZERO,
            Atlas::transform_uvs(Vec2::ZERO, rect, Vec2::splat(1.0))
        );
        assert_eq!(
            Vec2::ZERO,
            Atlas::transform_uvs(Vec2::ZERO, rect, Vec2::splat(256.0))
        );
        let rect = crunch::Rect {
            x: 10,
            y: 10,
            w: 1,
            h: 1,
        };
        assert_eq!(
            Vec2::splat(0.1),
            Atlas::transform_uvs(Vec2::ZERO, rect, Vec2::splat(100.0))
        );
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
        let transparent = Vec4::ZERO;
        let _dir_red = builder
            .new_directional_light()
            .with_direction(Vec3::NEG_Y)
            .with_diffuse_color(red)
            .with_specular_color(red)
            .with_ambient_color(transparent)
            .build();
        let _dir_green = builder
            .new_directional_light()
            .with_direction(Vec3::NEG_X)
            .with_diffuse_color(green)
            .with_specular_color(green)
            .with_ambient_color(transparent)
            .build();
        let _dir_blue = builder
            .new_directional_light()
            .with_direction(Vec3::NEG_Z)
            .with_diffuse_color(blue)
            .with_specular_color(blue)
            .with_ambient_color(transparent)
            .build();

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
            .with_lighting_model(LightingModel::PHONG_LIGHTING)
            .build();

        let mut scene = builder.build();

        let (projection, _) = camera::default_perspective(100.0, 100.0);
        let view = Mat4::look_at_rh(
            Vec3::new(1.8, 1.8, 1.8),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );
        scene.set_camera(projection, view);

        setup_scene_render_graph(scene, &mut r);
        r.graph.add_node(
            crate::node::PostRenderBufferCreate::create
                .into_node()
                .with_name("copy_frame_to_post")
                .run_after("scene_render")
                .run_before("present"),
        );

        let img = r.render_image().unwrap();
        crate::img_diff::assert_img_eq("scene_cube_directional", "scene_cube_directional.png", img)
            .unwrap();
    }

    #[test]
    // tests that nested children are transformed by their parent's transform
    fn scene_parent_sanity() {
        let (mut ui, _cam) = ui_renderling();
        ui.set_background_color(Vec4::splat(0.0));
        let size = 1.0;
        let yellow_tri = ui
            .new_object()
            .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                            UiVertex::default()
                                .with_position(0.0, 0.0, 0.0)
                                .with_color(1.0, 1.0, 0.0, 1.0),
                            UiVertex::default()
                                .with_position(size, 0.0, 0.0)
                                .with_color(1.0, 1.0, 0.0, 1.0),
                            UiVertex::default()
                                .with_position(size, size, 0.0)
                                .with_color(1.0, 1.0, 0.0, 1.0),
                        ]))
            .with_position(Vec3::new(25.0, 25.0, 0.0))
            .build()
            .unwrap();
        let _cyan_tri = ui
            .new_object()
            .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                            UiVertex::default()
                                .with_position(0.0, 0.0, 0.0)
                                .with_color(0.0, 1.0, 1.0, 1.0),
                            UiVertex::default()
                                .with_position(size, 0.0, 0.0)
                                .with_color(0.0, 1.0, 1.0, 1.0),
                            UiVertex::default()
                                .with_position(size, size, 0.0)
                                .with_color(0.0, 1.0, 1.0, 1.0),
                        ]))
            .with_position(Vec3::new(25.0, 25.0, 0.0))
            .with_scale(Vec3::new(25.0, 25.0, 1.0))
            .with_child(&yellow_tri)
            .build()
            .unwrap();

        assert_eq!(
            WorldTransform::default()
                .with_position(Vec3::new(50.0, 50.0, 0.0))
                .with_scale(Vec3::new(25.0, 25.0, 1.0)),
            yellow_tri.get_world_transform()
        );

        let img = ui.render_image().unwrap();
        crate::img_diff::assert_img_eq_save(Save::No, "parent_sanity", "parent_sanity.png", img)
            .unwrap();
    }

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
