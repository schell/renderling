//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage objects for rendering.
use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use crabslab::{Array, CpuSlab, GrowableSlab, Id, Slab, SlabItem, WgpuBuffer};
use glam::{UVec3, Vec2, Vec3, Vec4};
use moongraph::{View, ViewMut};
use snafu::Snafu;
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

use crate::{
    gltf::GltfRendering,
    math::IsVector,
    pbr::{debug::DebugMode, light::Light, PbrConfig},
    sdf::Sdf,
    texture::GpuTexture,
    Atlas, AtlasError, AtlasImage, AtlasImageError, Camera, CpuCubemap, DepthTexture, Device,
    HdrSurface, Queue, Skybox, WgpuSlabError,
};

#[cfg(feature = "gltf")]
mod gltf_support;

#[cfg(feature = "gltf")]
pub use gltf_support::*;

#[derive(Debug, Snafu)]
pub enum StageError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },

    #[snafu(display("{source}"))]
    Slab { source: WgpuSlabError },
}

impl From<AtlasError> for StageError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
    }
}

impl From<WgpuSlabError> for StageError {
    fn from(source: WgpuSlabError) -> Self {
        Self::Slab { source }
    }
}

/// A vertex in a mesh.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, SlabItem)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub normal: Vec4,
    pub tangent: Vec4,
    // Indices that point to this vertex's joints by indexing into an array of Id<GpuEntity>
    // provided by the GpuEntity that is using this vertex
    pub joints: [u32; 4],
    // The weights of influence that each joint has over this vertex
    pub weights: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Default::default(),
            color: Vec4::splat(1.0),
            uv: Vec4::splat(0.0),
            normal: Vec4::Z,
            tangent: Vec4::Y,
            joints: [0; 4],
            weights: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn with_position(mut self, p: impl Into<Vec3>) -> Self {
        self.position = p.into().extend(0.0);
        self
    }

    pub fn with_color(mut self, c: impl Into<Vec4>) -> Self {
        self.color = c.into();
        self
    }

    pub fn with_uv0(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uv.x = uv.x;
        self.uv.y = uv.y;
        self
    }

    pub fn with_uv1(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uv.z = uv.x;
        self.uv.w = uv.y;
        self
    }

    pub fn with_normal(mut self, n: impl Into<Vec3>) -> Self {
        self.normal = n.into().extend(0.0);
        self
    }

    pub fn with_tangent(mut self, t: impl Into<Vec4>) -> Self {
        self.tangent = t.into();
        self
    }

    ///// Return the matrix needed to bring vertices into the coordinate space of
    ///// the joint node.
    //pub fn get_joint_matrix(
    //    &self,
    //    i: usize,
    //    joint_ids: &[Id<GpuEntity>; 32],
    //    entities: &[GpuEntity],
    //) -> Mat4 {
    //    if i >= self.joints.len() {
    //        return Mat4::IDENTITY;
    //    }
    //    let joint_index = self.joints[i];
    //    let joint_id = if joint_index as usize >= joint_ids.len() {
    //        Id::NONE
    //    } else {
    //        joint_ids[joint_index as usize]
    //    };
    //    if joint_id.is_none() {
    //        return Mat4::IDENTITY;
    //    }
    //    let entity_index = joint_id.index();
    //    if entity_index >= entities.len() {
    //        return Mat4::IDENTITY;
    //    }
    //    let joint_entity = &entities[entity_index];
    //    let (t, r, s) = joint_entity.get_world_transform(entities);
    //    let trs = Mat4::from_scale_rotation_translation(s, r, t);
    //    trs * joint_entity.inverse_bind_matrix
    //}

    ///// Return the result of adding all joint matrices multiplied by their
    ///// weights for the given vertex.
    //// See the [khronos gltf viewer reference](https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/47a191931461a6f2e14de48d6da0f0eb6ec2d147/source/Renderer/shaders/animation.glsl#L47)
    //pub fn get_skin_matrix(&self, joint_ids: &[Id<GpuEntity>; 32], entities:
    // &[GpuEntity]) -> Mat4 {    let mut mat = Mat4::ZERO;
    //    for i in 0..self.joints.len() {
    //        mat += self.weights[i] * self.get_joint_matrix(i, joint_ids,
    // entities);    }
    //    if mat == Mat4::ZERO {
    //        return Mat4::IDENTITY;
    //    }
    //    mat
    //}

    pub fn generate_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        let ab = a - b;
        let ac = a - c;
        ab.cross(ac).normalize()
    }

    pub fn generate_tangent(a: Vec3, a_uv: Vec2, b: Vec3, b_uv: Vec2, c: Vec3, c_uv: Vec2) -> Vec4 {
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac);
        let d_uv1 = b_uv - a_uv;
        let d_uv2 = c_uv - a_uv;
        let denom = d_uv1.x * d_uv2.y - d_uv2.x * d_uv1.y;
        let denom_sign = if denom >= 0.0 { 1.0 } else { -1.0 };
        let denom = denom.abs().max(f32::EPSILON) * denom_sign;
        let f = 1.0 / denom;
        let s = f * Vec3::new(
            d_uv2.y * ab.x - d_uv1.y * ac.x,
            d_uv2.y * ab.y - d_uv1.y * ac.y,
            d_uv2.y * ab.z - d_uv1.y * ac.z,
        );
        let t = f * Vec3::new(
            d_uv1.x * ac.x - d_uv2.x * ab.x,
            d_uv1.x * ac.y - d_uv2.x * ab.y,
            d_uv1.x * ac.z - d_uv2.x * ab.z,
        );
        let n_cross_t_dot_s_sign = if n.cross(t).dot(s) >= 0.0 { 1.0 } else { -1.0 };
        (s - s.dot(n) * n)
            .alt_norm_or_zero()
            .extend(n_cross_t_dot_s_sign)
    }
}

/// Pointer to a renderable unit.
#[derive(Default, SlabItem)]
pub enum Rendering {
    #[default]
    None,
    /// Render the scene using the gltf vertex and fragment shaders.
    Gltf(Id<crate::gltf::GltfRendering>),
    /// Render the scene using the sdf vertex and fragment shaders.
    Sdf(Id<crate::sdf::Sdf>),
}

/// Uber vertex shader.
///
/// This reads the "instance" by index and proxies to a specific vertex shader.
#[spirv(vertex)]
pub fn vertex(
    // Points at a `Rendering`
    #[spirv(instance_index)] instance_index: u32,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] out_instance_index: &mut u32,
    #[spirv(flat)] out_camera: &mut u32,
    #[spirv(flat)] out_material: &mut u32,
    out_color: &mut Vec4,
    out_uv0: &mut Vec2,
    out_uv1: &mut Vec2,
    out_norm: &mut Vec3,
    out_tangent: &mut Vec3,
    out_bitangent: &mut Vec3,
    // position of the vertex/fragment in local space
    local_pos: &mut Vec3,
    // position of the vertex/fragment in world space
    world_pos: &mut Vec3,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    *out_instance_index = instance_index;
    let rendering = slab.read(Id::<Rendering>::new(instance_index));
    match rendering {
        Rendering::None => {}
        Rendering::Gltf(unit_id) => {
            crate::gltf::vertex(
                unit_id,
                vertex_index,
                slab,
                out_camera,
                out_material,
                out_color,
                out_uv0,
                out_uv1,
                out_norm,
                out_tangent,
                out_bitangent,
                world_pos,
                clip_pos,
            );
        }
        Rendering::Sdf(_sdf_id) => {
            *local_pos = Vec3::ZERO;
            //crate::sdf::vertex(
            //    sdf_id,
            //    vertex_index,
            //    slab,
            //    out_camera,
            //    out_material,
            //    out_color,
            //    out_uv0,
            //    out_uv1,
            //    out_norm,
            //    out_tangent,
            //    out_bitangent,
            //    local_pos,
            //    world_pos,
            //    clip_pos,
            //);
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
/// Uber fragment shader.
///
/// This reads the "instance" by index and proxies to a specific vertex shader.
pub fn fragment(
    #[spirv(descriptor_set = 1, binding = 0)] atlas: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] atlas_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 2)] irradiance: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 3)] irradiance_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 4)] prefiltered: &Cubemap,
    #[spirv(descriptor_set = 1, binding = 5)] prefiltered_sampler: &Sampler,

    #[spirv(descriptor_set = 1, binding = 6)] brdf: &Image2d,
    #[spirv(descriptor_set = 1, binding = 7)] brdf_sampler: &Sampler,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    #[spirv(flat)] in_rendering: Id<Rendering>,
    #[spirv(flat)] in_camera: u32,
    #[spirv(flat)] in_material: u32,
    in_color: Vec4,
    in_uv0: Vec2,
    in_uv1: Vec2,
    in_norm: Vec3,
    in_tangent: Vec3,
    in_bitangent: Vec3,
    local_pos: Vec3,
    world_pos: Vec3,

    output: &mut Vec4,
) {
    let rendering = slab.read(in_rendering);
    match rendering {
        Rendering::None => {}
        Rendering::Gltf(_) => {
            crate::pbr::fragment_impl(
                atlas,
                atlas_sampler,
                irradiance,
                irradiance_sampler,
                prefiltered,
                prefiltered_sampler,
                brdf,
                brdf_sampler,
                slab,
                0u32.into(),
                in_camera,
                in_material,
                in_color,
                in_uv0,
                in_uv1,
                in_norm,
                in_tangent,
                in_bitangent,
                world_pos,
                output,
            );
        }
        Rendering::Sdf(_sdf_id) => {
            *output = local_pos.extend(1.0);
            //crate::sdf::fragment(
            //    sdf_id,
            //    atlas,
            //    atlas_sampler,
            //    irradiance,
            //    irradiance_sampler,
            //    prefiltered,
            //    prefiltered_sampler,
            //    brdf,
            //    brdf_sampler,
            //    slab,
            //    in_camera,
            //    in_material,
            //    in_color,
            //    in_uv0,
            //    in_uv1,
            //    in_norm,
            //    in_tangent,
            //    in_bitangent,
            //    local_pos,
            //    world_pos,
            //    output,
            //);
        }
    }
}

//#[spirv(compute(threads(32)))]
///// Compute the draw calls for this frame.
/////
///// This should be called with `groupcount = (entities.len() / threads) + 1`.
//pub fn compute_cull_entities(
//    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] entities:
// &[GpuEntity],    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)]
// draws: &mut [DrawIndirect],    #[spirv(global_invocation_id)] global_id:
// UVec3,
//) {
//    let i = global_id.x as usize;
//
//    if i > entities.len() {
//        return;
//    }
//
//    // when the vertex count and/or instance count is 0, it effectively
// filters    // the draw call
//    let mut call = DrawIndirect {
//        vertex_count: 0,
//        instance_count: 0,
//        base_vertex: 0,
//        base_instance: i as u32,
//    };
//    let entity = &entities[i];
//    let is_visible = entity.visible != 0;
//    if entity.is_alive() && is_visible {
//        //// once naga supports atomics we can use this to compact the array
//        // let index = unsafe {
//        //    spirv_std::arch::atomic_i_increment::<
//        //        u32,
//        //        { spirv_std::memory::Scope::Device as u32 },
//        //        { spirv_std::memory::Semantics::NONE.bits() as u32 },
//        //    >(count)
//        //};
//        call.instance_count = 1;
//        call.base_vertex = entity.mesh_first_vertex;
//        call.vertex_count = entity.mesh_vertex_count;
//    }
//    draws[i] = call;
//}

#[spirv(compute(threads(32)))]
/// A shader to ensure that we can extract i8 and i16 values from a storage
/// buffer.
pub fn test_i8_i16_extraction(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &mut [u32],
    #[spirv(global_invocation_id)] global_id: UVec3,
) {
    let index = global_id.x as usize;
    let (value, _, _) = crate::bits::extract_i8(index, 2, slab);
    if value > 0 {
        slab[index] = value as u32;
    }
    let (value, _, _) = crate::bits::extract_i16(index, 2, slab);
    if value > 0 {
        slab[index] = value as u32;
    }
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
#[derive(Clone)]
pub struct Stage {
    pub(crate) slab: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) atlas: Arc<RwLock<Atlas>>,
    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) pipeline: Arc<Mutex<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) has_skybox: Arc<AtomicBool>,
    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) buffers_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) draws: Arc<RwLock<StageDrawStrategy>>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Slab for Stage {
    fn len(&self) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().len()
    }

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().read(id)
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().write_indexed(t, index)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().write_indexed_slice(t, index)
    }
}

impl GrowableSlab for Stage {
    fn capacity(&self) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().capacity()
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().reserve_capacity(capacity)
    }

    fn increment_len(&mut self, n: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().increment_len(n)
    }
}

impl Stage {
    /// Create a new stage.
    pub fn new(device: Device, queue: Queue) -> Self {
        let atlas = Atlas::empty(&device, &queue);
        let pbr_config = PbrConfig {
            atlas_size: atlas.size,
            ..Default::default()
        };
        let mut s = Self {
            slab: Arc::new(RwLock::new(CpuSlab::new(WgpuBuffer::new(
                device.0.clone(),
                queue.0.clone(),
                256,
            )))),
            pipeline: Default::default(),
            atlas: Arc::new(RwLock::new(atlas)),
            skybox: Arc::new(RwLock::new(Skybox::empty(device.clone(), queue.clone()))),
            skybox_bindgroup: Default::default(),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            has_bloom: Arc::new(AtomicBool::new(false)),
            buffers_bindgroup: Default::default(),
            textures_bindgroup: Default::default(),
            draws: Arc::new(RwLock::new(StageDrawStrategy::Direct(vec![]))),
            device,
            queue,
        };
        s.append(&pbr_config);
        s
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        let id: Id<DebugMode> = Id::from(PbrConfig::offset_of_debug_mode());
        self.write(id, &debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&mut self, use_lighting: bool) {
        let id = Id::<bool>::from(PbrConfig::offset_of_has_lighting());
        self.write(id, &use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(mut self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set the lights to use for shading.
    pub fn set_lights(&mut self, lights: Array<Light>) {
        let id = Id::<Array<Light>>::from(PbrConfig::offset_of_light_array());
        self.write(id, &lights);
    }

    /// Set the images to use for the atlas.
    ///
    /// Resets the atlas, packing it with the given images and returning a
    /// vector of the textures ready to be staged.
    ///
    /// ## WARNING
    /// This invalidates any currently staged `GpuTextures`.
    pub fn set_images(
        &mut self,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<Vec<GpuTexture>, StageError> {
        // UNWRAP: if we can't write the atlas we want to panic
        let mut atlas = self.atlas.write().unwrap();
        *atlas = Atlas::pack(&self.device, &self.queue, images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();
        // The atlas size must be reset
        let size_id = Id::<glam::UVec2>::from(PbrConfig::offset_of_atlas_size());
        // UNWRAP: if we can't write to the stage legend we want to panic
        self.slab.write().unwrap().write(size_id, &atlas.size);

        let textures = atlas
            .frames()
            .map(|(i, (offset_px, size_px))| GpuTexture {
                offset_px,
                size_px,
                atlas_index: i,
                ..Default::default()
            })
            .collect();
        Ok(textures)
    }

    /// Set the skybox.
    pub fn set_skybox(&self, skybox: Skybox) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut guard = self.skybox.write().unwrap();
        *guard = skybox;
        self.has_skybox
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Turn the bloom effect on or off.
    pub fn set_has_bloom(&self, has_bloom: bool) {
        self.has_bloom
            .store(has_bloom, std::sync::atomic::Ordering::Relaxed);
    }

    /// Turn the bloom effect on or off.
    pub fn with_bloom(self, has_bloom: bool) -> Self {
        self.set_has_bloom(has_bloom);
        self
    }

    fn buffers_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let visibility =
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE;
        let stage_slab = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let entries = vec![stage_slab];
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("stage slab buffers"),
            entries: &entries,
        })
    }

    fn textures_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        fn image2d_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
            let img = wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            };
            let sampler = wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            };
            (img, sampler)
        }

        fn cubemap_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
            let img = wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: false,
                },
                count: None,
            };
            let sampler = wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            };
            (img, sampler)
        }

        let (atlas, atlas_sampler) = image2d_entry(0);
        let (irradiance, irradiance_sampler) = cubemap_entry(2);
        let (prefilter, prefilter_sampler) = cubemap_entry(4);
        let (brdf, brdf_sampler) = image2d_entry(6);
        let (environment, environment_sampler) = cubemap_entry(8);
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("stage textures"),
            entries: &[
                atlas,
                atlas_sampler,
                irradiance,
                irradiance_sampler,
                prefilter,
                prefilter_sampler,
                brdf,
                brdf_sampler,
                environment,
                environment_sampler,
            ],
        })
    }

    /// Return the skybox render pipeline, creating it if necessary.
    pub fn get_skybox_pipeline_and_bindgroup(
        &self,
    ) -> (Arc<wgpu::RenderPipeline>, Arc<wgpu::BindGroup>) {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline = self.skybox_pipeline.write().unwrap();
        let pipeline = if let Some(pipeline) = pipeline.as_ref() {
            pipeline.clone()
        } else {
            let p = Arc::new(
                crate::skybox::create_skybox_render_pipeline(
                    &self.device,
                    crate::hdr::HdrSurface::TEXTURE_FORMAT,
                )
                .0,
            );
            *pipeline = Some(p.clone());
            p
        };
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.skybox_bindgroup.lock().unwrap();
        let bindgroup = if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let slab = self.slab.read().unwrap();
            let bg = Arc::new(crate::skybox::create_skybox_bindgroup(
                &self.device,
                slab.as_ref().get_buffer(),
                &self.skybox.read().unwrap().environment_cubemap,
            ));
            *bindgroup = Some(bg.clone());
            bg
        };
        (pipeline, bindgroup)
    }

    /// Return the main render pipeline, creating it if necessary.
    pub fn get_pipeline(&self) -> Arc<wgpu::RenderPipeline> {
        fn create_stage_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
            log::trace!("creating stage render pipeline");
            let label = Some("stage render pipeline");
            let vertex_shader =
                device.create_shader_module(wgpu::include_spirv!("linkage/stage-vertex.spv"));
            let fragment_shader =
                device.create_shader_module(wgpu::include_spirv!("linkage/stage-fragment.spv"));
            let stage_slab_buffers_layout = Stage::buffers_bindgroup_layout(device);
            let textures_layout = Stage::textures_bindgroup_layout(device);
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&stage_slab_buffers_layout, &textures_layout],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "stage::vertex",
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader,
                    entry_point: "stage::fragment",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });
            pipeline
        }

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline = self.pipeline.lock().unwrap();
        if let Some(pipeline) = pipeline.as_ref() {
            pipeline.clone()
        } else {
            let p = Arc::new(create_stage_render_pipeline(&self.device));
            *pipeline = Some(p.clone());
            p
        }
    }

    pub fn get_slab_buffers_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.buffers_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new({
                let device: &wgpu::Device = &self.device;
                let pipeline: &wgpu::RenderPipeline = &self.get_pipeline();
                let label = Some("stage slab buffer");
                let stage_slab_buffers_bindgroup =
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label,
                        layout: &pipeline.get_bind_group_layout(0),
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self
                                .slab
                                .read()
                                .unwrap()
                                .as_ref()
                                .get_buffer()
                                .as_entire_binding(),
                        }],
                    });
                stage_slab_buffers_bindgroup
            });
            *bindgroup = Some(b.clone());
            b
        }
    }

    pub fn get_textures_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        fn create_textures_bindgroup(
            device: &wgpu::Device,
            pipeline: &wgpu::RenderPipeline,
            atlas: &Atlas,
            skybox: &Skybox,
        ) -> wgpu::BindGroup {
            let label = Some("stage textures");
            let textures_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label,
                layout: &pipeline.get_bind_group_layout(1),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&atlas.texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&atlas.texture.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &skybox.irradiance_cubemap.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(
                            &skybox.irradiance_cubemap.sampler,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(
                            &skybox.prefiltered_environment_cubemap.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(
                            &skybox.prefiltered_environment_cubemap.sampler,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 6,
                        resource: wgpu::BindingResource::TextureView(&skybox.brdf_lut.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 7,
                        resource: wgpu::BindingResource::Sampler(&skybox.brdf_lut.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 8,
                        resource: wgpu::BindingResource::TextureView(
                            &skybox.environment_cubemap.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 9,
                        resource: wgpu::BindingResource::Sampler(
                            &skybox.environment_cubemap.sampler,
                        ),
                    },
                ],
            });
            textures_bindgroup
        }

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.textures_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(create_textures_bindgroup(
                &self.device,
                &self.get_pipeline(),
                // UNWRAP: if we can't acquire locks we want to panic
                &self.atlas.read().unwrap(),
                &self.skybox.read().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    /// Draw a GLTF rendering each frame.
    ///
    /// Returns the id of the stored `GltfRendering` and the id of the
    /// [`Rendering`].
    pub fn draw_gltf_rendering(
        &mut self,
        unit: &GltfRendering,
    ) -> (Id<GltfRendering>, Id<Rendering>) {
        let unit_id = self.append(unit);
        let rendering = Rendering::Gltf(unit_id);
        let id = self.append(&rendering);
        let draw = DrawUnit {
            id,
            vertex_count: unit.vertex_count,
            visible: true,
        };
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.push(draw);
            }
        }
        (unit_id, id)
    }

    /// Draw a signed distance field rendering each frame.
    ///
    /// Returns the id of the stored `Sdf` and the id of the [`Rendering`].
    pub fn draw_sdf_rendering(&mut self, sdf: &Sdf) -> (Id<Sdf>, Id<Rendering>) {
        let sdf_id = self.append(sdf);
        let rendering = Rendering::Sdf(sdf_id);
        let id = self.append(&rendering);
        let draw = DrawUnit {
            id,
            vertex_count: sdf.vertex_count(),
            visible: true,
        };
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.push(draw);
            }
        }
        (sdf_id, id)
    }

    /// Erase the [`RenderUnit`] with the given `Id` from the stage.
    pub fn erase_unit(&self, id: Id<Rendering>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.retain(|unit| unit.id != id);
            }
        }
    }

    /// Returns all the draw operations on the stage.
    pub fn get_draws(&self) -> Vec<DrawUnit> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.draws.read().unwrap();
        match draws.deref() {
            StageDrawStrategy::Direct(units) => units.clone(),
        }
    }

    /// Show the [`RenderUnit`] with the given `Id` for rendering.
    pub fn show_unit(&self, id: Id<Rendering>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                for unit in units.iter_mut() {
                    if unit.id == id {
                        unit.visible = true;
                    }
                }
            }
        }
    }

    /// Hide the [`RenderUnit`] with the given `Id` from rendering.
    pub fn hide_unit(&self, id: Id<Rendering>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                for unit in units.iter_mut() {
                    if unit.id == id {
                        unit.visible = false;
                    }
                }
            }
        }
    }

    /// Configure [`Renderling`] to render this stage.
    pub fn configure_graph(&self, r: &mut crate::Renderling, should_copy_frame_to_post: bool) {
        // set up the render graph
        use crate::{
            frame::{copy_frame_to_post, create_frame, present},
            graph::{graph, Graph},
            hdr::{clear_surface_hdr_and_depth, create_hdr_render_surface},
            tonemapping::tonemapping,
        };

        let (hdr_surface,) = r.graph.visit(create_hdr_render_surface).unwrap().unwrap();
        r.graph.add_resource(hdr_surface);
        r.graph.add_resource(self.clone());

        // pre-render
        r.graph
            .add_subgraph(graph!(create_frame, clear_surface_hdr_and_depth))
            .add_barrier();

        // render
        if should_copy_frame_to_post {
            r.graph.add_subgraph(graph!(
                stage_render
                    < tonemapping
                    < copy_frame_to_post
                    < present
            ));
        } else {
            r.graph.add_subgraph(graph!(
                stage_render
                    < tonemapping
                    < present
            ));
        }
    }

    /// Read the atlas image from the GPU.
    ///
    /// This is primarily used for debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_atlas_image(&self) -> image::RgbaImage {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.atlas
            .read()
            .unwrap()
            .atlas_img(&self.device, &self.queue)
    }

    /// Read the brdf image from the GPU.
    ///
    /// This is primarily used for debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_brdf_image(&self) -> image::Rgba32FImage {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let skybox = self.skybox.read().unwrap();
        let texture = &skybox.brdf_lut;
        let extent = texture.texture.size();
        let buffer = crate::Texture::read(
            &texture.texture,
            &self.device,
            &self.queue,
            extent.width as usize,
            extent.height as usize,
            2,
            2,
        );
        let pixels = buffer.pixels(&self.device);
        let pixels: Vec<f32> = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
            .iter()
            .copied()
            .map(|bits| half::f16::from_bits(bits).to_f32())
            .collect();
        let pixels: Vec<f32> = pixels
            .chunks_exact(2)
            .flat_map(|pixel| match pixel {
                [r, g] => [*r, *g, 0.0, 1.0],
                _ => unreachable!(),
            })
            .collect();
        let img: image::ImageBuffer<image::Rgba<f32>, Vec<f32>> =
            image::ImageBuffer::from_vec(extent.width, extent.height, pixels).unwrap();
        img
    }

    fn read_cubemap_mip0(&self, texture: &crate::Texture) -> CpuCubemap {
        let placeholder_image =
            image::Rgba32FImage::from_pixel(1, 1, image::Rgba([0.0, 0.0, 0.0, 0.0]));
        let mut out: [image::Rgba32FImage; 6] = [
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
        ];
        let extent = texture.texture.size();
        let width = extent.width;
        let height = extent.height;
        for i in 0..6 {
            let copied_buffer = crate::Texture::read_from(
                &texture.texture,
                &self.device,
                &self.queue,
                width as usize,
                height as usize,
                4,
                2,
                0,
                Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
            );
            let pixels = copied_buffer.pixels(&self.device);
            let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                .iter()
                .map(|p| half::f16::from_bits(*p).to_f32())
                .collect::<Vec<_>>();
            let img: image::Rgba32FImage =
                image::ImageBuffer::from_vec(width, height, pixels).unwrap();
            out[i as usize] = img;
        }
        CpuCubemap {
            images: out.map(|img| img.into()),
        }
    }

    /// Read the irradiance cubemap images from the GPU.
    ///
    /// This only reads the top level of mips. This is primarily used for
    /// debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_irradiance_cubemap(&self) -> CpuCubemap {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let skybox = self.skybox.read().unwrap();
        let texture = &skybox.irradiance_cubemap;
        self.read_cubemap_mip0(texture)
    }

    /// Read the prefiltered cubemap images from the GPU.
    ///
    /// This only reads the top level of mips. This is primarily used for
    /// debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_prefiltered_cubemap(&self) -> CpuCubemap {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let skybox = self.skybox.read().unwrap();
        let texture = &skybox.prefiltered_environment_cubemap;
        self.read_cubemap_mip0(texture)
    }

    /// Read all the data from the stage.
    ///
    /// This blocks until the GPU buffer is mappable, and then copies the data
    /// into a vector.
    ///
    /// This is primarily used for debugging.
    pub fn read_slab(&self) -> Result<Vec<u32>, WgpuSlabError> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab
            .read()
            .unwrap()
            .as_ref()
            .block_on_read_raw(0, self.len())
    }

    pub fn new_skybox_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
        camera: Id<Camera>,
    ) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_path(path)?;
        Ok(Skybox::new(
            self.device.clone(),
            self.queue.clone(),
            hdr,
            camera,
        ))
    }
}

/// A unit of work to be drawn.
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawUnit {
    pub id: Id<Rendering>,
    pub vertex_count: u32,
    pub visible: bool,
}

/// Provides a way to communicate with the stage about how you'd like your
/// objects drawn.
pub(crate) enum StageDrawStrategy {
    // TODO: Add `Indirect` to `StageDrawStrategy` which uses `RenderPass::multi_draw_indirect`
    Direct(Vec<DrawUnit>),
}

/// Render the stage.
pub fn stage_render(
    (stage, hdr_frame, depth): (ViewMut<Stage>, View<HdrSurface>, View<DepthTexture>),
) -> Result<(), WgpuSlabError> {
    let label = Some("stage render");
    let pipeline = stage.get_pipeline();
    let slab_buffers_bindgroup = stage.get_slab_buffers_bindgroup();
    let textures_bindgroup = stage.get_textures_bindgroup();
    let has_skybox = stage.has_skybox.load(std::sync::atomic::Ordering::Relaxed);
    let may_skybox_pipeline_and_bindgroup = if has_skybox {
        Some(stage.get_skybox_pipeline_and_bindgroup())
    } else {
        None
    };
    //let mut may_bloom_filter = if
    // stage.has_bloom.load(std::sync::atomic::Ordering::Relaxed) {    // UNWRAP:
    // if we can't acquire the lock we want to panic.    Some(stage.bloom.
    // write().unwrap())
    //} else {
    //    None
    //};
    // UNWRAP: if we can't read we want to panic.
    let draws = stage.draws.read().unwrap();

    let mut encoder = stage
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label,
            color_attachments: &hdr_frame.color_attachments(),
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        render_pass.set_pipeline(&pipeline);
        render_pass.set_bind_group(0, &slab_buffers_bindgroup, &[]);
        render_pass.set_bind_group(1, &textures_bindgroup, &[]);
        match draws.deref() {
            StageDrawStrategy::Direct(units) => {
                for unit in units {
                    if unit.visible {
                        render_pass
                            .draw(0..unit.vertex_count, unit.id.inner()..unit.id.inner() + 1);
                    }
                }
            } /* render_pass.multi_draw_indirect(&indirect_buffer, 0,
               * stage.number_of_indirect_draws()); */
        }

        if let Some((pipeline, bindgroup)) = may_skybox_pipeline_and_bindgroup.as_ref() {
            log::trace!("rendering skybox");
            // UNWRAP: if we can't acquire the lock we want to panic.
            let skybox = stage.skybox.read().unwrap();
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bindgroup, &[]);
            render_pass.draw(0..36, skybox.camera.inner()..skybox.camera.inner() + 1);
        }
    }
    stage.queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}
