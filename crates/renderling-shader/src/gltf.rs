//! Gltf types that are used in shaders.
use glam::Vec4;

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

macro_rules! println {
    ($($arg:tt)*) => {
        #[cfg(not(target_arch = "spirv"))]
        {
            std::println!($($arg)*);
        }
    }
}

impl GltfAccessor {
    /// Retreive one component of the nth element.
    pub fn get(&self, index: usize, component_index: usize, slab: &[u32]) -> u32 {
        println!("get {index} {component_index}");
        let buffer = slab.read(self.buffer);
        println!("buffer: {buffer:?}");
        let buffer_start = buffer.0.at(0);
        let buffer_start_bytes = buffer_start.index() * 4;
        let (mask, component_bytes, mut component_shift) = match self.data_type {
            DataType::I8 => (0xF, 1, 8),
            DataType::U8 => (0xF, 1, 8),
            DataType::I16 => (0xFF, 2, 16),
            DataType::U16 => (0xFF, 2, 16),
            DataType::U32 => (0xFFFF, 4, 0),
            DataType::F32 => (0xFFFF, 4, 0),
        };
        component_shift *= component_index as u32;
        let component_byte_offset = component_bytes * component_index;
        let byte_offset = buffer_start_bytes
            + self.view_offset as usize
            + index * self.size as usize
            + component_byte_offset;
        println!("byte_offset: {byte_offset}");
        let u32_offset = byte_offset / 4;
        println!("u32_offset: {u32_offset}");
        let mut t = 0u32;
        t.read_slab(u32_offset, slab);
        let byte_mod = byte_offset as u32 % 4;
        println!("byte_mod: {byte_mod}");
        let shift = match byte_mod {
            0 => 0,
            1 => 8,
            2 => 16,
            3 => 24,
            _ => 0, // unreachable
        };
        println!("mask: {mask:04x}");
        println!("shift: {shift}");
        println!("component_shift: {component_shift}");
        let u = crate::bits::extract(t, (shift + component_shift, mask));
        println!("u: {u}");
        u
    }

    pub fn get_u32(&self, vertex_index: usize, slab: &[u32]) -> u32 {
        self.get(vertex_index, 0, slab)
    }

    pub fn get_f32(&self, vertex_index: usize, slab: &[u32]) -> f32 {
        f32::from_bits(self.get(vertex_index, 0, slab))
    }

    pub fn get_vec2(&self, vertex_index: usize, slab: &[u32]) -> glam::Vec2 {
        let x = f32::from_bits(self.get(vertex_index, 0, slab));
        let y = f32::from_bits(self.get(vertex_index, 1, slab));
        match self.dimensions {
            Dimensions::Scalar => glam::Vec2::new(x, 0.0),
            _ => glam::Vec2::new(x, y),
        }
    }

    pub fn get_vec3(&self, vertex_index: usize, slab: &[u32]) -> glam::Vec3 {
        let x = f32::from_bits(self.get(vertex_index, 0, slab));
        let y = f32::from_bits(self.get(vertex_index, 1, slab));
        let z = f32::from_bits(self.get(vertex_index, 2, slab));
        match self.dimensions {
            Dimensions::Scalar => glam::Vec3::new(x, 0.0, 0.0),
            Dimensions::Vec2 => glam::Vec3::new(x, y, 0.0),
            _ => glam::Vec3::new(x, y, z),
        }
    }

    pub fn get_vec4(&self, vertex_index: usize, slab: &[u32]) -> glam::Vec4 {
        let x = f32::from_bits(self.get(vertex_index, 0, slab));
        let y = f32::from_bits(self.get(vertex_index, 1, slab));
        let z = f32::from_bits(self.get(vertex_index, 2, slab));
        let w = f32::from_bits(self.get(vertex_index, 3, slab));
        match self.dimensions {
            Dimensions::Scalar => glam::Vec4::new(x, 0.0, 0.0, 0.0),
            Dimensions::Vec2 => glam::Vec4::new(x, y, 0.0, 0.0),
            Dimensions::Vec3 => glam::Vec4::new(x, y, z, 0.0),
            _ => glam::Vec4::new(x, y, z, w),
        }
    }

    pub fn get_uvec4(&self, vertex_index: usize, slab: &[u32]) -> glam::UVec4 {
        let x = self.get_u32(vertex_index, slab);
        let y = self.get_u32(vertex_index, slab);
        let z = self.get_u32(vertex_index, slab);
        let w = self.get_u32(vertex_index, slab);
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
    pub tangents: Id<GltfAccessor>,
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
        let positions = slab.read(self.positions);
        let position = positions.get_vec3(index, slab);
        let normals = slab.read(self.normals);
        let normal = normals.get_vec3(index, slab);
        let tangents = slab.read(self.tangents);
        let tangent = tangents.get_vec4(index, slab);
        let colors = slab.read(self.colors);
        let color = colors.get_vec4(index, slab);
        let tex_coords0 = slab.read(self.tex_coords0);
        let tex_coords0 = tex_coords0.get_vec2(index, slab);
        let tex_coords1 = slab.read(self.tex_coords1);
        let tex_coords1 = tex_coords1.get_vec2(index, slab);
        let uv = Vec4::new(tex_coords0.x, tex_coords0.y, tex_coords1.x, tex_coords1.y);
        let joints = slab.read(self.joints);
        let joints = joints.get_uvec4(index, slab);
        let joints = [joints.x, joints.y, joints.z, joints.w];
        let weights = slab.read(self.weights);
        let weights = weights.get_vec4(index, slab);
        let weights = [weights.x, weights.y, weights.z, weights.w];
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
    pub meshes: Array<GltfMesh>,
    pub nodes: Array<GltfNode>,
    pub scenes: Array<GltfScene>,
    pub skins: Array<GltfSkin>,
    // TODO: Think about making a `GltfTexture`
    pub textures: Array<GpuTexture>,
    pub views: Array<GltfBufferView>,
}
