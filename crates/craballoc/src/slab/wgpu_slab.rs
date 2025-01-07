//! Slab allocation of WebGPU buffers.
use std::{ops::Deref, sync::Arc};

use crabslab::{Array, Slab, SlabItem};
use snafu::{OptionExt, ResultExt};
use tracing::Instrument;

use crate::{
    runtime::{IsRuntime, SlabUpdate},
    slab::{AsyncRecvSnafu, AsyncSnafu, NoInternalBufferSnafu},
};

use super::{SlabAllocator, SlabAllocatorError};

/// A slab allocation runtime that creates and updates [`wgpu::Buffer`]s.
#[derive(Clone)]
pub struct WgpuRuntime {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

impl IsRuntime for WgpuRuntime {
    type Buffer = wgpu::Buffer;
    type BufferUsages = wgpu::BufferUsages;

    fn buffer_write<U: Iterator<Item = SlabUpdate>>(&self, updates: U, buffer: &Self::Buffer) {
        for SlabUpdate { array, elements } in updates {
            let offset = array.starting_index() as u64 * std::mem::size_of::<u32>() as u64;
            self.queue
                .write_buffer(buffer, offset, bytemuck::cast_slice(&elements));
        }
        self.queue.submit(std::iter::empty());
    }

    fn buffer_create(
        &self,
        capacity: usize,
        label: Option<&str>,
        usages: wgpu::BufferUsages,
    ) -> Self::Buffer {
        let size = (capacity * std::mem::size_of::<u32>()) as u64;
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage: usages,
            mapped_at_creation: false,
        })
    }

    fn buffer_copy(
        &self,
        source_buffer: &Self::Buffer,
        destination_buffer: &Self::Buffer,
        label: Option<&str>,
    ) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        encoder.copy_buffer_to_buffer(
            source_buffer,
            0,
            destination_buffer,
            0,
            source_buffer.size(),
        );
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    #[tracing::instrument(skip_all)]
    async fn buffer_read(
        &self,
        buffer: &Self::Buffer,
        buffer_len: usize,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Result<Vec<u32>, SlabAllocatorError> {
        let (start, _end, len) = crate::runtime::range_to_indices_and_len(buffer_len, range);
        let byte_offset = start * std::mem::size_of::<u32>();
        let length = len * std::mem::size_of::<u32>();
        let output_buffer_size = length as u64;
        let output_buffer = tracing::trace_span!("create-buffer").in_scope(|| {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: output_buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            })
        });

        let submission_index = tracing::trace_span!("copy_buffer").in_scope(|| {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            log::trace!(
                "copy_buffer_to_buffer byte_offset:{byte_offset}, \
             output_buffer_size:{output_buffer_size}",
            );
            encoder.copy_buffer_to_buffer(
                buffer,
                byte_offset as u64,
                &output_buffer,
                0,
                output_buffer_size,
            );
            self.queue.submit(std::iter::once(encoder.finish()))
        });

        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = async_channel::bounded(1);
        tracing::trace_span!("map_async").in_scope(|| {
            buffer_slice.map_async(wgpu::MapMode::Read, move |res| tx.try_send(res).unwrap());
        });
        tracing::trace_span!("poll").in_scope(|| {
            self.device.poll(wgpu::Maintain::wait_for(submission_index));
        });
        rx.recv()
            .instrument(tracing::info_span!("recv"))
            .await
            .context(AsyncRecvSnafu)?
            .context(AsyncSnafu)?;
        let output = tracing::trace_span!("get_mapped").in_scope(|| {
            let bytes = buffer_slice.get_mapped_range();
            bytemuck::cast_slice(bytes.deref()).to_vec()
        });
        Ok(output)
    }
}

impl SlabAllocator<WgpuRuntime> {
    #[tracing::instrument(skip_all)]
    pub async fn read(
        &self,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Result<Vec<u32>, SlabAllocatorError> {
        let internal_buffer = self.get_buffer().context(NoInternalBufferSnafu)?;
        self.runtime
            .buffer_read(&internal_buffer, self.len(), range)
            .await
    }

    #[tracing::instrument(skip_all)]
    pub async fn read_array<T: SlabItem + Default>(
        &self,
        array: Array<T>,
    ) -> Result<Vec<T>, SlabAllocatorError> {
        let arr = array.into_u32_array();
        let range = array.index as usize..(arr.index + arr.len) as usize;
        let data = self.read(range).await?;
        let t_array = Array::new(0, array.len() as u32);
        Ok(data.read_vec(t_array))
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.runtime.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.runtime.queue
    }
}
