//! Arrays of elements living on the GPU as an array.

use std::{any::Any, marker::PhantomData};

use async_channel::{Receiver, Sender};
use snafu::prelude::*;
use wgpu::util::DeviceExt;

#[derive(Debug, Snafu)]
pub enum BufferError {
    #[snafu(display(
        "{name} has run out of capacity. Capacity is {capacity} but the operation requires \
         {required}."
    ))]
    NoCapacity {
        name: &'static str,
        capacity: usize,
        required: usize,
    },

    #[snafu(display("Out of bounds, index is {index} but length is {length}."))]
    OutOfBounds { index: usize, length: usize },

    #[snafu(display("Read error"))]
    Read,
}

/// Read a vector from the GPU corresponding to the given range.
///
/// This creates an output buffer, creates an encoder, submits the queue and
/// then maps the output buffer and polls the device.
pub fn read_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    start: usize,
    length: usize,
) -> Result<Vec<T>, BufferError> {
    log::trace!(
        "reading {length} {} starting at index {start}",
        std::any::type_name::<T>()
    );
    let output_buffer_size = (length * std::mem::size_of::<T>()) as u64;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(&format!(
            "GpuArray output buffer {}",
            std::any::type_name::<T>()
        )),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(
        buffer,
        (start * std::mem::size_of::<T>()) as u64,
        &output_buffer,
        0,
        output_buffer_size,
    );
    queue.submit(std::iter::once(encoder.finish()));

    let buffer_slice = output_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let items = bytemuck::cast_slice::<u8, T>(&buffer_slice.get_mapped_range()).to_vec();
    output_buffer.unmap();
    Ok(items)
}

fn gpu_storage_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    label: Option<&str>,
    contents: &[T],
    capacity: usize,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let mut contents = contents.to_vec();
    contents.resize_with(capacity, T::zeroed);
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        usage,
        contents: bytemuck::cast_slice(contents.as_slice()),
    });
    buffer
}

pub struct GpuBuffer {
    // the gpu-side buffer
    pub buffer: wgpu::Buffer,
    // the number of elements stored in the buffer
    len: usize,
    // the total number of elements that can be stored in the buffer
    capacity: usize,
}

/// An array of `T` elements living on the GPU, backed by a storage buffer.
pub struct GpuArray<T: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: GpuBuffer,
    updates: (Sender<(usize, Vec<T>)>, Receiver<(usize, Vec<T>)>),
    _phantom: PhantomData<T>,
}

impl<T: Any + Clone + Copy + bytemuck::Pod + bytemuck::Zeroable> GpuArray<T> {
    /// Create a new buffer of [`GpuMeshVertex`] on the GPU.
    pub fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let capacity = capacity.max(1);
        GpuArray {
            buffer: GpuBuffer {
                buffer: gpu_storage_buffer(
                    device,
                    Some(&format!("GpuArray::new {}", std::any::type_name::<T>())),
                    &contents,
                    capacity,
                    usage,
                ),
                len: contents.len(),
                capacity,
            },
            updates: async_channel::unbounded(),
            _phantom: PhantomData,
        }
    }

    /// Push items onto the end of the array.
    ///
    /// Returns the index of the first item and the number of new items.
    ///
    /// Errs if the array has no capacity for the items.
    pub fn extend(
        &mut self,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(usize, usize), BufferError> {
        let items = items.into_iter().collect::<Vec<_>>();
        let items_len = items.len();
        let required = self.buffer.len + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity,
            NoCapacitySnafu {
                name: std::any::type_name::<Self>(),
                capacity: self.buffer.capacity,
                required
            }
        );
        let start = self.buffer.len;
        self.updates.0.try_send((start, items)).unwrap();
        self.buffer.len += items_len;
        Ok((start, items_len))
    }

    /// Push an item onto the end of the array.
    ///
    /// Returns the index of the item and `1`.
    ///
    /// Errs if the array has no capacity for the item.
    pub fn push(&mut self, item: T) -> Result<(usize, usize), BufferError> {
        self.extend(vec![item])
    }

    /// Overwrite a portion of items in the array.
    ///
    /// Returns the starting index and the length.
    ///
    /// Errs if the array has no capacity for the items.
    pub fn overwrite(
        &mut self,
        start_index: usize,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(usize, usize), BufferError> {
        let items = items.into_iter().collect::<Vec<_>>();
        let items_len = items.len();
        let required = start_index + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity,
            NoCapacitySnafu {
                name: std::any::type_name::<Self>(),
                capacity: self.buffer.capacity,
                required
            }
        );
        self.updates.0.try_send((start_index, items)).unwrap();
        self.buffer.len = self.buffer.len.max(start_index + items_len);
        Ok((start_index, items_len))
    }

    /// Return the length of the array.
    pub fn len(&self) -> usize {
        self.buffer.len
    }

    /// Return the capacity of the array.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity
    }

    /// Update the buffer on the GPU side.
    ///
    /// This array won't be fully updated on the GPU side until the queue has
    /// been submitted.
    pub fn update(&self, queue: &wgpu::Queue) {
        while let Ok((starting_index, items)) = self.updates.1.try_recv() {
            queue.write_buffer(
                &self.buffer.buffer,
                (starting_index * std::mem::size_of::<T>()) as u64,
                bytemuck::cast_slice(items.as_slice()),
            );
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer.buffer
    }

    /// Read a vector from the GPU corresponding to the given range.
    ///
    /// This creates an output buffer, creates an encoder, submits the queue and
    /// then maps the output buffer and polls the device.
    pub fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        length: usize,
    ) -> Result<Vec<T>, BufferError> {
        read_buffer(device, queue, self.get_buffer(), start, length)
    }
}

/// Like a GpuArray but with only one element.
pub struct Gpu<T: bytemuck::Pod + bytemuck::Zeroable> {
    inner: GpuArray<T>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> Gpu<T> {
    pub fn new(device: &wgpu::Device, contents: T, usage: wgpu::BufferUsages) -> Self {
        Self {
            inner: GpuArray::<T>::new(device, &[contents], 1, usage),
        }
    }

    pub fn set(&mut self, t: T) -> Result<(), BufferError> {
        let (start, len) = self.inner.overwrite(0, vec![t])?;
        debug_assert_eq!((0, 1), (start, len));
        Ok(())
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.inner.update(queue)
    }

    pub fn read(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<T, BufferError> {
        match self.inner.read(device, queue, 0, 1)?.as_slice() {
            &[t] => Ok(t),
            _ => Err(BufferError::Read),
        }
    }
}
