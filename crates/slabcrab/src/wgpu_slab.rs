//! CPU side of slab storage using `wgpu`.
use std::{
    ops::Deref,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use crate::{GrowableSlab, Id, Slab, SlabItem};
use snafu::{IntoError, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum SlabError {
    #[snafu(display(
        "Out of capacity. Tried to write {type_is}(slab size={slab_size}) at {index} but capacity \
         is {capacity}",
    ))]
    Capacity {
        type_is: &'static str,
        slab_size: usize,
        index: usize,
        capacity: usize,
    },

    #[snafu(display(
        "Out of capacity. Tried to write an array of {elements} {type_is}(each of slab \
         size={slab_size}) at {index} but capacity is {capacity}",
    ))]
    ArrayCapacity {
        type_is: &'static str,
        elements: usize,
        slab_size: usize,
        index: usize,
        capacity: usize,
    },

    #[snafu(display(
        "Array({type_is}) length mismatch. Tried to write {data_len} elements into array of \
         length {array_len}",
    ))]
    ArrayLen {
        type_is: &'static str,
        array_len: usize,
        data_len: usize,
    },

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

pub fn print_slab(slab: &[u32], starting_index: usize) {
    for (u, i) in slab.iter().zip(starting_index..) {
        println!("{i:02}: {u:032b} {u:010} {:?}", f32::from_bits(*u));
    }
}

/// A slab buffer used by the stage to store heterogeneous objects.
///
/// A clone of a buffer is a reference to the same buffer.
#[derive(Clone)]
pub struct WgpuBuffer {
    pub(crate) buffer: Arc<RwLock<wgpu::Buffer>>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    // The number of u32 elements currently stored in the buffer.
    //
    // This is the next index to write into.
    len: Arc<AtomicUsize>,
    // The total number of u32 elements that can be stored in the buffer.
    capacity: Arc<AtomicUsize>,
}

impl Slab for WgpuBuffer {
    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        futures_lite::future::block_on(self.read_async(id)).unwrap()
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        let byte_offset = index * std::mem::size_of::<u32>();
        let size = T::slab_size();
        let mut bytes = vec![0u32; size];
        let _ = bytes.write_indexed(t, 0);
        let capacity = self.capacity();
        if index + size > capacity {
            log::error!(
                "could not write to slab: {}",
                CapacitySnafu {
                    type_is: std::any::type_name::<T>(),
                    slab_size: T::slab_size(),
                    index,
                    capacity
                }
                .into_error(snafu::NoneError)
            );
            return index;
        }
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.queue.write_buffer(
            // UNWRAP: if we can't lock we want to panic
            &self.buffer.read().unwrap(),
            byte_offset as u64,
            bytemuck::cast_slice(bytes.as_slice()),
        );
        self.queue.submit(std::iter::once(encoder.finish()));

        index + size
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        let capacity = self.capacity();
        let size = T::slab_size() * t.len();
        if index + size > capacity {
            log::error!(
                "could not write array to slab: {}",
                ArrayCapacitySnafu {
                    capacity,
                    type_is: std::any::type_name::<T>(),
                    elements: t.len(),
                    slab_size: T::slab_size(),
                    index
                }
                .into_error(snafu::NoneError)
            );
            return index;
        }
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut u32_data = vec![0u32; size];
        let _ = u32_data.write_indexed_slice(t, 0);
        let byte_offset = index * std::mem::size_of::<u32>();
        self.queue.write_buffer(
            // UNWRAP: if we can't lock we want to panic
            &self.buffer.read().unwrap(),
            byte_offset as u64,
            bytemuck::cast_slice(&u32_data),
        );
        self.queue.submit(std::iter::once(encoder.finish()));

        index + size
    }
}

impl GrowableSlab for WgpuBuffer {
    fn capacity(&self) -> usize {
        self.capacity.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Resize the slab buffer.
    ///
    /// This creates a new buffer and writes the data from the old into the new.
    fn reserve_capacity(&mut self, new_capacity: usize) {
        let capacity = self.capacity();
        if new_capacity > capacity {
            log::trace!("resizing buffer from {capacity} to {new_capacity}");
            let len = self.len();
            let mut buffer = self.buffer.write().unwrap();
            let new_buffer = Self::new_buffer(&self.device, new_capacity, buffer.usage());
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            // UNWRAP: if we can't lock we want to panic
            encoder.copy_buffer_to_buffer(
                &buffer,
                0,
                &new_buffer,
                0,
                (len * std::mem::size_of::<u32>()) as u64,
            );
            self.queue.submit(std::iter::once(encoder.finish()));
            *buffer = new_buffer;
            self.capacity
                .store(new_capacity, std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn increment_len(&mut self, n: usize) -> usize {
        self.len.fetch_add(n, std::sync::atomic::Ordering::Relaxed)
    }
}

impl WgpuBuffer {
    fn new_buffer(
        device: &wgpu::Device,
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SlabBuffer"),
            size: (capacity * std::mem::size_of::<u32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC
                | usage,
            mapped_at_creation: false,
        })
    }

    /// Create a new slab buffer with a capacity of `capacity` u32 elements.
    pub fn new(
        device: impl Into<Arc<wgpu::Device>>,
        queue: impl Into<Arc<wgpu::Queue>>,
        capacity: usize,
    ) -> Self {
        Self::new_usage(device, queue, capacity, wgpu::BufferUsages::empty())
    }

    /// Create a new slab buffer with a capacity of `capacity` u32 elements.
    pub fn new_usage(
        device: impl Into<Arc<wgpu::Device>>,
        queue: impl Into<Arc<wgpu::Queue>>,
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let device = device.into();
        let queue = queue.into();
        Self {
            buffer: RwLock::new(Self::new_buffer(&device, capacity, usage)).into(),
            len: AtomicUsize::new(0).into(),
            capacity: AtomicUsize::new(capacity).into(),
            device,
            queue,
        }
    }

    #[cfg(feature = "futures-lite")]
    /// Read from the slab buffer synchronously.
    pub fn block_on_read_raw(&self, start: usize, len: usize) -> Result<Vec<u32>, SlabError> {
        futures_lite::future::block_on(self.read_raw(start, len))
    }

    /// Read from the slab buffer.
    pub async fn read_raw(&self, start: usize, len: usize) -> Result<Vec<u32>, SlabError> {
        let byte_offset = start * std::mem::size_of::<u32>();
        let length = len * std::mem::size_of::<u32>();
        let output_buffer_size = length as u64;
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SlabBuffer::read_raw"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        log::trace!(
            "copy_buffer_to_buffer byte_offset:{byte_offset}, \
             output_buffer_size:{output_buffer_size}",
        );
        encoder.copy_buffer_to_buffer(
            // UNWRAP: if we can't lock we want to panic
            &self.buffer.read().unwrap(),
            byte_offset as u64,
            &output_buffer,
            0,
            output_buffer_size,
        );
        self.queue.submit(std::iter::once(encoder.finish()));

        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = async_channel::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |res| tx.try_send(res).unwrap());
        self.device.poll(wgpu::Maintain::Wait);
        rx.recv()
            .await
            .context(AsyncRecvSnafu)?
            .context(AsyncSnafu)?;
        let bytes = buffer_slice.get_mapped_range();
        Ok(bytemuck::cast_slice(bytes.deref()).to_vec())
    }

    /// Read from the slab buffer.
    pub async fn read_async<T: SlabItem + Default>(&self, id: Id<T>) -> Result<T, SlabError> {
        let vec = self.read_raw(id.index(), T::slab_size()).await?;
        let t = Slab::read(vec.as_slice(), Id::<T>::new(0));
        Ok(t)
    }

    /// Get the underlying buffer.
    pub fn get_buffer(&self) -> impl Deref<Target = wgpu::Buffer> + '_ {
        // UNWRAP: if we can't lock we want to panic
        self.buffer.read().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::CpuSlab;

    use super::*;

    fn get_device_and_queue() -> (wgpu::Device, wgpu::Queue) {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = wgpu::Backends::all();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });

        let limits = wgpu::Limits::default();

        let adapter = futures_lite::future::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            },
        ))
        .unwrap();

        let info = adapter.get_info();
        log::trace!(
            "using adapter: '{}' backend:{:?} driver:'{}'",
            info.name,
            info.backend,
            info.driver
        );

        futures_lite::future::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features:
                // this one is a funny requirement, it seems it is needed if
                // using storage buffers in vertex shaders, even if those
                // shaders are read-only
                wgpu::Features::VERTEX_WRITABLE_STORAGE,
                limits,
                label: None,
            },
            None, // Trace path
        ))
        .unwrap()
    }

    #[test]
    fn slab_buffer_roundtrip() {
        println!("write");

        let (device, queue) = get_device_and_queue();
        let buffer = WgpuBuffer::new(device, queue, 2);
        let mut slab = CpuSlab::new(buffer);
        slab.append(&42);
        slab.append(&1);
        let id = Id::<[u32; 2]>::new(0);
        let t = slab.read(id);
        assert_eq!([42, 1], t, "read back what we wrote");

        println!("overflow");
        let id = Id::<u32>::new(2);
        slab.write(id, &666);
        assert_eq!(2, slab.len());

        println!("append");
        slab.append(&666);
        let id = Id::<[u32; 3]>::new(0);
        let t = slab.read(id);
        assert_eq!([42, 1, 666], t);

        println!("append slice");
        let a = glam::Vec3::new(0.0, 0.0, 0.0);
        let b = glam::Vec3::new(1.0, 1.0, 1.0);
        let c = glam::Vec3::new(2.0, 2.0, 2.0);
        let points = vec![a, b, c];
        let array = slab.append_array(&points);
        let slab_u32 =
            futures_lite::future::block_on(slab.as_ref().read_raw(0, slab.len())).unwrap();
        let points_out = slab_u32.read_vec::<glam::Vec3>(array);
        assert_eq!(points, points_out);

        println!("append slice 2");
        let points = vec![a, a, a, a, b, b, b, c, c];
        let array = slab.append_array(&points);
        let slab_u32 =
            futures_lite::future::block_on(slab.as_ref().read_raw(0, slab.len())).unwrap();
        let points_out = slab_u32.read_vec::<glam::Vec3>(array);
        assert_eq!(points, points_out);
    }
}
