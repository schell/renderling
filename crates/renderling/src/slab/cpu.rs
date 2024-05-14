//! Abstraction over a GPU buffer that provides near-automatic synchronization
//! and RAII memory management.
use core::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};
use crabslab::{Slab, SlabItem};
use rustc_hash::FxHashMap;
use snafu::prelude::*;
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock};

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
            !self.intersects(&other),
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

/// Manages slab allocations and updates.
///
/// Create a new instance using [`SlabAllocator::default`].
/// Upon creation you will need to call [`SlabAllocator::get_updated_buffer`] or
/// [`SlabAllocator::upkeep`] at least once before any data is written to the
/// GPU.
#[derive(Clone)]
pub struct SlabAllocator {
    pub(crate) notifier: (async_channel::Sender<usize>, async_channel::Receiver<usize>),
    len: Arc<AtomicUsize>,
    capacity: Arc<AtomicUsize>,
    needs_expansion: Arc<AtomicBool>,
    buffer: Arc<RwLock<Option<Arc<wgpu::Buffer>>>>,
    update_k: Arc<AtomicUsize>,
    update_sources: Arc<RwLock<FxHashMap<usize, Box<dyn UpdatesSlab>>>>,
    recycles: Arc<RwLock<RangeManager<Range>>>,
}

impl Default for SlabAllocator {
    fn default() -> Self {
        Self {
            notifier: async_channel::unbounded(),
            update_k: Default::default(),
            update_sources: Default::default(),
            recycles: Default::default(),
            len: Default::default(),
            capacity: Default::default(),
            needs_expansion: Arc::new(true.into()),
            buffer: Default::default(),
        }
    }
}

impl SlabAllocator {
    pub(crate) fn next_update_k(&self) -> usize {
        self.update_k.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) fn insert_update_source(&self, k: usize, source: impl UpdatesSlab) {
        log::debug!(
            "inserting update source {k} {}",
            std::any::type_name_of_val(&source)
        );
        let _ = self.notifier.0.try_send(k);
        // UNWRAP: panic on purpose
        self.update_sources
            .write()
            .unwrap()
            .insert(k, Box::new(source));
    }

    fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    pub(crate) fn allocate<T: SlabItem>(&mut self) -> Id<T> {
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

    fn allocate_array<T: SlabItem>(&mut self, len: usize) -> Array<T> {
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

    fn maybe_expand_to_fit<T: SlabItem>(&mut self, len: usize) {
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
    pub fn get_buffer(&self) -> Option<Arc<wgpu::Buffer>> {
        self.buffer.read().unwrap().as_ref().cloned()
    }

    /// Return an updated buffer.
    ///
    /// This is the only way to guarantee access to a buffer.
    ///
    /// Use [`SlabAllocator::upkeep`] when you only need the buffer after a
    /// change, for example to recreate bindgroups.
    pub fn get_updated_buffer(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usage: wgpu::BufferUsages,
    ) -> Arc<wgpu::Buffer> {
        if let Some(new_buffer) = self.upkeep(device, queue, label, usage) {
            new_buffer
        } else {
            // UNWRAP: safe because we know the buffer exists at this point,
            // as we've called `upkeep` above
            self.get_buffer().unwrap()
        }
    }

    /// Recreate this buffer, writing the contents of the previous buffer (if it
    /// exists) to the new one, then return the new buffer.
    fn recreate_buffer(
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
            encoder.copy_buffer_to_buffer(&old_buffer, 0, &new_buffer, 0, old_buffer.size());
            queue.submit(std::iter::once(encoder.finish()));
        }
        *guard = Some(new_buffer.clone());
        new_buffer
    }

    /// Stage a new value that lives on the GPU _and_ CPU.
    pub fn new_value<T: SlabItem + Clone + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Hybrid<T> {
        Hybrid::new(self, value)
    }

    /// Stage a contiguous array of new values that live on the GPU _and_ CPU.
    pub fn new_array<T: SlabItem + Clone + Send + Sync + 'static>(
        &mut self,
        values: impl IntoIterator<Item = T>,
    ) -> HybridArray<T> {
        HybridArray::new(self, values)
    }

    /// Perform upkeep on the slab, commiting changes to the GPU.
    ///
    /// Returns the new buffer if one was created due to a capacity resize.
    #[must_use]
    pub fn upkeep(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usage: wgpu::BufferUsages,
    ) -> Option<Arc<wgpu::Buffer>> {
        let new_buffer = if self.needs_expansion.swap(false, Ordering::Relaxed) {
            Some(self.recreate_buffer(device, queue, label, usage))
        } else {
            None
        };

        // Build the set of sources that require updates
        let mut update_set = rustc_hash::FxHashSet::<usize>::default();
        while let Ok(source_index) = self.notifier.1.try_recv() {
            update_set.insert(source_index);
        }
        if !update_set.is_empty() {
            log::trace!("sources {:?}", update_set);
        }
        // Prepare all of our GPU buffer writes
        let mut writes = RangeManager::<SlabUpdate>::default();
        {
            // Recycle any update sources that are no longer needed, and collect the active
            // sources' updates into `writes`.
            let mut updates_guard = self.update_sources.write().unwrap();
            let mut recycles_guard = self.recycles.write().unwrap();
            for key in update_set {
                let delete = if let Some(hybrid) = updates_guard.get_mut(&key) {
                    let count = hybrid.strong_count();
                    if count <= 1 {
                        // recycle this allocation
                        let input_array = hybrid.u32_array();
                        log::debug!("recycling {key} {input_array:?}",);
                        if hybrid.u32_array().is_null() || hybrid.u32_array().is_empty() {
                            log::debug!("  cannot recycle - empty or null");
                            true
                        } else {
                            recycles_guard.add_range(input_array.into());
                            true
                        }
                    } else {
                        hybrid
                            .get_update()
                            .into_iter()
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
            let ranges = std::mem::replace(&mut recycles_guard.ranges, vec![]);
            let num_ranges_to_defrag = ranges.len();
            for range in ranges.into_iter() {
                recycles_guard.add_range(range);
            }
            let num_ranges = recycles_guard.ranges.len();
            if num_ranges < num_ranges_to_defrag {
                log::trace!("{num_ranges_to_defrag} ranges before, {num_ranges} after");
            }
        }

        if !writes.ranges.is_empty() {
            // UNWRAP: safe because we know the buffer exists at this point, as we may have
            // recreated it above^
            let buffer = self.get_buffer().unwrap();
            for SlabUpdate { array, elements } in writes.ranges.into_iter() {
                let offset = array.starting_index() as u64 * std::mem::size_of::<u32>() as u64;
                queue.write_buffer(&buffer, offset, bytemuck::cast_slice(&elements));
            }
            queue.submit(std::iter::empty());
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

pub(crate) struct SlabUpdate {
    pub(crate) array: Array<u32>,
    pub(crate) elements: Vec<u32>,
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

pub(crate) trait UpdatesSlab: Send + Sync + std::any::Any {
    /// Returns the slab range of all possible updates.
    fn u32_array(&self) -> Array<u32>;

    /// Returns the number of references remaiting in the wild.
    fn strong_count(&self) -> usize;

    /// Return the latest update, if any.
    fn get_update(&self) -> Vec<SlabUpdate>;
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for Gpu<T> {
    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.update)
    }

    fn u32_array(&self) -> Array<u32> {
        Array::new(self.id.inner(), T::SLAB_SIZE as u32)
    }

    fn get_update(&self) -> Vec<SlabUpdate> {
        self.update.lock().unwrap().take().into_iter().collect()
    }
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for Hybrid<T> {
    fn strong_count(&self) -> usize {
        self.gpu_value.strong_count()
    }

    fn u32_array(&self) -> Array<u32> {
        self.gpu_value.u32_array()
    }

    fn get_update(&self) -> Vec<SlabUpdate> {
        self.gpu_value.get_update()
    }
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for GpuArray<T> {
    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.updates)
    }

    fn u32_array(&self) -> Array<u32> {
        self.array.into_u32_array()
    }

    fn get_update(&self) -> Vec<SlabUpdate> {
        std::mem::take(self.updates.lock().unwrap().as_mut())
    }
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for HybridArray<T> {
    fn strong_count(&self) -> usize {
        self.gpu_value.strong_count()
    }

    fn u32_array(&self) -> Array<u32> {
        self.gpu_value.u32_array()
    }

    fn get_update(&self) -> Vec<SlabUpdate> {
        self.gpu_value.get_update()
    }
}

/// A "hybrid" type that lives on the CPU and the GPU.
///
/// Updates are syncronized to the GPU once per frame.
///
/// Clones of a hybrid all point to the same CPU and GPU data.
pub struct Hybrid<T> {
    cpu_value: Arc<RwLock<T>>,
    gpu_value: Gpu<T>,
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
    pub fn new(mngr: &mut SlabAllocator, value: T) -> Self {
        let cpu_value = Arc::new(RwLock::new(value.clone()));
        let gpu_value = Gpu::new(mngr, value);
        Self {
            cpu_value,
            gpu_value,
        }
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
    pub(crate) update: Arc<Mutex<Option<SlabUpdate>>>,
}

impl<T> Drop for Gpu<T> {
    fn drop(&mut self) {
        let _ = self.notify.try_send(self.notifier_index);
    }
}

impl<T> Clone for Gpu<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            notifier_index: self.notifier_index,
            notify: self.notify.clone(),
            update: self.update.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> Gpu<T> {
    pub fn new(mngr: &mut SlabAllocator, value: T) -> Self {
        let id = mngr.allocate::<T>();
        let notifier_index = mngr.next_update_k();
        let s = Self {
            id,
            notifier_index,
            notify: mngr.notifier.0.clone(),
            update: Default::default(),
        };
        s.set(value);
        mngr.insert_update_source(notifier_index, s.clone());
        s
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn set(&self, value: T) {
        // UNWRAP: panic on purpose
        *self.update.lock().unwrap() = Some(SlabUpdate {
            array: Array::new(self.id.inner(), T::SLAB_SIZE as u32),
            elements: {
                let mut es = vec![0u32; T::SLAB_SIZE];
                es.write(Id::new(0), &value);
                es
            },
        });
        // UNWRAP: safe because it's unbound
        self.notify.try_send(self.notifier_index).unwrap();
    }

    /// Pair with a CPU value.
    ///
    /// ## Warning
    /// No effort is made to ensure that the value provided is the same as the
    /// value on the GPU.
    pub fn into_hybrid(self, value: T) -> Hybrid<T> {
        let cpu_value = Arc::new(RwLock::new(value));
        Hybrid {
            cpu_value,
            gpu_value: self,
        }
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
    pub fn new(mngr: &mut SlabAllocator, values: &[T]) -> Self {
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
        mngr.insert_update_source(notifier_index, g.clone());
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

    /// Pair with a CPU value.
    ///
    /// ## Warning
    /// No effort is made to ensure that the value provided is the same as the
    /// value on the GPU.
    pub fn into_hybrid(self, values: impl IntoIterator<Item = T>) -> HybridArray<T> {
        let cpu_value = Arc::new(RwLock::new(values.into_iter().collect()));
        HybridArray {
            cpu_value,
            gpu_value: self,
        }
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
    pub fn new(mngr: &mut SlabAllocator, values: impl IntoIterator<Item = T>) -> Self {
        let values = values.into_iter().collect::<Vec<_>>();
        let gpu_value = GpuArray::<T>::new(mngr, &values);
        let cpu_value = Arc::new(RwLock::new(values));
        HybridArray {
            cpu_value,
            gpu_value,
        }
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
        self.gpu_value.set_item(index, &t);
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
    use crate::Context;

    use super::*;

    #[test]
    fn mngr_updates_count_sanity() {
        let r = Context::headless(1, 1);
        let mut mngr = SlabAllocator::default();
        {
            let value = mngr.new_value(666u32);
            assert_eq!(2, value.strong_count());
        }
        let _ = mngr.upkeep(
            r.get_device(),
            r.get_queue(),
            Some("mngr updates count sanity 1"),
            wgpu::BufferUsages::empty(),
        );
        assert_eq!(0, mngr.update_sources.read().unwrap().len());
        {
            let values = mngr.new_array([666u32, 420u32]);
            assert_eq!(2, values.strong_count());
        }
        let _ = mngr.upkeep(
            r.get_device(),
            r.get_queue(),
            Some("mngr updates count sanity 2"),
            wgpu::BufferUsages::empty(),
        );
        assert_eq!(0, mngr.update_sources.read().unwrap().len());
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
        let r = Context::headless(1, 1);
        let mut m = SlabAllocator::default();
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
        let _ = m.upkeep(
            r.get_device(),
            r.get_queue(),
            None,
            wgpu::BufferUsages::empty(),
        );
        assert!(m.recycles.read().unwrap().ranges.is_empty());
        assert_eq!(4, m.update_sources.read().unwrap().len());
        let k = m.update_k.load(Ordering::Relaxed);
        assert_eq!(4, k);

        log::info!("dropping 4 update sources");
        drop(h4);
        drop(h5);
        drop(h6);
        drop(h7);
        let _ = m.upkeep(
            r.get_device(),
            r.get_queue(),
            None,
            wgpu::BufferUsages::empty(),
        );
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
        let _ = m.upkeep(
            r.get_device(),
            r.get_queue(),
            None,
            wgpu::BufferUsages::empty(),
        );
        assert_eq!(3, m.recycles.read().unwrap().ranges.len());
        assert_eq!(2, m.update_sources.read().unwrap().len());
        assert_eq!(9, m.update_k.load(Ordering::Relaxed));

        drop(h7);
        drop(h5);
        let _ = m.upkeep(
            r.get_device(),
            r.get_queue(),
            None,
            wgpu::BufferUsages::empty(),
        );
        m.defrag();
        assert_eq!(
            1,
            m.recycles.read().unwrap().ranges.len(),
            "ranges: {:#?}",
            m.recycles.read().unwrap().ranges
        );
    }
}
