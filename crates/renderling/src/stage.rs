//! Rendering objects in the scene graph.
//!
//! Provides a `Stage` object that can be used to render a scene graph.
use renderling_shader::{
    id::Id,
    slab::Slabbed,
    stage::GpuConstants, array::Array,
};
use snafu::Snafu;

use crate::{SlabBuffer, Device, Queue};

pub mod light;

#[derive(Debug, Snafu)]
pub enum StageError<T: std::fmt::Debug> {
    #[snafu(display("Out of capacity. Tried to write {:?} but capacity is {capacity}"))]
    Capacity { id: Id<T>, capacity: usize },

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

/// Builds the stage
pub struct StageSlab {
    pub(crate) slab: SlabBuffer,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl StageSlab {
    /// Create a new stage slab with `capacity`, which is
    pub fn new(device: Device, queue: Queue, constants: GpuConstants) -> Self {
        let mut s = Self {
            slab: SlabBuffer::new(&device, 256),
            device,
            queue,
        };
        let _ = s.append(&constants);
        s
    }

    /// Add an object to the slab and return its ID.
    pub fn append<T: Slabbed + Default>(&mut self, object: &T) -> Id<T> {
        self.slab.append(&self.device, &self.queue, object)
    }

    /// Add a slice of objects to the slab and return an [`Array`].
    pub fn append_slice<T: Slabbed + Default>(&mut self, objects: &[T]) -> Array<T> {
        self.slab.append_slice(&self.device, &self.queue, objects)
    }

    /// Create a new spot light and return its builder.
    pub fn new_spot_light(&mut self) -> light::GpuSpotLightBuilder {
        light::GpuSpotLightBuilder::new(self)
    }
}
