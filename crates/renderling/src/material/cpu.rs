//! CPU side of materials.

use craballoc::{
    runtime::WgpuRuntime,
    slab::SlabAllocator,
    value::{Hybrid, HybridArray},
};

use crate::{atlas::Atlas, pbr::Material};

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
        let slab =
            SlabAllocator::new_with_label(runtime, wgpu::BufferUsages::empty(), Some("materials"));
        let atlas = Atlas::new(&slab, atlas_size, None, Some("materials-atlas"), None);
        Self { slab, atlas }
    }

    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    pub fn upkeep(&self) {
        let _ = self.slab.commit();
        self.atlas.upkeep(self.runtime());
    }

    /// Create a new material.
    // TODO: move `Material` to material
    pub fn new_material(&self, material: Material) -> Hybrid<Material> {
        self.slab.new_value(material)
    }

    /// Create an array of materials, stored contiguously.
    pub fn new_materials(&self, data: impl IntoIterator<Item = Material>) -> HybridArray<Material> {
        self.slab.new_array(data)
    }
}
