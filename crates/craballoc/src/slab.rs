//! Slab allocators.
use core::sync::atomic::{AtomicUsize, Ordering};
use crabslab::{Array, Id, SlabItem};
use rustc_hash::{FxHashMap, FxHashSet};
use snafu::prelude::*;
use std::sync::{atomic::AtomicBool, Arc, RwLock};

use crate::{
    range::{Range, RangeManager},
    runtime::{IsRuntime, SlabUpdate},
    value::{Hybrid, HybridArray, WeakGpuRef},
};

#[cfg(feature = "wgpu")]
mod wgpu_slab;
#[cfg(feature = "wgpu")]
pub use wgpu_slab::*;

#[derive(Debug, Snafu)]
pub enum SlabAllocatorError {
    #[snafu(display(
        "Slab has no internal buffer. Please call SlabAllocator::upkeep or \
         SlabAllocator::get_updated_buffer first."
    ))]
    NoInternalBuffer,

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[cfg(feature = "wgpu")]
    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

/// Manages slab allocations and updates over a parameterised buffer.
///
/// Create a new instance using [`SlabAllocator::new`].
///
/// Upon creation you will need to call [`SlabAllocator::get_updated_buffer`] or
/// [`SlabAllocator::upkeep`] at least once before any data is written to the
/// internal buffer.
pub struct SlabAllocator<Runtime: IsRuntime> {
    pub(crate) notifier: (async_channel::Sender<usize>, async_channel::Receiver<usize>),
    runtime: Runtime,
    len: Arc<AtomicUsize>,
    capacity: Arc<AtomicUsize>,
    needs_expansion: Arc<AtomicBool>,
    buffer: Arc<RwLock<Option<Arc<Runtime::Buffer>>>>,
    buffer_usages: Runtime::BufferUsages,
    update_k: Arc<AtomicUsize>,
    update_sources: Arc<RwLock<FxHashMap<usize, WeakGpuRef>>>,
    update_queue: Arc<RwLock<FxHashSet<usize>>>,
    recycles: Arc<RwLock<RangeManager<Range>>>,
}

impl<R: IsRuntime> Clone for SlabAllocator<R> {
    fn clone(&self) -> Self {
        SlabAllocator {
            runtime: self.runtime.clone(),
            notifier: self.notifier.clone(),
            len: self.len.clone(),
            capacity: self.capacity.clone(),
            needs_expansion: self.needs_expansion.clone(),
            buffer: self.buffer.clone(),
            buffer_usages: self.buffer_usages.clone(),
            update_k: self.update_k.clone(),
            update_sources: self.update_sources.clone(),
            update_queue: self.update_queue.clone(),
            recycles: self.recycles.clone(),
        }
    }
}

impl<R: IsRuntime> SlabAllocator<R> {
    pub fn new(runtime: &R, default_buffer_usages: R::BufferUsages) -> Self {
        Self {
            runtime: runtime.clone(),
            notifier: async_channel::unbounded(),
            update_k: Default::default(),
            update_sources: Default::default(),
            update_queue: Default::default(),
            recycles: Default::default(),
            len: Default::default(),
            // Start with size 1, because some of `wgpu`'s validation depends on it.
            // See <https://github.com/gfx-rs/wgpu/issues/6414> for more info.
            capacity: Arc::new(AtomicUsize::new(1)),
            needs_expansion: Arc::new(true.into()),
            buffer: Default::default(),
            buffer_usages: default_buffer_usages,
        }
    }

    pub(crate) fn next_update_k(&self) -> usize {
        self.update_k.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) fn insert_update_source(&self, k: usize, source: WeakGpuRef) {
        log::trace!("slab insert_update_source {k}",);
        let _ = self.notifier.0.try_send(k);
        // UNWRAP: panic on purpose
        self.update_sources.write().unwrap().insert(k, source);
    }

    fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    pub(crate) fn allocate<T: SlabItem>(&self) -> Id<T> {
        // UNWRAP: we want to panic
        let may_range = self.recycles.write().unwrap().remove(T::SLAB_SIZE as u32);
        if let Some(range) = may_range {
            let id = Id::<T>::new(range.first_index);
            log::trace!(
                "slab allocate {}: dequeued {range:?} to {id:?}",
                std::any::type_name::<T>()
            );
            debug_assert_eq!(
                range.last_index,
                range.first_index + T::SLAB_SIZE as u32 - 1
            );
            id
        } else {
            self.maybe_expand_to_fit::<T>(1);
            let index = self.increment_len(T::SLAB_SIZE);
            Id::from(index)
        }
    }

    pub(crate) fn allocate_array<T: SlabItem>(&self, len: usize) -> Array<T> {
        if len == 0 {
            return Array::default();
        }

        // UNWRAP: we want to panic
        let may_range = self
            .recycles
            .write()
            .unwrap()
            .remove((T::SLAB_SIZE * len) as u32);
        if let Some(range) = may_range {
            let array = Array::<T>::new(range.first_index, len as u32);
            log::trace!(
                "slab allocate_array {len}x{}: dequeued {range:?} to {array:?}",
                std::any::type_name::<T>()
            );
            debug_assert_eq!(
                range.last_index,
                range.first_index + (T::SLAB_SIZE * len) as u32 - 1
            );
            array
        } else {
            self.maybe_expand_to_fit::<T>(len);
            let index = self.increment_len(T::SLAB_SIZE * len);
            Array::new(index as u32, len as u32)
        }
    }

    fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Relaxed)
    }

    fn reserve_capacity(&self, capacity: usize) {
        self.capacity.store(capacity, Ordering::Relaxed);
        self.needs_expansion.store(true, Ordering::Relaxed);
    }

    fn increment_len(&self, n: usize) -> usize {
        self.len.fetch_add(n, Ordering::Relaxed)
    }

    fn maybe_expand_to_fit<T: SlabItem>(&self, len: usize) {
        let capacity = self.capacity();
        // log::trace!(
        //    "append_slice: {size} * {ts_len} + {len} ({}) >= {capacity}",
        //    size * ts_len + len
        //);
        let capacity_needed = self.len() + T::SLAB_SIZE * len;
        if capacity_needed > capacity {
            let mut new_capacity = capacity * 2;
            while new_capacity < capacity_needed {
                new_capacity = (new_capacity * 2).max(2);
            }
            self.reserve_capacity(new_capacity);
        }
    }

    /// Return the internal buffer used by this slab.
    ///
    /// If the buffer needs recreating due to a capacity change this function
    /// will return `None`. In that case use [`Self::get_updated_buffer`].
    pub fn get_buffer(&self) -> Option<Arc<R::Buffer>> {
        self.buffer.read().unwrap().as_ref().cloned()
    }

    /// Return an updated buffer.
    ///
    /// This is the only way to guarantee access to a buffer.
    ///
    /// Use [`SlabAllocator::upkeep`] when you only need the buffer after a
    /// change, for example to recreate bindgroups.
    pub fn get_updated_buffer(&self) -> Arc<R::Buffer> {
        self.get_updated_buffer_and_check().0
    }

    /// Return an updated buffer, and whether or not it is different from the
    /// last one.
    ///
    /// This is the only way to guarantee access to a buffer.
    ///
    /// Use [`SlabAllocator::upkeep`] when you only need the buffer after a
    /// change, for example to recreate bindgroups.
    pub fn get_updated_buffer_and_check(&self) -> (Arc<R::Buffer>, bool) {
        if let Some(new_buffer) = self.upkeep() {
            (new_buffer, true)
        } else {
            // UNWRAP: safe because we know the buffer exists at this point,
            // as we've called `upkeep` above
            (self.get_buffer().unwrap(), false)
        }
    }

    /// Recreate this buffer, writing the contents of the previous buffer (if it
    /// exists) to the new one, then return the new buffer.
    fn recreate_buffer(&self) -> Arc<R::Buffer> {
        let new_buffer = Arc::new(self.runtime.buffer_create(
            self.capacity(),
            None,
            self.buffer_usages.clone(),
        ));
        let mut guard = self.buffer.write().unwrap();
        if let Some(old_buffer) = guard.take() {
            self.runtime.buffer_copy(&old_buffer, &new_buffer, None);
        }
        *guard = Some(new_buffer.clone());
        new_buffer
    }

    /// Stage a new value that lives on the GPU _and_ CPU.
    pub fn new_value<T: SlabItem + Clone + Send + Sync + 'static>(&self, value: T) -> Hybrid<T> {
        Hybrid::new(self, value)
    }

    /// Stage a contiguous array of new values that live on the GPU _and_ CPU.
    pub fn new_array<T: SlabItem + Clone + Send + Sync + 'static>(
        &self,
        values: impl IntoIterator<Item = T>,
    ) -> HybridArray<T> {
        HybridArray::new(self, values)
    }

    /// Return the ids of all sources that require updating.
    pub fn get_updated_source_ids(&self) -> FxHashSet<usize> {
        // UNWRAP: panic on purpose
        let mut update_set = self.update_queue.write().unwrap();
        while let Ok(source_index) = self.notifier.1.try_recv() {
            update_set.insert(source_index);
        }
        update_set.clone()
    }

    /// Build the set of sources that require updates, draining the source
    /// notifier and resetting the stored `update_queue`.
    ///
    /// This also places recycled items into the recycle bin.
    fn drain_updated_sources(&self) -> RangeManager<SlabUpdate> {
        let update_set = self.get_updated_source_ids();
        // UNWRAP: panic on purpose
        *self.update_queue.write().unwrap() = Default::default();
        // Prepare all of our GPU buffer writes
        let mut writes = RangeManager::<SlabUpdate>::default();
        {
            // Recycle any update sources that are no longer needed, and collect the active
            // sources' updates into `writes`.
            let mut updates_guard = self.update_sources.write().unwrap();
            let mut recycles_guard = self.recycles.write().unwrap();
            for key in update_set {
                let delete = if let Some(gpu_ref) = updates_guard.get_mut(&key) {
                    let count = gpu_ref.weak.strong_count();
                    if count == 0 {
                        // recycle this allocation
                        let array = gpu_ref.u32_array;
                        log::debug!("slab drain_updated_sources: recycling {key} {array:?}");
                        if array.is_null() {
                            log::debug!("  cannot recycle, null");
                        } else if array.is_empty() {
                            log::debug!("  cannot recycle, empty");
                        } else {
                            recycles_guard.add_range(gpu_ref.u32_array.into());
                        }
                        true
                    } else {
                        gpu_ref
                            .get_update()
                            .into_iter()
                            .flatten()
                            .for_each(|u| writes.add_range(u));
                        false
                    }
                } else {
                    log::debug!("could not find {key}");
                    false
                };
                if delete {
                    let _ = updates_guard.remove(&key);
                }
            }
            // Defrag the recycle ranges
            let ranges = std::mem::take(&mut recycles_guard.ranges);
            let num_ranges_to_defrag = ranges.len();
            for range in ranges.into_iter() {
                recycles_guard.add_range(range);
            }
            let num_ranges = recycles_guard.ranges.len();
            if num_ranges < num_ranges_to_defrag {
                log::trace!("{num_ranges_to_defrag} ranges before, {num_ranges} after");
            }
        }

        writes
    }

    /// Perform upkeep on the slab, commiting changes to the GPU.
    ///
    /// Returns the new buffer if one was created due to a capacity resize.
    #[must_use]
    pub fn upkeep(&self) -> Option<Arc<R::Buffer>> {
        let new_buffer = if self.needs_expansion.swap(false, Ordering::Relaxed) {
            Some(self.recreate_buffer())
        } else {
            None
        };

        let writes = self.drain_updated_sources();
        if !writes.is_empty() {
            // UNWRAP: safe because we know the buffer exists at this point, as we may have
            // recreated it above^
            let buffer = self.get_buffer().unwrap();
            self.runtime
                .buffer_write(writes.ranges.into_iter(), &buffer);
        }
        new_buffer
    }

    /// Defragments the internal "recycle" buffer.
    pub fn defrag(&self) {
        // UNWRAP: panic on purpose
        let mut recycle_guard = self.recycles.write().unwrap();
        for range in std::mem::take(&mut recycle_guard.ranges) {
            recycle_guard.add_range(range);
        }
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }
}
