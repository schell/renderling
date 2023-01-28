//! Cameras, projections and utilities.
use std::sync::mpsc::Sender;

use nalgebra::{Matrix4, Perspective3, Point3, Unit, UnitVector3, Vector3, Orthographic3};

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
    pub(crate) position: Point3<f32>,
    pub(crate) look_at: Point3<f32>,
    pub(crate) up: Unit<Vector3<f32>>,
    pub(crate) projection: Projection,
    pub(crate) dirty_uniform: bool,
}

impl CameraInner {
    pub fn as_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.look_at, &self.up)
    }

    pub fn as_projection_and_view(&self) -> (Matrix4<f32>, Matrix4<f32>) {
        let projection = self.projection.to_homogeneous();
        let view = Matrix4::look_at_rh(&self.position, &self.look_at, &self.up);
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
        let position = Point3::from([0.0, 0.0, 0.0]);
        let look_at = Point3::from([0.0, 0.0, -1.0]);
        let up = Vector3::y_axis();
        CameraInner {
            position,
            projection,
            look_at,
            up,
            dirty_uniform: false,
        }
    }

    pub fn new_perspective(width: f32, height: f32) -> Self {
        let aspect = width / height;
        let fovy = std::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Projection::Perspective(Perspective3::new(aspect, fovy, znear, zfar));
        //    Projection::Perspective {
        //};
        let position = Point3::new(0.0, 12.0, 20.0);
        let look_at = Point3::origin();
        let up = Vector3::y_axis();
        CameraInner {
            position,
            projection,
            look_at,
            up,
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

    pub fn set_position(&self, position: Point3<f32>) {
        self.update(|inner| {
            inner.position = position;
        });
    }

    pub fn look_at(&self, p: Point3<f32>) {
        self.update(|inner| {
            inner.look_at = p;
        });
    }

    pub fn set_up(&self, up: UnitVector3<f32>) {
        self.update(|inner| {
            inner.up = up;
        });
    }

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
