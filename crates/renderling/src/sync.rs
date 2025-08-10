//! GPU locks and atomics.

use crabslab::Id;

/// A spin lock.
pub struct SpinLock<'a> {
    lock_id: Id<u32>,
    slab: &'a mut [u32],
}

// TODO: find out why Drop doesn't work in rust-gpu
// impl<'a> Drop for SpinLock<'a> {
//     // fn drop(&mut self) {
//     //     self.release();
//     // }
// }

impl<'a> SpinLock<'a> {
    /// Acquire the spin lock at the given `Id<u32>`.
    ///
    /// If the lock is already taken, this will loop until unique access is possible.
    ///
    /// ## Panics
    /// This is GPU-only. Panics on CPU.
    pub fn acquire(slab: &'a mut [u32], lock_id: Id<u32>) -> Self {
        loop {
            let lock_ptr = &mut slab[lock_id.index()];
            let prev_lock = unsafe {
                spirv_std::arch::atomic_exchange::<
                    u32,
                    { spirv_std::memory::Scope::Workgroup as u32 },
                    // TODO: read about semantics and figure out if this is correct
                    { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                    // { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                >(lock_ptr, 1)
            };

            if prev_lock == 0 {
                return SpinLock { lock_id, slab };
            }
        }
    }

    /// Release the spin lock.
    pub fn release(&mut self) {
        let lock_ptr = &mut self.slab[self.lock_id.index()];

        loop {
            let prev_lock = unsafe {
                spirv_std::arch::atomic_exchange::<
                    u32,
                    { spirv_std::memory::Scope::Workgroup as u32 },
                    // TODO: read about semantics and figure out if this is correct
                    { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                    // { spirv_std::memory::Semantics::WORKGROUP_MEMORY.bits() },
                >(lock_ptr, 0)
            };

            if prev_lock == 1 {
                return;
            }
        }
    }

    /// Access the slab.
    pub fn slab(&mut self) -> &mut [u32] {
        self.slab
    }
}

/// Test using the spin lock.
#[spirv_std::spirv(compute(threads(16)))]
pub fn test_spin_lock(#[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &mut [u32]) {
    use crabslab::Slab;

    let mut lock = SpinLock::acquire(slab, Id::new(0));
    let id_f32: Id<f32> = Id::new(1);
    let read_f32 = lock.slab().read_unchecked(id_f32);
    lock.slab().write(id_f32, &(read_f32 + 1.0));
}
