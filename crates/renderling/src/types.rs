//! Type level machinery.

use craballoc::value::{GpuArrayContainer, GpuContainer, HybridArrayContainer, HybridContainer};

/// Specifies that a staged value has been unloaded from the CPU
/// and now lives solely on the GPU.
pub type GpuOnly = GpuContainer;

/// Specifies that a contiguous array of staged values has been
/// unloaded from the CPU and now lives solely on the GPU.
pub type GpuOnlyArray = GpuArrayContainer;

/// Specifies that a staged value lives on both the CPU and GPU,
/// with the CPU value being a synchronized copy of the GPU value.
///
/// Currently updates flow from the CPU to the GPU, but not back.
pub type GpuCpu = HybridContainer;

/// Specifies that a contiguous array of staged values lives on both
/// the CPU and GPU, with the CPU values being synchronized copies
/// of the GPU values.
///
/// Currently updates flow from the CPU to the GPU, but not back.
pub type GpuCpuArray = HybridArrayContainer;
