//! Texture atlas.
//!
//! All images are packed into an atlas at staging time.
//! Texture descriptors describing where in the atlas an image is,
//! and how callsites should sample pixels is packed into a buffer
//! on the GPU. This keeps the number of texture binds to a minimum.
//!
//! ## NOTE:
//! `Atlas` is a temporary work around until we can use bindless techniques
//! on web.
//!
//! `Atlas` is only available on CPU. Not available in shaders.
use crabslab::{Id, Slab, SlabItem};
use glam::{UVec2, UVec3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};

#[cfg(not(target_arch = "spirv"))]
mod atlas_image;
#[cfg(not(target_arch = "spirv"))]
pub use atlas_image::*;
#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;
use spirv_std::{image::Image2d, spirv, Sampler};

/// Describes various qualities of the atlas, to be used on the GPU.
#[derive(Clone, Copy, core::fmt::Debug, Default, PartialEq, SlabItem)]
pub struct AtlasDescriptor {
    pub size: UVec3,
}

/// Method of addressing the edges of a texture.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, SlabItem)]
pub struct TextureModes {
    pub s: TextureAddressMode,
    pub t: TextureAddressMode,
}

/// A texture inside the atlas.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, SlabItem)]
pub struct AtlasTexture {
    /// The top left offset of texture in the atlas.
    pub offset_px: UVec2,
    /// The size of the texture in the atlas.
    pub size_px: UVec2,
    /// The index of the layer within the atlas that this `AtlasTexture `belongs to.
    pub layer_index: u32,
    /// The index of this frame within the layer.
    pub frame_index: u32,
    /// Various toggles of texture modes.
    pub modes: TextureModes,
}

impl AtlasTexture {
    /// Transform the given `uv` coordinates for this texture's address mode
    /// and placement in the atlas of the given size.
    pub fn uv(&self, mut uv: Vec2, atlas_size: UVec2) -> Vec3 {
        uv.x = self.modes.s.wrap(uv.x);
        uv.y = self.modes.t.wrap(uv.y);

        // get the pixel index of the uv coordinate in terms of the original image
        let mut px_index_s = (uv.x * self.size_px.x as f32) as u32;
        let mut px_index_t = (uv.y * self.size_px.y as f32) as u32;

        // convert the pixel index from image to atlas space
        px_index_s += self.offset_px.x;
        px_index_t += self.offset_px.y;

        let sx = atlas_size.x as f32;
        let sy = atlas_size.y as f32;
        // normalize the pixels by dividing by the atlas size
        let uv_s = px_index_s as f32 / sx;
        let uv_t = px_index_t as f32 / sy;

        Vec2::new(uv_s, uv_t).extend(self.layer_index as f32)
    }

    /// Constrain the input `clip_pos` to be within the bounds of this texture
    /// within its atlas, in texture space.
    pub fn constrain_clip_coords_to_texture_space(
        &self,
        clip_pos: Vec2,
        atlas_size: UVec2,
    ) -> Vec2 {
        // Convert `clip_pos` into uv coords to figure out where in the texture
        // this point lives
        let input_uv = (clip_pos * Vec2::new(1.0, -1.0) + Vec2::splat(1.0)) * Vec2::splat(0.5);
        self.uv(input_uv, atlas_size).xy()
    }

    /// Constrain the input `clip_pos` to be within the bounds of this texture
    /// within its atlas.
    pub fn constrain_clip_coords(&self, clip_pos: Vec2, atlas_size: UVec2) -> Vec2 {
        let uv = self.constrain_clip_coords_to_texture_space(clip_pos, atlas_size);
        // Convert `uv` back into clip space
        (uv * Vec2::new(2.0, 2.0) - Vec2::splat(1.0)) * Vec2::new(1.0, -1.0)
    }

    #[cfg(cpu)]
    /// Returns the frame of this texture as a [`wgpu::Origin3d`].
    pub fn origin(&self) -> wgpu::Origin3d {
        wgpu::Origin3d {
            x: self.offset_px.x,
            y: self.offset_px.y,
            z: self.layer_index,
        }
    }

    #[cfg(cpu)]
    /// Returns the frame of this texture as a [`wgpu::Extent3d`].
    pub fn size_as_extent(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.size_px.x,
            height: self.size_px.y,
            depth_or_array_layers: 1,
        }
    }
}

/// Infinitely wrap the input between 0.0 and 1.0.
///
/// Only handles `input` >= 0.0.
pub fn repeat(mut input: f32) -> f32 {
    let gto = input >= 1.0;
    input %= 1.0;
    if gto && input == 0.0 {
        1.0
    } else {
        input
    }
}

/// Clamp the input between 0.0 and 1.0.
pub fn clamp(input: f32) -> f32 {
    if input > 1.0 {
        1.0 - f32::EPSILON
    } else if input < 0.0 {
        0.0 + f32::EPSILON
    } else {
        input
    }
}

/// How edges should be handled in texture addressing/wrapping.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(u32)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, SlabItem)]
pub enum TextureAddressMode {
    #[default]
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

impl core::fmt::Display for TextureAddressMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match *self {
            TextureAddressMode::ClampToEdge => "clamp to edge",
            TextureAddressMode::Repeat => "repeat",
            TextureAddressMode::MirroredRepeat => "mirrored repeat",
        })
    }
}

impl TextureAddressMode {
    /// Wrap the given s/t coord into a pixel index according to texture
    /// addressing.
    pub fn wrap(&self, input: f32) -> f32 {
        match self {
            TextureAddressMode::Repeat => {
                let sign = if input >= 0.0 { 1.0f32 } else { -1.0 };
                let input = repeat(input.abs());
                if sign > 0.0 {
                    input
                } else {
                    1.0 - input
                }
            }
            TextureAddressMode::MirroredRepeat => {
                let sign = if input >= 0.0 { 1.0f32 } else { -1.0 };
                let i = input.abs();
                let flip = i as u32 % 2 == 0;
                let t = repeat(i);
                if sign > 0.0 {
                    if flip {
                        t
                    } else {
                        1.0 - t
                    }
                } else if flip {
                    1.0 - t
                } else {
                    t
                }
            }
            _ => clamp(input),
        }
    }
}

#[derive(Clone, Copy, Default, SlabItem, core::fmt::Debug)]
pub struct AtlasBlittingDescriptor {
    pub atlas_texture_id: Id<AtlasTexture>,
    pub atlas_desc_id: Id<AtlasDescriptor>,
}

/// Vertex shader for blitting a texture into a the frame of an [`AtlasTexture`].
///
/// This is useful for copying textures of unsupported formats, or
/// textures of different sizes.
#[spirv(vertex)]
pub fn atlas_blit_vertex(
    #[spirv(vertex_index)] vertex_id: u32,
    #[spirv(instance_index)] atlas_blitting_desc_id: Id<AtlasBlittingDescriptor>,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] slab: &[u32],
    out_uv: &mut Vec2,
    #[spirv(position)] out_pos: &mut Vec4,
) {
    let i = vertex_id as usize;
    *out_uv = crate::math::UV_COORD_QUAD_CCW[i];

    crate::println!("atlas_blitting_desc_id: {atlas_blitting_desc_id:?}");
    let atlas_blitting_desc = slab.read_unchecked(atlas_blitting_desc_id);
    crate::println!("atlas_blitting_desc: {atlas_blitting_desc:?}");
    let atlas_texture = slab.read_unchecked(atlas_blitting_desc.atlas_texture_id);
    crate::println!("atlas_texture: {atlas_texture:?}");
    let atlas_desc = slab.read_unchecked(atlas_blitting_desc.atlas_desc_id);
    crate::println!("atlas_desc: {atlas_desc:?}");
    let clip_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
    crate::println!("clip_pos: {clip_pos:?}");
    *out_pos = atlas_texture
        .constrain_clip_coords(clip_pos.xy(), atlas_desc.size.xy())
        .extend(clip_pos.z)
        .extend(clip_pos.w);
    crate::println!("out_pos: {out_pos}");
}

/// Fragment shader for blitting a texture into a frame of an atlas.
#[spirv(fragment)]
pub fn atlas_blit_fragment(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    frag_color: &mut Vec4,
) {
    *frag_color = texture.sample(*sampler, in_uv);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_repeat() {
        assert_eq!(0.0, TextureAddressMode::Repeat.wrap(0.0));
        assert_eq!(1.0, TextureAddressMode::Repeat.wrap(2.0));
        assert_eq!(1.0, TextureAddressMode::Repeat.wrap(3.0));
    }

    #[test]
    /// Tests that clip coordinates can be converted into texture coords within
    /// a specific `AtlasTexture`, and back again.
    fn constrain_clip_coords_sanity() {
        let atlas_texture = AtlasTexture {
            offset_px: UVec2::splat(0),
            size_px: UVec2::splat(800),
            layer_index: 0,
            frame_index: 0,
            modes: TextureModes {
                s: TextureAddressMode::ClampToEdge,
                t: TextureAddressMode::ClampToEdge,
            },
        };
        let atlas_size = UVec3::new(1024, 1024, 4);
        let corners @ [tl, tr, br, bl] = [
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TL,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BL,
        ]
        .map(|coord| {
            atlas_texture.constrain_clip_coords_to_texture_space(coord.xy(), atlas_size.xy())
        });
        log::info!("uv_corners: {corners:#?}");

        let clip_br = crate::math::CLIP_SPACE_COORD_QUAD_CCW_BR.xy();
        log::info!("clip_br: {clip_br}");
        let input_uv_br = (clip_br * Vec2::new(1.0, -1.0) + Vec2::splat(1.0)) * Vec2::splat(0.5);
        log::info!("input_uv_br: {input_uv_br}");
        assert_eq!(Vec2::ONE, input_uv_br, "incorrect uv");

        let d = 800.0 / 1024.0;
        assert_eq!(Vec2::splat(0.0), tl, "incorrect tl");
        assert_eq!(Vec2::new(d, 0.0), tr, "incorrect tr");
        assert_eq!(Vec2::new(d, d), br, "incorrect br");
        assert_eq!(Vec2::new(0.0, d), bl, "incorrect bl");

        let corners = [
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TL,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_TR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BR,
            crate::math::CLIP_SPACE_COORD_QUAD_CCW_BL,
        ]
        .map(|coord| atlas_texture.constrain_clip_coords(coord.xy(), atlas_size.xy()));
        log::info!("clip_corners: {corners:#?}");
        //     [
        //     Vec2(
        //         -1.0,
        //         1.0,
        //     ),
        //     Vec2(
        //         0.5625,
        //         1.0,
        //     ),
        //     Vec2(
        //         0.5625,
        //         -0.5625,
        //     ),
        //     Vec2(
        //         -1.0,
        //         -0.5625,
        //     ),
        // ]
    }
}
