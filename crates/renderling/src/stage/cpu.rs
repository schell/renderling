//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage [`Renderlet`]s for rendering.
use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use crate::{
    pbr::{debug::DebugMode, light::Light, PbrConfig},
    Atlas, AtlasError, AtlasImage, AtlasImageError, AtlasTexture, Camera, CpuCubemap, DepthTexture,
    Device, HdrSurface, Queue, Skybox, WgpuSlabError,
};
use crabslab::{Array, CpuSlab, GrowableSlab, Id, Slab, SlabItem, WgpuBuffer};
use moongraph::{graph, Graph, NoDefault, View, ViewMut};
use snafu::Snafu;

use super::*;

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

impl From<StageError> for moongraph::GraphError {
    fn from(value: StageError) -> Self {
        moongraph::GraphError::other(value)
    }
}

/// Provides a way to communicate with the stage about how you'd like your
/// objects drawn.
pub(crate) enum StageDrawStrategy {
    // TODO: Add `Indirect` to `StageDrawStrategy` which uses `RenderPass::multi_draw_indirect`
    Direct(Vec<(Id<Renderlet>, Renderlet)>),
}

/// A "hybrid" type that lives on the CPU and the GPU.
///
/// Updates are syncronized to the GPU once per frame.
pub struct Hybrid<T> {
    cpu_value: Arc<RwLock<T>>,
    id: Id<T>,
    dirty: Arc<AtomicBool>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Hybrid<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Hybrid<{}>", std::any::type_name::<T>()))
            .field("id", &self.id)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .field("dirty", &self.get_dirty())
            .finish()
    }
}

impl<T> Clone for Hybrid<T> {
    fn clone(&self) -> Self {
        Hybrid {
            cpu_value: self.cpu_value.clone(),
            id: self.id,
            dirty: self.dirty.clone(),
        }
    }
}

impl<T> Hybrid<T> {
    pub(crate) fn set_dirty(&self, dirty: bool) {
        self.dirty
            .store(dirty, std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn get_dirty(&self) -> bool {
        self.dirty.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> Hybrid<T> {
    pub fn new(stage: &mut Stage, value: T) -> Self {
        let id = stage.allocate::<T>();
        let hybrid = Self::new_preallocated(id, value);
        stage.add_updates(hybrid.clone());

        hybrid
    }

    pub(crate) fn new_preallocated(id: Id<T>, value: T) -> Self {
        Self {
            cpu_value: Arc::new(RwLock::new(value)),
            id,
            dirty: Arc::new(true.into()),
        }
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn get(&self) -> T {
        self.cpu_value.read().unwrap().clone()
    }

    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut value_guard = self.cpu_value.write().unwrap();
        self.set_dirty(true);
        f(&mut value_guard);
    }

    pub fn set(&self, value: T) {
        self.modify(move |old| {
            *old = value;
        })
    }
}

/// A "hybrid" array type that lives on the CPU and the GPU.
///
/// Once created, the array cannot be resized.
///
/// Updates are syncronized to the GPU once per frame.
pub struct HybridArray<T> {
    cpu_value: Arc<RwLock<Vec<T>>>,
    array: Array<T>,
    updates: Arc<Mutex<Vec<SlabUpdate>>>,
}

impl<T: core::fmt::Debug> core::fmt::Debug for HybridArray<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("HybridArray<{}>", std::any::type_name::<T>()))
            .field("array", &self.array)
            .field("cpu_value", &self.cpu_value.read().unwrap())
            .finish()
    }
}

impl<T> Clone for HybridArray<T> {
    fn clone(&self) -> Self {
        HybridArray {
            cpu_value: self.cpu_value.clone(),
            array: self.array,
            updates: self.updates.clone(),
        }
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> HybridArray<T> {
    pub fn new(stage: &mut Stage, values: impl IntoIterator<Item = T>) -> Self {
        let value = values.into_iter().collect::<Vec<_>>();
        let array = stage.allocate_array::<T>(value.len());
        let hybrid = Self::new_preallocated(array, value);
        stage
            .slab_updates
            .write()
            .unwrap()
            .push(Box::new(hybrid.clone()));
        hybrid
    }

    fn new_preallocated(array: Array<T>, values: Vec<T>) -> Self {
        Self {
            array,
            updates: {
                let mut elements = vec![0u32; T::SLAB_SIZE * array.len()];
                elements.write_array(Array::new(0, array.len() as u32), &values);
                Arc::new(Mutex::new(vec![SlabUpdate {
                    array: array.into_u32_array(),
                    elements,
                    #[cfg(debug_assertions)]
                    type_is: std::any::type_name::<Vec<T>>(),
                }]))
            },
            cpu_value: Arc::new(RwLock::new(values)),
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn array(&self) -> Array<T> {
        self.array
    }

    pub fn at(&self, index: usize) -> Option<T> {
        self.cpu_value.read().unwrap().get(index).cloned()
    }

    pub fn modify<S>(&self, index: usize, f: impl FnOnce(&mut T) -> S) -> Option<S> {
        let mut value_guard = self.cpu_value.write().unwrap();
        let t = value_guard.get_mut(index)?;
        let output = Some(f(t));
        let t = t.clone();
        let id = self.array.at(index);
        let array = Array::<u32>::new(id.inner(), T::SLAB_SIZE as u32);
        let mut elements = vec![0u32; T::SLAB_SIZE];
        elements.write(id, &t);
        self.updates.lock().unwrap().push(SlabUpdate {
            array,
            elements,
            #[cfg(debug_assertions)]
            type_is: std::any::type_name::<T>(),
        });
        output
    }

    pub fn set_item(&self, index: usize, value: T) -> Option<T> {
        self.modify(index, move |t| std::mem::replace(t, value))
    }
}

pub struct SlabUpdate {
    array: Array<u32>,
    elements: Vec<u32>,
    #[cfg(debug_assertions)]
    type_is: &'static str,
}

pub trait UpdatesSlab: Send + Sync + 'static {
    /// Returns all current updates, clearing the queue.
    fn get_updates(&self) -> Vec<SlabUpdate>;

    /// Returns the slab range of all possible updates.
    fn array(&self) -> Array<u32>;

    /// Returns the number of references remaiting in the wild.
    fn strong_count(&self) -> usize;
}

impl<T: SlabItem + Clone + Send + Sync + 'static> UpdatesSlab for Hybrid<T> {
    fn get_updates(&self) -> Vec<SlabUpdate> {
        if self.dirty.swap(false, std::sync::atomic::Ordering::Relaxed) {
            let t = self.get();
            let array = Array::<u32>::new(self.id.inner(), T::SLAB_SIZE as u32);
            let mut elements = vec![0u32; T::SLAB_SIZE];
            elements.write(0u32.into(), &t);
            vec![SlabUpdate {
                array,
                elements,
                #[cfg(debug_assertions)]
                type_is: std::any::type_name::<T>(),
            }]
        } else {
            vec![]
        }
    }

    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.cpu_value)
    }

    fn array(&self) -> Array<u32> {
        Array::new(self.id.inner(), T::SLAB_SIZE as u32)
    }
}

impl<T: SlabItem + Clone + Send + Sync + 'static> UpdatesSlab for HybridArray<T> {
    fn get_updates(&self) -> Vec<SlabUpdate> {
        let mut guard = self.updates.lock().unwrap();
        let updates = std::mem::take(guard.as_mut());
        updates
    }

    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.cpu_value)
    }

    fn array(&self) -> Array<u32> {
        self.array.into_u32_array()
    }
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
///
/// ## Note
/// Only available on the CPU. Not available in shaders.
#[derive(Clone)]
pub struct Stage {
    pub(crate) slab: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) vertex_debug: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) fragment_debug: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) atlas: Arc<RwLock<Atlas>>,
    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) pipeline: Arc<Mutex<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) has_skybox: Arc<AtomicBool>,
    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) buffers_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) lights: HybridArray<Id<Light>>,
    pub(crate) draws: Arc<RwLock<StageDrawStrategy>>,
    pub(crate) slab_updates: Arc<RwLock<Vec<Box<dyn UpdatesSlab>>>>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Slab for Stage {
    fn len(&self) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().len()
    }

    fn read_unchecked<T: SlabItem>(&self, id: Id<T>) -> T {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().read_unchecked(id)
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
    pub fn new(device: Device, queue: Queue, resolution: UVec2) -> Self {
        let atlas = Atlas::empty(&device, &queue);
        let mut slab = CpuSlab::new(WgpuBuffer::new(device.0.clone(), queue.0.clone(), 256));
        let _ = slab.append(&PbrConfig {
            atlas_size: atlas.size,
            resolution,
            ..Default::default()
        });
        // TODO: make number of lights on the stage configurable
        let lights_values = vec![Id::<Light>::NONE; 16];
        let lights =
            HybridArray::new_preallocated(slab.append_array(&lights_values), lights_values);
        Self {
            slab: Arc::new(RwLock::new(slab)),
            vertex_debug: Arc::new(RwLock::new(CpuSlab::new(WgpuBuffer::new(
                device.0.clone(),
                queue.0.clone(),
                256,
            )))),
            fragment_debug: Arc::new(RwLock::new({
                let len = (resolution.x * resolution.y) as usize;
                let mut debug = CpuSlab::new(WgpuBuffer::new(
                    device.0.clone(),
                    queue.0.clone(),
                    RenderletFragmentLog::SLAB_SIZE * len,
                ));
                debug.append_array(&vec![RenderletFragmentLog::default(); len]);
                debug
            })),
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
            lights,
            slab_updates: Default::default(),
            device,
            queue,
        }
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

    pub fn set_resolution(&mut self, resolution: UVec2) {
        let id = Id::<UVec2>::from(PbrConfig::offset_of_resolution());
        self.write(id, &resolution);

        // Allocate space for our fragment shader logs...
        let len = resolution.x * resolution.y;
        // UNWRAP: if we can't write we want to panic.
        let mut debug = self.fragment_debug.write().unwrap();
        let array = Array::<RenderletFragmentLog>::new(0, len);
        debug.maybe_expand_to_fit::<RenderletFragmentLog>(len as usize);
        debug.write_array(
            array,
            vec![RenderletFragmentLog::default(); len as usize].as_slice(),
        );
    }

    pub fn with_resolution(mut self, resolution: UVec2) -> Self {
        self.set_resolution(resolution);
        self
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
    ) -> Result<Vec<AtlasTexture>, StageError> {
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
            .map(|(i, (offset_px, size_px))| AtlasTexture {
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
            let vertex_linkage = crate::linkage::renderlet_vertex::linkage(device);
            let fragment_linkage = crate::linkage::renderlet_fragment::linkage(device);
            let stage_slab_buffers_layout = crate::linkage::slab_bindgroup_layout(device);
            let atlas_and_skybox_layout = crate::linkage::atlas_and_skybox_bindgroup_layout(device);
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&stage_slab_buffers_layout, &atlas_and_skybox_layout],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_linkage.module,
                    entry_point: vertex_linkage.entry_point,
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
                    module: &fragment_linkage.module,
                    entry_point: fragment_linkage.entry_point,
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
                crate::linkage::slab_bindgroup(
                    device,
                    self.slab.read().unwrap().as_ref().get_buffer(),
                    self.vertex_debug.read().unwrap().as_ref().get_buffer(),
                    self.fragment_debug.read().unwrap().as_ref().get_buffer(),
                    &pipeline.get_bind_group_layout(0),
                )
            });
            *bindgroup = Some(b.clone());
            b
        }
    }

    pub fn get_textures_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.textures_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(crate::linkage::atlas_and_skybox_bindgroup(
                &self.device,
                &self.get_pipeline().get_bind_group_layout(1),
                // UNWRAP: if we can't acquire locks we want to panic
                &self.atlas.read().unwrap(),
                &self.skybox.read().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    /// Draw a [`Renderlet`] each frame.
    ///
    /// Returns the id of the stored `Renderlet`.
    pub fn draw(&mut self, renderlet: &crate::Renderlet) -> Id<crate::Renderlet> {
        let mut renderlet = *renderlet;
        self.draw_debug(&mut renderlet)
    }

    /// Draw a [`Renderlet`] each frame.
    ///
    /// Returns the id of the stored `Renderlet`.
    ///
    /// Updates the input `renderlet` with a `debug_index`, which can be used to retrieve
    /// debugging information after running a shader.
    pub fn draw_debug(&mut self, renderlet: &mut crate::Renderlet) -> Id<crate::Renderlet> {
        let id = self.append(renderlet);
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                renderlet.debug_index = units.len() as u32;
                units.push((id, *renderlet));
            }
        }

        // Ensure we have space to write GPU debugging info to our vertex debug buffer
        {
            let vertex_debug_data = vec![RenderletVertexLog::default(); renderlet.vertices.len()];
            // UNWRAP: if we can't read we want to panic.
            let mut debug = self.vertex_debug.write().unwrap();
            debug.append_array(&vertex_debug_data);
        }
        id
    }

    /// Erase the [`Renderlet`] with the given [`Id`] from the stage.
    ///
    /// This removes the [`Renderlet`] from the stage.
    pub fn erase(&self, id: Id<Renderlet>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.retain(|(renderlet_id, _)| renderlet_id != &id);
            }
        }
    }

    /// Returns all the draw operations ([`Renderlet`]s) on the stage, paired with their [`Id`]s.
    pub fn get_draws(&self) -> Vec<(Id<Renderlet>, Renderlet)> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.draws.read().unwrap();
        match draws.deref() {
            StageDrawStrategy::Direct(units) => units.clone(),
        }
    }

    /// Show the [`Renderlet`] with the given `Id` for rendering.
    pub fn show(&self, id: Id<Renderlet>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                for (r_id, r) in units.iter_mut() {
                    if r_id == &id {
                        r.visible = true;
                    }
                }
            }
        }
    }

    /// Hide the [`Renderlet`] with the given `Id` from rendering.
    pub fn hide(&self, id: Id<Renderlet>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                for (r_id, r) in units.iter_mut() {
                    if r_id == &id {
                        r.visible = false;
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
        self.slab.read().unwrap().as_ref().block_on_read_raw(..)
    }

    /// Read the vertex debug messages.
    pub fn read_vertex_debug_logs(&self) -> Vec<RenderletVertexLog> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.get_draws();
        let len = draws
            .into_iter()
            .map(|(_, r)| r.vertices.len())
            .sum::<usize>();
        let array = Array::<RenderletVertexLog>::new(0, len as u32);
        let logs = self.vertex_debug.read().unwrap().as_ref().read_vec(array);
        logs
    }

    /// Clear the vertex debug messages.
    pub fn clear_vertex_debug_logs(&self) {
        let len = {
            // UNWRAP: if we can't read we want to panic.
            let draws = self.draws.read().unwrap();
            match draws.deref() {
                StageDrawStrategy::Direct(units) => {
                    let len = units
                        .iter()
                        .map(|(_, renderlet)| renderlet.vertices.len())
                        .sum::<usize>();
                    len
                }
            }
        };
        // UNWRAP: if we can't read we want to panic.
        let mut debug = self.vertex_debug.write().unwrap();
        let array = Array::<RenderletVertexLog>::new(0, len as u32);
        debug.write_array(array, vec![RenderletVertexLog::default(); len].as_slice());
    }

    /// Read the fragment debug messages.
    pub fn read_fragment_debug_logs(&self) -> Vec<RenderletFragmentLog> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let len = {
            let slab = self.slab.read().unwrap();
            let cfg_id = Id::<PbrConfig>::new(0);
            let resolution = slab.read(cfg_id + PbrConfig::offset_of_resolution());
            resolution.x * resolution.y
        };
        // UNWRAP: if we can't read we want to panic.
        let array = Array::<RenderletFragmentLog>::new(0, len);
        log::trace!("reading {len} logs from fragment_debug buffer");
        self.fragment_debug.read().unwrap().read_vec(array)
    }

    /// Clear the fragment debug messages.
    pub fn clear_fragment_debug_logs(&self) {
        let len = {
            let slab = self.slab.read().unwrap();
            let cfg_id = Id::<PbrConfig>::new(0);
            let resolution = slab.read(cfg_id + PbrConfig::offset_of_resolution());
            resolution.x * resolution.y
        };
        // UNWRAP: if we can't write we want to panic.
        let mut debug = self.fragment_debug.write().unwrap();
        let array = Array::<RenderletFragmentLog>::new(0, len);
        debug.write_array(
            array,
            vec![RenderletFragmentLog::default(); len as usize].as_slice(),
        );
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

    pub fn new_nested_transform(&mut self) -> NestedTransform {
        NestedTransform::new(self)
    }

    pub(crate) fn add_updates(&mut self, updates: impl UpdatesSlab) {
        self.slab_updates.write().unwrap().push(Box::new(updates));
    }

    pub fn new_hybrid<T: SlabItem + Clone + Send + Sync + 'static>(
        &mut self,
        value: T,
    ) -> Hybrid<T> {
        Hybrid::new(self, value)
    }

    pub fn new_hybrid_array<T: SlabItem + Clone + Send + Sync + 'static>(
        &mut self,
        values: impl IntoIterator<Item = T>,
    ) -> HybridArray<T> {
        HybridArray::new(self, values)
    }
    fn write_updates(&mut self) {
        let writes = {
            let mut writes = vec![];
            let mut guard = self.slab_updates.write().unwrap();
            guard.retain(|hybrid| {
                writes.extend(hybrid.get_updates());
                let count = hybrid.strong_count();
                count > 1
            });
            writes
        };
        for (
            i,
            SlabUpdate {
                array,
                elements,
                type_is,
            },
        ) in writes
            .into_iter()
            .filter(|u| !u.array.is_empty() && !u.array.is_null())
            .enumerate()
        {
            log::trace!("writing update {i} {type_is} {array:?}");
            self.write_array(array, &elements);
        }
    }
}

/// Render the stage.
pub fn stage_render(
    (mut stage, hdr_frame, depth): (
        ViewMut<Stage, NoDefault>,
        View<HdrSurface, NoDefault>,
        View<DepthTexture, NoDefault>,
    ),
) -> Result<(), StageError> {
    log::trace!("rendering the stage");
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
    stage.write_updates();
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
                for (id, rlet) in units {
                    if rlet.visible {
                        let vertex_range = if rlet.indices.is_null() {
                            0..rlet.vertices.len() as u32
                        } else {
                            0..rlet.indices.len() as u32
                        };
                        let instance_range = id.inner()..id.inner() + 1;
                        log::trace!(
                            "drawing vertices {vertex_range:?} and instances {instance_range:?}"
                        );
                        render_pass.draw(vertex_range, instance_range);
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

/// Manages scene heirarchy on the [`Stage`].
///
/// Clones all reference the same nested transform.
#[derive(Clone)]
pub struct NestedTransform {
    global_transform: Hybrid<Transform>,
    local_transform: Arc<RwLock<Transform>>,
    children: Arc<RwLock<Vec<NestedTransform>>>,
    parent: Arc<RwLock<Option<NestedTransform>>>,
}

impl core::fmt::Debug for NestedTransform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let local_transform = self.get_local_transform();
        let global_transform = self.get_global_transform();
        let children = self
            .children
            .read()
            .unwrap()
            .iter()
            .map(|nt| nt.global_transform.id)
            .collect::<Vec<_>>();
        let parent = self
            .parent
            .read()
            .unwrap()
            .as_ref()
            .map(|nt| nt.global_transform.id);
        f.debug_struct("NestedTransform")
            .field("global_transform", &self.global_transform)
            .field("local_transform", &local_transform)
            .field("global_transform", &global_transform)
            .field("children", &children)
            .field("parent", &parent)
            .finish()
    }
}

impl NestedTransform {
    pub fn new(stage: &mut Stage) -> Self {
        let global_transform = Hybrid::new(stage, Transform::default());
        let nested = NestedTransform {
            local_transform: Arc::new(RwLock::new(Transform::default())),
            global_transform,
            children: Default::default(),
            parent: Default::default(),
        };
        nested
    }

    pub fn set_local_transform(&mut self, transform: Transform) {
        *self.local_transform.write().unwrap() = transform;
        let global_transform = self.get_global_transform();
        self.global_transform.set(global_transform);
    }

    pub fn get_local_transform(&self) -> Transform {
        *self.local_transform.read().unwrap()
    }

    pub fn get_global_transform(&self) -> Transform {
        let maybe_parent_guard = self.parent.read().unwrap();
        let transform = self.get_local_transform();
        let parent_transform = maybe_parent_guard
            .as_ref()
            .map(|parent| parent.get_global_transform())
            .unwrap_or_default();
        Transform::from(Mat4::from(parent_transform) * Mat4::from(transform))
    }

    pub fn global_transform_id(&self) -> Id<Transform> {
        self.global_transform.id
    }

    pub fn add_child(&mut self, node: &NestedTransform) {
        *node.parent.write().unwrap() = Some(self.clone());
        let global_transform = node.get_global_transform();
        node.global_transform.set(global_transform);
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&mut self, node: &NestedTransform) {
        self.children.write().unwrap().retain_mut(|child| {
            if child.global_transform.id == node.global_transform.id {
                let local_transform = node.get_local_transform();
                node.global_transform.set(local_transform);
                let _ = node.parent.write().unwrap().take();
                false
            } else {
                true
            }
        });
    }
}

#[cfg(test)]
mod test {
    use crabslab::{Array, GrowableSlab, Slab};
    use glam::{Vec2, Vec3};

    use crate::{Camera, Renderlet, RenderletVertexLog, Renderling, Vertex};

    #[test]
    fn vertex_slab_roundtrip() {
        let initial_vertices = {
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(1.0, 0.0, 0.0))
                .with_uv0(Vec2::new(1.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, 1.0, 0.0))
                .with_uv0(Vec2::new(0.0, 1.0));
            let br = Vertex::default()
                .with_position(Vec3::new(1.0, 1.0, 0.0))
                .with_uv0(Vec2::splat(1.0));
            vec![tl, bl, br, tl, br, tr]
        };
        let mut slab = [0u32; 256];
        slab.write_indexed_slice(&initial_vertices, 0);
        let vertices = slab.read_vec(Array::<Vertex>::new(0, initial_vertices.len() as u32));
        pretty_assertions::assert_eq!(initial_vertices, vertices);
    }

    #[test]
    fn can_read_shader_debug_logs() {
        let mut r = Renderling::headless(10, 10);
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = crate::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera::new(projection, view));
        let geometry = stage.append_array(&crate::test::right_tri_vertices());
        let mut renderlet = Renderlet {
            camera,
            vertices: geometry,
            ..Default::default()
        };
        let tri_id = stage.draw_debug(&mut renderlet);
        let _ = r.render_linear_image().unwrap();

        // read vertex logs
        {
            let vertex_logs = stage.read_vertex_debug_logs();
            for (i, log) in vertex_logs.iter().enumerate() {
                println!("log_{i}:{log:#?}");
                assert_eq!(tri_id, log.renderlet_id);
                assert!(log.started);
                assert!(log.completed);
            }

            stage.clear_vertex_debug_logs();
            let blank_logs = stage.read_vertex_debug_logs();
            assert_ne!(vertex_logs, blank_logs);
            for log in blank_logs.iter() {
                assert_eq!(&RenderletVertexLog::default(), log);
            }
        }

        // read fragment logs
        {
            let fragment_logs = stage.read_fragment_debug_logs();
            let (w, h) = r.get_screen_size();
            assert_eq!((w * h) as usize, fragment_logs.len());
            for (i, log) in fragment_logs.iter().enumerate() {
                // only those fragments covered by the vertex shader's output are
                // evaluated by the fragment shader, so only check those ones...
                // @see DEVLOG.md's "Tue Apr 9 - Better debugging"
                if log.started {
                    assert!(log.completed);
                    println!("log_{i}:{log:#?}");
                }
            }
            stage.clear_fragment_debug_logs();
        }
    }
}
