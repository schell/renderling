//! Gltf types that are used in shaders.
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::{
    math::{IsMatrix, IsVector},
    pbr::Material,
    stage::Vertex,
    AtlasTexture, Camera, Transform,
};

#[repr(transparent)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfBuffer(pub Array<u32>);

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfBufferView {
    // Pointer to the parent buffer.
    pub buffer: Id<GltfBuffer>,
    // The offset relative to the start of the parent buffer in bytes.
    pub offset: u32,
    // The length of the buffer view in bytes.
    pub length: u32,
    // The stride in bytes between vertex attributes or other interleavable data.
    //
    // When 0, data is assumed to be tightly packed.
    pub stride: u32,
}

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

impl SlabItem for DataType {
    const SLAB_SIZE: usize = { 1 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let hash = u32::read_slab(index, slab);
        match hash {
            0 => DataType::I8,
            1 => DataType::U8,
            2 => DataType::I16,
            3 => DataType::U16,
            4 => DataType::U32,
            5 => DataType::F32,
            _ => DataType::U32,
        }
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

impl SlabItem for Dimensions {
    const SLAB_SIZE: usize = { 1 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let hash = u32::read_slab(index, slab);
        match hash {
            0 => Dimensions::Scalar,
            1 => Dimensions::Vec2,
            2 => Dimensions::Vec3,
            3 => Dimensions::Vec4,
            4 => Dimensions::Mat2,
            5 => Dimensions::Mat3,
            6 => Dimensions::Mat4,
            _ => Dimensions::Scalar,
        }
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
#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfAccessor {
    // The byte size of each element that this accessor describes.
    /// For example, if the accessor describes a `Vec3` of F32s, then
    // the size is 3 * 4 = 12.
    pub size: u32,
    // A point to the parent view this accessor reads from.
    /// This may be Id::NONE if the corresponding accessor is sparse.
    pub view: Id<GltfBufferView>,
    // The offset relative to the start of the parent **buffer view** in bytes.
    //
    // This will be 0 if the corresponding accessor is sparse.
    pub offset: u32,
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
        let view = slab.read(self.view);
        let buffer = slab.read(view.buffer);
        let buffer_start = buffer.0.starting_index();
        let buffer_start_bytes = buffer_start * 4;
        let stride = if self.size > view.stride {
            self.size
        } else {
            view.stride
        } as usize;
        let byte_offset = buffer_start_bytes
            + view.offset as usize
            + self.offset as usize
            + element_index as usize * stride;
        let slab_index = byte_offset / 4;
        let relative_byte_offset = byte_offset % 4;
        (slab_index, relative_byte_offset)
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
#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfPrimitive {
    pub vertex_count: u32,
    pub material: Id<Material>,
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

        let position = if self.positions.is_none() {
            Vec3::ZERO
        } else {
            let positions = slab.read(self.positions);
            positions.get_vec3(index, slab)
        };

        let normal = if self.normals.is_none() {
            Vec3::Z
        } else {
            let normals = slab.read(self.normals);
            // If the normals were generated on the CPU, the index from
            // `indices` won't be the same as the index from `normals`.
            if self.normals_were_generated {
                normals.get_vec3(vertex_index, slab)
            } else {
                normals.get_vec3(index, slab)
            }
        };

        let tangent = if self.tangents.is_none() {
            Vec4::Y
        } else {
            let tangents = slab.read(self.tangents);
            // If the tangents were generated on the CPU, the index from
            // `indices` won't be the same as the index from `tangents`.
            if self.tangents_were_generated {
                tangents.get_vec4(vertex_index, slab)
            } else {
                tangents.get_vec4(index, slab)
            }
        };

        let color = if self.colors.is_none() {
            Vec4::ONE
        } else {
            let colors = slab.read(self.colors);
            colors.get_vec4(index, slab)
        };

        let tex_coords0 = if self.tex_coords0.is_none() {
            Vec2::ZERO
        } else {
            let tex_coords0 = slab.read(self.tex_coords0);
            tex_coords0.get_vec2(index, slab)
        };

        let tex_coords1 = if self.tex_coords1.is_none() {
            Vec2::ZERO
        } else {
            let tex_coords1 = slab.read(self.tex_coords1);
            tex_coords1.get_vec2(index, slab)
        };

        let uv = Vec4::new(tex_coords0.x, tex_coords0.y, tex_coords1.x, tex_coords1.y);

        let joints = if self.joints.is_none() {
            [0; 4]
        } else {
            let joints = slab.read(self.joints);
            let joints = joints.get_uvec4(index, slab);
            [joints.x, joints.y, joints.z, joints.w]
        };

        let weights = if self.weights.is_none() {
            [0.0; 4]
        } else {
            let weights = slab.read(self.weights);
            let weights = weights.get_vec4(index, slab);
            [weights.x, weights.y, weights.z, weights.w]
        };

        crate::stage::Vertex {
            position,
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
#[derive(Default, Clone, Copy, SlabItem)]
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

impl SlabItem for GltfCamera {
    const SLAB_SIZE: usize = { 1 + 4 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let hash = u32::read_slab(index, slab);
        match hash {
            Self::ORTHOGRAPHIC_HASH => {
                let xmag = f32::read_slab(index + 1, slab);
                let ymag = f32::read_slab(index + 2, slab);
                let zfar = f32::read_slab(index + 3, slab);
                let znear = f32::read_slab(index + 4, slab);
                Self::Orthographic {
                    xmag,
                    ymag,
                    zfar,
                    znear,
                }
            }
            Self::PERSPECTIVE_HASH => {
                let aspect_ratio = f32::read_slab(index + 1, slab);
                let yfov = f32::read_slab(index + 2, slab);
                let zfar = f32::read_slab(index + 3, slab);
                let znear = f32::read_slab(index + 4, slab);
                Self::Perspective {
                    aspect_ratio,
                    yfov,
                    zfar,
                    znear,
                }
            }
            _ => Self::default(),
        }
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
        index + Self::SLAB_SIZE
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

impl SlabItem for GltfLightKind {
    const SLAB_SIZE: usize = {
        1 // hash
            + 2 // inner_cone_angle, outer_cone_angle
    };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let hash = u32::read_slab(index, slab);
        match hash {
            0 => Self::Directional,
            1 => Self::Point,
            2 => {
                let inner_cone_angle = f32::read_slab(index + 1, slab);
                let outer_cone_angle = f32::read_slab(index + 1, slab);
                Self::Spot {
                    inner_cone_angle,
                    outer_cone_angle,
                }
            }
            _ => Self::default(),
        }
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
        index + Self::SLAB_SIZE
    }
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfLight {
    pub color: glam::Vec3,
    pub intensity: f32,
    // If `range` is f32::MAX, then the light is a directional light.
    pub range: f32,
    pub kind: GltfLightKind,
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfSkin {
    pub joints: Array<Id<GltfNode>>,
    pub inverse_bind_matrices: Id<GltfAccessor>,
    pub skeleton: Id<GltfNode>,
}

#[derive(Clone, Copy, SlabItem)]
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

impl Default for GltfNode {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            children: Default::default(),
            mesh: Default::default(),
            light: Default::default(),
            skin: Default::default(),
            weights: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: glam::Vec3::ONE,
        }
    }
}

#[repr(u32)]
#[derive(Default, Clone, Copy, PartialEq)]
pub enum GltfInterpolation {
    #[default]
    Linear,
    Step,
    CubicSpline,
}

impl SlabItem for GltfInterpolation {
    const SLAB_SIZE: usize = { 1 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let proxy = u32::read_slab(index, slab);
        match proxy {
            0 => GltfInterpolation::Linear,
            1 => GltfInterpolation::Step,
            2 => GltfInterpolation::CubicSpline,
            _ => GltfInterpolation::Linear,
        }
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

#[derive(Default, Clone, Copy, PartialEq, SlabItem)]
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

impl SlabItem for GltfTargetProperty {
    const SLAB_SIZE: usize = { 1 };

    fn read_slab(index: usize, slab: &[u32]) -> Self {
        let proxy = u32::read_slab(index, slab);
        match proxy {
            0 => GltfTargetProperty::Translation,
            1 => GltfTargetProperty::Rotation,
            2 => GltfTargetProperty::Scale,
            3 => GltfTargetProperty::MorphTargetWeights,
            _ => GltfTargetProperty::Translation,
        }
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

#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfTarget {
    pub node: Id<GltfNode>,
    pub property: GltfTargetProperty,
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfChannel {
    pub sampler: Id<GltfAnimationSampler>,
    pub target: GltfTarget,
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfAnimation {
    pub channels: Array<GltfChannel>,
    pub samplers: Array<GltfAnimationSampler>,
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfScene {
    pub nodes: Array<Id<GltfNode>>,
}

/// A document of Gltf data.
///
/// This tells where certain parts of the Gltf document are stored in the
/// [`Stage`]'s slab.
#[derive(Default, Clone, Copy, SlabItem)]
pub struct GltfDocument {
    pub accessors: Array<GltfAccessor>,
    pub animations: Array<GltfAnimation>,
    pub buffers: Array<GltfBuffer>,
    pub cameras: Array<GltfCamera>,
    // TODO: Think about making a `GltfMaterial`
    pub materials: Array<Material>,
    pub default_material: Id<Material>,
    pub meshes: Array<GltfMesh>,
    pub nodes: Array<GltfNode>,
    pub scenes: Array<GltfScene>,
    pub skins: Array<GltfSkin>,
    // TODO: Think about making a `GltfTexture`
    pub textures: Array<AtlasTexture>,
    pub views: Array<GltfBufferView>,
}

/// A rendering of one gltf primitive from a top-level node.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, SlabItem)]
pub struct GltfRendering {
    // Which node are we rendering, and what is the path through its
    // ancestors to get to it.
    pub node_path: Array<Id<GltfNode>>,
    // Index of the mesh within the child node that we're rendering.
    pub mesh_index: u32,
    // Index of the primitive within the mesh that we're rendering.
    pub primitive_index: u32,
    // Points to a `Camera` in the stage's slab.
    pub camera: Id<Camera>,
    // Points to a top-level `Transform` in the stage's slab.
    //
    // This is used to transform your GLTF models.
    pub transform: Id<Transform>,
    // Number of vertices to draw for this unit.
    //
    // This is a cache for convenience on CPU.
    pub vertex_count: u32,
}

impl GltfRendering {
    pub fn get_vertex_details(
        &self,
        vertex_index: u32,
        slab: &[u32],
    ) -> (Vertex, Transform, Id<Material>) {
        let t = slab.read(self.transform);
        crate::println!("t: {t:#?}");
        let mut model =
            glam::Mat4::from_scale_rotation_translation(t.scale, t.rotation, t.translation);
        crate::println!("model: {model:#?}");
        let mut node = GltfNode::default();
        for id_id in self.node_path.iter() {
            let node_id = slab.read(id_id);
            crate::println!("  node_id: {node_id:?}");
            node = slab.read(node_id);
            crate::println!("  node.scale: {:?}", node.scale);
            crate::println!("  node.rotation: {:?}", node.rotation);
            crate::println!("  node.translation: {:?}", node.translation);
            let node_transform =
                Mat4::from_scale_rotation_translation(node.scale, node.rotation, node.translation);
            model = model * node_transform;
        }

        crate::println!("model(after): {model:#?}");
        // TODO: check nodes for skinning
        let mesh = slab.read(node.mesh);
        let primitive_id = mesh.primitives.at(self.primitive_index as usize);
        let primitive = slab.read(primitive_id);
        let material = primitive.material;
        let vertex = primitive.get_vertex(vertex_index as usize, slab);
        let (s, r, t) = model.to_scale_rotation_translation_or_id();
        let transform = Transform {
            translation: t,
            rotation: r,
            scale: s,
        };
        (vertex, transform, material)
    }
}

pub fn vertex(
    // Which render unit are we rendering
    render_id: Id<GltfRendering>,
    // Which vertex within the render unit are we rendering
    vertex_index: u32,
    slab: &[u32],
    out_camera: &mut u32,
    out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in world space
    out_pos: &mut Vec3,
    clip_pos: &mut Vec4,
) {
    let unit = slab.read(render_id);
    let (vertex, tfrm, material) = unit.get_vertex_details(vertex_index, slab);
    let model_matrix =
        Mat4::from_scale_rotation_translation(tfrm.scale, tfrm.rotation, tfrm.translation);
    *out_material = material.into();
    *out_color = vertex.color;
    *out_uv0 = vertex.uv.xy();
    *out_uv1 = vertex.uv.zw();
    let scale2 = tfrm.scale * tfrm.scale;
    let normal = vertex.normal.xyz().alt_norm_or_zero();
    let tangent = vertex.tangent.xyz().alt_norm_or_zero();
    let normal_w: Vec3 = (model_matrix * (normal / scale2).extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    let tangent_w: Vec3 = (model_matrix * tangent.extend(0.0))
        .xyz()
        .alt_norm_or_zero();
    let bitangent_w = normal_w.cross(tangent_w) * if vertex.tangent.w >= 0.0 { 1.0 } else { -1.0 };
    *out_tangent = tangent_w;
    *out_bitangent = bitangent_w;
    *out_norm = normal_w;
    let view_pos = model_matrix.transform_point3(vertex.position);
    *out_pos = view_pos;
    let camera = slab.read(unit.camera);
    *out_camera = unit.camera.into();
    *clip_pos = camera.projection * camera.view * view_pos.extend(1.0);
}
