//! A slab-allocated array.
use core::marker::PhantomData;

use crate as renderling_shader;
use crate::id::Id;
use crate::slab::Slabbed;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, Slabbed)]
pub struct Array<T: Slabbed> {
    index: u32,
    len: u32,
    _phantom: PhantomData<T>,
}

unsafe impl<T: Slabbed + bytemuck::Pod + bytemuck::Zeroable> bytemuck::Pod for Array<T> {}

impl<T: Slabbed> Default for Array<T> {
    fn default() -> Self {
        Self {
            index: u32::MAX,
            len: 0,
            _phantom: PhantomData,
        }
    }
}

impl<T: Slabbed> Array<T> {
    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains_index(&self, index: usize) -> bool {
        index >= self.index as usize && index < (self.index + self.len) as usize
    }

    pub fn at(&self, index: usize) -> Id<T> {
        if index >= self.len() {
            Id::NONE
        } else {
            Id::new(self.index + index as u32)
        }
    }
}

impl<T: Slabbed> Array<T> {
    fn slab_size() -> usize {
        2
    }

    pub fn read(&self, item: &mut T, item_index: usize, slab: &[u32]) {
        let size = T::slab_size();
        let start = self.index as usize + size * item_index;
        let _ = item.read_slab(start, slab);
    }
}
