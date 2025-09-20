//! Camera projection, view and utilities.
use glam::{Mat4, Vec3};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod shader;

/// Returns the projection and view matrices for a camera with default
/// perspective.
///
/// The default projection and view matrices are defined as:
///
/// ```rust
/// use glam::*;
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
/// assert_eq!(renderling::camera::default_perspective(width, height), (projection, view));
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
    use crate::camera::shader::CameraDescriptor;

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
            let camera = CameraDescriptor::new(projection, view);

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
