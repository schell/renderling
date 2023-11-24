//! Slab storage and ops used for storing on CPU and extracting on GPU.
use core::marker::PhantomData;

pub use renderling_derive::Slabbed;

use crate::id::Id;

/// Determines the "size" of a type when stored in a slab of `&[u32]`,
/// and how to read/write it from/to the slab.
///
/// `Slabbed` can be automatically derived for struct and tuple types,
/// so long as those types are relatively simple. So far, autoderiving
/// fields with these types will **not compile** on one or more targets:
/// * `PhantomData<T>` - will not compile on `target_arch = "spirv"`
pub trait Slabbed: core::any::Any + Sized {
    /// The number of `u32`s this type occupies in a slab of `&[u32]`.
    fn slab_size() -> usize;

    /// Read the type out of the slab at starting `index` and return
    /// the new index.
    ///
    /// If the type cannot be read, the returned index will be equal
    /// to `index`.
    // TODO: recondsider the mutability of `self` here.
    // We could make this a `&self` and that might help with using it
    // from multiple threads.
    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize;

    /// Write the type into the slab at starting `index` and return
    /// the new index.
    ///
    /// If the type cannot be written, the returned index will be equal
    /// to `index`.
    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize;
}

impl Slabbed for bool {
    fn slab_size() -> usize {
        1
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let mut proxy = 0u32;
        let index = proxy.read_slab(index, slab);
        *self = proxy == 1;
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if *self { 1u32 } else { 0u32 }.write_slab(index, slab)
    }
}

impl Slabbed for u32 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() > index {
            slab[index] = *self;
            index + 1
        } else {
            index
        }
    }
}

impl Slabbed for f32 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() > index {
            slab[index] = self.to_bits();
            index + 1
        } else {
            index
        }
    }
}

impl<T: Slabbed, const N: usize> Slabbed for [T; N] {
    fn slab_size() -> usize {
        <T as Slabbed>::slab_size() * N
    }

    fn read_slab(&mut self, mut index: usize, slab: &[u32]) -> usize {
        for i in 0..N {
            index = self[i].read_slab(index, slab);
        }
        index
    }

    fn write_slab(&self, mut index: usize, slab: &mut [u32]) -> usize {
        for i in 0..N {
            index = self[i].write_slab(index, slab);
        }
        index
    }
}

impl Slabbed for glam::Mat4 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 16 {
            return index;
        }
        let Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        } = self;
        let index = x_axis.write_slab(index, slab);
        let index = y_axis.write_slab(index, slab);
        let index = z_axis.write_slab(index, slab);
        w_axis.write_slab(index, slab)
    }
}

impl Slabbed for glam::Vec2 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 2 {
            return index;
        }
        let index = self.x.write_slab(index, slab);
        let index = self.y.write_slab(index, slab);
        index
    }
}

impl Slabbed for glam::Vec3 {
    fn slab_size() -> usize {
        3
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        if slab.len() < index + 3 {
            return index;
        }
        let Self { x, y, z } = self;
        let index = x.read_slab(index, slab);
        let index = y.read_slab(index, slab);
        let index = z.read_slab(index, slab);
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 3 {
            return index;
        }
        let Self { x, y, z } = self;
        let index = x.write_slab(index, slab);
        let index = y.write_slab(index, slab);
        let index = z.write_slab(index, slab);
        index
    }
}

impl Slabbed for glam::Vec4 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 4 {
            return index;
        }
        let Self { x, y, z, w } = self;
        let index = x.write_slab(index, slab);
        let index = y.write_slab(index, slab);
        let index = z.write_slab(index, slab);
        w.write_slab(index, slab)
    }
}

impl Slabbed for glam::Quat {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 4 {
            return index;
        }
        let Self { x, y, z, w } = self;
        let index = x.write_slab(index, slab);
        let index = y.write_slab(index, slab);
        let index = z.write_slab(index, slab);
        w.write_slab(index, slab)
    }
}

impl Slabbed for glam::UVec2 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 2 {
            return index;
        }
        let index = self.x.write_slab(index, slab);
        let index = self.y.write_slab(index, slab);
        index
    }
}

impl Slabbed for glam::UVec3 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 3 {
            return index;
        }
        let index = self.x.write_slab(index, slab);
        let index = self.y.write_slab(index, slab);
        let index = self.z.write_slab(index, slab);
        index
    }
}

impl Slabbed for glam::UVec4 {
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

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if slab.len() < index + 4 {
            return index;
        }
        let index = self.x.write_slab(index, slab);
        let index = self.y.write_slab(index, slab);
        let index = self.z.write_slab(index, slab);
        let index = self.w.write_slab(index, slab);
        index
    }
}

impl<T: core::any::Any> Slabbed for PhantomData<T> {
    fn slab_size() -> usize {
        0
    }

    fn read_slab(&mut self, index: usize, _: &[u32]) -> usize {
        index
    }

    fn write_slab(&self, index: usize, _: &mut [u32]) -> usize {
        index
    }
}

pub trait Slab {
    /// Read the type from the slab using the Id as the index.
    fn read<T: Slabbed + Default>(&self, id: Id<T>) -> T;

    /// Write the type into the slab at the index.
    ///
    /// Return the next index, or the same index if writing would overlap the slab.
    fn write<T: Slabbed + Default>(&mut self, t: &T, index: usize) -> usize;

    /// Write a slice of the type into the slab at the index.
    ///
    /// Return the next index, or the same index if writing would overlap the slab.
    fn write_slice<T: Slabbed + Default>(&mut self, t: &[T], index: usize) -> usize;
}

impl Slab for [u32] {
    fn read<T: Slabbed + Default>(&self, id: Id<T>) -> T {
        let mut t = T::default();
        let _ = t.read_slab(id.index(), self);
        t
    }

    fn write<T: Slabbed + Default>(&mut self, t: &T, index: usize) -> usize {
        t.write_slab(index, self)
    }

    fn write_slice<T: Slabbed + Default>(&mut self, t: &[T], index: usize) -> usize {
        let mut index = index;
        for item in t {
            index = item.write_slab(index, self);
        }
        index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slab_array_readwrite() {
        let mut slab = [0u32; 16];
        slab.write(&42, 0);
        slab.write(&666, 1);
        let t = slab.read(Id::<[u32; 2]>::new(0));
        assert_eq!([42, 666], t);
    }
}
