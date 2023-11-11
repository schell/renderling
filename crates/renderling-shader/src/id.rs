//! Typed identifiers that can also be used as indices.
use core::marker::PhantomData;

use crate::{self as renderling_shader, slab::Slabbed};

pub const ID_NONE: u32 = u32::MAX;

/// An identifier.
#[repr(transparent)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Slabbed)]
pub struct Id<T>(pub(crate) u32, PhantomData<T>);

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> core::hash::Hash for Id<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> From<Id<T>> for u32 {
    fn from(value: Id<T>) -> Self {
        value.0
    }
}

impl<T> From<usize> for Id<T> {
    fn from(value: usize) -> Self {
        Id::new(value as u32)
    }
}

impl<T> From<u32> for Id<T> {
    fn from(value: u32) -> Self {
        Id::new(value)
    }
}

/// `Id::NONE` is the default.
impl<T> Default for Id<T> {
    fn default() -> Self {
        Id::NONE
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("Id<{}>", std::any::type_name::<T>()))
            .field(&self.0)
            .finish()
    }
}

impl<T> Id<T> {
    pub const NONE: Self = Id::new(ID_NONE);

    pub const fn new(i: u32) -> Self {
        Id(i, PhantomData)
    }

    /// Convert this id into a usize for use as an index.
    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn is_none(&self) -> bool {
        *self == Id::NONE
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::stage::GpuEntity;

    use super::*;

    #[test]
    fn id_size() {
        assert_eq!(
            std::mem::size_of::<u32>(),
            std::mem::size_of::<Id<GpuEntity>>(),
            "id is not u32"
        );
    }
}
