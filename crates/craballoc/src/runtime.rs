//! The CPU side fo slab allocation.

use std::{future::Future, sync::Mutex};

use crabslab::Array;

use crate::slab::SlabAllocatorError;

/// An update to a slab.
///
/// This is a write that can be serialized for later syncronization.
#[derive(Clone, Debug)]
pub struct SlabUpdate {
    pub array: Array<u32>,
    pub elements: Vec<u32>,
}

impl SlabUpdate {
    // pub fn range(&self) -> Range {
    //     Range {
    //         first_index: self.array.starting_index() as u32,
    //         last_index: (self.array.starting_index() + self.array.len()) as u32 -
    // 1,     }
    // }

    pub fn intersects(&self, other: &Self) -> bool {
        let here_start = self.array.index;
        let there_start = other.array.index;
        let here_end = self.array.index + self.array.len;
        let there_end = other.array.index + other.array.len;
        !(here_start >= there_end || there_start >= here_end)
    }
}

/// Represents the runtime that provides the interface to the GPU buffer.
///
/// For example, this could be a struct that contains `wgpu::Device` and `wgpu::Queue`,
/// or it could be a struct that contains Vulkan types, etc.
pub trait IsRuntime: Clone {
    /// The type of buffer this runtime engages with.
    type Buffer;

    /// The type used to denote the configuration of the buffer.
    type BufferUsages: Clone;

    /// Create a new buffer with the given `capacity`, where `capacity` is the number of `u32`s
    /// that can be stored in the buffer.
    fn buffer_create(
        &self,
        capacity: usize,
        label: Option<&str>,
        usages: Self::BufferUsages,
    ) -> Self::Buffer;

    /// Copy the contents of one buffer into another at index 0.
    fn buffer_copy(
        &self,
        source_buffer: &Self::Buffer,
        destination_buffer: &Self::Buffer,
        label: Option<&str>,
    );

    /// Write the updates into the given buffer.
    fn buffer_write<U: Iterator<Item = SlabUpdate>>(&self, updates: U, buffer: &Self::Buffer);

    /// Read the range from the given buffer.
    ///
    /// ## Note
    /// This function is async.
    fn buffer_read(
        &self,
        buffer: &Self::Buffer,
        buffer_len: usize,
        range: impl std::ops::RangeBounds<usize>,
    ) -> impl Future<Output = Result<Vec<u32>, SlabAllocatorError>>;
}

pub(crate) fn range_to_indices_and_len(
    // Used in the case the range is unbounded
    max_len: usize,
    range: impl std::ops::RangeBounds<usize>,
) -> (usize, usize, usize) {
    let start = match range.start_bound() {
        core::ops::Bound::Included(start) => *start,
        core::ops::Bound::Excluded(start) => *start + 1,
        core::ops::Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        core::ops::Bound::Included(end) => *end + 1,
        core::ops::Bound::Excluded(end) => *end,
        core::ops::Bound::Unbounded => max_len,
    };
    let len = end - start;
    (start, end, len)
}

/// A runtime that only operates on the CPU.
///
/// `CpuRuntime` manages [`VecSlab`]s, which are used as a reference
/// implementation, mostly for testing.
#[derive(Clone, Copy)]
pub struct CpuRuntime;

/// A slab buffer used _only_ on the GPU.
///
/// This is mostly for testing.
pub struct VecSlab {
    inner: Mutex<Vec<u32>>,
}

impl IsRuntime for CpuRuntime {
    type Buffer = VecSlab;
    type BufferUsages = ();

    fn buffer_create(&self, capacity: usize, label: Option<&str>, _usages: ()) -> VecSlab {
        log::trace!(
            "creating vec buffer '{}' with capacity {capacity}",
            label.unwrap_or("unknown")
        );
        VecSlab {
            inner: Mutex::new(vec![0; capacity]),
        }
    }

    fn buffer_copy(
        &self,
        source_buffer: &VecSlab,
        destination_buffer: &VecSlab,
        label: Option<&str>,
    ) {
        log::trace!("performing copy '{}'", label.unwrap_or("unknown"));
        let this = &destination_buffer;
        let source = source_buffer.inner.lock().unwrap();
        let mut destination = this.inner.lock().unwrap();
        let destination_slice = &mut destination[0..source.len()];
        destination_slice.copy_from_slice(source.as_slice());
    }

    fn buffer_write<U: Iterator<Item = SlabUpdate>>(&self, updates: U, buffer: &Self::Buffer) {
        let mut guard = buffer.inner.lock().unwrap();
        log::trace!("writing to vec len:{}", guard.len());
        for SlabUpdate { array, elements } in updates {
            log::trace!("array: {array:?} elements: {elements:?}");
            let slice = &mut guard[array.starting_index()..array.starting_index() + array.len()];
            slice.copy_from_slice(&elements);
        }
    }

    async fn buffer_read(
        &self,
        buffer: &Self::Buffer,
        buffer_len: usize,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Result<Vec<u32>, SlabAllocatorError> {
        let v = buffer.inner.lock().unwrap();
        debug_assert_eq!(v.len(), buffer_len);
        let (start, end, len) = range_to_indices_and_len(v.len(), range);
        let mut output = vec![0; len];
        let slice = &v[start..end];
        output.copy_from_slice(slice);
        Ok(output)
    }
}
