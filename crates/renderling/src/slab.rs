//! Abstraction over a GPU buffer that provides near-automatic synchronization
//! and RAII memory management.
use core::ops::Deref;
use crabslab::{Array, CpuSlab, GrowableSlab, Slab, SlabItem, WgpuBuffer, WgpuSlabError};
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock, RwLockReadGuard};

use crabslab::Id;

use crate::{Device, Queue};

#[derive(Clone)]
pub struct SlabManager {
    pub(crate) slab: Arc<RwLock<crabslab::CpuSlab<crabslab::WgpuBuffer>>>,
    pub(crate) updates: Arc<RwLock<Vec<Box<dyn UpdatesSlab>>>>,
    pub(crate) recycles: Arc<RwLock<Vec<Array<u32>>>>,
}

impl crabslab::Slab for SlabManager {
    fn len(&self) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().len()
    }

    fn read_unchecked<T: SlabItem>(&self, id: Id<T>) -> T {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().read_unchecked(id)
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().write_indexed(t, index)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().write_indexed_slice(t, index)
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
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().capacity()
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().reserve_capacity(capacity)
    }

    fn increment_len(&mut self, n: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().increment_len(n)
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

struct SlabManagerBuffer<'a> {
    inner: RwLockReadGuard<'a, CpuSlab<WgpuBuffer>>,
}

impl<'a> Deref for SlabManagerBuffer<'a> {
    type Target = wgpu::Buffer;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().get_buffer()
    }
}

impl SlabManager {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        Self {
            slab: Arc::new(RwLock::new(CpuSlab::new(WgpuBuffer::new(
                device, queue, 256,
            )))),
            updates: Default::default(),
            recycles: Default::default(),
        }
    }

    pub fn get_buffer(&self) -> impl Deref<Target = wgpu::Buffer> + '_ {
        SlabManagerBuffer {
            inner: self.slab.read().unwrap(),
        }
    }

    pub(crate) fn add_updates(&mut self, updates: impl UpdatesSlab) {
        self.updates.write().unwrap().push(Box::new(updates));
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

    pub(crate) fn upkeep(&mut self) {
        let writes = {
            let mut writes = vec![];
            let mut updates_guard = self.updates.write().unwrap();
            let mut recycles_guard = self.recycles.write().unwrap();
            updates_guard.retain(|hybrid| {
                writes.extend(hybrid.get_updates());
                let count = hybrid.strong_count();
                if count > 1 {
                    log::trace!(
                        "{} {:?} has count {count}",
                        hybrid.type_name(),
                        hybrid.array()
                    );
                    true
                } else {
                    // recycle this allocation
                    let input_array = hybrid.array();
                    log::trace!("recycling {} {input_array:?}", hybrid.type_name());
                    recycles_guard.push(input_array);
                    // TODO: combine recycled contiguous arrays
                    false
                }
            });
            writes
        };
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
            self.write_array(array, &elements);
        }
    }

    /// Read all the data from the stage.
    ///
    /// This blocks until the GPU buffer is mappable, and then copies the data
    /// into a vector.
    ///
    /// This is primarily used for debugging.
    pub fn read_slab(&self) -> Result<Vec<u32>, WgpuSlabError> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().as_ref().block_on_read_raw(..)
    }
}

pub struct SlabUpdate {
    array: Array<u32>,
    elements: Vec<u32>,
    #[cfg(debug_assertions)]
    type_is: &'static str,
}

pub trait UpdatesSlab: Send + Sync + std::any::Any {
    /// Returns all current updates, clearing the queue.
    fn get_updates(&self) -> Vec<SlabUpdate>;

    /// Returns the slab range of all possible updates.
    fn array(&self) -> Array<u32>;

    /// Returns the number of references remaiting in the wild.
    fn strong_count(&self) -> usize;

    /// Returns the type name of Self
    fn type_name(&self) -> &'static str;
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for Hybrid<T> {
    fn get_updates(&self) -> Vec<SlabUpdate> {
        if self.dirty.swap(false, std::sync::atomic::Ordering::Relaxed) {
            let t = self.get();
            let array = Array::<u32>::new(self.id.inner(), T::SLAB_SIZE as u32);
            let mut elements = vec![0u32; T::SLAB_SIZE];
            elements.write(0u32.into(), &t);
            vec![SlabUpdate {
                array,
                elements,
                #[cfg(debug_assertions)]
                type_is: std::any::type_name::<T>(),
            }]
        } else {
            vec![]
        }
    }

    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.cpu_value)
    }

    fn array(&self) -> Array<u32> {
        Array::new(self.id.inner(), T::SLAB_SIZE as u32)
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }
}

impl<T: SlabItem + Clone + Send + Sync + std::any::Any> UpdatesSlab for HybridArray<T> {
    fn get_updates(&self) -> Vec<SlabUpdate> {
        let mut guard = self.updates.lock().unwrap();
        let updates = std::mem::take(guard.as_mut());
        updates
    }

    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.cpu_value)
    }

    fn array(&self) -> Array<u32> {
        self.array.into_u32_array()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }
}

/// A "hybrid" type that lives on the CPU and the GPU.
///
/// Updates are syncronized to the GPU once per frame.
pub struct Hybrid<T> {
    cpu_value: Arc<RwLock<T>>,
    id: Id<T>,
    dirty: Arc<AtomicBool>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Hybrid<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Hybrid<{}>", std::any::type_name::<T>()))
            .field("id", &self.id)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .field("dirty", &self.get_dirty())
            .finish()
    }
}

impl<T> Clone for Hybrid<T> {
    fn clone(&self) -> Self {
        Hybrid {
            cpu_value: self.cpu_value.clone(),
            id: self.id,
            dirty: self.dirty.clone(),
        }
    }
}

impl<T> Hybrid<T> {
    pub(crate) fn set_dirty(&self, dirty: bool) {
        self.dirty
            .store(dirty, std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn get_dirty(&self) -> bool {
        self.dirty.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> Hybrid<T> {
    pub fn new(mngr: &mut SlabManager, value: T) -> Self {
        let id = mngr.allocate::<T>();
        let hybrid = Self::new_preallocated(id, value);
        mngr.add_updates(hybrid.clone());

        hybrid
    }

    pub(crate) fn new_preallocated(id: Id<T>, value: T) -> Self {
        Self {
            cpu_value: Arc::new(RwLock::new(value)),
            id,
            dirty: Arc::new(true.into()),
        }
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn get(&self) -> T {
        self.cpu_value.read().unwrap().clone()
    }

    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut value_guard = self.cpu_value.write().unwrap();
        self.set_dirty(true);
        f(&mut value_guard);
    }

    pub fn set(&self, value: T) {
        self.modify(move |old| {
            *old = value;
        })
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
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> HybridArray<T> {
    pub fn new(mngr: &mut SlabManager, values: impl IntoIterator<Item = T>) -> Self {
        let value = values.into_iter().collect::<Vec<_>>();
        let array = mngr.allocate_array::<T>(value.len());
        let hybrid = Self::new_preallocated(array, value);
        mngr.updates.write().unwrap().push(Box::new(hybrid.clone()));
        hybrid
    }

    pub(crate) fn new_preallocated(array: Array<T>, values: Vec<T>) -> Self {
        Self {
            array,
            updates: {
                let mut elements = vec![0u32; T::SLAB_SIZE * array.len()];
                elements.write_array(Array::new(0, array.len() as u32), &values);
                Arc::new(Mutex::new(vec![SlabUpdate {
                    array: array.into_u32_array(),
                    elements,
                    #[cfg(debug_assertions)]
                    type_is: std::any::type_name::<Vec<T>>(),
                }]))
            },
            cpu_value: Arc::new(RwLock::new(values)),
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
}

#[cfg(test)]
mod test {
    use crate::Renderling;

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
        let r = Renderling::headless(1, 1);
        let (device, queue) = r.get_device_and_queue_owned();
        let mut mngr = SlabManager::new(&device, &queue);
        {
            let value = mngr.new_hybrid(666u32);
            assert_eq!(2, value.strong_count());
        }
        mngr.upkeep();
        assert_eq!(0, mngr.updates.read().unwrap().len());
        {
            let values = mngr.new_hybrid_array([666u32, 420u32]);
            assert_eq!(2, values.strong_count());
        }
        mngr.upkeep();
        assert_eq!(0, mngr.updates.read().unwrap().len());
    }
}
