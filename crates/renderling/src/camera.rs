//! Camera projection, view and utilities.
use crabslab::SlabItem;
use glam::{Mat4, Vec2, Vec3, Vec4};

use crate::{
    bvol::{dist_bpp, Frustum},
    math::IsVector,
};

/// A camera used for transforming the stage during rendering.
///
/// Use [`Camera::new`] to create a new camera.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq, SlabItem)]
pub struct Camera {
    projection: Mat4,
    view: Mat4,
    position: Vec3,
    frustum: Frustum,
    /// Nearest center point on the frustum
    z_near_point: Vec3,
    /// Furthest center point on the frustum
    z_far_point: Vec3,
}

impl Camera {
    pub fn new(projection: Mat4, view: Mat4) -> Self {
        Camera::default().with_projection_and_view(projection, view)
    }

    pub fn default_perspective(width: f32, height: f32) -> Self {
        let (projection, view) = default_perspective(width, height);
        Camera::new(projection, view)
    }

    pub fn default_ortho2d(width: f32, height: f32) -> Self {
        let (projection, view) = default_ortho2d(width, height);
        Camera::new(projection, view)
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
        dist_bpp(&self.frustum.planes[0], self.position)
    }

    pub fn z_far(&self) -> f32 {
        dist_bpp(&self.frustum.planes[5], self.position)
    }

    pub fn depth(&self) -> f32 {
        self.frustum.depth()
    }

    #[warn(soft_unstable, reason = "depth linearization is currently unreliable")]
    pub fn linearize_depth(&self, d: f32) -> f32 {
        // TODO: figure out why z_near and z_far come out equal in ortho
        let z_near = self.z_near();
        let z_far = self.z_far();
        z_near * z_far / (z_far + d * (z_near - z_far))
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

/// Returns the projection and view matrices for a camera with default
/// perspective.
///
/// The default projection and view matrices are defined as:
///
/// ```rust
/// use glam::*;
/// use renderling::prelude::*;
///
/// let width = 800.0;
/// let height = 600.0;
/// let aspect = width / height;
/// let fovy = core::f32::consts::PI / 4.0;
/// let znear = 0.1;
/// let zfar = 100.0;
/// let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
/// let eye = Vec3::new(0.0, 12.0, 20.0);
/// let target = Vec3::ZERO;
/// let up = Vec3::Y;
/// let view = Mat4::look_at_rh(eye, target, up);
/// assert_eq!(default_perspective(width, height), (projection, view));
/// ```
pub fn default_perspective(width: f32, height: f32) -> (Mat4, Mat4) {
    let projection = perspective(width, height);
    let eye = Vec3::new(0.0, 12.0, 20.0);
    let target = Vec3::ZERO;
    let up = Vec3::Y;
    let view = Mat4::look_at_rh(eye, target, up);
    (projection, view)
}

pub fn perspective(width: f32, height: f32) -> Mat4 {
    let aspect = width / height;
    let fovy = core::f32::consts::PI / 4.0;
    let znear = 0.1;
    let zfar = 100.0;
    Mat4::perspective_rh(fovy, aspect, znear, zfar)
}

pub fn ortho(width: f32, height: f32) -> Mat4 {
    let left = 0.0;
    let right = width;
    let bottom = height;
    let top = 0.0;
    let near = -1.0;
    let far = 1.0;
    Mat4::orthographic_rh(left, right, bottom, top, near, far)
}

pub fn look_at(eye: impl Into<Vec3>, target: impl Into<Vec3>, up: impl Into<Vec3>) -> Mat4 {
    Mat4::look_at_rh(eye.into(), target.into(), up.into())
}

/// Creates a typical 2d orthographic projection with +Y extending downward
/// and the +Z axis coming out towards the viewer.
pub fn default_ortho2d(width: f32, height: f32) -> (Mat4, Mat4) {
    let left = 0.0;
    let right = width;
    let bottom = height;
    let top = 0.0;
    let near = -1.0;
    let far = 1.0;
    let projection = Mat4::orthographic_rh(left, right, bottom, top, near, far);
    let view = Mat4::IDENTITY;
    (projection, view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn forward() {
        let eyes = [Vec3::new(0.0, 0.0, 5.0), Vec3::new(250.0, 200.0, 250.0)];

        let expected_forwards = [
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(-0.6154574, -0.49236593, -0.6154574),
        ];

        for (eye, expected_forward) in eyes.into_iter().zip(expected_forwards) {
            let projection = Mat4::perspective_rh(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);
            let view = Mat4::look_at_rh(eye, Vec3::ZERO, Vec3::Y);
            let camera = Camera::new(projection, view);

            let forward = camera.forward();
            let distance = forward.distance(expected_forward);
            const THRESHOLD: f32 = 1e-3;
            assert!(
                distance < THRESHOLD,
                "Forward vector is incorrect\n\
                forward: {forward}\n\
                expected: {expected_forward}\n\
                distance: {distance}, threshold: {THRESHOLD}"
            );
        }
    }
}
