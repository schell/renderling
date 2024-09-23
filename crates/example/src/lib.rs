//! Runs through all the gltf sample models to test and show-off renderling's
//! gltf capabilities.
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use renderling::{
    atlas::AtlasImage,
    bvol::Aabb,
    camera::Camera,
    math::{Mat4, UVec2, Vec3, Vec4},
    pbr::light::{DirectionalLight, Light},
    skybox::Skybox,
    slab::Hybrid,
    stage::{Animator, GltfDocument, Stage},
    Context,
};

pub mod camera;
use camera::{
    CameraControl, CameraController, TurntableCameraController, WasdMouseCameraController,
};

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
    _light: Option<(Hybrid<DirectionalLight>, Hybrid<Light>)>,

    document: Option<GltfDocument>,
    animators: Option<Vec<Animator>>,
    animations_conflict: bool,

    pub camera_controller: Box<dyn CameraController>,
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

    pub fn update_view(&mut self) {
        self.camera_controller
            .update_camera(self.stage.get_size(), &self.camera);
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
            .update_camera(self.stage.get_size(), &self.camera);

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
        self.camera_controller.tick();
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
