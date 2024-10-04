//! Abstraction over a GPU buffer that provides near-automatic synchronization
//! and RAII memory management.
use core::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};
use crabslab::{Slab, SlabItem};
use rustc_hash::{FxHashMap, FxHashSet};
use snafu::prelude::*;
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock, Weak};

pub use crabslab::{Array, Id};

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Range {
    first_index: u32,
    last_index: u32,
}

impl core::fmt::Debug for Range {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!("{}..={}", self.first_index, self.last_index))
    }
}

impl<T: SlabItem> From<Array<T>> for Range {
    fn from(array: Array<T>) -> Self {
        let array = array.into_u32_array();
        let first_index = array.starting_index() as u32;
        Range {
            first_index,
            last_index: first_index + array.len() as u32 - 1,
        }
    }
}

impl Range {
    pub fn len(&self) -> u32 {
        1 + self.last_index - self.first_index
    }

    pub fn intersects(&self, other: &Range) -> bool {
        !(self.first_index > other.last_index || self.last_index < other.first_index)
    }
}

trait IsRange {
    fn should_merge_with(&self, other: &Self) -> bool;

    fn union(&mut self, other: Self);
}

impl IsRange for Range {
    fn should_merge_with(&self, other: &Self) -> bool {
        debug_assert!(
            !self.intersects(other),
            "{self:?} intersects existing {other:?}, should never happen with Range"
        );

        self.last_index + 1 == other.first_index || self.first_index == other.last_index + 1
    }

    fn union(&mut self, other: Self) {
        *self = Range {
            first_index: self.first_index.min(other.first_index),
            last_index: self.last_index.max(other.last_index),
        };
    }
}

impl IsRange for SlabUpdate {
    fn should_merge_with(&self, other: &Self) -> bool {
        self.intersects(other)
    }

    fn union(&mut self, other: Self) {
        if self.array == other.array {
            *self = other;
            return;
        }

        let mut array = self.array;
        array.union(&other.array);

        let mut elements = vec![0u32; array.len()];

        let self_index = self.array.index - array.index;
        elements.write_indexed_slice(&self.elements, self_index as usize);
        let other_index = other.array.index - array.index;
        elements.write_indexed_slice(&other.elements, other_index as usize);

        self.array = array;
        self.elements = elements;
    }
}

struct RangeManager<R> {
    ranges: Vec<R>,
}

impl<R> Default for RangeManager<R> {
    fn default() -> Self {
        Self { ranges: vec![] }
    }
}

impl<R: IsRange> RangeManager<R> {
    pub fn add_range(&mut self, input_range: R) {
        for range in self.ranges.iter_mut() {
            if range.should_merge_with(&input_range) {
                range.union(input_range);
                return;
            }
        }
        self.ranges.push(input_range);
    }
}

impl RangeManager<Range> {
    /// Removes a range of `count` elements, if possible.
    pub fn remove(&mut self, count: u32) -> Option<Range> {
        let mut remove_index = usize::MAX;
        for (i, range) in self.ranges.iter_mut().enumerate() {
            // This is potentially a hot path, so use the `if` even
            // though clippy complains (because using match is slower)
            #[allow(clippy::comparison_chain)]
            if range.len() > count {
                let first_index = range.first_index;
                let last_index = range.first_index + count - 1;
                range.first_index += count;
                return Some(Range {
                    first_index,
                    last_index,
                });
            } else if range.len() == count {
                remove_index = i;
                break;
            }
        }

        if remove_index == usize::MAX {
            None
        } else {
            Some(self.ranges.swap_remove(remove_index))
        }
    }
}

#[derive(Debug, Snafu)]
pub enum SlabAllocatorError {
    #[snafu(display(
        "Slab has no internal buffer. Please call SlabAllocator::upkeep or \
         SlabAllocator::get_updated_buffer first."
    ))]
    NoInternalBuffer,

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

pub trait IsBuffer: Sized {
    type Resources<'a>: Clone;

    /// Create a new buffer with the given capacity.
    fn buffer_create(resources: Self::Resources<'_>, capacity: usize) -> Self;

    /// Copy the contents of one buffer into another at index 0.
    fn buffer_copy(resources: Self::Resources<'_>, source_buffer: &Self, destination_buffer: &Self);

    /// Write updates to the buffer.
    fn buffer_write<U: Iterator<Item = SlabUpdate>>(
        &self,
        resources: Self::Resources<'_>,
        updates: U,
    );
}

impl IsBuffer for Mutex<Vec<u32>> {
    type Resources<'a> = ();

    fn buffer_create((): Self::Resources<'_>, capacity: usize) -> Self {
        log::trace!("creating vec with capacity {capacity}");
        Mutex::new(vec![0; capacity])
    }

    fn buffer_copy((): Self::Resources<'_>, source_buffer: &Self, destination_buffer: &Self) {
        let source = source_buffer.lock().unwrap();
        let mut destination = destination_buffer.lock().unwrap();
        let destination_slice = &mut destination[0..source.len()];
        destination_slice.copy_from_slice(source.as_slice());
    }

    fn buffer_write<U: Iterator<Item = SlabUpdate>>(&self, (): Self::Resources<'_>, updates: U) {
        let mut guard = self.lock().unwrap();
        log::trace!("writing to vec len:{}", guard.len());
        for SlabUpdate { array, elements } in updates {
            log::trace!("array: {array:?} elements: {elements:?}");
            let slice = &mut guard[array.starting_index()..array.starting_index() + array.len()];
            slice.copy_from_slice(&elements);
        }
    }
}

/// Manages slab allocations and updates over a parameterised buffer.
///
/// Create a new instance using [`SlabAllocator::default`].
/// Upon creation you will need to call [`SlabAllocator::get_updated_buffer`] or
/// [`SlabAllocator::upkeep`] at least once before any data is written to the
/// internal buffer.
pub struct SlabAllocator<Buffer> {
    pub(crate) notifier: (async_channel::Sender<usize>, async_channel::Receiver<usize>),
    len: Arc<AtomicUsize>,
    capacity: Arc<AtomicUsize>,
    needs_expansion: Arc<AtomicBool>,
    buffer: Arc<RwLock<Option<Arc<Buffer>>>>,
    update_k: Arc<AtomicUsize>,
    update_sources: Arc<RwLock<FxHashMap<usize, WeakGpuRef>>>,
    update_queue: Arc<RwLock<FxHashSet<usize>>>,
    recycles: Arc<RwLock<RangeManager<Range>>>,
}

impl<Buffer> Clone for SlabAllocator<Buffer> {
    fn clone(&self) -> Self {
        SlabAllocator {
            notifier: self.notifier.clone(),
            len: self.len.clone(),
            capacity: self.capacity.clone(),
            needs_expansion: self.needs_expansion.clone(),
            buffer: self.buffer.clone(),
            update_k: self.update_k.clone(),
            update_sources: self.update_sources.clone(),
            update_queue: self.update_queue.clone(),
            recycles: self.recycles.clone(),
        }
    }
}

impl<Buffer> Default for SlabAllocator<Buffer> {
    fn default() -> Self {
        Self {
            notifier: async_channel::unbounded(),
            update_k: Default::default(),
            update_sources: Default::default(),
            update_queue: Default::default(),
            recycles: Default::default(),
            len: Default::default(),
            capacity: Default::default(),
            needs_expansion: Arc::new(true.into()),
            buffer: Default::default(),
        }
    }
}

impl IsBuffer for wgpu::Buffer {
    type Resources<'a> = (
        &'a wgpu::Device,
        &'a wgpu::Queue,
        Option<&'a str>,
        wgpu::BufferUsages,
    );

    fn buffer_write<U: Iterator<Item = SlabUpdate>>(
        &self,
        (_, queue, _, _): Self::Resources<'_>,
        updates: U,
    ) {
        for SlabUpdate { array, elements } in updates {
            let offset = array.starting_index() as u64 * std::mem::size_of::<u32>() as u64;
            queue.write_buffer(self, offset, bytemuck::cast_slice(&elements));
        }
        queue.submit(std::iter::empty());
    }

    fn buffer_create((device, _, label, usages): Self::Resources<'_>, capacity: usize) -> Self {
        let size = (capacity * std::mem::size_of::<u32>()) as u64;
        let usage = usages
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC;
        device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        })
    }

    fn buffer_copy(
        (device, queue, label, _): Self::Resources<'_>,
        source_buffer: &Self,
        destination_buffer: &Self,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        encoder.copy_buffer_to_buffer(
            source_buffer,
            0,
            destination_buffer,
            0,
            source_buffer.size(),
        );
        queue.submit(std::iter::once(encoder.finish()));
    }
}

impl<Buffer: IsBuffer> SlabAllocator<Buffer> {
    pub(crate) fn next_update_k(&self) -> usize {
        self.update_k.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) fn insert_update_source(&self, k: usize, source: WeakGpuRef) {
        log::trace!("inserting update source {k}",);
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
            log::trace!("dequeued {range:?} to {id:?}");
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
            log::trace!("dequeued {range:?} to {array:?}");
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
    pub fn get_buffer(&self) -> Option<Arc<Buffer>> {
        self.buffer.read().unwrap().as_ref().cloned()
    }

    /// Return an updated buffer.
    ///
    /// This is the only way to guarantee access to a buffer.
    ///
    /// Use [`SlabAllocator::upkeep`] when you only need the buffer after a
    /// change, for example to recreate bindgroups.
    pub fn get_updated_buffer(&self, resources: Buffer::Resources<'_>) -> Arc<Buffer> {
        if let Some(new_buffer) = self.upkeep(resources) {
            new_buffer
        } else {
            // UNWRAP: safe because we know the buffer exists at this point,
            // as we've called `upkeep` above
            self.get_buffer().unwrap()
        }
    }

    /// Recreate this buffer, writing the contents of the previous buffer (if it
    /// exists) to the new one, then return the new buffer.
    fn recreate_buffer(&self, resources: Buffer::Resources<'_>) -> Arc<Buffer> {
        let new_buffer = Arc::new(Buffer::buffer_create(resources.clone(), self.capacity()));
        let mut guard = self.buffer.write().unwrap();
        if let Some(old_buffer) = guard.take() {
            Buffer::buffer_copy(resources, &old_buffer, &new_buffer);
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
                        log::debug!("recycling {key} {:?}", gpu_ref.u32_array);
                        if gpu_ref.u32_array.is_null() || gpu_ref.u32_array.is_empty() {
                            log::debug!("  cannot recycle - empty or null");
                            true
                        } else {
                            recycles_guard.add_range(gpu_ref.u32_array.into());
                            true
                        }
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
    pub fn upkeep(&self, resources: Buffer::Resources<'_>) -> Option<Arc<Buffer>> {
        let new_buffer = if self.needs_expansion.swap(false, Ordering::Relaxed) {
            Some(self.recreate_buffer(resources.clone()))
        } else {
            None
        };

        let writes = self.drain_updated_sources();
        if !writes.ranges.is_empty() {
            // UNWRAP: safe because we know the buffer exists at this point, as we may have
            // recreated it above^
            let buffer = self.get_buffer().unwrap();
            buffer.buffer_write(resources, writes.ranges.into_iter());
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
}

impl SlabAllocator<wgpu::Buffer> {
    /// Read the range of data from the slab.
    ///
    /// This is primarily used for debugging.
    pub async fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Result<Vec<u32>, SlabAllocatorError> {
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

#[derive(Clone)]
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

pub(crate) struct WeakGpuRef {
    pub(crate) notifier_index: usize,
    pub(crate) u32_array: Array<u32>,
    pub(crate) weak: Weak<Mutex<Vec<SlabUpdate>>>,
    pub(crate) takes_update: bool,
}

impl WeakGpuRef {
    fn get_update(&self) -> Option<Vec<SlabUpdate>> {
        let strong = self.weak.upgrade()?;
        let mut guard = strong.lock().unwrap();
        let updates: Vec<_> = if self.takes_update {
            std::mem::take(guard.as_mut())
        } else {
            guard.clone()
        };

        if updates.is_empty() {
            None
        } else {
            Some(updates)
        }
    }

    pub(crate) fn from_gpu<T: SlabItem>(gpu: &Gpu<T>) -> Self {
        WeakGpuRef {
            notifier_index: gpu.notifier_index,
            u32_array: Array::new(gpu.id.inner(), T::SLAB_SIZE as u32),
            weak: Arc::downgrade(&gpu.update),
            takes_update: true,
        }
    }

    pub(crate) fn from_gpu_array<T: SlabItem>(gpu_array: &GpuArray<T>) -> Self {
        WeakGpuRef {
            notifier_index: gpu_array.notifier_index,
            u32_array: gpu_array.array.into_u32_array(),
            weak: Arc::downgrade(&gpu_array.updates),
            takes_update: true,
        }
    }
}

// impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for GpuArray<T> {
//     fn strong_count(&self) -> usize {
//         Arc::strong_count(&self.updates)
//     }

//     fn u32_array(&self) -> Array<u32> {
//         self.array.into_u32_array()
//     }

//     fn get_update(&self) -> Vec<SlabUpdate> {
//         std::mem::take(self.updates.lock().unwrap().as_mut())
//     }

//     fn id(&self) -> usize {
//         self.notifier_index
//     }
// }

// impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for HybridArray<T> {
//     fn strong_count(&self) -> usize {
//         self.gpu_value.strong_count()
//     }

//     fn u32_array(&self) -> Array<u32> {
//         self.gpu_value.u32_array()
//     }

//     fn get_update(&self) -> Vec<SlabUpdate> {
//         self.gpu_value.get_update()
//     }

//     fn id(&self) -> usize {
//         self.gpu_value.notifier_index
//     }
// }

#[derive(Debug)]
pub struct WeakGpu<T> {
    pub(crate) id: Id<T>,
    pub(crate) notifier_index: usize,
    pub(crate) notify: async_channel::Sender<usize>,
    pub(crate) update: Weak<Mutex<Vec<SlabUpdate>>>,
}

impl<T> Clone for WeakGpu<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            notifier_index: self.notifier_index,
            notify: self.notify.clone(),
            update: self.update.clone(),
        }
    }
}

impl<T> WeakGpu<T> {
    pub fn from_gpu(gpu: &Gpu<T>) -> Self {
        Self {
            id: gpu.id,
            notifier_index: gpu.notifier_index,
            notify: gpu.notify.clone(),
            update: Arc::downgrade(&gpu.update),
        }
    }

    pub fn upgrade(&self) -> Option<Gpu<T>> {
        Some(Gpu {
            id: self.id,
            notifier_index: self.notifier_index,
            notify: self.notify.clone(),
            update: self.update.upgrade()?,
        })
    }
}

#[derive(Debug)]
pub struct WeakHybrid<T> {
    pub(crate) weak_cpu: Weak<RwLock<T>>,
    pub(crate) weak_gpu: WeakGpu<T>,
}

impl<T> Clone for WeakHybrid<T> {
    fn clone(&self) -> Self {
        Self {
            weak_cpu: self.weak_cpu.clone(),
            weak_gpu: self.weak_gpu.clone(),
        }
    }
}

impl<T> WeakHybrid<T> {
    pub fn id(&self) -> Id<T> {
        self.weak_gpu.id
    }

    pub fn from_hybrid(h: &Hybrid<T>) -> Self {
        Self {
            weak_cpu: Arc::downgrade(&h.cpu_value),
            weak_gpu: WeakGpu::from_gpu(&h.gpu_value),
        }
    }

    pub fn upgrade(&self) -> Option<Hybrid<T>> {
        Some(Hybrid {
            cpu_value: self.weak_cpu.upgrade()?,
            gpu_value: self.weak_gpu.upgrade()?,
        })
    }

    pub fn strong_count(&self) -> usize {
        self.weak_gpu.update.strong_count()
    }
}

/// A "hybrid" type that lives on the CPU and the GPU.
///
/// Updates are syncronized to the GPU once per frame.
///
/// Clones of a hybrid all point to the same CPU and GPU data.
pub struct Hybrid<T> {
    pub(crate) cpu_value: Arc<RwLock<T>>,
    pub(crate) gpu_value: Gpu<T>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Hybrid<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Hybrid<{}>", std::any::type_name::<T>()))
            .field("id", &self.gpu_value.id)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .finish()
    }
}

impl<T> Clone for Hybrid<T> {
    fn clone(&self) -> Self {
        Hybrid {
            cpu_value: self.cpu_value.clone(),
            gpu_value: self.gpu_value.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> Hybrid<T> {
    pub fn new(mngr: &SlabAllocator<impl IsBuffer>, value: T) -> Self {
        let cpu_value = Arc::new(RwLock::new(value.clone()));
        let gpu_value = Gpu::new(mngr, value);
        Self {
            cpu_value,
            gpu_value,
        }
    }

    /// Returns the number of clones of this Hybrid on the CPU.
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.gpu_value.update)
    }

    pub fn id(&self) -> Id<T> {
        self.gpu_value.id()
    }

    pub fn get(&self) -> T {
        self.cpu_value.read().unwrap().clone()
    }

    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut value_guard = self.cpu_value.write().unwrap();
        f(&mut value_guard);
        let t = value_guard.clone();
        self.gpu_value.set(t);
    }

    pub fn set(&self, value: T) {
        self.modify(move |old| {
            *old = value;
        })
    }

    /// Drop the CPU portion of the hybrid value, returning a type that wraps
    /// only the GPU resources.
    pub fn into_gpu_only(self) -> Gpu<T> {
        self.gpu_value
    }
}

/// A type that lives on the GPU.
///
/// Updates are synchronized to the GPU during [`SlabAllocator::upkeep`].
pub struct Gpu<T> {
    pub(crate) id: Id<T>,
    pub(crate) notifier_index: usize,
    pub(crate) notify: async_channel::Sender<usize>,
    pub(crate) update: Arc<Mutex<Vec<SlabUpdate>>>,
}

impl<T> Drop for Gpu<T> {
    fn drop(&mut self) {
        let _ = self.notify.try_send(self.notifier_index);
    }
}

impl<T> Clone for Gpu<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            notifier_index: self.notifier_index,
            notify: self.notify.clone(),
            update: self.update.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> Gpu<T> {
    pub fn new(mngr: &SlabAllocator<impl IsBuffer>, value: T) -> Self {
        let id = mngr.allocate::<T>();
        let notifier_index = mngr.next_update_k();
        let s = Self {
            id,
            notifier_index,
            notify: mngr.notifier.0.clone(),
            update: Default::default(),
        };
        s.set(value);
        mngr.insert_update_source(notifier_index, WeakGpuRef::from_gpu(&s));
        s
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn set(&self, value: T) {
        // UNWRAP: panic on purpose
        *self.update.lock().unwrap() = vec![SlabUpdate {
            array: Array::new(self.id.inner(), T::SLAB_SIZE as u32),
            elements: {
                let mut es = vec![0u32; T::SLAB_SIZE];
                es.write(Id::new(0), &value);
                es
            },
        }];
        // UNWRAP: safe because it's unbound
        self.notify.try_send(self.notifier_index).unwrap();
    }
}

/// A array type that lives on the GPU.
///
/// Once created, the array cannot be resized.
///
/// Updates are syncronized to the GPU once per frame.
pub struct GpuArray<T> {
    array: Array<T>,
    notifier_index: usize,
    notifier: async_channel::Sender<usize>,
    updates: Arc<Mutex<Vec<SlabUpdate>>>,
}

impl<T> Drop for GpuArray<T> {
    fn drop(&mut self) {
        let _ = self.notifier.try_send(self.notifier_index);
    }
}

impl<T> Clone for GpuArray<T> {
    fn clone(&self) -> Self {
        GpuArray {
            notifier: self.notifier.clone(),
            notifier_index: self.notifier_index,
            array: self.array,
            updates: self.updates.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> GpuArray<T> {
    pub fn new(mngr: &SlabAllocator<impl IsBuffer>, values: &[T]) -> Self {
        let array = mngr.allocate_array::<T>(values.len());
        let update = {
            let mut elements = vec![0u32; T::SLAB_SIZE * array.len()];
            elements.write_indexed_slice(values, 0);
            SlabUpdate {
                array: array.into_u32_array(),
                elements,
            }
        };
        let notifier_index = mngr.next_update_k();
        let g = GpuArray {
            notifier_index,
            notifier: mngr.notifier.0.clone(),
            array,
            updates: Arc::new(Mutex::new(vec![update])),
        };
        mngr.insert_update_source(notifier_index, WeakGpuRef::from_gpu_array(&g));
        g
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn array(&self) -> Array<T> {
        self.array
    }

    pub fn get_id(&self, index: usize) -> Id<T> {
        self.array().at(index)
    }

    pub fn set_item(&self, index: usize, value: &T) {
        let id = self.array.at(index);
        let array = Array::<u32>::new(id.inner(), T::SLAB_SIZE as u32);
        let mut elements = vec![0u32; T::SLAB_SIZE];
        elements.write(0u32.into(), value);
        self.updates
            .lock()
            .unwrap()
            .push(SlabUpdate { array, elements });
        // UNWRAP: safe because it's unbounded
        self.notifier.try_send(self.notifier_index).unwrap();
    }
}

/// A "hybrid" array type that lives on the CPU and the GPU.
///
/// Once created, the array cannot be resized.
///
/// Updates are syncronized to the GPU once per frame.
pub struct HybridArray<T> {
    cpu_value: Arc<RwLock<Vec<T>>>,
    gpu_value: GpuArray<T>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for HybridArray<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("HybridArray<{}>", std::any::type_name::<T>()))
            .field("array", &self.gpu_value.array)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .finish()
    }
}

impl<T> Clone for HybridArray<T> {
    fn clone(&self) -> Self {
        HybridArray {
            cpu_value: self.cpu_value.clone(),
            gpu_value: self.gpu_value.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> HybridArray<T> {
    pub fn new(mngr: &SlabAllocator<impl IsBuffer>, values: impl IntoIterator<Item = T>) -> Self {
        let values = values.into_iter().collect::<Vec<_>>();
        let gpu_value = GpuArray::<T>::new(mngr, &values);
        let cpu_value = Arc::new(RwLock::new(values));
        HybridArray {
            cpu_value,
            gpu_value,
        }
    }

    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.gpu_value.updates)
    }

    pub fn len(&self) -> usize {
        self.gpu_value.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.gpu_value.is_empty()
    }

    pub fn array(&self) -> Array<T> {
        self.gpu_value.array()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.cpu_value.read().unwrap().get(index).cloned()
    }

    pub fn get_id(&self, index: usize) -> Id<T> {
        self.gpu_value.get_id(index)
    }

    pub fn modify<S>(&self, index: usize, f: impl FnOnce(&mut T) -> S) -> Option<S> {
        let mut value_guard = self.cpu_value.write().unwrap();
        let t = value_guard.get_mut(index)?;
        let output = Some(f(t));
        self.gpu_value.set_item(index, t);
        output
    }

    pub fn set_item(&self, index: usize, value: T) -> Option<T> {
        self.modify(index, move |t| std::mem::replace(t, value))
    }

    pub fn into_gpu_only(self) -> GpuArray<T> {
        self.gpu_value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mngr_updates_count_sanity() {
        let mngr = SlabAllocator::<Mutex<Vec<u32>>>::default();
        {
            let value = mngr.new_value(666u32);
            assert_eq!(
                1,
                value.ref_count(),
                "slab should not retain a count on value"
            );
        }
        let _ = mngr.upkeep(());
        assert_eq!(
            0,
            mngr.update_sources.read().unwrap().len(),
            "value should have dropped with no refs"
        );
        {
            let values = mngr.new_array([666u32, 420u32]);
            assert_eq!(
                1,
                values.ref_count(),
                "slab should not retain a count on array"
            );
        }
        let _ = mngr.upkeep(());
        assert_eq!(
            0,
            mngr.update_sources.read().unwrap().len(),
            "array should have dropped with no refs"
        );
    }

    #[test]
    fn range_sanity() {
        let a = Range {
            first_index: 1,
            last_index: 2,
        };
        let b = Range {
            first_index: 0,
            last_index: 0,
        };
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }

    #[test]
    fn slab_manager_sanity() {
        let m = SlabAllocator::<Mutex<Vec<u32>>>::default();
        log::info!("allocating 4 unused u32 slots");
        let _ = m.allocate::<u32>();
        let _ = m.allocate::<u32>();
        let _ = m.allocate::<u32>();
        let _ = m.allocate::<u32>();

        log::info!("creating 4 update sources");
        let h4 = m.new_value(0u32);
        let h5 = m.new_value(0u32);
        let h6 = m.new_value(0u32);
        let h7 = m.new_value(0u32);
        log::info!("running upkeep");
        let _ = m.upkeep(());
        assert!(m.recycles.read().unwrap().ranges.is_empty());
        assert_eq!(4, m.update_sources.read().unwrap().len());
        let k = m.update_k.load(Ordering::Relaxed);
        assert_eq!(4, k);

        log::info!("dropping 4 update sources");
        drop(h4);
        drop(h5);
        drop(h6);
        drop(h7);
        let _ = m.upkeep(());
        assert_eq!(1, m.recycles.read().unwrap().ranges.len());
        assert!(m.update_sources.read().unwrap().is_empty());

        log::info!("creating 4 update sources, round two");
        let h4 = m.new_value(0u32);
        let h5 = m.new_value(0u32);
        let h6 = m.new_value(0u32);
        let h7 = m.new_value(0u32);
        assert!(m.recycles.read().unwrap().ranges.is_empty());
        assert_eq!(4, m.update_sources.read().unwrap().len());
        let k = m.update_k.load(Ordering::Relaxed);
        // MAYBE_TODO: recycle "update_k"s instead of incrementing for each new source
        assert_eq!(8, k);

        log::info!("creating one more update source, immediately dropping it and two others");
        let h8 = m.new_value(0u32);
        drop(h8);
        drop(h4);
        drop(h6);
        let _ = m.upkeep(());
        assert_eq!(3, m.recycles.read().unwrap().ranges.len());
        assert_eq!(2, m.update_sources.read().unwrap().len());
        assert_eq!(9, m.update_k.load(Ordering::Relaxed));

        drop(h7);
        drop(h5);
        let _ = m.upkeep(());
        m.defrag();
        assert_eq!(
            1,
            m.recycles.read().unwrap().ranges.len(),
            "ranges: {:#?}",
            m.recycles.read().unwrap().ranges
        );
    }
}
