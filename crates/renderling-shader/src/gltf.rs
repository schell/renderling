//! Gltf types that are used in shaders.
use glam::{Vec2, Vec3, Vec4};

use crate::{
    self as renderling_shader,
    array::Array,
    id::Id,
    pbr::PbrMaterial,
    slab::{Slab, Slabbed},
    texture::GpuTexture,
};
#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfBuffer(pub Array<u32>);

#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq)]
pub enum DataType {
    I8,
    U8,
    I16,
    U16,
    #[default]
    U32,
    F32,
}

impl Slabbed for DataType {
    fn slab_size() -> usize {
        // 1
        u32::slab_size()
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let original_index = index;
        let mut hash = 0u32;
        let index = hash.read_slab(index, slab);
        match hash {
            0 => *self = DataType::I8,
            1 => *self = DataType::U8,
            2 => *self = DataType::I16,
            3 => *self = DataType::U16,
            4 => *self = DataType::U32,
            5 => *self = DataType::F32,
            _ => return original_index,
        }
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        let hash = *self as u32;
        hash.write_slab(index, slab)
    }
}

#[repr(u32)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy)]
pub enum Dimensions {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl Slabbed for Dimensions {
    fn slab_size() -> usize {
        1
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let original_index = index;
        let mut hash = 0u32;
        let index = hash.read_slab(index, slab);
        match hash {
            0 => *self = Dimensions::Scalar,
            1 => *self = Dimensions::Vec2,
            2 => *self = Dimensions::Vec3,
            3 => *self = Dimensions::Vec4,
            4 => *self = Dimensions::Mat2,
            5 => *self = Dimensions::Mat3,
            6 => *self = Dimensions::Mat4,
            _ => return original_index,
        }
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        match self {
            Dimensions::Scalar => 0,
            Dimensions::Vec2 => 1,
            Dimensions::Vec3 => 2,
            Dimensions::Vec4 => 3,
            Dimensions::Mat2 => 4,
            Dimensions::Mat3 => 5,
            Dimensions::Mat4 => 6,
        }
        .write_slab(index, slab)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfAccessor {
    // The byte size of each element that this accessor describes.
    //
    /// For example, if the accessor describes a `Vec3` of F32s, then
    // the size is 3 * 4 = 12.
    pub size: u32,
    pub buffer: Id<GltfBuffer>,
    // Returns the offset relative to the start of the parent buffer view in bytes.
    //
    // This will be 0 if the corresponding accessor is sparse.
    pub view_offset: u32,
    // The stride in bytes between vertex attributes or other interleavable data.
    pub view_stride: u32,
    // The number of elements within the buffer view - not to be confused with the
    // number of bytes in the buffer view.
    pub count: u32,
    // The data type of components in the attribute.
    pub data_type: DataType,
    // Specifies if the attribute is a scalar, vector, or matrix.
    pub dimensions: Dimensions,
    // Whether or not the attribute is normalized.
    pub normalized: bool,
}

/// Used to access contiguous u8 components from a buffer, contained in u32s.
pub struct IncU8 {
    slab_index: usize,
    byte_offset: usize,
}

impl IncU8 {
    pub fn extract(self, slab: &[u32]) -> (u32, Self) {
        let (value, slab_index, byte_offset) =
            crate::bits::extract_u8(self.slab_index, self.byte_offset, slab);
        (
            value,
            Self {
                slab_index,
                byte_offset,
            },
        )
    }
}

/// Used to access contiguous i8 components from a buffer, contained in u32s.
pub struct IncI8 {
    slab_index: usize,
    byte_offset: usize,
}

impl IncI8 {
    pub fn extract(self, slab: &[u32]) -> (i32, Self) {
        let (value, slab_index, byte_offset) =
            crate::bits::extract_i8(self.slab_index, self.byte_offset, slab);
        (
            value,
            Self {
                slab_index,
                byte_offset,
            },
        )
    }
}

/// Used to access contiguous u16 components from a buffer, contained in u32s.
pub struct IncU16 {
    slab_index: usize,
    byte_offset: usize,
}

impl IncU16 {
    pub fn extract(self, slab: &[u32]) -> (u32, Self) {
        let (value, slab_index, byte_offset) =
            crate::bits::extract_u16(self.slab_index, self.byte_offset, slab);
        crate::println!("value: {value:?}");
        (
            value,
            Self {
                slab_index,
                byte_offset,
            },
        )
    }
}

/// Used to access contiguous i16 components from a buffer, contained in u32s.
pub struct IncI16 {
    slab_index: usize,
    byte_offset: usize,
}

impl IncI16 {
    pub fn extract(self, slab: &[u32]) -> (i32, Self) {
        let (value, slab_index, byte_offset) =
            crate::bits::extract_i16(self.slab_index, self.byte_offset, slab);
        (
            value,
            Self {
                slab_index,
                byte_offset,
            },
        )
    }
}

impl GltfAccessor {
    fn slab_index_and_byte_offset(&self, element_index: usize, slab: &[u32]) -> (usize, usize) {
        crate::println!("index: {element_index:?}");
        let buffer = slab.read(self.buffer);
        crate::println!("buffer: {:?}", buffer);
        let buffer_start = buffer.0.starting_index();
        crate::println!("buffer_start: {buffer_start:?}");
        let buffer_start_bytes = buffer_start * 4;
        crate::println!("buffer_start_bytes: {buffer_start_bytes:?}");
        let byte_offset = buffer_start_bytes
            + self.view_offset as usize
            + element_index as usize
                * if self.size > self.view_stride {
                    self.size
                } else {
                    self.view_stride
                } as usize;
        crate::println!("byte_offset: {byte_offset:?}");
        let slab_index = byte_offset / 4;
        crate::println!("slab_index: {slab_index:?}");
        let byte_offset = byte_offset % 4;
        (slab_index, byte_offset)
    }

    pub fn inc_u8(&self, index: usize, slab: &[u32]) -> IncU8 {
        let (slab_index, byte_offset) = self.slab_index_and_byte_offset(index, slab);
        IncU8 {
            slab_index,
            byte_offset,
        }
    }

    pub fn inc_i8(&self, index: usize, slab: &[u32]) -> IncI8 {
        let (slab_index, byte_offset) = self.slab_index_and_byte_offset(index, slab);
        IncI8 {
            slab_index,
            byte_offset,
        }
    }

    pub fn inc_u16(&self, index: usize, slab: &[u32]) -> IncU16 {
        let (slab_index, byte_offset) = self.slab_index_and_byte_offset(index, slab);
        IncU16 {
            slab_index,
            byte_offset,
        }
    }

    pub fn inc_i16(&self, index: usize, slab: &[u32]) -> IncI16 {
        let (slab_index, byte_offset) = self.slab_index_and_byte_offset(index, slab);
        IncI16 {
            slab_index,
            byte_offset,
        }
    }

    pub fn get_u32(&self, vertex_index: usize, slab: &[u32]) -> u32 {
        let x;
        match self.data_type {
            DataType::I8 => {
                let inc = self.inc_i8(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix as u32;
            }
            DataType::U8 => {
                let inc = self.inc_u8(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix;
            }
            DataType::I16 => {
                let inc = self.inc_i16(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix as u32;
            }
            DataType::U16 => {
                let inc = self.inc_u16(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix;
            }
            DataType::U32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_u32(slab_index, 0, slab).0;
            }
            DataType::F32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_f32(slab_index, 0, slab).0 as u32;
            }
        }
        x
    }

    pub fn get_f32(&self, vertex_index: usize, slab: &[u32]) -> f32 {
        let x;
        match self.data_type {
            DataType::I8 => {
                let inc = self.inc_i8(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix as f32;
            }
            DataType::U8 => {
                let inc = self.inc_u8(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix as f32;
            }
            DataType::I16 => {
                let inc = self.inc_i16(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix as f32;
            }
            DataType::U16 => {
                let inc = self.inc_u16(vertex_index, slab);
                let (ix, _) = inc.extract(slab);
                x = ix as f32;
            }
            DataType::U32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_u32(slab_index, 0, slab).0 as f32;
            }
            DataType::F32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_f32(slab_index, 0, slab).0;
            }
        }
        x
    }

    pub fn get_vec2(&self, vertex_index: usize, slab: &[u32]) -> glam::Vec2 {
        let x;
        let y;
        match self.data_type {
            DataType::I8 => {
                let inc = self.inc_i8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
            }
            DataType::U8 => {
                let inc = self.inc_u8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
            }
            DataType::I16 => {
                let inc = self.inc_i16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
            }
            DataType::U16 => {
                let inc = self.inc_u16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
            }
            DataType::U32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_u32(slab_index, 0, slab).0 as f32;
                y = crate::bits::extract_u32(slab_index + 1, 0, slab).0 as f32;
            }
            DataType::F32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_f32(slab_index, 0, slab).0;
                y = crate::bits::extract_f32(slab_index + 1, 0, slab).0;
            }
        }
        match self.dimensions {
            Dimensions::Scalar => glam::Vec2::new(x, 0.0),
            _ => glam::Vec2::new(x, y),
        }
    }

    pub fn get_vec3(&self, vertex_index: usize, slab: &[u32]) -> glam::Vec3 {
        let x;
        let y;
        let z;
        match self.data_type {
            DataType::I8 => {
                let inc = self.inc_i8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
            }
            DataType::U8 => {
                let inc = self.inc_u8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
            }
            DataType::I16 => {
                let inc = self.inc_i16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
            }
            DataType::U16 => {
                let inc = self.inc_u16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
            }
            DataType::U32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_u32(slab_index, 0, slab).0 as f32;
                y = crate::bits::extract_u32(slab_index + 1, 0, slab).0 as f32;
                z = crate::bits::extract_u32(slab_index + 2, 0, slab).0 as f32;
            }
            DataType::F32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_f32(slab_index, 0, slab).0;
                y = crate::bits::extract_f32(slab_index + 1, 0, slab).0;
                z = crate::bits::extract_f32(slab_index + 2, 0, slab).0;
            }
        }
        match self.dimensions {
            Dimensions::Scalar => glam::Vec3::new(x, 0.0, 0.0),
            Dimensions::Vec2 => glam::Vec3::new(x, y, 0.0),
            _ => glam::Vec3::new(x, y, z),
        }
    }

    pub fn get_vec4(&self, vertex_index: usize, slab: &[u32]) -> glam::Vec4 {
        let x;
        let y;
        let z;
        let w;
        match self.data_type {
            DataType::I8 => {
                let inc = self.inc_i8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
                w = iw as f32;
            }
            DataType::U8 => {
                let inc = self.inc_u8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
                w = iw as f32;
            }
            DataType::I16 => {
                let inc = self.inc_i16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
                w = iw as f32;
            }
            DataType::U16 => {
                let inc = self.inc_u16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix as f32;
                y = iy as f32;
                z = iz as f32;
                w = iw as f32;
            }
            DataType::U32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_u32(slab_index, 0, slab).0 as f32;
                y = crate::bits::extract_u32(slab_index + 1, 0, slab).0 as f32;
                z = crate::bits::extract_u32(slab_index + 2, 0, slab).0 as f32;
                w = crate::bits::extract_u32(slab_index + 3, 0, slab).0 as f32;
            }
            DataType::F32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_f32(slab_index, 0, slab).0;
                y = crate::bits::extract_f32(slab_index + 1, 0, slab).0;
                z = crate::bits::extract_f32(slab_index + 2, 0, slab).0;
                w = crate::bits::extract_f32(slab_index + 3, 0, slab).0;
            }
        }
        match self.dimensions {
            Dimensions::Scalar => glam::Vec4::new(x, 0.0, 0.0, 0.0),
            Dimensions::Vec2 => glam::Vec4::new(x, y, 0.0, 0.0),
            Dimensions::Vec3 => glam::Vec4::new(x, y, z, 0.0),
            _ => glam::Vec4::new(x, y, z, w),
        }
    }

    pub fn get_uvec4(&self, vertex_index: usize, slab: &[u32]) -> glam::UVec4 {
        let x;
        let y;
        let z;
        let w;
        match self.data_type {
            DataType::I8 => {
                let inc = self.inc_i8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix as u32;
                y = iy as u32;
                z = iz as u32;
                w = iw as u32;
            }
            DataType::U8 => {
                let inc = self.inc_u8(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix;
                y = iy;
                z = iz;
                w = iw;
            }
            DataType::I16 => {
                let inc = self.inc_i16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix as u32;
                y = iy as u32;
                z = iz as u32;
                w = iw as u32;
            }
            DataType::U16 => {
                let inc = self.inc_u16(vertex_index, slab);
                let (ix, inc) = inc.extract(slab);
                let (iy, inc) = inc.extract(slab);
                let (iz, inc) = inc.extract(slab);
                let (iw, _) = inc.extract(slab);
                x = ix;
                y = iy;
                z = iz;
                w = iw;
            }
            DataType::U32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_u32(slab_index, 0, slab).0;
                y = crate::bits::extract_u32(slab_index + 1, 0, slab).0;
                z = crate::bits::extract_u32(slab_index + 2, 0, slab).0;
                w = crate::bits::extract_u32(slab_index + 3, 0, slab).0;
            }
            DataType::F32 => {
                let (slab_index, _) = self.slab_index_and_byte_offset(vertex_index, slab);
                x = crate::bits::extract_f32(slab_index, 0, slab).0 as u32;
                y = crate::bits::extract_f32(slab_index + 1, 0, slab).0 as u32;
                z = crate::bits::extract_f32(slab_index + 2, 0, slab).0 as u32;
                w = crate::bits::extract_f32(slab_index + 3, 0, slab).0 as u32;
            }
        }
        match self.dimensions {
            Dimensions::Scalar => glam::UVec4::new(x, 0, 0, 0),
            Dimensions::Vec2 => glam::UVec4::new(x, y, 0, 0),
            Dimensions::Vec3 => glam::UVec4::new(x, y, z, 0),
            _ => glam::UVec4::new(x, y, z, w),
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfPrimitive {
    pub vertex_count: u32,
    pub material: Id<PbrMaterial>,
    pub indices: Id<GltfAccessor>,
    pub positions: Id<GltfAccessor>,
    pub normals: Id<GltfAccessor>,
    pub normals_were_generated: bool,
    pub tangents: Id<GltfAccessor>,
    pub tangents_were_generated: bool,
    pub colors: Id<GltfAccessor>,
    pub tex_coords0: Id<GltfAccessor>,
    pub tex_coords1: Id<GltfAccessor>,
    pub joints: Id<GltfAccessor>,
    pub weights: Id<GltfAccessor>,
}

impl GltfPrimitive {
    pub fn get_vertex(&self, vertex_index: usize, slab: &[u32]) -> crate::stage::Vertex {
        let index = if self.indices.is_some() {
            let indices = slab.read(self.indices);
            let index = indices.get_u32(vertex_index, slab);
            index as usize
        } else {
            vertex_index
        };
        crate::println!("index: {index:?}");

        let position = if self.positions.is_none() {
            Vec3::ZERO
        } else {
            let positions = slab.read(self.positions);
            crate::println!("positions: {positions:?}");
            positions.get_vec3(index, slab)
        };
        crate::println!("position: {position:?}");

        let normal = if self.normals.is_none() {
            Vec3::Z
        } else {
            let normals = slab.read(self.normals);
            crate::println!("normals: {normals:?}");
            // If the normals were generated on the CPU, the index from
            // `indices` won't be the same as the index from `normals`.
            if self.normals_were_generated {
                normals.get_vec3(vertex_index, slab)
            } else {
                normals.get_vec3(index, slab)
            }
        };
        crate::println!("normal: {normal:?}");

        let tangent = if self.tangents.is_none() {
            Vec4::Y
        } else {
            let tangents = slab.read(self.tangents);
            crate::println!("tangents: {tangents:?}");
            // If the tangents were generated on the CPU, the index from
            // `indices` won't be the same as the index from `tangents`.
            if self.tangents_were_generated {
                tangents.get_vec4(vertex_index, slab)
            } else {
                tangents.get_vec4(index, slab)
            }
        };
        crate::println!("tangent: {tangent:?}");

        let color = if self.colors.is_none() {
            Vec4::ONE
        } else {
            let colors = slab.read(self.colors);
            crate::println!("colors: {colors:?}");
            colors.get_vec4(index, slab)
        };
        crate::println!("color: {color:?}");

        let tex_coords0 = if self.tex_coords0.is_none() {
            Vec2::ZERO
        } else {
            let tex_coords0 = slab.read(self.tex_coords0);
            crate::println!("tex_coords0: {tex_coords0:?}");
            tex_coords0.get_vec2(index, slab)
        };
        crate::println!("tex_coords0: {tex_coords0:?}");

        let tex_coords1 = if self.tex_coords1.is_none() {
            Vec2::ZERO
        } else {
            let tex_coords1 = slab.read(self.tex_coords1);
            crate::println!("tex_coords1: {tex_coords1:?}");
            tex_coords1.get_vec2(index, slab)
        };
        crate::println!("tex_coords1: {tex_coords1:?}");

        let uv = Vec4::new(tex_coords0.x, tex_coords0.y, tex_coords1.x, tex_coords1.y);
        crate::println!("uv: {uv:?}");

        let joints = if self.joints.is_none() {
            [0; 4]
        } else {
            let joints = slab.read(self.joints);
            crate::println!("joints: {joints:?}");
            let joints = joints.get_uvec4(index, slab);
            crate::println!("joints: {joints:?}");
            [joints.x, joints.y, joints.z, joints.w]
        };
        crate::println!("joints: {joints:?}");

        let weights = if self.weights.is_none() {
            [0.0; 4]
        } else {
            let weights = slab.read(self.weights);
            crate::println!("weights: {weights:?}");
            let weights = weights.get_vec4(index, slab);
            crate::println!("weights: {weights:?}");
            [weights.x, weights.y, weights.z, weights.w]
        };
        crate::println!("weights: {weights:?}");

        crate::stage::Vertex {
            position: position.extend(0.0),
            color,
            uv,
            normal: normal.extend(0.0),
            tangent,
            joints,
            weights,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfMesh {
    pub primitives: Array<GltfPrimitive>,
    pub weights: Array<f32>,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
pub enum GltfCamera {
    Orthographic {
        xmag: f32,
        ymag: f32,
        zfar: f32,
        znear: f32,
    },
    Perspective {
        aspect_ratio: f32,
        yfov: f32,
        zfar: f32,
        znear: f32,
    },
}

impl GltfCamera {
    const ORTHOGRAPHIC_HASH: u32 = 0;
    const PERSPECTIVE_HASH: u32 = 1;
}

impl Default for GltfCamera {
    fn default() -> Self {
        GltfCamera::Orthographic {
            xmag: 0.0,
            ymag: 0.0,
            zfar: 0.0,
            znear: 0.0,
        }
    }
}

impl Slabbed for GltfCamera {
    fn slab_size() -> usize {
        1 + 4
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let original_index = index;
        let mut hash = 0u32;
        let index = hash.read_slab(index, slab);
        match hash {
            Self::ORTHOGRAPHIC_HASH => {
                let mut xmag = 0.0;
                let mut ymag = 0.0;
                let mut zfar = 0.0;
                let mut znear = 0.0;
                let index = xmag.read_slab(index, slab);
                let index = ymag.read_slab(index, slab);
                let index = zfar.read_slab(index, slab);
                let _index = znear.read_slab(index, slab);
                *self = Self::Orthographic {
                    xmag,
                    ymag,
                    zfar,
                    znear,
                };
            }
            Self::PERSPECTIVE_HASH => {
                let mut aspect_ratio = 0.0;
                let mut yfov = 0.0;
                let mut zfar = 0.0;
                let mut znear = 0.0;
                let index = aspect_ratio.read_slab(index, slab);
                let index = yfov.read_slab(index, slab);
                let index = zfar.read_slab(index, slab);
                let _index = znear.read_slab(index, slab);
                *self = Self::Perspective {
                    aspect_ratio,
                    yfov,
                    zfar,
                    znear,
                };
            }
            _ => return index,
        }
        original_index + Self::slab_size()
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        match self {
            Self::Orthographic {
                xmag,
                ymag,
                zfar,
                znear,
            } => {
                let index = Self::ORTHOGRAPHIC_HASH.write_slab(index, slab);
                let index = xmag.write_slab(index, slab);
                let index = ymag.write_slab(index, slab);
                let index = zfar.write_slab(index, slab);
                let _ = znear.write_slab(index, slab);
            }
            Self::Perspective {
                aspect_ratio,
                yfov,
                zfar,
                znear,
            } => {
                let index = Self::PERSPECTIVE_HASH.write_slab(index, slab);
                let index = aspect_ratio.write_slab(index, slab);
                let index = yfov.write_slab(index, slab);
                let index = zfar.write_slab(index, slab);
                let _ = znear.write_slab(index, slab);
            }
        }
        index + Self::slab_size()
    }
}

#[derive(Default, Clone, Copy)]
pub enum GltfLightKind {
    #[default]
    Directional,
    Point,
    Spot {
        inner_cone_angle: f32,
        outer_cone_angle: f32,
    },
}

impl Slabbed for GltfLightKind {
    fn slab_size() -> usize {
        1 // hash
            + 2 // inner_cone_angle, outer_cone_angle
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let original_index = index;
        let mut hash = 0u32;
        let index = hash.read_slab(index, slab);
        match hash {
            0 => *self = Self::Directional,
            1 => *self = Self::Point,
            2 => {
                let mut inner_cone_angle = 0.0;
                let mut outer_cone_angle = 0.0;
                let index = inner_cone_angle.read_slab(index, slab);
                let _index = outer_cone_angle.read_slab(index, slab);
                *self = Self::Spot {
                    inner_cone_angle,
                    outer_cone_angle,
                };
            }
            _ => return index,
        }
        original_index + Self::slab_size()
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        match self {
            Self::Directional => {
                let _index = 0.write_slab(index, slab);
            }
            Self::Point => {
                let _index = 1.write_slab(index, slab);
            }
            Self::Spot {
                inner_cone_angle,
                outer_cone_angle,
            } => {
                let index = 2.write_slab(index, slab);
                let index = inner_cone_angle.write_slab(index, slab);
                let _index = outer_cone_angle.write_slab(index, slab);
            }
        }
        index + Self::slab_size()
    }
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfLight {
    pub color: glam::Vec3,
    pub intensity: f32,
    // If `range` is f32::MAX, then the light is a directional light.
    pub range: f32,
    pub kind: GltfLightKind,
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfSkin {
    pub joints: Array<Id<GltfNode>>,
    pub inverse_bind_matrices: Id<GltfAccessor>,
    pub skeleton: Id<GltfNode>,
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfNode {
    pub camera: Id<GltfCamera>,
    pub children: Array<Id<GltfNode>>,
    pub mesh: Id<GltfMesh>,
    pub light: Id<GltfLight>,
    pub skin: Id<GltfSkin>,
    pub weights: Array<f32>,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq)]
pub enum GltfInterpolation {
    #[default]
    Linear,
    Step,
    CubicSpline,
}

impl Slabbed for GltfInterpolation {
    fn slab_size() -> usize {
        1
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let original_index = index;
        let mut proxy = 0u32;
        let index = proxy.read_slab(index, slab);
        match proxy {
            0 => *self = GltfInterpolation::Linear,
            1 => *self = GltfInterpolation::Step,
            2 => *self = GltfInterpolation::CubicSpline,
            _ => return original_index,
        }
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        match self {
            GltfInterpolation::Linear => 0,
            GltfInterpolation::Step => 1,
            GltfInterpolation::CubicSpline => 2,
        }
        .write_slab(index, slab)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Slabbed)]
pub struct GltfAnimationSampler {
    pub input: Id<GltfAccessor>,
    pub output: Id<GltfAccessor>,
    pub interpolation: GltfInterpolation,
}

#[repr(u32)]
#[derive(Default, Clone, Copy)]
pub enum GltfTargetProperty {
    #[default]
    Translation,
    Rotation,
    Scale,
    MorphTargetWeights,
}

impl Slabbed for GltfTargetProperty {
    fn slab_size() -> usize {
        1
    }

    fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
        let original_index = index;
        let mut proxy = 0u32;
        let index = proxy.read_slab(index, slab);
        match proxy {
            0 => *self = GltfTargetProperty::Translation,
            1 => *self = GltfTargetProperty::Rotation,
            2 => *self = GltfTargetProperty::Scale,
            3 => *self = GltfTargetProperty::MorphTargetWeights,
            _ => return original_index,
        }
        index
    }

    fn write_slab(&self, index: usize, slab: &mut [u32]) -> usize {
        match self {
            GltfTargetProperty::Translation => 0,
            GltfTargetProperty::Rotation => 1,
            GltfTargetProperty::Scale => 2,
            GltfTargetProperty::MorphTargetWeights => 3,
        }
        .write_slab(index, slab)
    }
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfTarget {
    pub node: Id<GltfNode>,
    pub property: GltfTargetProperty,
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfChannel {
    pub sampler: Id<GltfAnimationSampler>,
    pub target: GltfTarget,
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfAnimation {
    pub channels: Array<GltfChannel>,
    pub samplers: Array<GltfAnimationSampler>,
}

#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfScene {
    pub nodes: Array<Id<GltfNode>>,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfBufferView {
    pub buffer: Id<GltfBuffer>,
    pub offset: u32,
    pub length: u32,
    pub stride: u32,
}

/// A document of Gltf data.
///
/// This tells where certain parts of the Gltf document are stored in the [`Stage`]'s slab.
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfDocument {
    pub accessors: Array<GltfAccessor>,
    pub animations: Array<GltfAnimation>,
    pub buffers: Array<GltfBuffer>,
    pub cameras: Array<GltfCamera>,
    // TODO: Think about making a `GltfMaterial`
    pub materials: Array<PbrMaterial>,
    pub default_material: Id<PbrMaterial>,
    pub meshes: Array<GltfMesh>,
    pub nodes: Array<GltfNode>,
    pub scenes: Array<GltfScene>,
    pub skins: Array<GltfSkin>,
    // TODO: Think about making a `GltfTexture`
    pub textures: Array<GpuTexture>,
    pub views: Array<GltfBufferView>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn indices_accessor_sanity() {
        // Taken from the indices accessor in the "simple meshes" gltf sample,
        // but with the buffer changed to match where we write it here.
        let buffer_id = Id::new(20);
        let accessor = GltfAccessor {
            size: 2,
            buffer: buffer_id,
            view_offset: 0,
            view_stride: 0,
            count: 3,
            data_type: DataType::U16,
            dimensions: Dimensions::Scalar,
            normalized: false,
        };
        let buffer = GltfBuffer(Array::new(0, 11));
        let mut slab: [u32; 22] = [
            65536, 2, 0, 0, 0, 1065353216, 0, 0, 0, 1065353216, 0, 0, 0, 1065353216, 0, 0,
            1065353216, 0, 0, 1065353216, 0, 0,
        ];
        slab.write(&buffer, buffer_id.index());
        let i0 = accessor.get_u32(0, &slab);
        let i1 = accessor.get_u32(1, &slab);
        let i2 = accessor.get_u32(2, &slab);
        assert_eq!([0, 1, 2], [i0, i1, i2]);
    }
}
