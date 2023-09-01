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

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },

    #[snafu(display("Buffer is not {wrong_type}"))]
    WrongType { wrong_type: &'static str },
}

/// Read a vector from the GPU corresponding to the given range.
///
/// This creates an output buffer, creates an encoder, submits the queue and
/// then maps the output buffer and polls the device.
pub async fn read_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    start: usize,
    length: usize,
) -> Result<Vec<T>, BufferError> {
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
    let (tx, rx) = async_channel::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |res| tx.try_send(res).unwrap());
    device.poll(wgpu::Maintain::Wait);
    rx.recv()
        .await
        .context(AsyncRecvSnafu)?
        .context(AsyncSnafu)?;
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
    // The gpu-side buffer
    pub buffer: wgpu::Buffer,
    // The number of elements stored in the buffer
    len: usize,
    // The total number of elements that can be stored in the buffer
    capacity: usize,
}

impl GpuBuffer {
    /// Read a vector from the buffer corresponding to the given range.
    pub async fn read<T: bytemuck::Zeroable + bytemuck::Pod>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        length: usize,
    ) -> Result<Vec<T>, BufferError> {
        read_buffer(device, queue, &self.buffer, start, length).await
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait IsBuffer<T: bytemuck::Zeroable + bytemuck::Pod> {
    fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self;

    fn len(&self) -> usize;
    fn set_len(&mut self, len: usize);

    fn capacity(&self) -> usize;

    ///// Read a vector from the buffer corresponding to the given range.
    // fn read(
    //    &self,
    //    device: &wgpu::Device,
    //    queue: &wgpu::Queue,
    //    start: usize,
    //    length: usize,
    //) -> Result<Vec<T>, BufferError>;

    fn write(&mut self, queue: &wgpu::Queue, starting_index: usize, items: Vec<T>);
}

impl<T: bytemuck::Zeroable + bytemuck::Pod> IsBuffer<T> for GpuBuffer {
    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    fn write(&mut self, queue: &wgpu::Queue, starting_index: usize, items: Vec<T>) {
        queue.write_buffer(
            &self.buffer,
            (starting_index * std::mem::size_of::<T>()) as u64,
            bytemuck::cast_slice(items.as_slice()),
        )
    }

    fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        GpuBuffer {
            buffer: gpu_storage_buffer(
                device,
                Some(&format!("GpuArray::new {}", std::any::type_name::<T>())),
                contents,
                capacity,
                usage,
            ),
            len: contents.len(),
            capacity,
        }
    }
}

pub struct CpuBuffer {
    pub buffer: any_vec::AnyVec<dyn Send + Sync>,
    len: usize,
    capacity: usize,
}

impl CpuBuffer {
    pub fn new<T>(contents: &[T], capacity: usize) -> Self
    where
        T: bytemuck::Zeroable + bytemuck::Pod + Send + Sync,
    {
        let mut v = any_vec::AnyVec::with_capacity::<T>(capacity);
        {
            // UNWRAP: safe because we just created this anyvec with `T`
            let mut vec = v.downcast_mut::<T>().unwrap();
            for i in 0..capacity {
                let t = contents.get(i).cloned().unwrap_or_else(|| T::zeroed());
                vec.push(t);
            }
        }

        CpuBuffer {
            buffer: v,
            len: contents.len(),
            capacity,
        }
    }

    pub fn read<T: bytemuck::Pod>(
        &self,
        start: usize,
        length: usize,
    ) -> Result<Vec<T>, BufferError> {
        let mut output = Vec::with_capacity(length);
        let vec = self
            .buffer
            .downcast_ref::<T>()
            .with_context(|| WrongTypeSnafu {
                wrong_type: std::any::type_name::<T>(),
            })?;
        let len = vec.len();
        for i in start..start + length {
            output.push(*vec.get(i).with_context(|| OutOfBoundsSnafu {
                index: i,
                length: len,
            })?);
        }
        Ok(output)
    }

    pub fn with_slice<T: bytemuck::Pod, A>(
        &self,
        f: impl FnOnce(&[T]) -> A,
    ) -> Result<A, BufferError> {
        let vec = self
            .buffer
            .downcast_ref::<T>()
            .with_context(|| WrongTypeSnafu {
                wrong_type: std::any::type_name::<T>(),
            })?;
        Ok(f(vec.as_slice()))
    }

    pub fn with_mut_slice<T: bytemuck::Pod, A>(
        &mut self,
        f: impl FnOnce(&mut [T]) -> A,
    ) -> Result<A, BufferError> {
        let mut vec = self
            .buffer
            .downcast_mut::<T>()
            .with_context(|| WrongTypeSnafu {
                wrong_type: std::any::type_name::<T>(),
            })?;
        Ok(f(vec.as_mut_slice()))
    }
}

impl<T: bytemuck::Zeroable + bytemuck::Pod + Send + Sync> IsBuffer<T> for CpuBuffer {
    fn new(_: &wgpu::Device, contents: &[T], capacity: usize, _: wgpu::BufferUsages) -> Self {
        CpuBuffer::new(contents, capacity)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn write(&mut self, _: &wgpu::Queue, starting_index: usize, items: Vec<T>) {
        // UNWRAP: safe because we know `T` is the type (if not we want to panic!).
        let mut vec = self.buffer.downcast_mut::<T>().unwrap();
        for (i, item) in (starting_index..).zip(items) {
            *vec.get_mut(i).unwrap() = item;
        }
    }
}

pub struct CpuAndGpuBuffer {
    pub cpu_buffer: CpuBuffer,
    pub gpu_buffer: GpuBuffer,
}

impl CpuAndGpuBuffer {
    /// Copy the GPU buffer to CPU.
    ///
    /// Useful for syncing after mutating on the GPU.
    pub async fn read_gpu_to_cpu<T: bytemuck::Zeroable + bytemuck::Pod + Send + Sync>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<(), BufferError> {
        let items = self
            .gpu_buffer
            .read::<T>(device, queue, 0, self.cpu_buffer.len)
            .await?;
        self.cpu_buffer.write(queue, 0, items);
        Ok(())
    }
}

impl<T: bytemuck::Zeroable + bytemuck::Pod + Send + Sync> IsBuffer<T> for CpuAndGpuBuffer {
    fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        CpuAndGpuBuffer {
            cpu_buffer: CpuBuffer::new(contents, capacity),
            gpu_buffer: GpuBuffer::new(device, contents, capacity, usage),
        }
    }

    fn len(&self) -> usize {
        debug_assert_eq!(self.cpu_buffer.len, self.gpu_buffer.len);
        self.cpu_buffer.len
    }

    fn set_len(&mut self, len: usize) {
        self.cpu_buffer.len = len;
        self.gpu_buffer.len = len;
    }

    fn capacity(&self) -> usize {
        debug_assert_eq!(self.cpu_buffer.capacity, self.gpu_buffer.capacity);
        self.cpu_buffer.capacity
    }

    fn write(&mut self, queue: &wgpu::Queue, starting_index: usize, items: Vec<T>) {
        self.cpu_buffer.write(queue, starting_index, items.clone());
        self.gpu_buffer.write(queue, starting_index, items);
    }
}

/// An array of `T` elements living on the GPU or CPU, backed by a storage
/// buffer or a `Vec`.
pub struct BufferArray<T: bytemuck::Pod + bytemuck::Zeroable, Storage: IsBuffer<T> = GpuBuffer> {
    pub buffer: Storage,
    #[allow(clippy::type_complexity)]
    updates: (Sender<(usize, Vec<T>)>, Receiver<(usize, Vec<T>)>),
    _phantom: PhantomData<T>,
}

impl<T: Any + Clone + Copy + bytemuck::Pod + bytemuck::Zeroable, Storage: IsBuffer<T>>
    BufferArray<T, Storage>
{
    /// Create a new buffer.
    pub fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let capacity = capacity.max(1);
        BufferArray {
            buffer: IsBuffer::new(device, contents, capacity, usage),
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
        let required = self.buffer.len() + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity(),
            NoCapacitySnafu {
                name: std::any::type_name::<Self>(),
                capacity: self.buffer.capacity(),
                required
            }
        );
        let start = self.buffer.len();
        self.updates.0.try_send((start, items)).unwrap();
        self.buffer.set_len(start + items_len);
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
            required <= self.buffer.capacity(),
            NoCapacitySnafu {
                name: std::any::type_name::<Self>(),
                capacity: self.buffer.capacity(),
                required
            }
        );
        self.updates.0.try_send((start_index, items)).unwrap();
        self.buffer
            .set_len(self.buffer.len().max(start_index + items_len));
        Ok((start_index, items_len))
    }

    /// Return the length of the array.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Return the capacity of the array.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Update the buffer.
    ///
    /// This array won't be fully updated until the queue has been submitted.
    pub fn update(&mut self, queue: &wgpu::Queue) {
        while let Ok((starting_index, items)) = self.updates.1.try_recv() {
            self.buffer.write(queue, starting_index, items);
        }
    }
}

impl<T: Any + Clone + Copy + bytemuck::Pod + bytemuck::Zeroable> BufferArray<T, GpuBuffer> {
    /// Create a new GPU buffer.
    pub fn new_gpu(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let capacity = capacity.max(1);
        BufferArray {
            buffer: IsBuffer::new(device, contents, capacity, usage),
            updates: async_channel::unbounded(),
            _phantom: PhantomData,
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer.buffer
    }

    pub async fn read_gpu(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        length: usize,
    ) -> Result<Vec<T>, BufferError> {
        self.buffer.read(device, queue, start, length).await
    }
}

impl<T: Any + Clone + Copy + bytemuck::Pod + bytemuck::Zeroable + Send + Sync>
    BufferArray<T, CpuBuffer>
{
    /// Create a new CPU buffer.
    pub fn new_cpu(contents: &[T], capacity: usize) -> Self {
        let capacity = capacity.max(1);
        BufferArray {
            buffer: CpuBuffer::new(contents, capacity),
            updates: async_channel::unbounded(),
            _phantom: PhantomData,
        }
    }
}

pub trait HasWgpuBuffer {
    fn get_wgpu_buffer(&self) -> &wgpu::Buffer;
}

impl HasWgpuBuffer for GpuBuffer {
    fn get_wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

impl HasWgpuBuffer for CpuAndGpuBuffer {
    fn get_wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.gpu_buffer.buffer
    }
}
