//! Cameras, projections and utilities.
use std::sync::mpsc::Sender;

use nalgebra::{Matrix4, Perspective3, Point3, Vector3, Orthographic3, Isometry3};
use renderling_core::{ViewProjection, create_camera_uniform};

use crate::resources::{Id, Shared};

#[derive(Clone, Debug)]
pub enum Projection {
    Perspective(Perspective3<f32>),
    Orthographic(Orthographic3<f32>),
    Any(Matrix4<f32>),
}

impl Projection {
    pub fn to_homogeneous(&self) -> Matrix4<f32> {
        match self {
            Projection::Perspective(p) => p.to_homogeneous(),
            Projection::Orthographic(o) => o.to_homogeneous(),
            Projection::Any(m) => m.clone(),
        }
    }
}

impl From<&Projection> for Matrix4<f32> {
    fn from(value: &Projection) -> Self {
        value.to_homogeneous()
    }
}

/// Camera primitive shared by both user-land and under-the-hood camera data.
#[derive(Clone)]
pub struct CameraInner {
    pub(crate) view: Isometry3<f32>,
    pub(crate) projection: Projection,
    pub(crate) dirty_uniform: bool,
}

impl CameraInner {
    pub fn as_projection_and_view(&self) -> (Matrix4<f32>, Matrix4<f32>) {
        let projection = self.projection.to_homogeneous();
        let view = self.view.to_homogeneous();
        (projection, view)
    }

    pub fn new_ortho2d(width: f32, height: f32) -> Self {
        let left = 0.0;
        let right = width;
        let bottom = height;
        let top = 0.0;
        let near = 1.0;
        let far = -1.0;
        let projection = Projection::Orthographic(Orthographic3::new(left, right, bottom, top, near, far));
        let eye = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(0.0, 0.0, -1.0);
        let up = Vector3::y();
        let view = Isometry3::look_at_rh(&eye, &target, &up);
        CameraInner {
            projection,
            view,
            dirty_uniform: false,
        }
    }

    pub fn new_perspective(width: f32, height: f32) -> Self {
        let aspect = width / height;
        let fovy = std::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Projection::Perspective(Perspective3::new(aspect, fovy, znear, zfar));
        let eye = Point3::new(0.0, 12.0, 20.0);
        let target = Point3::origin();
        let up = Vector3::y();
        let view = Isometry3::look_at_rh(&eye, &target, &up);
        CameraInner {
            projection,
            view,
            dirty_uniform: false,
        }
    }
}

pub(crate) struct CameraUpdateCmd {
    pub(crate) camera_id: Id,
}

/// A user-land camera object.
///
/// Used to update various camera properties in renderlings.
#[derive(Clone)]
pub struct Camera {
    pub(crate) id: Id,
    pub(crate) inner: Shared<CameraInner>,
    pub(crate) cmd: Sender<CameraUpdateCmd>,
}

impl Camera {
    fn update(&self, f: impl FnOnce(&mut CameraInner)) {
        let mut inner = self.inner.write();
        f(&mut inner);
        if !inner.dirty_uniform {
            self.cmd
                .send(CameraUpdateCmd { camera_id: self.id })
                .unwrap();
            inner.dirty_uniform = true;
        }
    }

    /// Set the view.
    ///
    /// This is a combination of the camera's position and rotation.
    pub fn set_view(&self, view: Isometry3<f32>) {
        self.update(|inner| {
            inner.view = view;
        });
    }


    /// Set the view to a position and rotation that looks in a direction.
    pub fn look_at(&self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) {
        self.update(|inner| {
            inner.view = Isometry3::look_at_rh(&eye, &target, &up)
        });
    }

    /// Set the projection of the camera.
    pub fn set_projection(&self, projection: Projection) {
        self.update(|inner| {
            inner.projection = projection;
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

/// Create a new camera uniform
pub(crate) fn new_camera_uniform(
    inner: &CameraInner,
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::BindGroup) {
    let (projection, view) = inner.as_projection_and_view();
    let viewproj = ViewProjection {
        projection: projection.into(),
        view: view.into(),
    };
    create_camera_uniform(device, viewproj, "new_camera_uniform")
}

pub struct CameraBuilder<'a> {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) device: &'a wgpu::Device,
    pub(crate) update_tx: Sender<CameraUpdateCmd>,
    pub(crate) scene: &'a mut crate::Scene,
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

    /// Create a perspective 3d camera positioned at 0,12,20 looking at the origin.
    pub fn with_projection_perspective(mut self) -> Self {
        self.inner = CameraInner::new_perspective(self.width, self.height);
        self
    }

    /// Set the projection.
    pub fn with_projection(mut self, projection: Projection) -> Self {
        self.inner.projection = projection;
        self
    }

    /// Set the view.
    pub fn with_view(mut self, view: Isometry3<f32>) -> Self {
        self.inner.view = view;
        self
    }

    /// Set the view to a position and rotation that looks in a direction.
    pub fn with_look_at(mut self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Self {
        self.inner.view = Isometry3::look_at_rh(&eye, &target, &up);
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
        let (buffer, bindgroup) = new_camera_uniform(&inner, device);
        let inner = Shared::new(inner);
        let camera_data = CameraData {
            buffer,
            bindgroup,
            inner: inner.clone(),
        };
        let camera = Camera { id, cmd, inner };
        scene.cameras.push((id, camera_data));
        camera
    }
}
