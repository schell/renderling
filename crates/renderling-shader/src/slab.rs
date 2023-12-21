//! Slab storage and operations.
use core::marker::PhantomData;

pub use renderling_derive::Slabbed;

use crate::{array::Array, id::Id};

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

impl<T: Slabbed + Default> Slabbed for Option<T> {
    fn slab_size() -> usize {
        1 + T::slab_size()
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let mut proxy = 0u32;
        let index = proxy.read_slab(index, slab);
        if proxy == 1 {
            let mut t = T::default();
            let index = t.read_slab(index, slab);
            *self = Some(t);
            index
        } else {
            *self = None;
            index + T::slab_size()
        }
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        if let Some(t) = self {
            let index = 1u32.write_slab(index, slab);
            t.write_slab(index, slab)
        } else {
            let index = 0u32.write_slab(index, slab);
            index + T::slab_size()
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
        let Self { x, y, z } = self;
        let index = x.read_slab(index, slab);
        let index = y.read_slab(index, slab);
        let index = z.read_slab(index, slab);
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
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
        let index = self.x.read_slab(index, slab);
        let index = self.y.read_slab(index, slab);
        let index = self.z.read_slab(index, slab);
        self.w.read_slab(index, slab)
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
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
        let Self { x, y, z, w } = self;
        let index = x.read_slab(index, slab);
        let index = y.read_slab(index, slab);
        let index = z.read_slab(index, slab);
        w.read_slab(index, slab)
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
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
        let index = self.x.read_slab(index, slab);
        let index = self.y.read_slab(index, slab);
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
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
        let index = self.x.read_slab(index, slab);
        let index = self.y.read_slab(index, slab);
        let index = self.z.read_slab(index, slab);
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
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

/// Trait for slabs of `u32`s that can store many types.
pub trait Slab {
    /// Return the number of u32 elements in the slab.
    fn len(&self) -> usize;

    /// Returns whether the slab may contain the value with the given id.
    fn contains<T: Slabbed>(&self, id: Id<T>) -> bool {
        id.index() + T::slab_size() <= self.len()
    }

    /// Read the type from the slab using the Id as the index.
    fn read<T: Slabbed + Default>(&self, id: Id<T>) -> T;

    #[cfg(not(target_arch = "spirv"))]
    fn read_vec<T: Slabbed + Default>(&self, array: crate::array::Array<T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(array.len());
        for i in 0..array.len() {
            let id = array.at(i);
            vec.push(self.read(id));
        }
        vec
    }

    /// Write the type into the slab at the index.
    ///
    /// Return the next index, or the same index if writing would overlap the slab.
    fn write_indexed<T: Slabbed>(&mut self, t: &T, index: usize) -> usize;

    /// Write a slice of the type into the slab at the index.
    ///
    /// Return the next index, or the same index if writing would overlap the slab.
    fn write_indexed_slice<T: Slabbed>(&mut self, t: &[T], index: usize) -> usize;

    /// Write the type into the slab at the position of the given `Id`.
    ///
    /// This likely performs a partial write if the given `Id` is out of bounds.
    fn write<T: Slabbed>(&mut self, id: Id<T>, t: &T) {
        let _ = self.write_indexed(t, id.index());
    }

    /// Write contiguous elements into the slab at the position of the given `Array`.
    ///
    /// ## NOTE
    /// This does nothing if the length of `Array` is greater than the length of `data`.
    fn write_array<T: Slabbed>(&mut self, array: Array<T>, data: &[T]) {
        if array.len() > data.len() {
            return;
        }
        for i in 0..array.len() {
            let _ = self.write(array.at(i), &data[i]);
        }
    }
}

impl Slab for [u32] {
    fn len(&self) -> usize {
        self.len()
    }

    fn read<T: Slabbed + Default>(&self, id: Id<T>) -> T {
        let mut t = T::default();
        let _ = t.read_slab(id.index(), self);
        t
    }

    fn write_indexed<T: Slabbed>(&mut self, t: &T, index: usize) -> usize {
        t.write_slab(index, self)
    }

    fn write_indexed_slice<T: Slabbed>(&mut self, t: &[T], index: usize) -> usize {
        let mut index = index;
        for item in t {
            index = item.write_slab(index, self);
        }
        index
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Slab for Vec<u32> {
    fn len(&self) -> usize {
        self.len()
    }

    fn read<T: Slabbed + Default>(&self, id: Id<T>) -> T {
        self.as_slice().read(id)
    }

    fn write_indexed<T: Slabbed>(&mut self, t: &T, index: usize) -> usize {
        self.as_mut_slice().write_indexed(t, index)
    }

    fn write_indexed_slice<T: Slabbed>(&mut self, t: &[T], index: usize) -> usize {
        self.as_mut_slice().write_indexed_slice(t, index)
    }
}

#[cfg(test)]
mod test {
    use glam::Vec4;

    use crate::{array::Array, stage::Vertex};

    use super::*;

    #[test]
    fn slab_array_readwrite() {
        let mut slab = [0u32; 16];
        slab.write_indexed(&42, 0);
        slab.write_indexed(&666, 1);
        let t = slab.read(Id::<[u32; 2]>::new(0));
        assert_eq!([42, 666], t);
        let t: Vec<u32> = slab.read_vec(Array::new(0, 2));
        assert_eq!([42, 666], t[..]);
        slab.write_indexed_slice(&[1, 2, 3, 4], 2);
        let t: Vec<u32> = slab.read_vec(Array::new(2, 4));
        assert_eq!([1, 2, 3, 4], t[..]);
        slab.write_indexed_slice(&[[1.0, 2.0, 3.0, 4.0], [5.5, 6.5, 7.5, 8.5]], 0);

        let arr = Array::<[f32; 4]>::new(0, 2);
        assert_eq!(Id::new(0), arr.at(0));
        assert_eq!(Id::new(4), arr.at(1));
        assert_eq!([1.0, 2.0, 3.0, 4.0], slab.read(arr.at(0)));
        assert_eq!([5.5, 6.5, 7.5, 8.5], slab.read(arr.at(1)));

        let geometry = vec![
            Vertex {
                position: Vec4::new(0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 1.0, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let geometry_slab_size = Vertex::slab_size() * geometry.len();
        let mut slab = vec![0u32; geometry_slab_size + Array::<Vertex>::slab_size()];
        let index = 0usize;
        let vertices = Array::<Vertex>::new(index as u32, geometry.len() as u32);
        let index = slab.write_indexed_slice(&geometry, index);
        assert_eq!(geometry_slab_size, index);
        let vertices_id = Id::<Array<Vertex>>::from(index);
        let index = slab.write_indexed(&vertices, index);
        assert_eq!(geometry_slab_size + Array::<Vertex>::slab_size(), index);
        assert_eq!(Vertex::slab_size() * 6, vertices_id.index());
        assert!(slab.contains(vertices_id),);

        let array = slab.read(vertices_id);
        assert_eq!(vertices, array);
    }
}
