//! A slab-allocated array.
use core::marker::PhantomData;

use crate as renderling_shader;
use crate::slab::FromSlab;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, FromSlab)]
pub struct Array<T: FromSlab> {
    index: u32,
    len: u32,
    _phantom: PhantomData<T>,
}

unsafe impl<T: FromSlab + bytemuck::Pod + bytemuck::Zeroable> bytemuck::Pod for Array<T> {}

impl<T: FromSlab> Default for Array<T> {
    fn default() -> Self {
        Self {
            index: u32::MAX,
            len: 0,
            _phantom: PhantomData,
        }
    }
}

impl<T: FromSlab> Array<T> {
    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains_index(&self, index: usize) -> bool {
        index >= self.index as usize && index < (self.index + self.len) as usize
    }
}

impl<T: FromSlab> Array<T> {
    fn slab_size() -> usize {
        2
    }
    pub fn extract(&self, item: &mut T, index: usize, slab: &[u32]) {
        let size = T::slab_size();
        let start = self.index as usize + size * index;
        let _ = item.read_slab(start, slab);
    }
}
