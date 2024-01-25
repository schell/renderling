//! Camera projections and utilities.
// TODO: remove the renderling::camera module
use glam::{Mat4, Vec3};

/// Returns the projection and view matrices for a camera with default
/// perspective.
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
    let fovy = std::f32::consts::PI / 4.0;
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
/// and the Z axis coming out towards the viewer.
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
