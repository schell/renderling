//! Camera control.
use std::str::FromStr;

use craballoc::prelude::Hybrid;
use renderling::prelude::glam::{Mat4, Quat, UVec2, Vec2, Vec3};
use renderling::{bvol::Aabb, camera::Camera};
use winit::{event::KeyEvent, keyboard::Key};

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

#[derive(Clone, Copy, Debug, Default)]
pub enum CameraControl {
    #[default]
    Turntable,
    WasdMouse,
}

impl FromStr for CameraControl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turntable" => Ok(CameraControl::Turntable),
            "wasdmouse" => Ok(CameraControl::WasdMouse),
            _ => Err("must be 'turntable' or 'wasdmouse'".to_owned()),
        }
    }
}

pub struct TurntableCameraController {
    /// look at
    pub center: Vec3,
    /// distance from the origin
    pub radius: f32,
    /// Determines the distance between the camera's near and far planes
    depth: f32,
    /// anglular position on a circle `radius` away from the origin on x,z
    pub phi: f32,
    /// angular distance from y axis
    pub theta: f32,
    /// is the left mouse button down
    left_mb_down: bool,
}

impl Default for TurntableCameraController {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 6.0,
            depth: 12.0,
            phi: 0.0,
            theta: std::f32::consts::FRAC_PI_4,
            left_mb_down: false,
        }
    }
}

impl CameraController for TurntableCameraController {
    fn tick(&mut self) {}

    fn reset(&mut self, bounds: Aabb) {
        log::debug!("resetting turntable bounds to {bounds:?}");
        let diagonal_length = bounds.diagonal_length();
        self.radius = diagonal_length * 1.25;
        self.depth = 2.0 * diagonal_length;
        self.center = bounds.center();
        self.left_mb_down = false;
    }

    fn update_camera(&self, UVec2 { x: w, y: h }: UVec2, current_camera: &Hybrid<Camera>) {
        let camera_position = Self::camera_position(self.radius, self.phi, self.theta);
        let znear = self.depth / 1000.0;
        let camera = Camera::new(
            Mat4::perspective_rh(
                std::f32::consts::FRAC_PI_4,
                w as f32 / h as f32,
                znear,
                self.depth,
            ),
            Mat4::look_at_rh(camera_position, self.center, Vec3::Y),
        );
        debug_assert!(
            camera.view().is_finite(),
            "camera view is borked w:{w} h:{h} camera_position: {camera_position} center: {} \
             radius: {} phi: {} theta: {}",
            self.center,
            self.radius,
            self.phi,
            self.theta
        );
        if current_camera.get() != camera {
            current_camera.set(camera);
        }
    }

    fn mouse_scroll(&mut self, delta: f32) {
        self.zoom(delta);
    }

    fn mouse_moved(&mut self, _position: Vec2) {}

    fn mouse_motion(&mut self, delta: Vec2) {
        self.pan(delta);
    }

    fn mouse_button(&mut self, is_pressed: bool, is_left_button: bool) {
        self.left_mb_down = is_left_button && is_pressed;
    }

    fn key(&mut self, _event: KeyEvent) {}
}

impl TurntableCameraController {
    fn camera_position(radius: f32, phi: f32, theta: f32) -> Vec3 {
        // convert from spherical to cartesian
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        // in renderling Y is up so switch the y and z axis
        Vec3::new(x, z, y)
    }

    fn zoom(&mut self, delta: f32) {
        self.radius = (self.radius - (delta * RADIUS_SCROLL_DAMPENING)).max(0.0);
    }

    fn pan(&mut self, delta: Vec2) {
        if self.left_mb_down {
            self.phi += delta.x * DX_DY_DRAG_DAMPENING;

            let next_theta = self.theta - delta.y * DX_DY_DRAG_DAMPENING;
            self.theta = next_theta.clamp(0.0001, std::f32::consts::PI);
        }
    }
}

#[derive(Default)]
pub struct WasdMouseCameraController {
    position: Vec3,
    theta: f32,
    phi: f32,
    forward_is_down: bool,
    backward_is_down: bool,
    left_is_down: bool,
    right_is_down: bool,
    up_is_down: bool,
    down_is_down: bool,
    speed: f32,
    last_tick: Option<f64>,
}

impl CameraController for WasdMouseCameraController {
    fn tick(&mut self) {
        let now = super::now();
        if let Some(prev) = self.last_tick.replace(now) {
            let dt = now - prev;

            // We want the direction to be based solely on self.theta, because we
            // don't want the camera to move in Y.
            let forward_direction = Vec3::NEG_Z;
            let left_direction = Vec3::NEG_X;
            let up_direction = Vec3::Y;
            let rotation = Quat::from_rotation_y(-self.theta);

            if self.forward_is_down {
                let direction = rotation * forward_direction;
                let velocity = dt as f32 * self.speed * direction;
                self.position += velocity;
            }
            if self.backward_is_down {
                let direction = rotation * forward_direction;
                let velocity = dt as f32 * self.speed * direction;
                self.position -= velocity;
            }
            if self.left_is_down {
                let direction = rotation * left_direction;
                let velocity = dt as f32 * self.speed * direction;
                self.position += velocity;
            }
            if self.right_is_down {
                let direction = rotation * left_direction;
                let velocity = dt as f32 * self.speed * direction;
                self.position -= velocity;
            }
            if self.up_is_down {
                let velocity = dt as f32 * self.speed * up_direction;
                self.position += velocity;
            }
            if self.down_is_down {
                let velocity = dt as f32 * self.speed * up_direction;
                self.position -= velocity;
            }
        }
    }

    fn update_camera(&self, UVec2 { x: w, y: h }: UVec2, camera: &Hybrid<Camera>) {
        let camera_rotation = Quat::from_euler(
            renderling::prelude::glam::EulerRot::XYZ,
            self.phi,
            self.theta,
            0.0,
        );
        let projection =
            Mat4::perspective_infinite_rh(std::f32::consts::FRAC_PI_4, w as f32 / h as f32, 0.01);
        let view = Mat4::from_quat(camera_rotation) * Mat4::from_translation(-self.position);
        camera.modify(|c| c.set_projection_and_view(projection, view));
    }

    fn reset(&mut self, _bounds: Aabb) {
        self.position = Vec3::ZERO; //center_max_z + (center_max_z - center_min_z) * 0.5;
        self.theta = 0.0;
        self.phi = 0.0;
        self.speed = 2.0;
        log::info!("resetting to position: {}", self.position);
    }

    fn mouse_scroll(&mut self, _delta: f32) {}

    fn mouse_moved(&mut self, _position: Vec2) {}

    fn mouse_motion(&mut self, delta: Vec2) {
        const DAMPER: f32 = 0.005;
        self.theta = (self.theta + DAMPER * delta.x).rem_euclid(2.0 * std::f32::consts::PI);
        self.phi = (self.phi + DAMPER * delta.y).clamp(-1.2, 1.2);
    }

    fn mouse_button(&mut self, _is_pressed: bool, _is_left_button: bool) {}

    fn key(
        &mut self,
        KeyEvent {
            logical_key, state, ..
        }: KeyEvent,
    ) {
        match logical_key {
            Key::Character(c) => match c.as_str() {
                "p" => self.forward_is_down = state.is_pressed(),
                "k" => self.backward_is_down = state.is_pressed(),
                "i" => self.right_is_down = state.is_pressed(),
                "y" => self.left_is_down = state.is_pressed(),
                "u" => {
                    self.down_is_down = false;
                    self.up_is_down = state.is_pressed();
                }
                "U" => {
                    self.up_is_down = false;
                    self.down_is_down = state.is_pressed();
                }
                s => log::info!("unused key char '{s}'"),
            },

            k => log::info!("key: {k:#?}"),
        }
    }
}

pub trait CameraController {
    fn reset(&mut self, bounds: Aabb);
    fn tick(&mut self);
    fn update_camera(&self, size: UVec2, camera: &Hybrid<Camera>);
    fn mouse_scroll(&mut self, delta: f32);
    fn mouse_moved(&mut self, position: Vec2);
    fn mouse_motion(&mut self, delta: Vec2);
    fn mouse_button(&mut self, is_pressed: bool, is_left_button: bool);
    fn key(&mut self, event: KeyEvent);

    fn window_event(&mut self, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, up) => up,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };

                self.mouse_scroll(delta);
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_moved(Vec2::new(position.x as f32, position.y as f32));
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = matches!(state, winit::event::ElementState::Pressed);
                let is_left_button = matches!(button, winit::event::MouseButton::Left);
                self.mouse_button(is_pressed, is_left_button);
            }
            winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.key(event);
            }

            _ => {}
        }
    }

    fn device_event(&mut self, event: winit::event::DeviceEvent) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            self.mouse_motion(Vec2::new(delta.0 as f32, delta.1 as f32))
        }
    }
}
