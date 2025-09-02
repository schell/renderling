//! CPU side of materials.

use craballoc::{
    runtime::WgpuRuntime,
    slab::{SlabAllocator, SlabBuffer},
    value::{Hybrid, HybridArray},
};

use crate::{atlas::Atlas, material::MaterialDescriptor};

/// Wrapper around the materials slab, which holds material textures in an atlas.
#[derive(Clone)]
pub struct Materials {
    slab: SlabAllocator<WgpuRuntime>,
    atlas: Atlas,
}

impl AsRef<WgpuRuntime> for Materials {
    fn as_ref(&self) -> &WgpuRuntime {
        self.slab.runtime()
    }
}

impl Materials {
    pub fn new(runtime: impl AsRef<WgpuRuntime>, atlas_size: wgpu::Extent3d) -> Self {
        let slab = SlabAllocator::new(runtime, "materials", wgpu::BufferUsages::empty());
        let atlas = Atlas::new(&slab, atlas_size, None, Some("materials-atlas"), None);
        Self { slab, atlas }
    }

    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    pub fn slab_allocator(&self) -> &SlabAllocator<WgpuRuntime> {
        &self.slab
    }

    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    /// Runs atlas upkeep and commits all changes to the GPU.
    ///
    /// Returns `true` if the atlas texture was recreated.
    #[must_use]
    pub fn commit(&self) -> (bool, SlabBuffer<wgpu::Buffer>) {
        // Atlas upkeep must be called first because it generates updates into the slab
        (self.atlas.upkeep(self.runtime()), self.slab.commit())
    }

    /// Create a new material.
    // TODO: move `Material` to material
    pub fn new_material(&self, material: MaterialDescriptor) -> Hybrid<MaterialDescriptor> {
        self.slab.new_value(material)
    }

    /// Create an array of materials, stored contiguously.
    pub fn new_materials(&self, data: impl IntoIterator<Item = MaterialDescriptor>) -> HybridArray<MaterialDescriptor> {
        self.slab.new_array(data)
    }
}
