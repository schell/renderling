//! [`CameraDescriptor`] and camera shader utilities.

use crabslab::SlabItem;
use glam::{Mat4, Vec2, Vec3, Vec4};

use crate::{bvol::Frustum, math::IsVector};

/// GPU descriptor of a camera.
///
/// Used for transforming the stage during rendering.
///
/// Use [`CameraDescriptor::new`] to create a new camera.
#[derive(Default, Clone, Copy, PartialEq, SlabItem, core::fmt::Debug)]
pub struct CameraDescriptor {
    projection: Mat4,
    view: Mat4,
    position: Vec3,
    frustum: Frustum,
    /// Nearest center point on the frustum
    z_near_point: Vec3,
    /// Furthest center point on the frustum
    z_far_point: Vec3,
}

impl CameraDescriptor {
    pub fn new(projection: Mat4, view: Mat4) -> Self {
        CameraDescriptor::default().with_projection_and_view(projection, view)
    }

    pub fn default_perspective(width: f32, height: f32) -> Self {
        let (projection, view) = super::default_perspective(width, height);
        CameraDescriptor::new(projection, view)
    }

    pub fn default_ortho2d(width: f32, height: f32) -> Self {
        let (projection, view) = super::default_ortho2d(width, height);
        CameraDescriptor::new(projection, view)
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    pub fn set_projection_and_view(&mut self, projection: Mat4, view: Mat4) {
        self.projection = projection;
        self.view = view;
        self.position = view.inverse().transform_point3(Vec3::ZERO);
        let inverse = (projection * view).inverse();
        self.z_near_point = inverse.project_point3(Vec3::ZERO);
        self.z_far_point = inverse.project_point3(Vec2::ZERO.extend(1.0));
        self.frustum = Frustum::from_camera(self);
    }

    pub fn with_projection_and_view(mut self, projection: Mat4, view: Mat4) -> Self {
        self.set_projection_and_view(projection, view);
        self
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.set_projection_and_view(projection, self.view);
    }

    pub fn with_projection(mut self, projection: Mat4) -> Self {
        self.set_projection(projection);
        self
    }

    pub fn view(&self) -> Mat4 {
        self.view
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.set_projection_and_view(self.projection, view);
    }

    pub fn with_view(mut self, view: Mat4) -> Self {
        self.set_view(view);
        self
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn frustum(&self) -> Frustum {
        self.frustum
    }

    pub fn view_projection(&self) -> Mat4 {
        self.projection * self.view
    }

    pub fn near_plane(&self) -> Vec4 {
        self.frustum.planes[0]
    }

    pub fn far_plane(&self) -> Vec4 {
        self.frustum.planes[5]
    }

    /// Returns **roughly** the location of the znear plane.
    pub fn z_near(&self) -> f32 {
        self.z_near_point.distance(self.position)
    }

    pub fn z_far(&self) -> f32 {
        self.z_far_point.distance(self.position)
    }

    pub fn depth(&self) -> f32 {
        (self.z_far() - self.z_near()).abs()
    }

    /// Returns the normalized forward vector which points in the direction the camera is looking.
    pub fn forward(&self) -> Vec3 {
        (self.z_far_point - self.z_near_point).alt_norm_or_zero()
    }

    pub fn frustum_near_point(&self) -> Vec3 {
        self.forward() * self.z_near()
    }

    pub fn frustum_far_point(&self) -> Vec3 {
        self.forward() * self.z_far()
    }
}
