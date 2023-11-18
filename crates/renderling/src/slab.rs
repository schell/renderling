//! CPU side of the slab storage.
use std::ops::Deref;

use renderling_shader::{
    id::Id,
    slab::{Slab, Slabbed}, array::Array,
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

/// A slab buffer used by the stage.
pub struct SlabBuffer {
    buffer: wgpu::Buffer,
    // The number of u32 elements currently stored in the buffer.
    //
    // This is the next index to write into.
    len: usize,
    // The total number of u32 elements that can be stored in the buffer.
    capacity: usize,
}

impl SlabBuffer {
    pub fn new(device: &wgpu::Device, capacity: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("stage buffer"),
            size: (capacity * std::mem::size_of::<u32>()) as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            len: 0,
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Write into the slab buffer, modifying in place.
    ///
    /// NOTE: This has no effect on the length of the buffer.
    pub fn write<T: Slabbed + Default>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: Id<T>,
        data: &T,
    ) -> Result<(), SlabError<T>> {
        let byte_offset = id.index() * std::mem::size_of::<u32>();
        let size = T::slab_size();
        let mut bytes = vec![0u32; size];
        let _ = bytes.write(data, 0);
        snafu::ensure!(
            id.index() + size <= self.capacity,
            CapacitySnafu {
                id,
                capacity: self.capacity
            }
        );
        let encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        queue.write_buffer(
            &self.buffer,
            byte_offset as u64,
            bytemuck::cast_slice(bytes.as_slice()),
        );
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    pub async fn read<T: Slabbed + Default>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: Id<T>,
    ) -> Result<T, SlabError<T>> {
        let byte_offset = id.index() * std::mem::size_of::<u32>();
        let length = T::slab_size() * std::mem::size_of::<u32>();
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
            &self.buffer,
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
        let t = Slab::read(bytemuck::cast_slice(bytes.deref()), Id::<T>::new(0));
        Ok(t)
    }

    /// Append to the end of the buffer.
    pub fn append<T: Slabbed + Default>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        t: &T,
    ) -> Id<T> {
        if T::slab_size() + self.len >= self.capacity {
            self.resize(device, queue, self.capacity * 2);
        }
        let id = Id::<T>::from(self.len);
        // UNWRAP: We just checked that there is enough capacity, and added some if not.
        self.write(device, queue, id, t).unwrap();
        self.len += T::slab_size();
        id
    }

    /// Append a slice to the end of the buffer, returning a slab array.
    pub fn append_slice<T: Slabbed + Default>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        ts: &[T],
    ) -> Array<T> {
        let len = ts.len();
        let size = T::slab_size();
        if size * len + self.len >= self.capacity {
            self.resize(device, queue, self.capacity * 2);
        }
        let index = self.len as u32;
        for t in ts.iter() {
            // UNWRAP: Safe because we just checked that there is enough capacity,
            // and added some if not.
            self.write(device, queue, Id::<T>::from(self.len), t).unwrap();
        }
        self.len += size * len;
        Array::new(index, len as u32)
    }

    /// Resize the slab buffer.
    ///
    /// This creates a new buffer and writes the data from the old into the new.
    pub fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, capacity: usize) {
        if capacity > self.capacity {
            let new_buffer = Self::new(device, capacity);
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            encoder.copy_buffer_to_buffer(
                &self.buffer,
                0,
                &new_buffer.buffer,
                0,
                (self.len * std::mem::size_of::<u32>()) as u64,
            );
            queue.submit(std::iter::once(encoder.finish()));
            self.buffer = new_buffer.buffer;
            self.capacity = capacity;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Renderling;

    use super::*;

    #[test]
    fn slab_buffer_roundtrip() {
        let _ = env_logger::builder().is_test(true).try_init();
        let r = Renderling::headless(10, 10).unwrap();
        let device = r.get_device();
        let queue = r.get_queue();
        let mut slab = SlabBuffer::new(device, 2);
        let id = Id::<u32>::new(0);
        slab.append(device, queue, &42).unwrap();
        slab.append(device, queue, &1).unwrap();
        let id = Id::<[u32; 2]>::new(0);
        let t = futures_lite::future::block_on(slab.read(device, queue, id)).unwrap();
        assert_eq!([42, 1], t, "read back what we wrote");
        let id = Id::<u32>::new(2);
        let err = slab.write(device, queue, id, &666).unwrap_err();
        assert_eq!(
            "Out of capacity. Tried to write u32(slab size=1) at 2 but capacity is 2",
            err.to_string()
        );
        assert_eq!(2, slab.len);
        slab.append(device, queue, &666).unwrap();
        let id = Id::<[u32; 3]>::new(0);
        let t = futures_lite::future::block_on(slab.read(device, queue, id)).unwrap();
        assert_eq!([42, 1, 666], t);
    }
}
