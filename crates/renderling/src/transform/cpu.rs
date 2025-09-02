//! CPU side of transform.

use craballoc::value::{GpuContainer, HybridContainer, IsContainer};

/// A decomposed 3d transformation.
pub struct Transform<Ct: IsContainer = HybridContainer> {
    descriptor: Ct::Container<super::TransformDescriptor>,
}

pub type GpuOnlyTransform = Transform<GpuContainer>;

impl Transform {
    pub fn into_gpu_only(self) -> GpuOnlyTransform {
        Transform {
            descriptor: self.descriptor.into_gpu_only(),
        }
    }
}
