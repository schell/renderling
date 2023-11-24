//! Rendering objects in the scene graph.
//!
//! Provides a `Stage` object that can be used to render a scene graph.
use renderling_shader::{
    array::Array,
    debug::DebugMode,
    id::Id,
    slab::Slabbed,
    stage::{GpuLight, StageLegend},
};
use snafu::Snafu;

use crate::{Atlas, Device, Queue, SlabBuffer};

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

/// Represents an entire scene worth of rendering data.
pub struct Stage {
    pub(crate) stage_slab: SlabBuffer,
    pub(crate) render_unit_slab: SlabBuffer,
    //pub(crate) atlas: Atlas,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Stage {
    /// Create a new stage slab with `capacity`, which is
    pub fn new(device: Device, queue: Queue, legend: StageLegend) -> Self {
        let mut s = Self {
            stage_slab: SlabBuffer::new(&device, 256),
            render_unit_slab: SlabBuffer::new(&device, 256),
            //atlas: Atlas::new(&device, &queue, 0, 0),
            device,
            queue,
        };
        let _ = s.append(&legend);
        s
    }

    /// Add an object to the slab and return its ID.
    pub fn append<T: Slabbed + Default + std::fmt::Debug>(&mut self, object: &T) -> Id<T> {
        self.stage_slab.append(&self.device, &self.queue, object)
    }

    /// Add a slice of objects to the slab and return an [`Array`].
    pub fn append_slice<T: Slabbed + Default + std::fmt::Debug>(
        &mut self,
        objects: &[T],
    ) -> Array<T> {
        self.stage_slab
            .append_slice(&self.device, &self.queue, objects)
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        let id = Id::<DebugMode>::from(StageLegend::offset_of_debug_mode());
        // UNWRAP: safe because the debug mode offset is guaranteed to be valid.
        self.stage_slab
            .write(&self.device, &self.queue, id, &debug_mode)
            .unwrap();
    }

    /// Set the debug mode.
    pub fn with_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&mut self, use_lighting: bool) {
        let id = Id::<bool>::from(StageLegend::offset_of_has_lighting());
        // UNWRAP: safe because the has lighting offset is guaranteed to be valid.
        self.stage_slab
            .write(&self.device, &self.queue, id, &use_lighting)
            .unwrap();
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(mut self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Create a new spot light and return its builder.
    pub fn new_spot_light(&mut self) -> light::GpuSpotLightBuilder {
        light::GpuSpotLightBuilder::new(self)
    }

    /// Create a new directional light and return its builder.
    pub fn new_directional_light(&mut self) -> light::GpuDirectionalLightBuilder {
        light::GpuDirectionalLightBuilder::new(self)
    }

    /// Create a new point light and return its builder.
    pub fn new_point_light(&mut self) -> light::GpuPointLightBuilder {
        light::GpuPointLightBuilder::new(self)
    }

    /// Set the light array.
    ///
    /// This should be an iterator over the ids of all the lights on the stage.
    pub fn set_light_array(
        &mut self,
        lights: impl IntoIterator<Item = Id<GpuLight>>,
    ) -> Array<Id<GpuLight>> {
        let lights = lights.into_iter().collect::<Vec<_>>();
        let light_array = self.append_slice(&lights);
        let id = Id::<Array<Id<GpuLight>>>::from(StageLegend::offset_of_light_array());
        // UNWRAP: safe because we just appended the array, and the light array offset is
        // guaranteed to be valid.
        self.stage_slab
            .write(&self.device, &self.queue, id, &light_array)
            .unwrap();
        light_array
    }

    /// Set the light array.
    ///
    /// This should be an iterator over the ids of all the lights on the stage.
    pub fn with_light_array(mut self, lights: impl IntoIterator<Item = Id<GpuLight>>) -> Self {
        self.set_light_array(lights);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::Renderling;

    use super::*;

    #[test]
    fn stage_new() {
        let r = Renderling::headless(10, 10).unwrap();
        let (device, queue) = r.get_device_and_queue_owned();
        let _stage = Stage::new(device, queue, StageLegend::default());
    }
}
