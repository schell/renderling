//! CPU side of the [super::geometry](geometry) module.
//!

use craballoc::{
    runtime::WgpuRuntime,
    slab::{SlabAllocator, SlabBuffer},
    value::{Hybrid, HybridArray},
};
use crabslab::{Array, Id};
use glam::{Mat4, UVec2};

use crate::{
    camera::Camera,
    geometry::GeometryDescriptor,
    prelude::Transform,
    stage::{MorphTarget, Renderlet, Skin, Vertex},
};

// TODO: Move `Renderlet` to geometry.

/// Wrapper around the geometry slab, which holds mesh data and more.
#[derive(Clone)]
pub struct Geometry {
    slab: SlabAllocator<WgpuRuntime>,
    descriptor: Hybrid<GeometryDescriptor>,
}

impl AsRef<WgpuRuntime> for Geometry {
    fn as_ref(&self) -> &WgpuRuntime {
        self.slab.runtime()
    }
}

impl AsRef<SlabAllocator<WgpuRuntime>> for Geometry {
    fn as_ref(&self) -> &SlabAllocator<WgpuRuntime> {
        &self.slab
    }
}

impl Geometry {
    // TODO: move atlas size into materials.
    pub fn new(runtime: impl AsRef<WgpuRuntime>, resolution: UVec2, atlas_size: UVec2) -> Self {
        let runtime = runtime.as_ref();
        let slab = SlabAllocator::new(runtime, "geometry", wgpu::BufferUsages::empty());
        let descriptor = slab.new_value(GeometryDescriptor {
            atlas_size,
            resolution,
            ..Default::default()
        });
        Self { slab, descriptor }
    }

    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    pub fn slab_allocator(&self) -> &SlabAllocator<WgpuRuntime> {
        self.as_ref()
    }

    pub fn descriptor(&self) -> &Hybrid<GeometryDescriptor> {
        &self.descriptor
    }

    #[must_use]
    pub fn commit(&self) -> SlabBuffer<wgpu::Buffer> {
        self.slab.commit()
    }

    /// Create a new camera.
    ///
    /// If this is the first camera created, it will be automatically used.
    pub fn new_camera(&self, camera: Camera) -> Hybrid<Camera> {
        let c = self.slab.new_value(camera);
        if self.descriptor.get().camera_id.is_none() {
            self.use_camera(&c);
        }
        c
    }

    /// Set all geometry to use the given camera.
    pub fn use_camera(&self, camera: impl AsRef<Hybrid<Camera>>) {
        let c = camera.as_ref();
        log::info!("using camera: {:?}", c.id());
        self.descriptor.modify(|cfg| cfg.camera_id = c.id());
    }

    pub fn new_transform(&self, transform: Transform) -> Hybrid<Transform> {
        self.slab.new_value(transform)
    }

    /// Create new geometry data.
    // TODO: Move `Vertex` to geometry.
    pub fn new_vertices(&self, data: impl IntoIterator<Item = Vertex>) -> HybridArray<Vertex> {
        self.slab.new_array(data)
    }

    /// Create new indices that point to offsets of an array of vertices.
    pub fn new_indices(&self, data: impl IntoIterator<Item = u32>) -> HybridArray<u32> {
        self.slab.new_array(data)
    }

    /// Create new morph targets.
    // TODO: Move `MorphTarget` to geometry.
    pub fn new_morph_targets(
        &self,
        data: impl IntoIterator<Item = MorphTarget>,
    ) -> HybridArray<MorphTarget> {
        self.slab.new_array(data)
    }

    /// Create an array of morph target arrays.
    pub fn new_morph_targets_array(
        &self,
        data: impl IntoIterator<Item = Array<MorphTarget>>,
    ) -> HybridArray<Array<MorphTarget>> {
        let morph_targets = data.into_iter();
        self.slab.new_array(morph_targets)
    }

    /// Create new morph target weights.
    pub fn new_weights(&self, data: impl IntoIterator<Item = f32>) -> HybridArray<f32> {
        self.slab.new_array(data)
    }

    /// Create a new array of joint transform ids that each point to a [`Transform`].
    pub fn new_joint_transform_ids(
        &self,
        data: impl IntoIterator<Item = Id<Transform>>,
    ) -> HybridArray<Id<Transform>> {
        self.slab.new_array(data)
    }

    /// Create a new array of matrices.
    pub fn new_matrices(&self, data: impl IntoIterator<Item = Mat4>) -> HybridArray<Mat4> {
        self.slab.new_array(data)
    }

    /// Create a new skin.
    // TODO: move `Skin` to geometry.
    pub fn new_skin(&self, skin: Skin) -> Hybrid<Skin> {
        self.slab.new_value(skin)
    }

    pub fn new_renderlet(&self, renderlet: Renderlet) -> Hybrid<Renderlet> {
        self.slab.new_value(renderlet)
    }
}
