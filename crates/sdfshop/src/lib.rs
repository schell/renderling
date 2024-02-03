//! A web application for exploring and authoring SDFs.
use crabslab::GrowableSlab;
use glam::{Mat4, Vec2, Vec3};
use renderling::{
    sdf::{
        raymarch::{renderer::RaymarchingRenderer, *},
        Scene, Sdf,
    },
    Camera, Renderling,
};

//pub mod wgsl;

pub fn create_raymarch(raymarcher: &mut RaymarchingRenderer, renderling: &mut Renderling) {
    let (width, height) = renderling.get_screen_size();
    let width = width as f32;
    let height = height as f32;
    let projection = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 3.0, 20.0), Vec3::ZERO, Vec3::Y);
    let camera = Camera::new(projection, view);
    let camera_id = raymarcher.slab.append(&camera);
    //let (device, queue) = r.renderling.get_device_and_queue_owned();
    // let hdr = AtlasImage::from_hdr_path("../../img/hdr/helipad.hdr")
    //     .unwrap_or_else(|e| panic!("could not load hdr: {e}\n{:?}", std::env::current_dir()));
    // let skybox = Skybox::new(device, queue, hdr, camera_id);
    // r.set_skybox(skybox);

    let mut sdfs = vec![];

    let sdf = Sdf {
        distance: raymarcher
            .slab
            .append_array(&sdf_sphere(input_position(), 0.1)),
        albedo: raymarcher
            .slab
            .append_array(&constant(Vec3::new(1.0, 1.0, 0.0))),
    };
    sdfs.push(sdf);

    // let sdf = Sdf {
    //     distance: r
    //         .slab
    //         .append_array(&sdf_plane(input_position(), Vec3::Y, 0.0)),
    //     albedo: r.slab.append_array(&constant(Vec3::new(0.0, 1.0, 1.0))),
    // };
    // sdfs.push(sdf);

    let sdfs = raymarcher.slab.append_array(&sdfs);

    let scene = raymarcher.slab.append(&Scene {
        camera: camera_id,
        sdfs,
        ..Default::default()
    });
    let raymarch = raymarcher.slab.append(&Raymarch {
        scene,
        screen_resolution: Vec2::new(width, height),
        ..Default::default()
    });
    raymarcher.raymarch = raymarch;
}
