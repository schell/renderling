//! Cameras, projections and utilities.
use std::ops::{Deref, DerefMut};

use async_channel::{unbounded, Receiver, Sender};
use glam::{Mat4, Vec3};
use moongraph::{Edges, Read, Write, Move};
use renderling_shader::CameraRaw;

use crate::{
    bank::Bank,
    linkage::create_camera_uniform,
    resources::{Id, Shared},
    Object, Objects, Renderling, Device, WgpuStateError,
};

/// Returns the projection and view matrices for a camera with default perspective.
pub fn default_perspective(width: f32, height: f32) -> (Mat4, Mat4) {
    let aspect = width / height;
    let fovy = std::f32::consts::PI / 4.0;
    let znear = 0.1;
    let zfar = 100.0;
    let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
    let eye = Vec3::new(0.0, 12.0, 20.0);
    let target = Vec3::ZERO;
    let up = Vec3::Y;
    let view = Mat4::look_at_rh(eye, target, up);
    (projection, view)
}

/// Creates a typical 2d orthographic projection with +Y extending downward
/// and the Z axis coming out towards the viewer.
pub fn default_ortho2d(width: f32, height: f32) -> (Mat4, Mat4) {
    let left = 0.0;
    let right = width;
    let bottom = height;
    let top = 0.0;
    let near = 1.0;
    let far = -1.0;
    let projection = Mat4::orthographic_rh(left, right, bottom, top, near, far);
    let eye = Vec3::new(0.0, 0.0, 0.0);
    let target = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let view = Mat4::look_at_rh(eye, target, up);
    (projection, view)
}

/// Camera primitive shared by both user-land and under-the-hood camera data.
#[derive(Clone)]
pub struct CameraInner {
    pub(crate) camera: CameraRaw,
    pub(crate) dirty_uniform: bool,
}

impl CameraInner {
    pub fn new_ortho2d(width: f32, height: f32) -> Self {
        let (projection, view) = default_ortho2d(width, height);
        CameraInner {
            camera: CameraRaw { projection, view },
            dirty_uniform: false,
        }
    }

    pub fn new_perspective(width: f32, height: f32) -> Self {
        let (projection, view) = default_perspective(width, height);
        CameraInner {
            camera: CameraRaw { projection, view },
            dirty_uniform: false,
        }
    }
}

pub enum CameraUpdateCmd {
    Update { camera_id: Id<Camera> },
    Destroy { camera_id: Id<Camera> },
}

/// A user-land camera object.
///
/// Used to update various camera properties in renderlings.
///
/// Dropping this struct will result in its GPU resources being cleaned up
/// and/or recycled.
#[derive(Clone)]
pub struct Camera {
    pub(crate) id: Id<Camera>,
    pub(crate) inner: Shared<CameraInner>,
    pub(crate) cmd: Sender<CameraUpdateCmd>,
}

impl Drop for Camera {
    fn drop(&mut self) {
        if self.inner.count() <= 1 {
            let _ = self
                .cmd
                .send(CameraUpdateCmd::Destroy { camera_id: self.id });
        }
    }
}

impl Camera {
    fn update(&self, f: impl FnOnce(&mut CameraInner)) {
        let mut inner = self.inner.write();
        f(&mut inner);
        if !inner.dirty_uniform {
            self.cmd
                .try_send(CameraUpdateCmd::Update { camera_id: self.id })
                .unwrap();
            inner.dirty_uniform = true;
        }
    }

    /// Get the id of this camera.
    pub fn get_id(&self) -> usize {
        *self.id
    }

    /// Set the view.
    ///
    /// This is a combination of the camera's position and rotation.
    pub fn set_view(&self, view: Mat4) {
        self.update(|inner| {
            inner.camera.view = view;
        });
    }

    /// Set the view to a position and rotation that looks in a direction.
    pub fn look_at(&self, eye: Vec3, target: Vec3, up: Vec3) {
        self.update(|inner| inner.camera.view = Mat4::look_at_rh(eye, target, up));
    }

    /// Set the projection of the camera.
    pub fn set_projection(&self, projection: Mat4) {
        self.update(|inner| {
            inner.camera.projection = projection;
        });
    }

    /// Destroy the camera.
    pub fn destroy(self) {
        // UNWRAP: safe because the channel is unbounded
        self.cmd.try_send(CameraUpdateCmd::Destroy { camera_id: self.id }).unwrap();
    }
}

/// Under-the-hood camera data.
///
/// Used by renderlings to update uniforms, etc.
pub struct CameraData {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bindgroup: wgpu::BindGroup,
    pub(crate) inner: Shared<CameraInner>,
}

pub struct CameraBuilder<'a> {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) inner: CameraInner,
    pub(crate) renderer: &'a mut Renderling
}

impl<'a> CameraBuilder<'a> {
    /// Create an orthographic 2d camera with a projection where the x axis
    /// increases to the right, the y axis increases down and z increases
    /// out of the screen towards the viewer.
    pub fn with_projection_ortho2d(mut self) -> Self {
        self.inner = CameraInner::new_ortho2d(self.width, self.height);
        self
    }

    /// Create a perspective 3d camera positioned at 0,12,20 looking at the
    /// origin.
    pub fn with_projection_perspective(mut self) -> Self {
        self.inner = CameraInner::new_perspective(self.width, self.height);
        self
    }

    /// Set the projection.
    pub fn with_projection(mut self, projection: Mat4) -> Self {
        self.inner.camera.projection = projection;
        self
    }

    /// Set the view.
    pub fn with_view(mut self, view: Mat4) -> Self {
        self.inner.camera.view = view;
        self
    }

    /// Set the view to a position and rotation that looks in a direction.
    pub fn with_look_at(mut self, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        self.inner.camera.view = Mat4::look_at_rh(eye, target, up);
        self
    }

    pub fn build(self) -> Camera {
        let CameraBuilder {
            width: _,
            height: _,
            inner,
            renderer
        } = self;
        let (buffer, bindgroup) =
            create_camera_uniform(renderer.get_device(), &inner.camera, "CameraBuilder::build");
        let inner = Shared::new(inner);
        let camera_data = CameraData {
            buffer,
            bindgroup,
            inner: inner.clone(),
        };

        let cameras = renderer.get_cameras_mut();
        let id = cameras.bank.insert_with(move |_| camera_data);
        let camera = Camera {
            id: id.into(),
            cmd: cameras.update_channel(),
            inner,
        };
        cameras.should_sort = true;
        camera
    }
}

impl From<Id<CameraData>> for Id<Camera> {
    fn from(value: Id<CameraData>) -> Self {
        Id::new(*value)
    }
}

impl From<Id<Camera>> for Id<CameraData> {
    fn from(value: Id<Camera>) -> Self {
        Id::new(*value)
    }
}

pub struct Cameras {
    bank: Bank<CameraData>,
    // all object ids, sorted by distance to camera
    camera_objects_by_distance: Vec<Vec<Id<Object>>>,
    // whether we need to sort on the next update
    should_sort: bool,
    // queue/channel of updates from cameras to make before the next render
    pub(crate) camera_update_queue: (Sender<CameraUpdateCmd>, Receiver<CameraUpdateCmd>),
}

impl Default for Cameras {
    fn default() -> Self {
        Self {
            bank: Default::default(),
            camera_objects_by_distance: Default::default(),
            should_sort: Default::default(),
            camera_update_queue: unbounded(),
        }
    }
}

impl Deref for Cameras {
    type Target = Bank<CameraData>;

    fn deref(&self) -> &Self::Target {
        &self.bank
    }
}

impl DerefMut for Cameras {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bank
    }
}

impl Cameras {
    pub(crate) fn update_queue(&self) -> Sender<CameraUpdateCmd> {
        self.camera_update_queue.0.clone()
    }

    /// Sorts objects by their distance to each camera.
    ///
    /// This also generates the lists of objects sorted per camera.
    pub fn sort_objects(&mut self, objects: &Objects) {
        log::trace!("sorting objects by distance to camera");
        self.should_sort = false;

        let mut sorted = vec![];
        struct Sorter {
            object_id: Id<Object>,
            distance: f32,
        }
        for camera in self.bank.iter() {
            if let Some(cam_data) = camera {
                let cam_pos = cam_data
                    .inner
                    .read()
                    .camera
                    .view
                    .project_point3(Vec3::default());
                let mut objects = objects
                    .iter()
                    .enumerate()
                    .filter_map(|(i, obj_data)| {
                        let obj_data = obj_data.as_ref()?;
                        let object_id = Id::new(i);
                        let distance = obj_data.world_position.distance(cam_pos);
                        Some(Sorter {
                            object_id,
                            distance,
                        })
                    })
                    .collect::<Vec<_>>();
                // we want to sort back to front
                objects.sort_by(|a, b| b.distance.total_cmp(&a.distance));
                sorted.push(objects.into_iter().map(|s| s.object_id).collect::<Vec<_>>());
            }
        }
        self.camera_objects_by_distance = sorted;
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<&CameraData>> {
        self.bank.iter()
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        camera_objects_need_sorting: bool,
        objects: &Objects,
    ) {
        self.should_sort = self.should_sort || camera_objects_need_sorting;
        while let Ok(cmd) = self.camera_update_queue.1.try_recv() {
            match cmd {
                CameraUpdateCmd::Destroy { camera_id } => {
                    log::debug!("destroying {:?}", camera_id);
                    self.bank.destroy(camera_id.into());
                }
                CameraUpdateCmd::Update { camera_id } => {
                    self.should_sort = true;
                    if let Some(camera_data) = self.bank.get_mut(&camera_id.into()) {
                        let mut inner = camera_data.inner.write();
                        let (buffer, bindgroup) =
                            create_camera_uniform(device, &inner.camera, "Renderling::update");
                        inner.dirty_uniform = false;
                        camera_data.buffer = buffer;
                        camera_data.bindgroup = bindgroup;
                    }
                }
            }
        }

        if self.should_sort {
            self.sort_objects(objects);
        }
    }

    pub fn update_channel(&self) -> Sender<CameraUpdateCmd> {
        self.camera_update_queue.0.clone()
    }

    pub fn get_object_ids_sorted_by_distance_to_camera(&self, camera_id: &Id<CameraData>) -> &Vec<Id<Object>> {
        &self.camera_objects_by_distance[camera_id.0]
    }
}

/// Wrapper type to be used as a result of the ObjectUpdate node.
pub struct CamerasNeedSorting(pub bool);

#[derive(Edges)]
pub struct CameraUpdate {
    device: Read<Device>,
    camera_objects_need_sorting: Move<CamerasNeedSorting>,
    cameras: Write<Cameras>,
    objects: Read<Objects>,
}

impl CameraUpdate {
    pub fn run(mut self) -> Result<(), WgpuStateError> {
        self.cameras.update(&self.device, self.camera_objects_need_sorting.0, &self.objects);
        Ok(())
    }
}
