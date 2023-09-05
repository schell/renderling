//! Slab storage and ops used for storing on CPU and extracting on GPU.

use core::marker::PhantomData;

pub trait FromSlab<const N: usize>: Sized {
    fn read_slab(&mut self, slab: [u32; N]);
}

impl FromSlab<16> for glam::Mat4 {
    fn read_slab(&mut self, slab: [u32; 16]) {
        let slab = slab.map(f32::from_bits);
        *self = glam::Mat4::from_cols_array(&slab);
    }
}

impl FromSlab<4> for glam::Vec4 {
    fn read_slab(&mut self, [x, y, z, w]: [u32; 4]) {
        self.x = f32::from_bits(x);
        self.y = f32::from_bits(y);
        self.z = f32::from_bits(z);
        self.w = f32::from_bits(w);
    }
}

impl FromSlab<4> for glam::Quat {
    fn read_slab(&mut self, [x, y, z, w]: [u32; 4]) {
        self.x = f32::from_bits(x);
        self.y = f32::from_bits(y);
        self.z = f32::from_bits(z);
        self.w = f32::from_bits(w);
    }
}

impl FromSlab<1> for u32 {
    fn read_slab(&mut self, [n]: [u32; 1]) {
        *self = n;
    }
}

impl FromSlab<1> for f32 {
    fn read_slab(&mut self, [n]: [u32; 1]) {
        *self = f32::from_bits(n);
    }
}

impl<T: FromSlab<N>, const N: usize> FromSlab<2> for Array<T, N> {
    fn read_slab(&mut self, [index, len]: [u32; 2]) {
        let Self {
            index: i,
            len: l,
            _phantom: _,
        } = self;
        *i = index;
        *l = len;
    }
}

impl<T> FromSlab<1> for crate::id::Id<T> {
    fn read_slab(&mut self, [n]: [u32; 1]) {
        self.0 = n;
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable)]
pub struct Array<T: FromSlab<N>, const N: usize> {
    index: u32,
    len: u32,
    _phantom: PhantomData<T>,
}

unsafe impl<T: FromSlab<N> + bytemuck::Pod + bytemuck::Zeroable, const N: usize> bytemuck::Pod
    for Array<T, N>
{
}

impl<T: FromSlab<N>, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Self {
            index: u32::MAX,
            len: 0,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize, T> Array<T, N>
where
    T: FromSlab<N>,
{
    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn read(&self, into_item: &mut T, nth_item: usize, slab: &[u32]) {
        let offset = nth_item as usize * N;
        read::<N, T>(into_item, self.index as usize + offset, slab)
    }
}

pub fn read<const N: usize, T: FromSlab<N>>(thing: &mut T, start: usize, slab: &[u32]) {
    let end = start + N;
    if slab.len() < end {
        return;
    }

    let mut array = [0u32; N];
    for i in 0..N {
        let storage_index = i + start;
        array[i] = slab[storage_index];
    }

    thing.read_slab(array);
}

/// An index into the slab.
pub struct Slabbed<T> {
    pub index: u32,
    _phantom: PhantomData<T>,
}
