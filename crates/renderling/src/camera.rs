//! Cameras, projections and utilities.
use async_channel::Sender;

use glam::{Mat4, Vec3};
use renderling_shader::CameraRaw;

use crate::{
    linkage::create_camera_uniform,
    resources::{Id, Shared},
};

/// Camera primitive shared by both user-land and under-the-hood camera data.
#[derive(Clone)]
pub struct CameraInner {
    pub(crate) camera: CameraRaw,
    pub(crate) dirty_uniform: bool,
}

impl CameraInner {
    pub fn new_ortho2d(width: f32, height: f32) -> Self {
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
        CameraInner {
            camera: CameraRaw { projection, view },
            dirty_uniform: false,
        }
    }

    pub fn new_perspective(width: f32, height: f32) -> Self {
        let aspect = width / height;
        let fovy = std::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
        let eye = Vec3::new(0.0, 12.0, 20.0);
        let target = Vec3::ZERO;
        let up = Vec3::Y;
        let view = Mat4::look_at_rh(eye, target, up);
        CameraInner {
            camera: CameraRaw { projection, view },
            dirty_uniform: false,
        }
    }
}

pub(crate) enum CameraUpdateCmd {
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
    pub(crate) device: &'a wgpu::Device,
    pub(crate) update_tx: Sender<CameraUpdateCmd>,
    pub(crate) scene: &'a mut crate::Stage,
    pub(crate) inner: CameraInner,
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
        let id = self.scene.new_camera_id();
        let CameraBuilder {
            device,
            update_tx: cmd,
            scene,
            width: _,
            height: _,
            inner,
        } = self;
        let (buffer, bindgroup) =
            create_camera_uniform(device, &inner.camera, "CameraBuilder::build");
        let inner = Shared::new(inner);
        let camera_data = CameraData {
            buffer,
            bindgroup,
            inner: inner.clone(),
        };
        let camera = Camera { id, cmd, inner };
        scene.cameras[id.0] = Some(camera_data);
        scene.should_sort = true;
        camera
    }
}
