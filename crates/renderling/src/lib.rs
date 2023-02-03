//! A collection of renderers (callend "renderlings") with a focus on simplicity and ease of use.
//! Backed by WebGPU render pipelines and simple types for marshalling data to the GPU.
//!
//! # WARNING
//! This is very much a work in progress.
//! YMMV.
//! PRs are very welcomed :)
//!
//! # renderlings 🍖
//! Individual renderers are called "renderlings" for maximum cuteness.
//! Renderlings manage their own resources and come in a couple flavors depending on the shader used.
//!
//! ## Features
//! Features are used to enable specific renderlings, by default all renderlings are enabled.
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
//! You can also use the [shaders module](crate::shaders) without renderlings and manage your own resources for maximum flexibility.
pub use renderling_core::*;

#[cfg(feature = "forward")]
mod forward;
#[cfg(feature = "forward")]
pub use forward::*;

#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use ui::*;

pub mod shaders {
    //! Raw `wgpu` shaders.

    #[cfg(feature = "forward")]
    pub mod forward {
        //! Forward shader `wgpu` types and operations.
        pub use renderling_forward::*;
    }
    #[cfg(feature = "ui")]
    pub mod ui {
        //! User interface shader `wgpu` types and operations.
        pub use renderling_ui::*;
    }
}

mod camera;
mod light;
mod material;
mod mesh;
mod object;
mod pipeline;
mod renderer;
mod resources;
mod state;
mod texture;
mod transform;

pub use camera::*;
pub use light::*;
pub use material::*;
pub use mesh::*;
pub use object::*;
pub use pipeline::*;
pub use renderer::*;
pub use resources::*;
pub use state::*;
pub use texture::*;
pub use transform::*;

pub mod math;

#[cfg(test)]
mod img_diff;

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;
    use nalgebra::{Perspective3, Point2, Point3, UnitQuaternion, Vector3};

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

    struct CmyTri {
        gpu: WgpuState,
        ui: Renderling,
        _cam: Camera,
        tri: Object,
    }

    fn cmy_triangle_setup() -> CmyTri {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::WHITE;

        let mut ui: Renderling = gpu.new_ui_renderling();
        let cam = ui.new_camera().with_projection_ortho2d().build();
        let tri = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(right_tri_builder())
            .build()
            .unwrap();
        CmyTri {
            gpu,
            ui,
            _cam: cam,
            tri,
        }
    }

    #[test]
    fn cmy_triangle() {
        let mut c = cmy_triangle_setup();
        let (frame, depth) = c.gpu.next_frame().unwrap();
        c.gpu.clear(Some(&frame), Some(&depth));
        c.ui.update().unwrap();
        c.ui.render(&frame, &depth).unwrap();
        let img = c.gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_triangle", "../../img/cmy_triangle.png", img).unwrap();
    }

    #[test]
    fn cmy_triangle_update_transform() {
        let mut c = cmy_triangle_setup();
        let (frame, depth) = c.gpu.next_frame().unwrap();
        c.gpu.clear(Some(&frame), Some(&depth));
        c.ui.update().unwrap();
        c.ui.render(&frame, &depth).unwrap();
        c.gpu.present().unwrap();

        let (frame, depth) = c.gpu.next_frame().unwrap();
        c.gpu.clear(Some(&frame), Some(&depth));
        c.tri.set_transform(
            WorldTransform::default()
                .with_position(Point3::new(100.0, 0.0, 0.0))
                .with_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::z_axis(),
                    std::f32::consts::FRAC_PI_2,
                ))
                .with_scale(Vector3::new(0.5, 0.5, 1.0)),
        );
        c.ui.update().unwrap();
        c.ui.render(&frame, &depth).unwrap();

        let img = c.gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_triangle_update_transform",
            "../../img/cmy_triangle_update_transform.png",
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
    fn pyramid_points() -> [Point3<f32>; 5] {
        let tl = Point3::new(-0.5, -0.5, -0.5);
        let tr = Point3::new(0.5, -0.5, -0.5);
        let br = Point3::new(0.5, -0.5, 0.5);
        let bl = Point3::new(-0.5, -0.5, 0.5);
        let top = Point3::new(0.0, 0.5, 0.0);
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

    fn cmy_vertex(p: Point3<f32>) -> UiVertex {
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
    fn cmy_cube() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::WHITE;

        let mut ui: Renderling = gpu.new_ui_renderling();

        // test updating the camera by starting with ortho2d
        let cam = ui.new_camera().with_projection_ortho2d().build();
        cam.look_at(Point3::new(0.0, 12.0, 20.0), Point3::origin(), Vector3::y());
        cam.set_projection(Projection::Perspective(Perspective3::new(
            1.0,
            std::f32::consts::PI / 4.0,
            0.1,
            100.0,
        )));

        let _cube = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_scale(Vector3::new(6.0, 6.0, 6.0))
            .with_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                -std::f32::consts::FRAC_PI_4,
            ))
            .build()
            .unwrap();

        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube", "../../img/cmy_cube.png", img).unwrap();
    }

    #[test]
    fn cmy_cube_visible() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::WHITE;

        let mut ui: Renderling = gpu.new_ui_renderling();

        let cam = ui.new_camera().with_projection_perspective().build();

        let _cube_one = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_position(Point3::new(-4.0, 0.0, 0.0))
            .with_scale(Vector3::new(6.0, 6.0, 6.0))
            .with_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                -std::f32::consts::FRAC_PI_4,
            ))
            .build()
            .unwrap();

        let cube_two = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_position(Point3::new(4.0, 0.0, 0.0))
            .with_scale(Vector3::new(6.0, 6.0, 6.0))
            .with_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                std::f32::consts::FRAC_PI_4,
            ))
            .build()
            .unwrap();

        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_visible_before.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_before",
            "../../img/cmy_cube_visible_before.png",
            img,
        )
        .unwrap();

        cube_two.set_visible(false);
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_visible_after.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_after",
            "../../img/cmy_cube_visible_after.png",
            img,
        )
        .unwrap();

        cube_two.set_visible(true);
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_before_again",
            "../../img/cmy_cube_visible_before.png",
            img,
        )
        .unwrap();
    }

    #[test]
    fn cmy_cube_remesh() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::TRANSPARENT;
        let mut ui: Renderling = gpu.new_ui_renderling();
        let cam = ui.new_camera().with_projection_perspective().build();
        let cube = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_scale(Vector3::new(10.0, 10.0, 10.0))
            .build()
            .unwrap();
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_remesh_before.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_remesh_before",
            "../../img/cmy_cube_remesh_before.png",
            img,
        )
        .unwrap();

        let pyramid_mesh = pyramid_builder().build(Some("pyramid mesh"), &gpu.device);
        cube.set_mesh(Arc::new(pyramid_mesh));
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_remesh_after.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_remesh_after",
            "../../img/cmy_cube_remesh_after.png",
            img,
        )
        .unwrap();
    }

    #[test]
    fn cmy_cube_material() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::TRANSPARENT;
        let mut ui: Renderling = gpu.new_ui_renderling();
        let cam = ui.new_camera().with_projection_perspective().build();
        let png = image::open("../../img/sandstone.png").unwrap();
        let tex = gpu
            .create_texture(Some("sandstone_material"), &png.to_rgba8())
            .unwrap();
        let material = UiMaterial {
            diffuse_texture: tex,
            color_blend: UiColorBlend::UvOnly,
        };
        let builder = MeshBuilder::default().with_vertices({
            let p: [Point3<f32>; 8] = crate::math::unit_points();
            let tl = Point2::from([0.0, 0.0]);
            let tr = Point2::from([1.0, 0.0]);
            let bl = Point2::from([0.0, 1.0]);
            let br = Point2::from([1.0, 1.0]);

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
        });
        let cube = ui
            .new_object()
            .with_camera(&cam)
            .with_material(material)
            .with_mesh_builder(builder)
            .with_scale(Vector3::new(10.0, 10.0, 10.0))
            .build()
            .unwrap();
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_material_before.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_material_before",
            "../../img/cmy_cube_material_before.png",
            img,
        )
        .unwrap();

        let png = image::open("../../img/dirt.jpg").unwrap();
        let tex = gpu
            .create_texture(Some("dirt_material"), &png.to_rgba8())
            .unwrap();
        let material = UiMaterial {
            diffuse_texture: tex,
            color_blend: UiColorBlend::UvOnly,
        };
        cube.set_material(material);
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_material_after.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_material_after",
            "../../img/cmy_cube_material_after.png",
            img,
        )
        .unwrap();
    }

    fn init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_module("renderling", log::LevelFilter::Trace)
            .try_init();
    }

    #[test]
    /// Ensures that the directional light coloring works.
    fn forward_cube_directional() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::TRANSPARENT;
        let mut r = gpu.new_forward_renderling();
        let red = wgpu::Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        let green = wgpu::Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        };
        let blue = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        let dark_grey = wgpu::Color {
            r: 0.01,
            g: 0.01,
            b: 0.01,
            a: 1.0,
        };
        let _dir_red = r
            .new_directional_light()
            .with_direction(Vector3::new(0.0, -1.0, 0.0))
            .with_diffuse_color(red)
            .with_specular_color(red)
            .with_ambient_color(dark_grey)
            .build();
        let _dir_green = r
            .new_directional_light()
            .with_direction(Vector3::new(-1.0, 0.0, 0.0))
            .with_diffuse_color(green)
            .with_specular_color(green)
            .with_ambient_color(dark_grey)
            .build();
        let _dir_blue = r
            .new_directional_light()
            .with_direction(Vector3::new(0.0, 0.0, -1.0))
            .with_diffuse_color(blue)
            .with_specular_color(blue)
            .with_ambient_color(dark_grey)
            .build();
        let cam = r
            .new_camera()
            .with_projection_perspective()
            .with_look_at(Point3::new(1.8, 1.8, 1.8), Point3::origin(), Vector3::y())
            .build();
        let _tri = r
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(MeshBuilder::default().with_vertices(
                crate::math::unit_cube().into_iter().map(|(p, n)| {
                    ForwardVertex::default()
                        .with_position(p.x, p.y, p.z)
                        .with_normal(n.x, n.y, n.z)
                }),
            ))
            .build()
            .unwrap();
        r.update().unwrap();
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(Some(&frame), Some(&depth));
        r.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/forward_cube_directional.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "forward_cube_directional",
            "../../img/forward_cube_directional.png",
            img,
        )
        .unwrap();
    }
}
