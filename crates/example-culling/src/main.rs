//! An example app showing (and verifying) how frustum culling works in
//! `renderling`.
use std::{any::Any, sync::Arc};

use example::{camera::CameraController, utils::*};
use glam::*;
use renderling::{
    bvol::{Aabb, BoundingSphere},
    math::hex_to_vec4,
    prelude::*,
    tonemapping::srgba_to_linear,
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent},
    event_loop::ActiveEventLoop,
    keyboard::Key,
};

const MIN_SIZE: f32 = 0.5;
const MAX_SIZE: f32 = 4.0;
const MAX_DIST: f32 = 40.0;
const BOUNDS: Aabb = Aabb {
    min: Vec3::new(-MAX_DIST, -MAX_DIST, -MAX_DIST),
    max: Vec3::new(MAX_DIST, MAX_DIST, MAX_DIST),
};

struct AppCamera(Hybrid<Camera>);
struct FrustumCamera(Camera);

#[allow(dead_code)]
struct CullingExample {
    app_camera: AppCamera,
    controller: example::camera::TurntableCameraController,
    stage: Stage,
    dlights: [AnalyticalLightBundle; 2],
    material_aabb_overlapping: Hybrid<Material>,
    material_aabb_outside: Hybrid<Material>,
    material_frustum: Hybrid<Material>,
    frustum_camera: FrustumCamera,
    frustum_vertices: HybridArray<Vertex>,
    frustum_renderlet: Hybrid<Renderlet>,
    resources: BagOfResources,
    next_k: u64,
}

impl CullingExample {
    fn make_aabb(center: Vec3, half_size: Vec3) -> Aabb {
        let min = center - half_size;
        let max = center + half_size;
        Aabb::new(min, max)
    }

    fn make_aabbs(
        seed: u64,
        stage: &Stage,
        frustum_camera: &FrustumCamera,
        material_outside: &Hybrid<Material>,
        material_overlapping: &Hybrid<Material>,
    ) -> Box<dyn Any> {
        log::info!("generating aabbs with seed {seed}");
        fastrand::seed(seed);
        Box::new(
            (0..25u32)
                .map(|i| {
                    log::info!("aabb {i}");
                    let x = fastrand::f32() * MAX_DIST - MAX_DIST / 2.0;
                    let y = fastrand::f32() * MAX_DIST - MAX_DIST / 2.0;
                    let z = fastrand::f32() * MAX_DIST - MAX_DIST / 2.0;
                    let w = fastrand::f32() * (MAX_SIZE - MIN_SIZE) + MIN_SIZE;
                    let h = fastrand::f32() * (MAX_SIZE - MIN_SIZE) + MIN_SIZE;
                    let l = fastrand::f32() * (MAX_SIZE - MIN_SIZE) + MIN_SIZE;

                    let rx = std::f32::consts::PI * fastrand::f32();
                    let ry = std::f32::consts::PI * fastrand::f32();
                    let rz = std::f32::consts::PI * fastrand::f32();

                    let rotation = Quat::from_euler(EulerRot::XYZ, rx, ry, rz);

                    let center = Vec3::new(x, y, z);
                    let half_size = Vec3::new(w, h, l);
                    let aabb = Self::make_aabb(Vec3::ZERO, half_size);
                    let aabb_transform = Transform {
                        translation: center,
                        rotation,
                        ..Default::default()
                    };

                    let transform = stage.new_transform(aabb_transform);
                    let (aabb_vertices, aabb_renderlet) = {
                        let material_id = if BoundingSphere::from(aabb)
                            .is_inside_camera_view(&frustum_camera.0, transform.get())
                            .0
                        {
                            material_overlapping.id()
                        } else {
                            material_outside.id()
                        };
                        let (renderlet, vertices) = stage
                            .builder()
                            .with_vertices(aabb.get_mesh().into_iter().map(|(position, normal)| {
                                Vertex {
                                    position,
                                    normal,
                                    ..Default::default()
                                }
                            }))
                            .with_transform_id(transform.id())
                            .with_material_id(material_id)
                            .build();
                        (renderlet, vertices.into_gpu_only())
                    };
                    (aabb_renderlet, aabb_vertices, transform)
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl ApplicationHandler for CullingExample {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("culling-example resumed");
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Character(c),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if c.as_str() == "r" {
                    self.resources.drain();
                    let _ = self.stage.commit();
                    self.resources.push(Self::make_aabbs(
                        self.next_k,
                        &self.stage,
                        &self.frustum_camera,
                        &self.material_aabb_outside,
                        &self.material_aabb_overlapping,
                    ));
                    self.next_k += 1;
                }
            }
            winit::event::WindowEvent::Resized(physical_size) => {
                log::info!("window resized to {physical_size:?}");
                let size = UVec2 {
                    x: physical_size.width,
                    y: physical_size.height,
                };
                self.stage.set_size(size);
                self.controller.update_camera(size, &self.app_camera.0);
            }
            event => self.controller.window_event(event),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.controller.device_event(event);
    }
}

impl TestAppHandler for CullingExample {
    fn new(
        _event_loop: &ActiveEventLoop,
        _window: Arc<winit::window::Window>,
        ctx: &Context,
    ) -> Self {
        let mut seed = 46;
        let mut resources = BagOfResources::default();
        let stage = ctx.new_stage().with_lighting(true);
        let sunlight_a = stage.new_analytical_light(
            DirectionalLightDescriptor {
                direction: Vec3::new(-0.8, -1.0, 0.5).normalize(),
                color: Vec4::ONE,
                intensity: 10.0,
            },
            None,
        );
        let sunlight_b = stage.new_analytical_light(
            DirectionalLightDescriptor {
                direction: Vec3::new(1.0, 1.0, -0.1).normalize(),
                color: Vec4::ONE,
                intensity: 1.0,
            },
            None,
        );

        let dlights = [sunlight_a, sunlight_b];

        let frustum_camera = FrustumCamera({
            let aspect = 1.0;
            let fovy = core::f32::consts::FRAC_PI_4;
            let znear = 4.0;
            let zfar = 1000.0;
            let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
            let eye = Vec3::new(0.0, 0.0, 10.0);
            let target = Vec3::ZERO;
            let up = Vec3::Y;
            let view = Mat4::look_at_rh(eye, target, up);
            // let projection = Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, -10.0,
            // 10.0); let view = Mat4::IDENTITY;
            Camera::new(projection, view)
        });

        let frustum = frustum_camera.0.frustum();
        log::info!("frustum.planes: {:#?}", frustum.planes);

        let blue_color = srgba_to_linear(hex_to_vec4(0x7EACB5FF));
        let red_color = srgba_to_linear(hex_to_vec4(0xC96868FF));
        let yellow_color = srgba_to_linear(hex_to_vec4(0xFADFA1FF));

        let material_aabb_overlapping = stage.new_material(Material {
            albedo_factor: blue_color,
            ..Default::default()
        });
        let material_aabb_outside = stage.new_material(Material {
            albedo_factor: red_color,
            ..Default::default()
        });
        let material_frustum = stage.new_material(Material {
            albedo_factor: yellow_color,
            ..Default::default()
        });
        let app_camera = AppCamera(stage.new_camera(Camera::default()));
        resources.push(Self::make_aabbs(
            seed,
            &stage,
            &frustum_camera,
            &material_aabb_outside,
            &material_aabb_overlapping,
        ));
        seed += 1;

        let frustum_vertices =
            stage.new_vertices(frustum_camera.0.frustum().get_mesh().into_iter().map(
                |(position, normal)| Vertex {
                    position,
                    normal,
                    ..Default::default()
                },
            ));
        let frustum_renderlet = stage.new_renderlet(Renderlet {
            vertices_array: frustum_vertices.array(),
            material_id: material_frustum.id(),
            ..Default::default()
        });
        stage.add_renderlet(&frustum_renderlet);

        Self {
            next_k: seed,
            app_camera,
            frustum_camera,
            dlights,
            controller: {
                let mut c = example::camera::TurntableCameraController::default();
                c.reset(BOUNDS);
                c.phi = 0.5;
                c
            },
            stage,
            material_aabb_overlapping,
            material_aabb_outside,
            material_frustum,
            frustum_vertices,
            frustum_renderlet,
            resources,
        }
    }

    fn render(&mut self, ctx: &Context) {
        let size = self.stage.get_size();
        self.controller.update_camera(size, &self.app_camera.0);

        let frame = ctx.get_next_frame().unwrap();
        self.stage.render(&frame.view());
        frame.present();
    }
}

fn main() {
    env_logger::builder().init();
    log::info!("starting example-culling");
    TestApp::<CullingExample>::new(winit::dpi::LogicalSize::new(800, 600)).run();
}
