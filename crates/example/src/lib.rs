//! Runs through all the gltf sample models to test and show-off renderling's
//! gltf capabilities.
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use renderling::{
    atlas::AtlasImage,
    camera::Camera,
    math::{Mat4, UVec2, Vec3, Vec4},
    skybox::Skybox,
    slab::Hybrid,
    stage::{Animator, GltfDocument, Node, Stage},
    transform::Transform,
    Context,
};

const RADIUS_SCROLL_DAMPENING: f32 = 0.001;
const DX_DY_DRAG_DAMPENING: f32 = 0.01;

const DARK_BLUE_BG_COLOR: Vec4 = Vec4::new(
    0x30 as f32 / 255.0,
    0x35 as f32 / 255.0,
    0x42 as f32 / 255.0,
    1.0,
);

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

    document: Option<GltfDocument>,
    nodes: Option<Vec<Node>>,
    animators: Option<Vec<Animator>>,
    animations_conflict: bool,

    // look at
    eye: Vec3,
    // distance from the origin
    radius: f32,
    // anglular position on a circle `radius` away from the origin on x,z
    phi: f32,
    // angular distance from y axis
    theta: f32,
    // is the left mouse button down
    left_mb_down: bool,
    // cursor position
    last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
}

impl App {
    pub fn new(r: &Context) -> Self {
        let mut stage = r
            .new_stage()
            .with_background_color(DARK_BLUE_BG_COLOR)
            .with_bloom_mix_strength(0.5)
            .with_bloom_filter_radius(4.0);
        let camera = stage.new_value(Camera::default());

        let radius = 6.0;
        let phi = 0.0;
        let theta = std::f32::consts::FRAC_PI_4;
        let left_mb_down: bool = false;
        let last_cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;

        Self {
            stage,
            camera,

            document: None,
            nodes: None,
            animators: None,
            animations_conflict: false,

            skybox_image_bytes: None,
            loads: Arc::new(Mutex::new(HashMap::default())),
            last_frame_instant: now(),
            radius,
            eye: Vec3::ZERO,
            phi,
            theta,
            left_mb_down,
            last_cursor_position,
        }
    }

    fn camera_position(radius: f32, phi: f32, theta: f32) -> Vec3 {
        // convert from spherical to cartesian
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        // in renderling Y is up so switch the y and z axis
        Vec3::new(x, z, y)
    }

    pub fn update_camera_view(&self) {
        let UVec2 { x: w, y: h } = self.stage.get_size();
        let camera = Camera::new(
            Mat4::perspective_infinite_rh(std::f32::consts::FRAC_PI_4, w as f32 / h as f32, 0.01),
            Mat4::look_at_rh(
                Self::camera_position(self.radius, self.phi, self.theta),
                self.eye,
                Vec3::Y,
            ),
        );
        if self.camera.get() != camera {
            self.camera.set(camera);
        }
    }

    fn load_hdr_skybox(&mut self, bytes: Vec<u8>) {
        let img = AtlasImage::from_hdr_bytes(&bytes).unwrap();
        let (device, queue) = self.stage.get_device_and_queue_owned();
        let skybox = Skybox::new(device, queue, img, self.camera.id());
        self.skybox_image_bytes = Some(bytes);
        self.stage.set_skybox(skybox);
    }

    fn load_gltf_model(&mut self, bytes: &[u8]) {
        log::info!("loading gltf");
        self.phi = 0.0;
        self.theta = std::f32::consts::FRAC_PI_4;
        self.left_mb_down = false;
        self.last_cursor_position = None;
        self.stage.set_images(std::iter::empty()).unwrap();
        self.document = None;
        self.nodes = None;
        log::debug!("ticking stage to reclaim buffers");
        self.stage.tick();

        let mut doc = match self.stage.load_gltf_document_from_bytes(bytes) {
            Err(e) => {
                log::error!("gltf loading error: {e}");
                return;
            }
            Ok(doc) => doc,
        };

        // self.entities = builder.entities.clone();

        // find the bounding box of the model so we can display it correctly
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        let scene = doc.default_scene.unwrap_or(0);
        log::info!("Displaying scene {scene}");
        let nodes = doc
            .scenes
            .get(scene)
            .map(Vec::clone)
            .unwrap_or_else(|| (0..doc.nodes.len()).collect());
        log::trace!("  nodes:");
        for node_index in nodes.iter() {
            // UNWRAP: safe because we know the node exists
            let node = doc.nodes.get(*node_index).unwrap();
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
        let nodes = match self.stage.draw_gltf_scene(&doc, nodes, self.camera.id()) {
            Err(e) => {
                log::error!("could not draw scene: {e}");
                vec![]
            }
            Ok(ns) => ns,
        };
        if doc.animations.is_empty() {
            log::trace!("  animations: none");
        } else {
            log::trace!("  animations:");
        }
        let mut animated_nodes = HashSet::default();
        let mut has_conflicting_animations = false;
        self.animators = Some(
            std::mem::take(&mut doc.animations)
                .into_iter()
                .enumerate()
                .map(|(i, a)| {
                    let target_nodes = a.target_node_indices().collect::<HashSet<_>>();
                    has_conflicting_animations =
                        has_conflicting_animations || !animated_nodes.is_disjoint(&target_nodes);
                    animated_nodes.extend(target_nodes);

                    log::trace!("    {i} {:?} {}s", a.name, a.length_in_seconds());
                    for (t, tween) in a.tweens.iter().enumerate() {
                        log::trace!(
                            "      tween {t} targets node {} {}",
                            tween.target_node_index,
                            tween.properties.description()
                        );
                    }
                    Animator::new(&nodes, a)
                })
                .collect(),
        );
        if has_conflicting_animations {
            log::trace!("  and some animations conflict");
        }
        self.animations_conflict = has_conflicting_animations;
        self.nodes = Some(nodes);
        self.document = Some(doc);

        let halfway_point = min + ((max - min).normalize() * ((max - min).length() / 2.0));
        let length = min.distance(max);
        let radius = length * 1.25;

        self.radius = radius;
        self.eye = halfway_point;
        self.last_frame_instant = now();

        self.update_camera_view();
    }

    pub fn tick_loads(&mut self) {
        let loaded = std::mem::take(&mut *self.loads.lock().unwrap());
        for (path, bytes) in loaded.into_iter() {
            log::info!("loaded {}bytes from {}", bytes.len(), path.display());
            match is_file_supported(&path) {
                Some(SupportedFileType::Gltf) => self.load_gltf_model(&bytes),
                Some(SupportedFileType::Hdr) => self.load_hdr_skybox(bytes),
                None => {}
            }
        }
    }

    pub fn zoom(&mut self, delta: f32) {
        self.radius = (self.radius - (delta * RADIUS_SCROLL_DAMPENING)).max(0.0);
        self.update_camera_view();
    }

    pub fn pan(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        if self.left_mb_down {
            if let Some(last_cursor_position) = self.last_cursor_position.as_ref() {
                let dx = position.x - last_cursor_position.x;
                let dy = position.y - last_cursor_position.y;

                self.phi += dx as f32 * DX_DY_DRAG_DAMPENING;

                let next_theta = self.theta - dy as f32 * DX_DY_DRAG_DAMPENING;
                self.theta = next_theta.max(0.0001).min(std::f32::consts::PI);
            }
            self.last_cursor_position = Some(position);
            self.update_camera_view();
        }
    }

    pub fn mouse_button(&mut self, is_pressed: bool, is_left_button: bool) {
        if is_left_button {
            self.left_mb_down = is_pressed;
            if !self.left_mb_down {
                self.last_cursor_position = None;
            }
        }
    }

    // fn key_input(
    //     &mut self,
    //     r: &mut Context,
    //     KeyboardInput {
    //         state,
    //         virtual_keycode,
    //         ..
    //     }: winit::event::KeyboardInput,
    // ) {
    //     if matches!(state, winit::event::ElementState::Pressed) {
    //         return;
    //     }
    //     match virtual_keycode {
    //         Some(winit::event::VirtualKeyCode::Space) => {
    //             // clear all objects, cameras and lights
    //             let scene = r.new_scene().build().unwrap();
    //             r.graph.add_resource(scene);
    //             self.loader = None;
    //             self.ui
    //                 .set_text_title("awaiting drag and dropped `.gltf` or `.glb`
    // file");         }
    //         _ => {}
    //     };
    // }

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
                let bytes = std::fs::read(&path).unwrap();
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
