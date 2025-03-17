//! CPU side of the [super::geometry](geometry) module.
//!

use craballoc::{
    runtime::WgpuRuntime,
    slab::SlabAllocator,
    value::{Hybrid, HybridArray},
};
use crabslab::{Array, Id};
use glam::{Mat4, UVec2};

use crate::{
    camera::Camera,
    geometry::GeometryDescriptor,
    pbr::Material,
    prelude::Transform,
    stage::{MorphTarget, Renderlet, Skin, Vertex},
};

// TODO: Move `Renderlet` to geometry.

/// A helper struct to build [`Renderlet`]s in the [`Geometry`] manager.
pub struct RenderletBuilder<'a> {
    data: Renderlet,
    geometry: &'a Geometry,
}

impl<'a> RenderletBuilder<'a> {
    pub fn new(geometry: &'a Geometry) -> Self {
        RenderletBuilder {
            data: Renderlet::default(),
            geometry,
        }
    }

    pub fn with_vertices(
        mut self,
        vertices: impl IntoIterator<Item = Vertex>,
    ) -> (Self, HybridArray<Vertex>) {
        let vertices = self.geometry.new_vertices(vertices);
        self.data.vertices_array = vertices.array();
        (self, vertices)
    }

    pub fn with_indices(
        mut self,
        indices: impl IntoIterator<Item = u32>,
    ) -> (Self, HybridArray<u32>) {
        let indices = self.geometry.new_indices(indices);
        self.data.indices_array = indices.array();
        (self, indices)
    }

    pub fn with_transform_id(mut self, transform_id: Id<Transform>) -> Self {
        self.data.transform_id = transform_id;
        self
    }

    pub fn with_material_id(mut self, material_id: Id<Material>) -> Self {
        self.data.material_id = material_id;
        self
    }

    pub fn with_skin_id(mut self, skin_id: Id<Skin>) -> Self {
        self.data.skin_id = skin_id;
        self
    }

    pub fn with_morph_targets(
        mut self,
        morph_targets: impl IntoIterator<Item = Array<MorphTarget>>,
    ) -> (Self, HybridArray<Array<MorphTarget>>) {
        let morph_targets = morph_targets.into_iter();
        let morph_targets = self.geometry.slab.new_array(morph_targets);
        self.data.morph_targets = morph_targets.array();
        (self, morph_targets)
    }

    pub fn with_morph_weights(
        mut self,
        morph_weights: impl IntoIterator<Item = f32>,
    ) -> (Self, HybridArray<f32>) {
        let morph_weights = self.geometry.new_weights(morph_weights);
        self.data.morph_weights = morph_weights.array();
        (self, morph_weights)
    }

    pub fn with_geometry_descriptor_id(mut self, pbr_config_id: Id<GeometryDescriptor>) -> Self {
        self.data.geometry_descriptor_id = pbr_config_id;
        self
    }

    pub fn build(self) -> Hybrid<Renderlet> {
        let RenderletBuilder { data, geometry } = self;
        geometry.new_renderlet(data)
    }
}

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
        let slab =
            SlabAllocator::new_with_label(runtime, wgpu::BufferUsages::empty(), Some("geometry"));
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

    /// Create a new camera.
    ///
    /// If this is the first camera created, it will be automatically used.
    pub fn new_camera(&self, camera: Camera) -> Hybrid<Camera> {
        let c = self.slab.new_value(camera);
        if self.descriptor.get().camera_id.is_none() {
            log::info!("automatically using camera: {:?}", c.id());
            self.descriptor.modify(|cfg| {
                cfg.camera_id = c.id();
            });
        }
        c
    }

    /// Set all geometry to use the given camera.
    pub fn use_camera(&self, camera: &Hybrid<Camera>) {
        self.descriptor.modify(|cfg| cfg.camera_id = camera.id());
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
