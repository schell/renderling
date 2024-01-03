//! Slab traits.
use core::{default::Default, marker::PhantomData};
pub use crabslab_derive::SlabItem;

use crate::{array::Array, id::Id};

/// Determines the "size" of a type when stored in a slab of `&[u32]`,
/// and how to read/write it from/to the slab.
///
/// `SlabItem` can be automatically derived for struct and tuple types,
/// so long as those types are relatively simple. So far, autoderiving
/// fields with these types will **not compile** on one or more targets:
/// * `PhantomData<T>` - will not compile on `target_arch = "spirv"`
pub trait SlabItem: core::any::Any + Sized {
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

impl SlabItem for bool {
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

impl SlabItem for u32 {
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

impl SlabItem for f32 {
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

impl<T: SlabItem + Default> SlabItem for Option<T> {
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

impl<T: SlabItem, const N: usize> SlabItem for [T; N] {
    fn slab_size() -> usize {
        <T as SlabItem>::slab_size() * N
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

#[cfg(feature = "glam")]
impl SlabItem for glam::Mat4 {
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

#[cfg(feature = "glam")]
impl SlabItem for glam::Vec2 {
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

impl SlabItem for glam::Vec3 {
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

#[cfg(feature = "glam")]
impl SlabItem for glam::Vec4 {
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

impl SlabItem for glam::Quat {
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

#[cfg(feature = "glam")]
impl SlabItem for glam::UVec2 {
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

#[cfg(feature = "glam")]
impl SlabItem for glam::UVec3 {
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

#[cfg(feature = "glam")]
impl SlabItem for glam::UVec4 {
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

impl<T: core::any::Any> SlabItem for PhantomData<T> {
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
    fn contains<T: SlabItem>(&self, id: Id<T>) -> bool {
        id.index() + T::slab_size() <= self.len()
    }

    /// Read the type from the slab using the Id as the index.
    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T;

    #[cfg(not(target_arch = "spirv"))]
    fn read_vec<T: SlabItem + Default>(&self, array: crate::array::Array<T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(array.len());
        for i in 0..array.len() {
            let id = array.at(i);
            vec.push(self.read(id));
        }
        vec
    }

    /// Write the type into the slab at the index.
    ///
    /// Return the next index, or the same index if writing would overlap the
    /// slab.
    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize;

    /// Write a slice of the type into the slab at the index.
    ///
    /// Return the next index, or the same index if writing would overlap the
    /// slab.
    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize;

    /// Write the type into the slab at the position of the given `Id`.
    ///
    /// This likely performs a partial write if the given `Id` is out of bounds.
    fn write<T: SlabItem>(&mut self, id: Id<T>, t: &T) {
        let _ = self.write_indexed(t, id.index());
    }

    /// Write contiguous elements into the slab at the position of the given
    /// `Array`.
    ///
    /// ## NOTE
    /// This does nothing if the length of `Array` is greater than the length of
    /// `data`.
    fn write_array<T: SlabItem>(&mut self, array: Array<T>, data: &[T]) {
        if array.len() > data.len() {
            return;
        }
        let _ = self.write_indexed_slice(data, array.starting_index());
    }
}

impl Slab for [u32] {
    fn len(&self) -> usize {
        self.len()
    }

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        let mut t = T::default();
        let _ = t.read_slab(id.index(), self);
        t
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        t.write_slab(index, self)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
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

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        self.as_slice().read(id)
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        self.as_mut_slice().write_indexed(t, index)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        self.as_mut_slice().write_indexed_slice(t, index)
    }
}

/// Trait for slabs of `u32`s that can store many types, and can grow to fit.
pub trait GrowableSlab: Slab {
    /// Return the current capacity of the slab.
    fn capacity(&self) -> usize;

    /// Reserve enough space on the slab to fit the given capacity.
    fn reserve_capacity(&mut self, capacity: usize);

    /// Increment the length of the slab by `n` u32s.
    ///
    /// Returns the previous length.
    fn increment_len(&mut self, n: usize) -> usize;


    /// Expands the slab to fit the given number of `T`s, if necessary.
    fn maybe_expand_to_fit<T: SlabItem>(&mut self, len: usize) {
        let size = T::slab_size();
        let capacity = self.capacity();
        //log::trace!(
        //    "append_slice: {size} * {ts_len} + {len} ({}) >= {capacity}",
        //    size * ts_len + len
        //);
        let capacity_needed = self.len() + size * len;
        if capacity_needed > capacity {
            let mut new_capacity = capacity * 2;
            while new_capacity < capacity_needed {
                new_capacity = (new_capacity * 2).max(2);
            }
            self.reserve_capacity(new_capacity);
        }
    }

    /// Preallocate space for one `T` element, but don't write anything to the
    /// buffer.
    ///
    /// The returned `Id` can be used to write later with [`Self::write`].
    ///
    /// NOTE: This changes the next available buffer index and may change the
    /// buffer capacity.
    fn allocate<T: SlabItem>(&mut self) -> Id<T> {
        self.maybe_expand_to_fit::<T>(1);
        let index = self.increment_len(T::slab_size());
        Id::from(index)
    }

    /// Preallocate space for `len` `T` elements, but don't write to
    /// the buffer.
    ///
    /// This can be used to allocate space for a bunch of elements that get
    /// written later with [`Self::write_array`].
    ///
    /// NOTE: This changes the length of the buffer and may change the capacity.
    fn allocate_array<T: SlabItem>(&mut self, len: usize) -> Array<T> {
        if len == 0 {
            return Array::default();
        }
        self.maybe_expand_to_fit::<T>(len);
        let index = self.increment_len(T::slab_size() * len);
        Array::new(index as u32, len as u32)
    }

    /// Append to the end of the buffer.
    ///
    /// Returns the `Id` of the written element.
    fn append<T: SlabItem + Default>(&mut self, t: &T) -> Id<T> {
        let id = self.allocate::<T>();
        // IGNORED: safe because we just allocated the id
        let _ = self.write(id, t);
        id
    }

    /// Append a slice to the end of the buffer, resizing if necessary
    /// and returning a slabbed array.
    ///
    /// Returns the `Array` of the written elements.
    fn append_array<T: SlabItem + Default>(&mut self, ts: &[T]) -> Array<T> {
        let array = self.allocate_array::<T>(ts.len());
        // IGNORED: safe because we just allocated the array
        let _ = self.write_array(array, ts);
        array
    }
}

/// A wrapper around a `GrowableSlab` that provides convenience methods for
/// working with CPU-side slabs.
///
/// Working with slabs on the CPU is much more convenient because the underlying
/// buffer `B` is often a growable type, like `Vec<u32>`. This wrapper provides
/// methods for appending to the end of the buffer with automatic resizing and
/// for preallocating space for elements that will be written later.
pub struct CpuSlab<B> {
    slab: B,
}

impl<B> AsRef<B> for CpuSlab<B> {
    fn as_ref(&self) -> &B {
        &self.slab
    }
}

impl<B> AsMut<B> for CpuSlab<B> {
    fn as_mut(&mut self) -> &mut B {
        &mut self.slab
    }
}

impl<B: Slab> Slab for CpuSlab<B> {
    fn len(&self) -> usize {
        self.slab.len()
    }

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        self.slab.read(id)
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        self.slab.write_indexed(t, index)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        self.slab.write_indexed_slice(t, index)
    }
}

impl<B: GrowableSlab> GrowableSlab for CpuSlab<B> {
    fn capacity(&self) -> usize {
        self.slab.capacity()
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        self.slab.reserve_capacity(capacity);
    }

    fn increment_len(&mut self, n: usize) -> usize {
        self.slab.increment_len(n)
    }
}

impl<B: GrowableSlab> CpuSlab<B> {
    /// Create a new `SlabBuffer` with the given slab.
    pub fn new(slab: B) -> Self {
        Self { slab }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl GrowableSlab for Vec<u32> {
    fn capacity(&self) -> usize {
        Vec::capacity(self)
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        Vec::reserve(self, capacity - self.capacity());
    }

    fn increment_len(&mut self, n: usize) -> usize {
        let index = self.len();
        self.extend(core::iter::repeat(0).take(n));
        index
    }
}

#[cfg(test)]
mod test {
    use glam::Vec4;

    use crate::{self as crabslab, Array, CpuSlab, SlabItem};

    use super::*;

    #[derive(Debug, Default, PartialEq, SlabItem)]
    struct Vertex {
        position: Vec4,
        color: Vec4,
        uv: glam::Vec2,
    }

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

    #[test]
    fn cpuslab_sanity() {
        let mut slab = CpuSlab::new(vec![]);
        let v = Vertex {
            position: Vec4::new(0.5, -0.5, 0.0, 1.0),
            color: Vec4::new(1.0, 0.0, 0.0, 1.0),
            ..Default::default()
        };
        let id = slab.append(&v);
        assert_eq!(Id::new(0), id);
        assert_eq!(v, slab.read(id));

        let f32s = [1.1, 2.2, 3.3, 4.4f32];
        let array = slab.append_array(&f32s);
        assert_eq!(1.1, slab.read(array.at(0)));
        assert_eq!(2.2, slab.read(array.at(1)));
        assert_eq!(3.3, slab.read(array.at(2)));
        assert_eq!(4.4, slab.read(array.at(3)));

        let f32_vec = slab.read_vec(array);
        assert_eq!(f32s, f32_vec[..]);
    }
}
