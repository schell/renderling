//! GPU locks and atomics.

use crabslab::Id;

/// A spin lock.
pub struct SpinLock<'a> {
    lock_id: Id<u32>,
    slab: &'a mut [u32],
}

// TODO: find out why Drop doesn't work in rust-gpu
impl<'a> Drop for SpinLock<'a> {
    fn drop(&mut self) {
        self.release();
    }
}

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

#[cfg(feature = "test-sync")]
/// Test using the spin lock.
#[spirv_std::spirv(compute(threads(16)))]
pub fn test_spin_lock(#[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &mut [u32]) {
    use crabslab::Slab;

    {
        let mut lock = SpinLock::acquire(slab, Id::new(0));
        let id_f32: Id<f32> = Id::new(1);
        let read_f32 = lock.slab().read_unchecked(id_f32);
        lock.slab().write(id_f32, &(read_f32 + 1.0));
    }
}

/// Perform an [atomic_i_increment](spirv_std::arch::atomic_i_increment) operation.
///
/// ## Note
/// This is **not** atomic on CPU.
pub fn atomic_i_increment<const SCOPE: u32, const SEMANTICS: u32>(
    slab: &mut [u32],
    id: Id<u32>,
) -> u32 {
    #[cfg(gpu)]
    {
        let ptr = &mut slab[id.index()];
        unsafe { spirv_std::arch::atomic_i_increment::<u32, SCOPE, SEMANTICS>(ptr) }
    }
    #[cfg(cpu)]
    {
        let prev = slab[id.index()];
        slab[id.index()] = prev + 1;
        prev
    }
}

/// Perform an [atomic_u_min](spirv_std::arch::atomic_u_min) operation.
///
/// ## Note
/// This is **not** atomic on CPU.
pub fn atomic_u_min<const SCOPE: u32, const SEMANTICS: u32>(
    slab: &mut [u32],
    id: Id<u32>,
    val: u32,
) -> u32 {
    #[cfg(gpu)]
    {
        let ptr = &mut slab[id.index()];
        unsafe { spirv_std::arch::atomic_u_min::<u32, SCOPE, SEMANTICS>(ptr, val) }
    }
    #[cfg(cpu)]
    {
        let prev = slab[id.index()];
        let new = prev.min(val);
        slab[id.index()] = new;
        prev
    }
}

/// Perform an [atomic_u_max](spirv_std::arch::atomic_u_max) operation.
///
/// ## Note
/// This is **not** atomic on CPU.
pub fn atomic_u_max<const SCOPE: u32, const SEMANTICS: u32>(
    slab: &mut [u32],
    id: Id<u32>,
    val: u32,
) -> u32 {
    #[cfg(gpu)]
    {
        let ptr = &mut slab[id.index()];
        unsafe { spirv_std::arch::atomic_u_max::<u32, SCOPE, SEMANTICS>(ptr, val) }
    }
    #[cfg(cpu)]
    {
        let prev = slab[id.index()];
        let new = prev.max(val);
        slab[id.index()] = new;
        prev
    }
}
