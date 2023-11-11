//! Slab storage and ops used for storing on CPU and extracting on GPU.
use core::marker::PhantomData;

pub use renderling_derive::FromSlab;

/// Determines the "size" of a type when stored in a slab of `&[u32]`,
/// and how to read it from the slab.
pub trait FromSlab: Sized {
    /// The number of `u32`s this type occupies in a slab of `&[u32]`.
    fn slab_size() -> usize;
    /// Read the type out of the slab at starting `index` and return
    /// the new index.
    ///
    /// If the type cannot be read, The returned index will be equal
    /// to `index`.
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize;
}

impl FromSlab for u32 {
    fn slab_size() -> usize {
        1
    }

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
    fn slab_size() -> usize {
        1
    }

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
    fn slab_size() -> usize {
        <T as FromSlab>::slab_size() * N
    }

    fn read_slab(&mut self, mut index: usize, slab: &[u32]) -> usize {
        for i in 0..N {
            index = self[i].read_slab(index, slab);
        }
        index
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

    fn slab_size() -> usize {
        16
    }
}

impl FromSlab for glam::Vec4 {
    fn slab_size() -> usize {
        4
    }
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
    fn slab_size() -> usize {
        16
    }
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

impl FromSlab for glam::UVec2 {
    fn slab_size() -> usize {
        2
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 2 {
            return index;
        }
        let index = self.x.read_slab(index, slab);
        let index = self.y.read_slab(index, slab);
        index
    }
}

impl FromSlab for glam::UVec3 {
    fn slab_size() -> usize {
        3
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 3 {
            return index;
        }
        let index = self.x.read_slab(index, slab);
        let index = self.y.read_slab(index, slab);
        let index = self.z.read_slab(index, slab);
        index
    }
}

impl FromSlab for glam::UVec4 {
    fn slab_size() -> usize {
        4
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 4 {
            return index;
        }
        let index = self.x.read_slab(index, slab);
        let index = self.y.read_slab(index, slab);
        let index = self.z.read_slab(index, slab);
        let index = self.w.read_slab(index, slab);
        index
    }
}

impl<T> FromSlab for PhantomData<T> {
    fn slab_size() -> usize {
        0
    }

    fn read_slab(&mut self, index: usize, _: &[u32]) -> usize {
        index
    }
}
