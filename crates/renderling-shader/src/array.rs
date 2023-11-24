//! A slab-allocated array.
use core::marker::PhantomData;

use crate::id::Id;
use crate::slab::Slabbed;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Array<T: Slabbed> {
    index: u32,
    len: u32,
    _phantom: PhantomData<T>,
}

impl<T: Slabbed> core::fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Array")
            .field("index", &self.index)
            .field("len", &self.len)
            .field("_phantom", &self._phantom)
            .finish()
    }
}

impl<T: Slabbed> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.len == other.len
    }
}

impl<T: Slabbed> Slabbed for Array<T> {
    fn slab_size() -> usize {
        2
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if index + Self::slab_size() >= slab.len() {
            index
        } else {
            let index = self.index.read_slab(index, slab);
            let index = self.len.read_slab(index, slab);
            index
        }
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if index + Self::slab_size() >= slab.len() {
            index
        } else {
            let index = self.index.write_slab(index, slab);
            let index = self.len.write_slab(index, slab);
            index
        }
    }
}

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
    pub fn new(index: u32, len: u32) -> Self {
        Self {
            index,
            len,
            _phantom: PhantomData,
        }
    }
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
