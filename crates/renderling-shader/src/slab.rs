//! Slab storage and ops used for storing on CPU and extracting on GPU.
use core::marker::PhantomData;

pub trait FromSlab: Sized {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize;
}

impl FromSlab for u32 {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() > index {
            *self = slab[index];
            index + 1
        } else {
            index
        }
    }
}

impl FromSlab for f32 {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() > index {
            *self = f32::from_bits(slab[index]);
            index + 1
        } else {
            index
        }
    }
}

impl<T: FromSlab, const N: usize> FromSlab for [T; N] {
    fn read_slab(&mut self, mut index: usize, slab: &[u32]) -> usize {
        for i in 0..N {
            index = self[i].read_slab(index, slab);
        }
        index
    }
}

impl<T> FromSlab for crate::id::Id<T> {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        self.0.read_slab(index, slab)
    }
}

impl FromSlab for glam::Mat4 {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 16 {
            return index;
        }
        let Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        } = self;
        let index = x_axis.read_slab(index, slab);
        let index = y_axis.read_slab(index, slab);
        let index = z_axis.read_slab(index, slab);
        w_axis.read_slab(index, slab)
    }
}

impl FromSlab for glam::Vec4 {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 4 {
            return index;
        }
        let Self { x, y, z, w } = self;
        let index = x.read_slab(index, slab);
        let index = y.read_slab(index, slab);
        let index = z.read_slab(index, slab);
        w.read_slab(index, slab)
    }
}

impl FromSlab for glam::Quat {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 4 {
            return index;
        }
        let Self { x, y, z, w } = self;
        let index = x.read_slab(index, slab);
        let index = y.read_slab(index, slab);
        let index = z.read_slab(index, slab);
        w.read_slab(index, slab)
    }
}

impl<T: FromSlab> FromSlab for Array<T> {
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 2 {
            return index;
        }

        let Self { index: ndx, len, _phantom: _ } = self;
        let index = ndx.read_slab(index, slab);
        len.read_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable)]
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
        index >= self.index as usize
            && index < (self.index + self.len) as usize
    }
}

impl<T: FromSlab + SlabSized> Array<T> {
    pub fn extract(&self, item: &mut T, index: usize, slab: &[u32]) {
        if self.len() <= index {
            return;
        }

        let size = T::slab_size();
        let start = self.index as usize + size * index;
        let end = item.read_slab(start, slab);
        debug_assert_eq!(size, end - start);
    }
}

pub trait SlabSized {
    fn slab_size() -> usize;
}

impl<T> SlabSized for crate::id::Id<T> {
    fn slab_size() -> usize {
        1
    }
}
