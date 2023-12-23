//! CPU side of slab storage.
// TODO: part out Id, Slab, etc into new "slab" libs.
// https://discord.com/channels/750717012564770887/750717499737243679/1187537792910229544
use std::{
    ops::Deref,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use renderling_shader::{array::Array, id::Id, slab::GrowableSlab};
use snafu::{ResultExt, Snafu};

pub use renderling_shader::slab::{Slab, SlabItem};

#[derive(Debug, Snafu)]
pub enum SlabError {
    #[snafu(display(
        "Out of capacity. Tried to write {type_is}(slab size={slab_size}) \
         at {index} but capacity is {capacity}",
    ))]
    Capacity {
        type_is: &'static str,
        slab_size: usize,
        index: usize,
        capacity: usize,
    },

    #[snafu(display(
        "Out of capacity. Tried to write an array of {elements} {type_is}\
         (each of slab size={slab_size}) \
         at {index} but capacity is {capacity}",
    ))]
    ArrayCapacity {
        type_is: &'static str,
        elements: usize,
        slab_size: usize,
        index: usize,
        capacity: usize,
    },

    #[snafu(display(
        "Array({type_is}) length mismatch. Tried to write {data_len} elements \
         into array of length {array_len}",
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
pub struct SlabBuffer {
    pub(crate) buffer: Arc<RwLock<wgpu::Buffer>>,
    device: crate::Device,
    queue: crate::Queue,
    // The number of u32 elements currently stored in the buffer.
    //
    // This is the next index to write into.
    len: Arc<AtomicUsize>,
    // The total number of u32 elements that can be stored in the buffer.
    capacity: Arc<AtomicUsize>,
}

impl Slab for SlabBuffer {
    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        todo!()
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        let byte_offset = index * std::mem::size_of::<u32>();
        let size = T::slab_size();
        let mut bytes = vec![0u32; size];
        let _ = bytes.write_indexed(t, 0);
        let capacity = self.capacity();
        if index + size <= capacity {
            log::error!(
                "could not write to slab: {}",
                CapacitySnafu {
                    type_is: std::any::type_name::<T>(),
                    slab_size: T::slab_size(),
                    index,
                    capacity
                }
                .fail()
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
        if index + size <= capacity {
            log::error!(
                "could not write array to slab: {}",
                ArrayCapacitySnafu {
                    capacity,
                    type_is: std::any::type_name::<T>(),
                    elements: t.len(),
                    slab_size: T::slab_size(),
                    index
                }
                .fail()
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

impl GrowableSlab for SlabBuffer {
    fn capacity(&self) -> usize {
        self.capacity.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Resize the slab buffer.
    ///
    /// This creates a new buffer and writes the data from the old into the new.
    fn resize(&mut self, new_capacity: usize) {
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

impl SlabBuffer {
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
    pub fn new(device: crate::Device, queue: crate::Queue, capacity: usize) -> Self {
        Self::new_usage(device, queue, capacity, wgpu::BufferUsages::empty())
    }

    /// Create a new slab buffer with a capacity of `capacity` u32 elements.
    pub fn new_usage(
        device: crate::Device,
        queue: crate::Queue,
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        Self {
            buffer: RwLock::new(Self::new_buffer(&device, capacity, usage)).into(),
            len: AtomicUsize::new(0).into(),
            capacity: AtomicUsize::new(capacity).into(),
            device,
            queue,
        }
    }

    /// Read from the slab buffer synchronously.
    pub fn block_on_read_raw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        len: usize,
    ) -> Result<Vec<u32>, SlabError> {
        futures_lite::future::block_on(self.read_raw(device, queue, start, len))
    }

    /// Read from the slab buffer.
    pub async fn read_raw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        len: usize,
    ) -> Result<Vec<u32>, SlabError> {
        let byte_offset = start * std::mem::size_of::<u32>();
        let length = len * std::mem::size_of::<u32>();
        let output_buffer_size = length as u64;
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SlabBuffer::read_raw"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
        queue.submit(std::iter::once(encoder.finish()));

        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = async_channel::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |res| tx.try_send(res).unwrap());
        device.poll(wgpu::Maintain::Wait);
        rx.recv()
            .await
            .context(AsyncRecvSnafu)?
            .context(AsyncSnafu)?;
        let bytes = buffer_slice.get_mapped_range();
        Ok(bytemuck::cast_slice(bytes.deref()).to_vec())
    }

    /// Read from the slab buffer.
    pub async fn read<T: SlabItem + Default + std::fmt::Debug>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: Id<T>,
    ) -> Result<T, SlabError> {
        let vec = self
            .read_raw(device, queue, id.index(), T::slab_size())
            .await?;
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
    use crate::Renderling;

    use super::*;

    #[test]
    fn slab_buffer_roundtrip() {
        println!("write");
        let _ = env_logger::builder().is_test(true).try_init();
        let r = Renderling::headless(10, 10);
        let device = r.get_device();
        let queue = r.get_queue();
        let slab = SlabBuffer::new(device, 2);
        slab.append(device, queue, &42);
        slab.append(device, queue, &1);
        let id = Id::<[u32; 2]>::new(0);
        let t = futures_lite::future::block_on(slab.read(device, queue, id)).unwrap();
        assert_eq!([42, 1], t, "read back what we wrote");

        println!("overflow");
        let id = Id::<u32>::new(2);
        let err = slab.write(device, queue, id, &666).unwrap_err();
        assert_eq!(
            "Out of capacity. Tried to write u32(slab size=1) at 2 but capacity is 2",
            err.to_string()
        );
        assert_eq!(2, slab.len());

        println!("append");
        slab.append(device, queue, &666);
        let id = Id::<[u32; 3]>::new(0);
        let t = futures_lite::future::block_on(slab.read(device, queue, id)).unwrap();
        assert_eq!([42, 1, 666], t);

        println!("append slice");
        let a = glam::Vec3::new(0.0, 0.0, 0.0);
        let b = glam::Vec3::new(1.0, 1.0, 1.0);
        let c = glam::Vec3::new(2.0, 2.0, 2.0);
        let points = vec![a, b, c];
        let array = slab.append_array(device, queue, &points);
        let slab_u32 =
            futures_lite::future::block_on(slab.read_raw(device, queue, 0, slab.len())).unwrap();
        let points_out = slab_u32.read_vec::<glam::Vec3>(array);
        assert_eq!(points, points_out);

        println!("append slice 2");
        let points = vec![a, a, a, a, b, b, b, c, c];
        let array = slab.append_array(device, queue, &points);
        let slab_u32 =
            futures_lite::future::block_on(slab.read_raw(device, queue, 0, slab.len())).unwrap();
        let points_out = slab_u32.read_vec::<glam::Vec3>(array);
        assert_eq!(points, points_out);
    }
}
