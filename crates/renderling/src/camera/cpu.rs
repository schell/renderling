//! CPU side of [crate::camera].

use craballoc::{runtime::IsRuntime, slab::SlabAllocator, value::Hybrid};
use crabslab::Id;

use crate::camera::shader::CameraDescriptor;

use super::*;

/// A camera used for transforming the stage during rendering.
///
/// * Use [`Stage::new_camera`](crate::stage::Stage::new_camera) to create a new camera.
/// * Use [`Stage::use_camera`](crate::stage::Stage::use_camera) to set a camera on the stage.
///
/// ## Note
///
/// Clones of this type all point to the same underlying data.
#[derive(Clone, Debug)]
pub struct Camera {
    inner: Hybrid<CameraDescriptor>,
}

impl AsRef<Camera> for Camera {
    fn as_ref(&self) -> &Camera {
        self
    }
}

impl Camera {
    /// Stage a new camera on the given slab.
    pub fn new(slab: &SlabAllocator<impl IsRuntime>) -> Self {
        Self {
            inner: slab.new_value(CameraDescriptor::default()),
        }
    }

    /// Returns a pointer to the underlying descriptor on the GPU.
    pub fn id(&self) -> Id<CameraDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> CameraDescriptor {
        self.inner.get()
    }

    /// Set the camera to a default perspective projection and view based
    /// on the width and height of the viewport.
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
    pub fn set_default_perspective(&self, width: f32, height: f32) -> &Self {
        self.inner
            .modify(|d| *d = CameraDescriptor::default_perspective(width, height));
        self
    }

    /// Set the camera to a default perspective projection and view based
    /// on the width and height of the viewport.
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
    pub fn with_default_perspective(self, width: f32, height: f32) -> Self {
        self.set_default_perspective(width, height);
        self
    }

    /// Set the camera to a default orthographic 2d projection and view based
    /// on the width and height of the viewport.
    pub fn set_default_ortho2d(&self, width: f32, height: f32) -> &Self {
        self.inner
            .modify(|d| *d = CameraDescriptor::default_ortho2d(width, height));
        self
    }

    /// Set the camera to a default orthographic 2d projection and view based
    /// on the width and height of the viewport.
    pub fn with_default_ortho2d(self, width: f32, height: f32) -> Self {
        self.set_default_ortho2d(width, height);
        self
    }

    /// Set the projection and view matrices of this camera.
    pub fn set_projection_and_view(
        &self,
        projection: impl Into<Mat4>,
        view: impl Into<Mat4>,
    ) -> &Self {
        self.inner
            .modify(|d| d.set_projection_and_view(projection.into(), view.into()));
        self
    }

    /// Set the projection and view matrices and return this camera.
    pub fn with_projection_and_view(
        self,
        projection: impl Into<Mat4>,
        view: impl Into<Mat4>,
    ) -> Self {
        self.set_projection_and_view(projection, view);
        self
    }

    /// Returns the projection and view matrices.
    pub fn projection_and_view(&self) -> (Mat4, Mat4) {
        let d = self.inner.get();
        (d.projection(), d.view())
    }

    /// Set the projection matrix of this camera.
    pub fn set_projection(&self, projection: impl Into<Mat4>) -> &Self {
        self.inner.modify(|d| d.set_projection(projection.into()));
        self
    }

    /// Set the projection matrix and return this camera.
    pub fn with_projection(self, projection: impl Into<Mat4>) -> Self {
        self.set_projection(projection);
        self
    }

    /// Returns the projection matrix.
    pub fn projection(&self) -> Mat4 {
        self.inner.get().projection()
    }

    /// Set the view matrix of this camera.
    pub fn set_view(&self, view: impl Into<Mat4>) -> &Self {
        self.inner.modify(|d| d.set_view(view.into()));
        self
    }

    /// Set the view matrix and return this camera.
    pub fn with_view(self, view: impl Into<Mat4>) -> Self {
        self.set_view(view);
        self
    }

    /// Returns the view matrix.
    pub fn view(&self) -> Mat4 {
        self.inner.get().view()
    }
}

#[cfg(test)]
mod test {
    use craballoc::{runtime::CpuRuntime, slab::SlabAllocator};

    use super::*;

    #[test]
    fn camera_position_sanity() {
        let slab = SlabAllocator::new(CpuRuntime, "camera test", ());
        let camera = Camera::new(&slab);
        let projection = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.01, 10.0);
        let view = Mat4::look_at_rh(Vec3::ONE, Vec3::ZERO, Vec3::Y);
        camera.set_projection_and_view(projection, view);
        let position = camera.descriptor().position();
        assert_eq!(Vec3::ONE, position);
    }
}
