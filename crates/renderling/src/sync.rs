//! GPU locks and atomics.

use crabslab::Id;

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
