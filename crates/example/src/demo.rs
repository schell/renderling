//! The standard renderling demo
use std::time::Instant;

use renderling::{
    math::{Quat, Vec3, Vec4},
    ForwardVertex, MeshBuilder, Renderling, Transform, UiPipeline, WgpuState,
};

/// Sets up the historical renderling demo and returns a closure that iterates it frame by frame.
pub fn historical(gpu: &mut WgpuState) -> impl FnMut(&mut WgpuState) {
    let window_size = gpu.get_size();
    // Get our ui renderling
    let mut ui: Renderling<UiPipeline> = gpu.new_ui_renderling();

    let _ui_camera = ui.new_camera().with_projection_ortho2d().build();
    let _triangle = ui
        .new_object()
        .with_mesh_builder(MeshBuilder::default().with_vertices(vec![
                    renderling::UiVertex::default()
                        .with_position(10.0, 10.0, 0.0)
                        .with_color(0.0, 1.0, 1.0, 1.0),
                    renderling::UiVertex::default()
                        .with_position(window_size.0 as f32 - 10.0, 10.0, 0.0)
                        .with_color(1.0, 0.0, 1.0, 1.0),
                    renderling::UiVertex::default()
                        .with_position(10.0, window_size.1 as f32 - 10.0, 0.0)
                        .with_color(1.0, 1.0, 0.0, 1.0),
                ]))
        .build()
        .unwrap();

    let mut forward = gpu.new_forward_renderling();
    let _forward_camera = forward
        .new_camera()
        .with_projection_perspective()
        .with_look_at(Vec3::new(0.0, 1.0, 2.5), Vec3::ZERO, Vec3::Y)
        .build();

    let mut icosphere = icosahedron::Polyhedron::new_isocahedron(0.65, 5);
    icosphere.compute_triangle_normals();
    let icosahedron::Polyhedron {
        positions,
        normals,
        cells,
        ..
    } = icosphere;
    log::info!("icosphere created");

    let to_vertex = |ndx: &usize| -> ForwardVertex {
        let mut v = ForwardVertex::default();
        v.position = positions[*ndx].0.into();
        v.normal = normals[*ndx].0.into();
        v
    };
    let sphere_vertices = cells.iter().flat_map(|icosahedron::Triangle { a, b, c }| {
        let p0 = to_vertex(&a);
        let p1 = to_vertex(&b);
        let p2 = to_vertex(&c);
        vec![p2, p1, p0]
    });
    let cube_vertices = renderling::math::unit_cube().into_iter().map(|(p, n)| {
        ForwardVertex::default()
            .with_position(p.x, p.y, p.z)
            .with_normal(n.x, n.y, n.z)
    });

    let _sphere = forward
        .new_object()
        .with_mesh_builder(MeshBuilder::default().with_vertices(sphere_vertices))
        .build()
        .unwrap();
    let cube = forward
        .new_object()
        .with_mesh_builder(MeshBuilder::default().with_vertices(cube_vertices))
        .build()
        .unwrap();

    let _spot_light = forward
        .new_spot_light()
        .with_position(Vec3::new(0.0, 10.0, 0.0))
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_ambient_color(Vec3::splat(0.0627).extend(1.0))
        .with_diffuse_color(Vec4::new(0.0627, 0.0627, 1.0, 1.0))
        .with_specular_color(Vec4::new(0.694, 0.694, 1.0, 1.0))
        .build();

    let _point_light = forward
        .new_point_light()
        .with_position(Vec3::new(2.0, 2.0, 0.0))
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(Vec3::splat(0.1).extend(1.0))
        .with_diffuse_color(Vec4::splat(1.0))
        .with_specular_color(Vec3::splat(0.5).extend(1.0))
        .build();

    let mut last_frame = Instant::now();
    let rotation_speed = std::f32::consts::FRAC_PI_4; // per second
    let mut rotation_y = 0.0;

    move |gpu| {
        // rotate the cube a bit
        let now = Instant::now();
        let dt = now - last_frame;
        let rotation = rotation_speed * dt.as_secs_f32();
        rotation_y += rotation;
        last_frame = now;
        cube.set_transform(
            Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Y, rotation_y)),
        );

        let (frame, depth) = gpu.next_frame_cleared().unwrap();
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();
        // just clear the depth texture, because we want to render over the 2d background
        gpu.clear(None, Some(&depth));
        forward.update().unwrap();
        forward.render(&frame, &depth).unwrap();
        gpu.present().unwrap();
    }
}
