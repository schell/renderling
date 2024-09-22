//! Runs through all the gltf sample models to test and show-off renderling's
//! gltf capabilities.
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::{Arc, Mutex},
};

use renderling::{
    atlas::AtlasImage,
    bvol::Aabb,
    camera::Camera,
    math::{EulerRot, Mat4, Quat, UVec2, Vec2, Vec3, Vec3Swizzles, Vec4},
    pbr::light::{DirectionalLight, Light},
    skybox::Skybox,
    slab::Hybrid,
    stage::{Animator, GltfDocument, Stage},
    transform::Transform,
    Context,
};
use winit::{
    event::KeyEvent,
    keyboard::{Key, NamedKey},
};

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

const DARK_BLUE_BG_COLOR: Vec4 = Vec4::new(
    0x30 as f32 / 255.0,
    0x35 as f32 / 255.0,
    0x42 as f32 / 255.0,
    1.0,
);

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

pub enum SupportedFileType {
    Gltf,
    Hdr,
}

pub fn is_file_supported(file: impl AsRef<std::path::Path>) -> Option<SupportedFileType> {
    let ext = file.as_ref().extension()?;
    Some(match ext.to_str()? {
        "hdr" => SupportedFileType::Hdr,
        _ => SupportedFileType::Gltf,
    })
}

#[cfg(not(target_arch = "wasm32"))]
lazy_static::lazy_static! {
    static ref START: std::time::Instant = std::time::Instant::now();
}

struct TurntableCameraController {
    /// look at
    center: Vec3,
    /// distance from the origin
    radius: f32,
    /// anglular position on a circle `radius` away from the origin on x,z
    phi: f32,
    /// angular distance from y axis
    theta: f32,
    /// is the left mouse button down
    left_mb_down: bool,
}

impl Default for TurntableCameraController {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 6.0,
            phi: 0.0,
            theta: std::f32::consts::FRAC_PI_4,
            left_mb_down: false,
        }
    }
}

impl CameraController for TurntableCameraController {
    fn reset(&mut self, bounds: Aabb) {
        self.radius = bounds.diagonal_length() * 1.25;
        self.center = bounds.center();
        self.left_mb_down = false;
    }

    fn update(&mut self, UVec2 { x: w, y: h }: UVec2, current_camera: &Hybrid<Camera>) {
        let camera_position = Self::camera_position(self.radius, self.phi, self.theta);
        let camera = Camera::new(
            Mat4::perspective_infinite_rh(std::f32::consts::FRAC_PI_4, w as f32 / h as f32, 0.01),
            Mat4::look_at_rh(camera_position, self.center, Vec3::Y),
        );
        debug_assert!(
            camera.view.is_finite(),
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
struct WasdMouseCameraController {
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
    fn update(&mut self, UVec2 { x: w, y: h }: UVec2, camera: &Hybrid<Camera>) {
        let now = now();
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

        let camera_rotation =
            Quat::from_euler(renderling::math::EulerRot::XYZ, self.phi, self.theta, 0.0);
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
            Key::Character(c) => {
                match c.as_str() {
                    "p" => self.forward_is_down = state.is_pressed(),
                    "k" => self.backward_is_down = state.is_pressed(),
                    "i" => self.right_is_down = state.is_pressed(),
                    "y" => self.left_is_down = state.is_pressed(),
                    "u" => self.up_is_down = state.is_pressed(),
                    "U" => self.down_is_down = state.is_pressed(),
                    s => log::info!("unused key char '{s}'"),
                }
                log::info!(
                    "forward,backward: {},{}",
                    self.forward_is_down,
                    self.backward_is_down
                );
            }

            k => log::info!("key: {k:#?}"),
        }
    }
}

pub trait CameraController {
    fn reset(&mut self, bounds: Aabb);
    fn update(&mut self, size: UVec2, camera: &Hybrid<Camera>);
    fn mouse_scroll(&mut self, delta: f32);
    fn mouse_moved(&mut self, position: Vec2);
    fn mouse_motion(&mut self, delta: Vec2);
    fn mouse_button(&mut self, is_pressed: bool, is_left_button: bool);
    fn key(&mut self, event: KeyEvent);
}

fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        let doc = web_sys::window().unwrap();
        let perf = doc.performance().unwrap();
        perf.now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = std::time::Instant::now();
        let duration = now.duration_since(*START);
        duration.as_secs_f64()
    }
}

pub struct App {
    last_frame_instant: f64,
    skybox_image_bytes: Option<Vec<u8>>,
    loads: Arc<Mutex<HashMap<std::path::PathBuf, Vec<u8>>>>,

    pub stage: Stage,
    camera: Hybrid<Camera>,
    _light: Option<(Hybrid<DirectionalLight>, Hybrid<Light>)>,

    document: Option<GltfDocument>,
    animators: Option<Vec<Animator>>,
    animations_conflict: bool,

    camera_controller: Box<dyn CameraController>,
}

impl CameraController for App {
    fn mouse_scroll(&mut self, delta: f32) {
        self.camera_controller.mouse_scroll(delta);
        self.update_camera();
    }

    /// Set the mouse position relative to the top left of the window.
    fn mouse_moved(&mut self, position: Vec2) {
        self.camera_controller.mouse_moved(position);
        self.update_camera();
    }

    /// Update from device mouse motion.
    fn mouse_motion(&mut self, delta: Vec2) {
        self.camera_controller.mouse_motion(delta);
        self.update_camera();
    }

    fn mouse_button(&mut self, is_pressed: bool, is_left_button: bool) {
        self.camera_controller
            .mouse_button(is_pressed, is_left_button);
        self.update_camera();
    }

    fn key(&mut self, event: KeyEvent) {
        self.camera_controller.key(event);
        self.update_camera();
    }

    fn reset(&mut self, bounds: Aabb) {
        self.camera_controller.reset(bounds);
    }

    fn update(&mut self, size: UVec2, camera: &Hybrid<Camera>) {
        self.camera_controller.update(size, camera);
    }
}

impl App {
    pub fn new(r: &Context, camera_control: CameraControl) -> Self {
        let stage = r
            .new_stage()
            .with_background_color(DARK_BLUE_BG_COLOR)
            .with_bloom_mix_strength(0.5)
            .with_bloom_filter_radius(4.0)
            .with_msaa_sample_count(4);
        let camera = stage.new_value(Camera::default());
        // let sunlight = stage.new_value(DirectionalLight {
        //     direction: Vec3::NEG_Y,
        //     color: hex_to_vec4(0xFDFBD3FF),
        //     intensity: 10.0,
        // });
        // let light = stage.new_value(Light::from(sunlight.id()));
        // stage.set_lights([light.id()]);

        stage
            .set_atlas_size(wgpu::Extent3d {
                width: 2048,
                height: 2048,
                depth_or_array_layers: 32,
            })
            .unwrap();

        Self {
            stage,
            camera,
            _light: None,

            document: None,
            animators: None,
            animations_conflict: false,

            skybox_image_bytes: None,
            loads: Arc::new(Mutex::new(HashMap::default())),
            last_frame_instant: now(),

            camera_controller: match camera_control {
                CameraControl::Turntable => Box::new(TurntableCameraController::default()),
                CameraControl::WasdMouse => Box::new(WasdMouseCameraController::default()),
            },
        }
    }

    pub fn update_camera(&mut self) {
        self.camera_controller
            .update(self.stage.get_size(), &self.camera);
    }

    fn load_hdr_skybox(&mut self, bytes: Vec<u8>) {
        let img = AtlasImage::from_hdr_bytes(&bytes).unwrap();
        let (device, queue) = self.stage.get_device_and_queue_owned();
        let skybox = Skybox::new(&device, &queue, img, self.camera.id());
        self.skybox_image_bytes = Some(bytes);
        self.stage.set_skybox(skybox);
    }

    fn load_gltf_model(&mut self, path: impl AsRef<std::path::Path>, bytes: &[u8]) {
        log::info!("loading gltf");
        self.camera_controller
            .reset(Aabb::new(Vec3::NEG_ONE, Vec3::ONE));
        self.stage.clear_images().unwrap();
        self.document = None;
        log::debug!("ticking stage to reclaim buffers");
        self.stage.tick();
        let doc = match self
            .stage
            .load_gltf_document_from_bytes(bytes, self.camera.id())
        {
            Err(e) => {
                log::error!("gltf loading error: {e}");
                if cfg!(not(target_arch = "wasm32")) {
                    log::info!("attempting to load by filesystem");
                    match self
                        .stage
                        .load_gltf_document_from_path(path, self.camera.id())
                    {
                        Ok(doc) => doc,
                        Err(e) => {
                            log::error!("gltf loading error: {e}");
                            return;
                        }
                    }
                } else {
                    return;
                }
            }
            Ok(doc) => doc,
        };

        // self.entities = builder.entities.clone();

        // find the bounding box of the model so we can display it correctly
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        let scene = doc.default_scene.unwrap_or(0);
        log::info!("Displaying scene {scene}");
        fn get_children(doc: &GltfDocument, n: usize) -> Vec<usize> {
            let mut children = vec![];
            if let Some(parent) = doc.nodes.get(n) {
                children.extend(parent.children.iter().copied());
                let descendants = parent
                    .children
                    .iter()
                    .copied()
                    .flat_map(|n| get_children(doc, n));
                children.extend(descendants);
            }
            children
        }

        let nodes = doc.nodes_in_scene(scene).flat_map(|n| {
            let mut all_nodes = vec![n];
            for child_index in get_children(&doc, n.index) {
                if let Some(child_node) = doc.nodes.get(child_index) {
                    all_nodes.push(child_node);
                }
            }
            all_nodes
        });
        log::trace!("  nodes:");
        for node in nodes {
            let tfrm = Mat4::from(node.global_transform());
            let decomposed = Transform::from(tfrm);
            log::trace!("    {} {:?} {decomposed:?}", node.index, node.name);
            if let Some(mesh_index) = node.mesh {
                // UNWRAP: safe because we know the node exists
                for primitive in doc.meshes.get(mesh_index).unwrap().primitives.iter() {
                    let bbmin = tfrm.transform_point3(primitive.bounding_box.0);
                    let bbmax = tfrm.transform_point3(primitive.bounding_box.1);
                    min = min.min(bbmin);
                    max = max.max(bbmax);
                }
            }
        }

        log::trace!("Bounding box: {min} {max}");
        let bounding_box = Aabb::new(min, max);
        self.camera_controller.reset(bounding_box);
        self.camera_controller
            .update(self.stage.get_size(), &self.camera);

        self.last_frame_instant = now();

        if doc.animations.is_empty() {
            log::trace!("  animations: none");
        } else {
            log::trace!("  animations:");
        }
        let mut animated_nodes = HashSet::default();
        let mut has_conflicting_animations = false;
        self.animators = Some(
            doc.animations
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let target_nodes = a.target_node_indices().collect::<HashSet<_>>();
                    has_conflicting_animations =
                        has_conflicting_animations || !animated_nodes.is_disjoint(&target_nodes);
                    animated_nodes.extend(target_nodes);

                    log::trace!("    {i} {:?} {}s", a.name, a.length_in_seconds());
                    // for (t, tween) in a.tweens.iter().enumerate() {
                    //     log::trace!(
                    //         "      tween {t} targets node {} {}",
                    //         tween.target_node_index,
                    //         tween.properties.description()
                    //     );
                    // }
                    Animator::new(doc.nodes.iter(), a.clone())
                })
                .collect(),
        );
        if has_conflicting_animations {
            log::trace!("  and some animations conflict");
        }
        self.animations_conflict = has_conflicting_animations;
        self.document = Some(doc);
    }

    pub fn tick_loads(&mut self) {
        let loaded = std::mem::take(&mut *self.loads.lock().unwrap());
        for (path, bytes) in loaded.into_iter() {
            log::info!("loaded {}bytes from {}", bytes.len(), path.display());
            match is_file_supported(&path) {
                Some(SupportedFileType::Gltf) => self.load_gltf_model(path, &bytes),
                Some(SupportedFileType::Hdr) => self.load_hdr_skybox(bytes),
                None => {}
            }
        }
    }

    /// Queues a load operation.
    pub fn load(&mut self, path: &str) {
        let path = std::path::PathBuf::from(path);
        let loads = self.loads.clone();

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let path_str = format!("{}", path.display());
                let bytes = loading_bytes::load(&path_str).await.unwrap();
                let mut loads = loads.lock().unwrap();
                loads.insert(path, bytes);
                log::debug!("loaded {path_str}");
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::thread::spawn(move || {
                let bytes = std::fs::read(&path)
                    .unwrap_or_else(|e| panic!("could not load '{}': {e}", path.display()));
                let mut loads = loads.lock().unwrap();
                loads.insert(path, bytes);
            });
        }
    }

    pub fn set_size(&mut self, size: UVec2) {
        self.stage.set_size(size);
    }

    pub fn animate(&mut self) {
        let now = now();
        let dt_seconds = now - self.last_frame_instant;
        self.last_frame_instant = now;
        if let Some(animators) = self.animators.as_mut() {
            if self.animations_conflict {
                if let Some(animator) = animators.first_mut() {
                    animator.progress(dt_seconds as f32).unwrap();
                }
            } else {
                for animator in animators.iter_mut() {
                    animator.progress(dt_seconds as f32).unwrap();
                }
            }
        }
    }
}

// /// Sets up the demo for a given model
// pub async fn demo(
//     ctx: &Context,
//     model: Option<impl AsRef<str>>,
//     skybox: Option<impl AsRef<str>>,
// ) -> impl FnMut(&mut Context, Option<&winit::event::WindowEvent>) {
//     let mut app = App::new(ctx).await;
//     if let Some(file) = model {
//         app.load(file.as_ref());
//     }
//     if let Some(file) = skybox {
//         app.load(file.as_ref());
//     }
//     let mut event_state = renderling_gpui::EventState::default();
//     move |r, ev: Option<&winit::event::WindowEvent>| {
//         if let Some(ev) = ev {
//             match ev {
//                 winit::event::WindowEvent::MouseWheel { delta, .. } => {
//                     let delta = match delta {
//                         winit::event::MouseScrollDelta::LineDelta(_, up) =>
// *up,                         winit::event::MouseScrollDelta::PixelDelta(pos)
// => pos.y as f32,                     };

//                     app.zoom(r, delta);
//                 }
//                 winit::event::WindowEvent::CursorMoved { position, .. } => {
//                     app.pan(r, *position);
//                 }
//                 winit::event::WindowEvent::MouseInput { state, button, .. }
// => {                     app.mouse_button(*state, *button);
//                 }
//                 winit::event::WindowEvent::KeyboardInput { input, .. } => {
//                     app.key_input(r, *input);
//                 }
//                 winit::event::WindowEvent::Resized(size) => {
//                     log::trace!("resizing to {size:?}");
//                     app.resize(r, size.width, size.height);
//                     let _ = app
//                         .gpui
//                         .0
//                         .graph
//                         .get_resource::<ScreenSize>()
//                         .unwrap()
//                         .unwrap();
//                 }
//                 winit::event::WindowEvent::DroppedFile(path) => {
//                     log::trace!("got dropped file event: {}",
// path.display());                     let path = format!("{}",
// path.display());                     app.load(&path);
//                 }
//                 _ => {}
//             }

//             if let Some(ev) = event_state.event_from_winit(ev) {
//                 let scene =
// r.graph.get_resource_mut::<Scene>().unwrap().unwrap();                 let
// channel = scene.get_debug_channel();                 let mut
// set_debug_channel = |mode| {                     log::debug!("setting debug
// mode to {mode:?}");                     if channel != mode {
//                         scene.set_debug_channel(mode);
//                     } else {
//                         scene.set_debug_channel(DebugChannel::None);
//                     }
//                 };

//                 match app.ui.event(ev) {
//                     None => {}
//                     Some(ev) => match ev {
//                         UiEvent::ToggleDebugChannel(channel) =>
// set_debug_channel(channel),                     },
//                 }
//             }
//         } else {
//             app.tick_loads(r);
//             app.update_camera_view(r);
//             app.animate(r);
//             app.gpui.layout(&mut app.ui);
//             app.gpui.render(&mut app.ui);
//             r.render().unwrap();
//         }
//     }
// }
