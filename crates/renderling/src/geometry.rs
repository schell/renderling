//! Holds geometry on CPU and GPU.
use crabslab::{Id, SlabItem};

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

use crate::camera::Camera;

/// Holds configuration info for vertex and shading render passes of
/// geometry.
///
/// This descriptor lives at the root (index 0) of the geometry slab.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, SlabItem)]
#[offsets]
pub struct GeometryDescriptor {
    pub camera_id: Id<Camera>,
    pub atlas_size: glam::UVec2,
    pub resolution: glam::UVec2,
    pub debug_channel: crate::pbr::debug::DebugChannel,
    pub has_lighting: bool,
    pub has_skinning: bool,
    pub perform_frustum_culling: bool,
    pub perform_occlusion_culling: bool,
}

impl Default for GeometryDescriptor {
    fn default() -> Self {
        Self {
            camera_id: Id::NONE,
            atlas_size: Default::default(),
            resolution: glam::UVec2::ONE,
            debug_channel: Default::default(),
            has_lighting: true,
            has_skinning: true,
            perform_frustum_culling: true,
            perform_occlusion_culling: false,
        }
    }
}
