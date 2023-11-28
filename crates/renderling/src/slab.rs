//! CPU side of slab storage.
use std::{
    ops::Deref,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use renderling_shader::{
    array::Array,
    id::Id,
    slab::{Slab, Slabbed},
};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum SlabError<T: Slabbed> {
    #[snafu(display(
        "Out of capacity. Tried to write {}(slab size={}) \
         at {} but capacity is {capacity}",
        std::any::type_name::<T>(), T::slab_size(), id.index()
    ))]
    Capacity { id: Id<T>, capacity: usize },

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

/// A slab buffer used by the stage to store heterogeneous objects.
///
/// A clone of a buffer is a reference to the same buffer.
#[derive(Clone)]
pub struct SlabBuffer {
    pub(crate) buffer: Arc<RwLock<wgpu::Buffer>>,
    // The number of u32 elements currently stored in the buffer.
    //
    // This is the next index to write into.
    len: Arc<AtomicUsize>,
    // The total number of u32 elements that can be stored in the buffer.
    capacity: Arc<AtomicUsize>,
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
    pub fn new(device: &wgpu::Device, capacity: usize) -> Self {
        Self::new_usage(device, capacity, wgpu::BufferUsages::empty())
    }

    /// Create a new slab buffer with a capacity of `capacity` u32 elements.
    pub fn new_usage(device: &wgpu::Device, capacity: usize, usage: wgpu::BufferUsages) -> Self {
        Self {
            buffer: RwLock::new(Self::new_buffer(device, capacity, usage)).into(),
            len: AtomicUsize::new(0).into(),
            capacity: AtomicUsize::new(capacity).into(),
        }
    }

    /// The number of u32 elements currently stored in the buffer.
    pub fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// The total number of u32 elements that can be stored in the buffer.
    pub fn capacity(&self) -> usize {
        self.capacity.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Write into the slab buffer, modifying in place.
    ///
    /// NOTE: This has no effect on the length of the buffer.
    pub fn write<T: Slabbed + Default>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: Id<T>,
        data: &T,
    ) -> Result<(), SlabError<T>> {
        let byte_offset = id.index() * std::mem::size_of::<u32>();
        let size = T::slab_size();
        let mut bytes = vec![0u32; size];
        let _ = bytes.write(data, 0);
        let capacity = self.capacity();
        snafu::ensure!(
            id.index() + size <= capacity,
            CapacitySnafu { id, capacity }
        );
        let encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        queue.write_buffer(
            // UNWRAP: if we can't lock we want to panic
            &self.buffer.read().unwrap(),
            byte_offset as u64,
            bytemuck::cast_slice(bytes.as_slice()),
        );
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    /// Read from the slab buffer.
    ///
    /// `T` is only for the error message.
    pub async fn read_raw<T: Slabbed + Default + std::fmt::Debug>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        len: usize,
    ) -> Result<Vec<u32>, SlabError<T>> {
        let byte_offset = start * std::mem::size_of::<u32>();
        let length = len * std::mem::size_of::<u32>();
        let output_buffer_size = length as u64;
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("SlabBuffer::read<{}>", std::any::type_name::<T>())),
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
    pub async fn read<T: Slabbed + Default + std::fmt::Debug>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: Id<T>,
    ) -> Result<T, SlabError<T>> {
        let vec = self
            .read_raw(device, queue, id.index(), T::slab_size())
            .await?;
        let t = Slab::read(vec.as_slice(), Id::<T>::new(0));
        Ok(t)
    }

    /// Append to the end of the buffer.
    pub fn append<T: Slabbed + Default + std::fmt::Debug>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        t: &T,
    ) -> Id<T> {
        let len = self.len();
        let capacity = self.capacity();
        if T::slab_size() + len > capacity {
            self.resize(device, queue, capacity * 2);
        }
        let id = Id::<T>::from(len);
        // UNWRAP: We just checked that there is enough capacity, and added some if not.
        self.write(device, queue, id, t).unwrap();
        self.len
            .store(len + T::slab_size(), std::sync::atomic::Ordering::Relaxed);
        id
    }

    /// Append a slice to the end of the buffer, returning a slab array.
    pub fn append_slice<T: Slabbed + Default + std::fmt::Debug>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        ts: &[T],
    ) -> Array<T> {
        let ts_len = ts.len();
        let size = T::slab_size();
        let capacity = self.capacity();
        let len = self.len();
        //log::trace!(
        //    "append_slice: {size} * {ts_len} + {len} ({}) >= {capacity}",
        //    size * ts_len + len
        //);
        let capacity_needed = size * ts_len + len;
        if capacity_needed >= capacity {
            let mut new_capacity = capacity * 2;
            while new_capacity < capacity_needed {
                new_capacity *= 2;
            }
            self.resize(device, queue, new_capacity);
        }
        let starting_index = len as u32;
        for (i, t) in ts.iter().enumerate() {
            // UNWRAP: Safe because we just checked that there is enough capacity,
            // and added some if not.
            self.write(
                device,
                queue,
                Id::<T>::from(starting_index + (size * i) as u32),
                t,
            )
            .unwrap();
        }
        self.len
            .store(len + size * ts_len, std::sync::atomic::Ordering::Relaxed);
        Array::new(starting_index, ts_len as u32)
    }

    /// Resize the slab buffer.
    ///
    /// This creates a new buffer and writes the data from the old into the new.
    pub fn resize(&self, device: &wgpu::Device, queue: &wgpu::Queue, new_capacity: usize) {
        let capacity = self.capacity();
        if new_capacity > capacity {
            log::trace!("resizing buffer from {capacity} to {new_capacity}");
            let len = self.len();
            let mut buffer = self.buffer.write().unwrap();
            let new_buffer = Self::new_buffer(device, new_capacity, buffer.usage());
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            // UNWRAP: if we can't lock we want to panic
            encoder.copy_buffer_to_buffer(
                &buffer,
                0,
                &new_buffer,
                0,
                (len * std::mem::size_of::<u32>()) as u64,
            );
            queue.submit(std::iter::once(encoder.finish()));
            *buffer = new_buffer;
            self.capacity
                .store(new_capacity, std::sync::atomic::Ordering::Relaxed);
        }
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
        let r = Renderling::headless(10, 10).unwrap();
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
        let array = slab.append_slice(device, queue, &points);
        let slab_u32 =
            futures_lite::future::block_on(slab.read_raw::<u32>(device, queue, 0, slab.len()))
                .unwrap();
        let points_out = slab_u32.read_vec::<glam::Vec3>(array);
        assert_eq!(points, points_out);

        println!("append slice 2");
        let points = vec![a, a, a, a, b, b, b, c, c];
        let array = slab.append_slice(device, queue, &points);
        let slab_u32 =
            futures_lite::future::block_on(slab.read_raw::<u32>(device, queue, 0, slab.len()))
                .unwrap();
        let points_out = slab_u32.read_vec::<glam::Vec3>(array);
        assert_eq!(points, points_out);
    }
}