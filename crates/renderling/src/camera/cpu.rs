//! CPU side of [crate::camera].

use craballoc::value::Hybrid;
use crabslab::Id;

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
    /// Stage a new camera on the GPU.
    pub fn new(geometry: &crate::geometry::Geometry) -> Self {
        Self {
            inner: geometry
                .slab_allocator()
                .new_value(CameraDescriptor::default()),
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
        (d.projection, d.view())
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
        self.inner.get().projection
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
