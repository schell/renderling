//! Allocated values.

use std::sync::{Arc, Mutex, RwLock, Weak};

use crabslab::{Array, Id, Slab, SlabItem};

use crate::{
    runtime::{IsRuntime, SlabUpdate},
    slab::SlabAllocator,
};

pub struct WeakGpuRef {
    pub(crate) u32_array: Array<u32>,
    pub(crate) weak: Weak<Mutex<Vec<SlabUpdate>>>,
    pub(crate) takes_update: bool,
}

impl WeakGpuRef {
    /// Take any queued updates.
    pub fn get_update(&self) -> Option<Vec<SlabUpdate>> {
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

    fn from_gpu<T: SlabItem>(gpu: &Gpu<T>) -> Self {
        WeakGpuRef {
            u32_array: Array::new(gpu.id.inner(), T::SLAB_SIZE as u32),
            weak: Arc::downgrade(&gpu.update),
            takes_update: true,
        }
    }

    fn from_gpu_array<T: SlabItem>(gpu_array: &GpuArray<T>) -> Self {
        WeakGpuRef {
            u32_array: gpu_array.array.into_u32_array(),
            weak: Arc::downgrade(&gpu_array.updates),
            takes_update: true,
        }
    }
}

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
    pub fn new(mngr: &SlabAllocator<impl IsRuntime>, value: T) -> Self {
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

    pub fn modify<A: std::any::Any>(&self, f: impl FnOnce(&mut T) -> A) -> A {
        let mut value_guard = self.cpu_value.write().unwrap();
        let a = f(&mut value_guard);
        let t = value_guard.clone();
        self.gpu_value.set(t);
        a
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
    pub fn new(mngr: &SlabAllocator<impl IsRuntime>, value: T) -> Self {
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
#[derive(Debug)]
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
    pub fn new(mngr: &SlabAllocator<impl IsRuntime>, values: &[T]) -> Self {
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
    pub fn new(mngr: &SlabAllocator<impl IsRuntime>, values: impl IntoIterator<Item = T>) -> Self {
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

    pub fn get_vec(&self) -> Vec<T> {
        self.cpu_value.read().unwrap().clone()
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
