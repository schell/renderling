//! Abstraction over a GPU buffer that provides near-automatic synchronization
//! and RAII memory management.
use core::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};
use crabslab::{Array, GrowableSlab, Slab, SlabItem};
use snafu::prelude::*;
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock};

use crabslab::Id;

#[derive(Debug, Snafu)]
pub enum SlabManagerError {
    #[snafu(display(
        "Slab has no internal buffer. Please call SlabManager::recreate_buffer first."
    ))]
    NoInternalBuffer,

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

/// Manages slab allocations and updates.
///
/// Create a new instance using [`SlabManager::default`].
/// Upon creation you will need to call [`SlabManager::recreate_buffer`] or
/// [`SlabManager::upkeep`] at least once before any data is written to the GPU.
#[derive(Clone)]
pub struct SlabManager {
    pub(crate) len: Arc<AtomicUsize>,
    pub(crate) capacity: Arc<AtomicUsize>,
    pub(crate) needs_expansion: Arc<AtomicBool>,
    pub(crate) buffer: Arc<RwLock<Option<Arc<wgpu::Buffer>>>>,
    pub(crate) update_sources: Arc<RwLock<Vec<Box<dyn UpdatesSlab>>>>,
    pub(crate) updates: Arc<Mutex<Vec<SlabUpdate>>>,
    pub(crate) recycles: Arc<RwLock<Vec<Array<u32>>>>,
}

impl crabslab::Slab for SlabManager {
    fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    fn read_unchecked<T: SlabItem>(&self, _: Id<T>) -> T {
        panic!("slab is read-only")
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut guard = self.updates.lock().unwrap();
        guard.push(SlabUpdate {
            array: Array::<u32>::new(index as u32, T::SLAB_SIZE as u32),
            elements: {
                let mut e = vec![0u32; T::SLAB_SIZE];
                e.write_indexed(t, 0);
                e
            },
            #[cfg(debug_assertions)]
            type_is: std::any::type_name::<T>(),
        });
        index + T::SLAB_SIZE
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, ts: &[T], index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let size = T::SLAB_SIZE * ts.len();
        let mut guard = self.updates.lock().unwrap();
        guard.push(SlabUpdate {
            array: Array::<u32>::new(index as u32, size as u32),
            elements: {
                let mut e = vec![0u32; size];
                let mut i = 0;
                for t in ts {
                    i = e.write_indexed(t, i);
                }
                debug_assert_eq!(size, i);
                e
            },
            #[cfg(debug_assertions)]
            type_is: std::any::type_name::<T>(),
        });
        index + size
    }
}

impl crabslab::GrowableSlab for SlabManager {
    fn allocate<T: SlabItem>(&mut self) -> Id<T> {
        // UNWRAP: we want to panic
        let mut guard = self.recycles.write().unwrap();
        let len = T::SLAB_SIZE;
        let mut id = Id::<T>::NONE;
        guard.retain_mut(|recycled| {
            if let Some((array, maybe_leftover)) = split_array(*recycled, len as u32) {
                id = Id::new(array.starting_index() as u32);
                if let Some(leftover) = maybe_leftover {
                    *recycled = leftover;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        });
        drop(guard);

        if id.is_some() {
            log::trace!("recycled {id:?}");
            id
        } else {
            self.maybe_expand_to_fit::<T>(1);
            let index = self.increment_len(T::SLAB_SIZE);
            Id::from(index)
        }
    }

    fn allocate_array<T: SlabItem>(&mut self, len: usize) -> Array<T> {
        if len == 0 {
            return Array::default();
        }

        // UNWRAP: we want to panic
        let mut guard = self.recycles.write().unwrap();
        let slab_size = T::SLAB_SIZE * len;
        let mut output_array = Array::<T>::default();
        guard.retain_mut(|recycled| {
            if let Some((array, maybe_leftover)) = split_array(*recycled, slab_size as u32) {
                output_array = Array::new(array.starting_index() as u32, array.len() as u32);
                if let Some(leftover) = maybe_leftover {
                    *recycled = leftover;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        });
        drop(guard);

        if output_array.is_null() || output_array.is_empty() {
            self.maybe_expand_to_fit::<T>(len);
            let index = self.increment_len(slab_size);
            Array::new(index as u32, len as u32)
        } else {
            log::trace!("recycled {output_array:?}");
            output_array
        }
    }

    fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Relaxed)
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        let chosen_capacity = (2..13u32)
            .map(|n| 2usize.pow(n))
            .find(|pc| *pc >= capacity)
            .unwrap_or(capacity);
        self.capacity.store(chosen_capacity, Ordering::Relaxed);
        self.needs_expansion.store(true, Ordering::Relaxed);
    }

    fn increment_len(&mut self, n: usize) -> usize {
        self.len.fetch_add(n, Ordering::Relaxed)
    }
}

fn _arrays_are_contiguous_or_overlapping(a: Array<u32>, b: Array<u32>) -> bool {
    // overlapping
    a.contains_index(b.starting_index())
        || b.contains_index(a.starting_index())
        || (
            // contiguous
            a.starting_index() + a.len() == b.starting_index()
                || b.starting_index() + b.len() == a.starting_index()
        )
}

fn _combine_arrays(a: Array<u32>, b: Array<u32>) -> Array<u32> {
    let starting_index = a.starting_index().min(b.starting_index());
    let ending_index = (a.starting_index() + a.len()).max(b.starting_index() + b.len());
    let len = ending_index - starting_index;
    Array::new(starting_index as u32, len as u32)
}

fn split_array(a: Array<u32>, len: u32) -> Option<(Array<u32>, Option<Array<u32>>)> {
    if a.len() >= len as usize {
        let array = Array::new(a.starting_index() as u32, len);
        let leftover_len = a.len() as u32 - len;
        let leftover = if leftover_len > 0 {
            Some(Array::new(a.starting_index() as u32 + len, leftover_len))
        } else {
            None
        };
        Some((array, leftover))
    } else {
        None
    }
}

impl Default for SlabManager {
    fn default() -> Self {
        Self {
            update_sources: Default::default(),
            recycles: Default::default(),
            len: Default::default(),
            capacity: Default::default(),
            needs_expansion: Arc::new(true.into()),
            buffer: Default::default(),
            updates: Default::default(),
        }
    }
}

impl SlabManager {
    /// Return the internal buffer used by this slab.
    ///
    /// If the buffer needs recreating due to a capacity change this function
    /// will return `None`. In that case use [`Self::recreate_buffer`].
    pub fn get_buffer(&self) -> Option<Arc<wgpu::Buffer>> {
        self.buffer.read().unwrap().as_ref().cloned()
    }

    /// Recreate this buffer, writing the contents of the previous buffer (if it exists)
    /// to the new one, then return the new buffer.
    pub fn recreate_buffer(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usages: wgpu::BufferUsages,
    ) -> Arc<wgpu::Buffer> {
        let capacity = self.capacity() as u64;
        let size = capacity * std::mem::size_of::<u32>() as u64;
        log::trace!(
            "recreating '{}' buffer - new size {capacity} ({size}bytes)",
            label.unwrap_or("unknown")
        );
        let usage = usages
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC;
        let new_buffer = Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        }));
        let mut guard = self.buffer.write().unwrap();
        if let Some(old_buffer) = guard.take() {
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            encoder.copy_buffer_to_buffer(&old_buffer, 0, &new_buffer, 0, size);
            queue.submit(std::iter::once(encoder.finish()));
        }
        *guard = Some(new_buffer.clone());
        new_buffer
    }

    pub(crate) fn add_updates(&mut self, updates: impl UpdatesSlab) {
        self.update_sources.write().unwrap().push(Box::new(updates));
    }

    pub fn new_hybrid<T: SlabItem + Clone + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Hybrid<T> {
        Hybrid::new(self, value)
    }

    pub fn new_hybrid_array<T: SlabItem + Clone + Send + Sync + 'static>(
        &mut self,
        values: impl IntoIterator<Item = T>,
    ) -> HybridArray<T> {
        HybridArray::new(self, values)
    }

    /// Perform upkeep on the slab, commiting changes to the GPU.
    ///
    /// Returns the new buffer if one was created due to a capacity resize.
    pub fn upkeep(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usage: wgpu::BufferUsages,
    ) -> Option<Arc<wgpu::Buffer>> {
        log::trace!("upkeep");
        let new_buffer = if self.needs_expansion.swap(false, Ordering::Relaxed) {
            Some(self.recreate_buffer(device, queue, label, usage))
        } else {
            None
        };
        {
            let mut updates_guard = self.update_sources.write().unwrap();
            let mut recycles_guard = self.recycles.write().unwrap();
            updates_guard.retain(|hybrid| {
                let count = hybrid.strong_count();
                if count <= 1 {
                    // recycle this allocation
                    let input_array = hybrid.array();
                    log::trace!("recycling {} {input_array:?}", hybrid.type_name());
                    if !hybrid.forgotten() {
                        recycles_guard.push(input_array);
                    } else {
                        log::debug!("  cannot recycle - forgotten!");
                    }
                    // TODO: combine recycled contiguous arrays
                    false
                } else {
                    true
                }
            });
        }

        let writes = std::mem::replace(self.updates.lock().unwrap().as_mut(), vec![]);
        if !writes.is_empty() {
            // UNWRAP: safe because we know the buffer exists at this point, as we may have
            // recreated it above^
            let buffer = self.get_buffer().unwrap();
            for (
                i,
                SlabUpdate {
                    array,
                    elements,
                    type_is,
                },
            ) in writes
                .into_iter()
                .filter(|u| !u.array.is_empty() && !u.array.is_null())
                .enumerate()
            {
                log::trace!("writing update {i} {type_is} {array:?}");
                let offset = array.starting_index() as u64 * std::mem::size_of::<u32>() as u64;
                queue.write_buffer(&buffer, offset, bytemuck::cast_slice(&elements));
            }
            queue.submit(std::iter::empty());
        }
        new_buffer
    }

    /// Read the range of data from the slab.
    ///
    /// This is primarily used for debugging.
    pub async fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Result<Vec<u32>, SlabManagerError> {
        let start = match range.start_bound() {
            core::ops::Bound::Included(start) => *start,
            core::ops::Bound::Excluded(start) => *start + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            core::ops::Bound::Included(end) => *end + 1,
            core::ops::Bound::Excluded(end) => *end,
            core::ops::Bound::Unbounded => self.len(),
        };
        let len = end - start;
        let byte_offset = start * std::mem::size_of::<u32>();
        let length = len * std::mem::size_of::<u32>();
        let output_buffer_size = length as u64;
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let internal_buffer = self.get_buffer().context(NoInternalBufferSnafu)?;

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        log::trace!(
            "copy_buffer_to_buffer byte_offset:{byte_offset}, \
             output_buffer_size:{output_buffer_size}",
        );
        encoder.copy_buffer_to_buffer(
            &internal_buffer,
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
}

pub struct SlabUpdate {
    array: Array<u32>,
    elements: Vec<u32>,
    #[cfg(debug_assertions)]
    type_is: &'static str,
}

pub trait UpdatesSlab: Send + Sync + std::any::Any {
    /// Returns the slab range of all possible updates.
    fn array(&self) -> Array<u32>;

    /// Returns the number of references remaiting in the wild.
    fn strong_count(&self) -> usize;

    /// Returns the type name of Self
    fn type_name(&self) -> &'static str;

    /// Returns if the hybrid has been forgotten.
    fn forgotten(&self) -> bool;
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for Hybrid<T> {
    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.cpu_value)
    }

    fn array(&self) -> Array<u32> {
        Array::new(self.id.inner(), T::SLAB_SIZE as u32)
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }

    fn forgotten(&self) -> bool {
        self.forgotten.load(Ordering::Relaxed)
    }
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for HybridArray<T> {
    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.cpu_value)
    }

    fn array(&self) -> Array<u32> {
        self.array.into_u32_array()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }

    fn forgotten(&self) -> bool {
        self.forgotten.load(Ordering::Relaxed)
    }
}

/// A "hybrid" type that lives on the CPU and the GPU.
///
/// Updates are syncronized to the GPU once per frame.
pub struct Hybrid<T> {
    cpu_value: Arc<RwLock<T>>,
    id: Id<T>,
    updates: Arc<Mutex<Vec<SlabUpdate>>>,
    forgotten: Arc<AtomicBool>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Hybrid<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Hybrid<{}>", std::any::type_name::<T>()))
            .field("id", &self.id)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .finish()
    }
}

impl<T> Clone for Hybrid<T> {
    fn clone(&self) -> Self {
        Hybrid {
            cpu_value: self.cpu_value.clone(),
            id: self.id,
            updates: self.updates.clone(),
            forgotten: self.forgotten.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> Hybrid<T> {
    pub fn new(mngr: &mut SlabManager, value: T) -> Self {
        let id = mngr.allocate::<T>();
        let hybrid = Self::new_preallocated(id, value, mngr.updates.clone());
        mngr.add_updates(hybrid.clone());

        hybrid
    }

    pub(crate) fn new_preallocated(
        id: Id<T>,
        value: T,
        updates: Arc<Mutex<Vec<SlabUpdate>>>,
    ) -> Self {
        let s = Self {
            cpu_value: Arc::new(RwLock::new(value.clone())),
            id,
            updates,
            forgotten: Default::default(),
        };
        s.modify(move |old| *old = value);
        s
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn get(&self) -> T {
        self.cpu_value.read().unwrap().clone()
    }

    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut value_guard = self.cpu_value.write().unwrap();
        f(&mut value_guard);
        let t = value_guard.clone();
        self.updates.lock().unwrap().push(SlabUpdate {
            array: Array::new(self.id.inner(), T::SLAB_SIZE as u32),
            elements: {
                let mut es = vec![0u32; T::SLAB_SIZE];
                es.write(Id::new(0), &t);
                es
            },
            #[cfg(debug_assertions)]
            type_is: std::any::type_name::<T>(),
        })
    }

    pub fn set(&self, value: T) {
        self.modify(move |old| {
            *old = value;
        })
    }

    /// Forgets the hybrid value, preventing the GPU memory from being recycled and re-allocated.
    pub fn forget(self) {
        self.forgotten.store(true, Ordering::Relaxed);
    }
}

/// A "hybrid" array type that lives on the CPU and the GPU.
///
/// Once created, the array cannot be resized.
///
/// Updates are syncronized to the GPU once per frame.
pub struct HybridArray<T> {
    cpu_value: Arc<RwLock<Vec<T>>>,
    array: Array<T>,
    updates: Arc<Mutex<Vec<SlabUpdate>>>,
    forgotten: Arc<AtomicBool>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for HybridArray<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("HybridArray<{}>", std::any::type_name::<T>()))
            .field("array", &self.array)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .finish()
    }
}

impl<T> Clone for HybridArray<T> {
    fn clone(&self) -> Self {
        HybridArray {
            cpu_value: self.cpu_value.clone(),
            array: self.array,
            updates: self.updates.clone(),
            forgotten: self.forgotten.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> HybridArray<T> {
    pub fn new(mngr: &mut SlabManager, values: impl IntoIterator<Item = T>) -> Self {
        let value = values.into_iter().collect::<Vec<_>>();
        let array = mngr.allocate_array::<T>(value.len());
        let hybrid = Self::new_preallocated(array, value, mngr.updates.clone());
        mngr.update_sources
            .write()
            .unwrap()
            .push(Box::new(hybrid.clone()));
        hybrid
    }

    pub(crate) fn new_preallocated(
        array: Array<T>,
        values: Vec<T>,
        updates: Arc<Mutex<Vec<SlabUpdate>>>,
    ) -> Self {
        {
            let mut elements = vec![0u32; T::SLAB_SIZE * array.len()];
            elements.write_array(Array::new(0, array.len() as u32), &values);
            updates.lock().unwrap().push(SlabUpdate {
                array: array.into_u32_array(),
                elements,
                #[cfg(debug_assertions)]
                type_is: std::any::type_name::<Vec<T>>(),
            });
        }
        Self {
            array,
            updates,
            cpu_value: Arc::new(RwLock::new(values)),
            forgotten: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn array(&self) -> Array<T> {
        self.array
    }

    pub fn at(&self, index: usize) -> Option<T> {
        self.cpu_value.read().unwrap().get(index).cloned()
    }

    pub fn modify<S>(&self, index: usize, f: impl FnOnce(&mut T) -> S) -> Option<S> {
        let mut value_guard = self.cpu_value.write().unwrap();
        let t = value_guard.get_mut(index)?;
        let output = Some(f(t));
        let t = t.clone();
        let id = self.array.at(index);
        let array = Array::<u32>::new(id.inner(), T::SLAB_SIZE as u32);
        let mut elements = vec![0u32; T::SLAB_SIZE];
        elements.write(0u32.into(), &t);
        self.updates.lock().unwrap().push(SlabUpdate {
            array,
            elements,
            #[cfg(debug_assertions)]
            type_is: std::any::type_name::<T>(),
        });
        output
    }

    pub fn set_item(&self, index: usize, value: T) -> Option<T> {
        self.modify(index, move |t| std::mem::replace(t, value))
    }

    pub fn forget(self) {
        self.forgotten.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use crate::Context;

    use super::*;

    #[test]
    fn arrays_overlapping_or_contiguous_arrays_sanity() {
        let overlapping_a = Array::<u32>::new(0, 11);
        let overlapping_b = Array::<u32>::new(5, 25);
        assert!(_arrays_are_contiguous_or_overlapping(
            overlapping_a,
            overlapping_b
        ));

        let contiguous_a = Array::<u32>::new(0, 11);
        let contiguous_b = Array::<u32>::new(11, 20);
        assert!(_arrays_are_contiguous_or_overlapping(
            contiguous_a,
            contiguous_b
        ));

        let not_contiguous_a = Array::<u32>::new(0, 5);
        let not_contiguous_b = Array::<u32>::new(6, 8);
        assert!(!_arrays_are_contiguous_or_overlapping(
            not_contiguous_a,
            not_contiguous_b
        ));
    }

    #[test]
    fn arrays_combine_arrays_sanity() {
        let overlapping_a = Array::<u32>::new(0, 11);
        let overlapping_b = Array::<u32>::new(5, 25);
        assert_eq!(
            Array::new(0, 30),
            _combine_arrays(overlapping_a, overlapping_b)
        );

        let contiguous_a = Array::<u32>::new(0, 10);
        let contiguous_b = Array::<u32>::new(10, 10);
        assert_eq!(
            Array::new(0, 20),
            _combine_arrays(contiguous_a, contiguous_b)
        );
    }

    #[test]
    fn arrays_splitting_sanity() {
        let array = Array::<u32>::new(0, 10);
        assert_eq!(
            Some((Array::new(0, 5), Some(Array::new(5, 5)))),
            split_array(array, 5)
        );
        assert_eq!(Some((array, None)), split_array(array, array.len() as u32));
        assert_eq!(None, split_array(array, 11));
    }

    #[test]
    fn mngr_updates_count_sanity() {
        let r = Context::headless(1, 1);
        let mut mngr = SlabManager::default();
        {
            let value = mngr.new_hybrid(666u32);
            assert_eq!(2, value.strong_count());
        }
        mngr.upkeep(
            r.get_device(),
            r.get_queue(),
            Some("mngr updates count sanity 1"),
            wgpu::BufferUsages::empty(),
        );
        assert_eq!(0, mngr.update_sources.read().unwrap().len());
        {
            let values = mngr.new_hybrid_array([666u32, 420u32]);
            assert_eq!(2, values.strong_count());
        }
        mngr.upkeep(
            r.get_device(),
            r.get_queue(),
            Some("mngr updates count sanity 2"),
            wgpu::BufferUsages::empty(),
        );
        assert_eq!(0, mngr.update_sources.read().unwrap().len());
    }
}
