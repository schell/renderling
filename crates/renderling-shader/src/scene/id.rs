//! Typed identifiers that can also be used as indices.
use core::marker::PhantomData;

pub const ID_NONE: u32 = u32::MAX;

/// An identifier.
#[derive(Ord)]
#[repr(transparent)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Id<T>(pub(crate) u32, PhantomData<T>);

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
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
        // `u32` representing "null" or "none".
        self == &Id::NONE
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::scene::GpuEntity;

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