//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage [`Renderlet`]s for rendering.
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use craballoc::prelude::*;
use crabslab::Id;
use snafu::Snafu;
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock};

use crate::{
    atlas::{AtlasError, AtlasImage, AtlasImageError, AtlasTexture},
    bindgroup::ManagedBindGroup,
    bloom::Bloom,
    camera::Camera,
    debug::DebugOverlay,
    draw::DrawCalls,
    geometry::Geometry,
    light::{AnalyticalLightBundle, Light, LightDetails, Lighting, LightingBindGroupLayoutEntries},
    material::Materials,
    pbr::debug::DebugChannel,
    skybox::{Skybox, SkyboxRenderPipeline},
    stage::Renderlet,
    texture::{DepthTexture, Texture},
    tonemapping::Tonemapping,
    transform::Transform,
    tuple::Bundle,
};

use super::*;

#[derive(Debug, Snafu)]
pub enum StageError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },
}

impl From<AtlasError> for StageError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
    }
}

fn create_msaa_textureview(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("stage msaa render target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

pub struct StageCommitResult {
    pub geometry_buffer: SlabBuffer<wgpu::Buffer>,
    pub lighting_buffer: SlabBuffer<wgpu::Buffer>,
    pub materials_buffer: SlabBuffer<wgpu::Buffer>,
}

impl StageCommitResult {
    pub fn latest_creation_time(&self) -> usize {
        [
            &self.geometry_buffer,
            &self.materials_buffer,
            &self.lighting_buffer,
        ]
        .iter()
        .map(|buffer| buffer.creation_time())
        .max()
        .unwrap_or_default()
    }
}

struct RenderletBindGroup<'a> {
    device: &'a wgpu::Device,
    layout: &'a wgpu::BindGroupLayout,
    geometry_buffer: &'a wgpu::Buffer,
    material_buffer: &'a wgpu::Buffer,
    light_buffer: &'a wgpu::Buffer,
    atlas_texture_view: &'a wgpu::TextureView,
    atlas_texture_sampler: &'a wgpu::Sampler,
    irradiance_texture_view: &'a wgpu::TextureView,
    irradiance_texture_sampler: &'a wgpu::Sampler,
    prefiltered_texture_view: &'a wgpu::TextureView,
    prefiltered_texture_sampler: &'a wgpu::Sampler,
    brdf_texture_view: &'a wgpu::TextureView,
    brdf_texture_sampler: &'a wgpu::Sampler,
    shadow_map_texture_view: &'a wgpu::TextureView,
    shadow_map_texture_sampler: &'a wgpu::Sampler,
}

impl RenderletBindGroup<'_> {
    pub fn create(self) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("renderlet"),
            layout: self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.geometry_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.material_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(self.atlas_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(self.atlas_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(self.irradiance_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(self.irradiance_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(self.prefiltered_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::Sampler(self.prefiltered_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::TextureView(self.brdf_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 9,
                    resource: wgpu::BindingResource::Sampler(self.brdf_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 10,
                    resource: self.light_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 11,
                    resource: wgpu::BindingResource::TextureView(self.shadow_map_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 12,
                    resource: wgpu::BindingResource::Sampler(self.shadow_map_texture_sampler),
                },
            ],
        })
    }
}

/// Performs a rendering of an entire scene, given the resources at hand.
pub struct StageRendering<'a> {
    // TODO: include the rest of the needed paramaters from `stage`, and then remove `stage`
    pub stage: &'a Stage,
    pub pipeline: &'a wgpu::RenderPipeline,
    pub color_attachment: wgpu::RenderPassColorAttachment<'a>,
    pub depth_stencil_attachment: wgpu::RenderPassDepthStencilAttachment<'a>,
}

impl StageRendering<'_> {
    /// Run the stage rendering.
    ///
    /// Returns the queue submission index and the indirect draw buffer, if available.
    pub fn run(self) -> (wgpu::SubmissionIndex, Option<SlabBuffer<wgpu::Buffer>>) {
        let commit_result = self.stage.commit();
        let current_renderlet_bind_group_creation_time = commit_result.latest_creation_time();
        let previous_renderlet_bind_group_creation_time =
            self.stage.renderlet_bind_group_created.swap(
                current_renderlet_bind_group_creation_time,
                std::sync::atomic::Ordering::Relaxed,
            );
        let should_invalidate_renderlet_bind_group = current_renderlet_bind_group_creation_time
            > previous_renderlet_bind_group_creation_time;
        let renderlet_bind_group =
            self.stage
                .renderlet_bind_group
                .get(should_invalidate_renderlet_bind_group, || {
                    let atlas_texture = self.stage.materials.atlas().get_texture();
                    let skybox = self.stage.skybox.read().unwrap();
                    let shadow_map = self.stage.lighting.shadow_map_atlas.get_texture();
                    RenderletBindGroup {
                        device: self.stage.device(),
                        layout: &Stage::renderlet_pipeline_bindgroup_layout(self.stage.device()),
                        geometry_buffer: &commit_result.geometry_buffer,
                        material_buffer: &commit_result.materials_buffer,
                        light_buffer: &commit_result.lighting_buffer,
                        atlas_texture_view: &atlas_texture.view,
                        atlas_texture_sampler: &atlas_texture.sampler,
                        irradiance_texture_view: &skybox.irradiance_cubemap.view,
                        irradiance_texture_sampler: &skybox.irradiance_cubemap.sampler,
                        prefiltered_texture_view: &skybox.prefiltered_environment_cubemap.view,
                        prefiltered_texture_sampler: &skybox
                            .prefiltered_environment_cubemap
                            .sampler,
                        brdf_texture_view: &skybox.brdf_lut.view,
                        brdf_texture_sampler: &skybox.brdf_lut.sampler,
                        shadow_map_texture_view: &shadow_map.view,
                        shadow_map_texture_sampler: &shadow_map.sampler,
                    }
                    .create()
                });

        self.stage.draw_calls.write().unwrap().upkeep();
        let mut draw_calls = self.stage.draw_calls.write().unwrap();
        let depth_texture = self.stage.depth_texture.read().unwrap();
        // UNWRAP: safe because we know the depth texture format will always match
        let maybe_indirect_buffer = draw_calls.pre_draw(&depth_texture).unwrap();
        {
            log::trace!("rendering");
            let label = Some("stage render");

            let mut encoder = self
                .stage
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(self.color_attachment)],
                    depth_stencil_attachment: Some(self.depth_stencil_attachment),
                    ..Default::default()
                });

                render_pass.set_pipeline(self.pipeline);
                render_pass.set_bind_group(0, Some(renderlet_bind_group.as_ref()), &[]);
                draw_calls.draw(&mut render_pass);

                let has_skybox = self.stage.has_skybox.load(Ordering::Relaxed);
                if has_skybox {
                    let (pipeline, bindgroup) = self
                        .stage
                        .get_skybox_pipeline_and_bindgroup(&commit_result.geometry_buffer);
                    render_pass.set_pipeline(&pipeline.pipeline);
                    render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
                    let camera_id = self.stage.geometry.descriptor().get().camera_id.inner();
                    render_pass.draw(0..36, camera_id..camera_id + 1);
                }
            }
            let sindex = self.stage.queue().submit(std::iter::once(encoder.finish()));
            (sindex, maybe_indirect_buffer)
        }
    }
}

/// A helper struct to build [`Renderlet`]s in the [`Geometry`] manager.
pub struct RenderletBuilder<'a, T> {
    data: Renderlet,
    resources: T,
    stage: &'a Stage,
}

impl<'a> RenderletBuilder<'a, ()> {
    pub fn new(stage: &'a Stage) -> Self {
        RenderletBuilder {
            data: Renderlet::default(),
            resources: (),
            stage,
        }
    }
}

impl<'a, T: Bundle> RenderletBuilder<'a, T> {
    fn suffix<S>(self, element: S) -> RenderletBuilder<'a, T::Suffixed<S>> {
        RenderletBuilder {
            data: self.data,
            resources: self.resources.suffix(element),
            stage: self.stage,
        }
    }

    pub fn with_vertices_array(mut self, array: Array<Vertex>) -> Self {
        self.data.vertices_array = array;
        self
    }

    pub fn with_vertices(
        mut self,
        vertices: impl IntoIterator<Item = Vertex>,
    ) -> RenderletBuilder<'a, T::Suffixed<HybridArray<Vertex>>> {
        let vertices = self.stage.geometry().new_vertices(vertices);
        self.data.vertices_array = vertices.array();
        self.suffix(vertices)
    }

    pub fn with_indices(
        mut self,
        indices: impl IntoIterator<Item = u32>,
    ) -> RenderletBuilder<'a, T::Suffixed<HybridArray<u32>>> {
        let indices = self.stage.geometry().new_indices(indices);
        self.data.indices_array = indices.array();
        self.suffix(indices)
    }

    pub fn with_transform_id(mut self, transform_id: Id<Transform>) -> Self {
        self.data.transform_id = transform_id;
        self
    }

    pub fn with_transform(
        mut self,
        transform: Transform,
    ) -> RenderletBuilder<'a, T::Suffixed<Hybrid<Transform>>> {
        let transform = self.stage.geometry().new_transform(transform);
        self.data.transform_id = transform.id();
        self.suffix(transform)
    }

    pub fn with_nested_transform(mut self, transform: &NestedTransform) -> Self {
        self.data.transform_id = transform.global_transform_id();
        self
    }

    pub fn with_material_id(mut self, material_id: Id<Material>) -> Self {
        self.data.material_id = material_id;
        self
    }

    pub fn with_material(
        mut self,
        material: Material,
    ) -> RenderletBuilder<'a, T::Suffixed<Hybrid<Material>>> {
        let material = self.stage.materials.new_material(material);
        self.data.material_id = material.id();
        self.suffix(material)
    }

    pub fn with_skin_id(mut self, skin_id: Id<Skin>) -> Self {
        self.data.skin_id = skin_id;
        self
    }

    pub fn with_morph_targets(
        mut self,
        morph_targets: impl IntoIterator<Item = Array<MorphTarget>>,
    ) -> (Self, HybridArray<Array<MorphTarget>>) {
        let morph_targets = self.stage.geometry().new_morph_targets_array(morph_targets);
        self.data.morph_targets = morph_targets.array();
        (self, morph_targets)
    }

    pub fn with_morph_weights(
        mut self,
        morph_weights: impl IntoIterator<Item = f32>,
    ) -> (Self, HybridArray<f32>) {
        let morph_weights = self.stage.geometry().new_weights(morph_weights);
        self.data.morph_weights = morph_weights.array();
        (self, morph_weights)
    }

    pub fn with_geometry_descriptor_id(mut self, pbr_config_id: Id<GeometryDescriptor>) -> Self {
        self.data.geometry_descriptor_id = pbr_config_id;
        self
    }

    pub fn with_bounds(mut self, bounds: impl Into<BoundingSphere>) -> Self {
        self.data.bounds = bounds.into();
        self
    }

    /// Build the [`Renderlet`], add it to the [`Stage`] with [`Stage::add_renderlet`] and
    /// return the [`Hybrid`] along with any resources that were staged.
    ///
    /// The returned value will be a tuple with the [`Hybrid<Renderlet>`] as the head, and
    /// all other resources added as the tail.
    pub fn build(self) -> <T::Suffixed<Hybrid<Renderlet>> as Bundle>::Reduced
    where
        T::Suffixed<Hybrid<Renderlet>>: Bundle,
    {
        let renderlet = self.stage.geometry().new_renderlet(self.data);
        self.stage.add_renderlet(&renderlet);
        self.resources.suffix(renderlet).reduce()
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
    pub(crate) geometry: Geometry,
    pub(crate) materials: Materials,
    pub(crate) lighting: Lighting,

    pub(crate) renderlet_pipeline: Arc<RwLock<wgpu::RenderPipeline>>,
    pub(crate) renderlet_bind_group: ManagedBindGroup,
    pub(crate) renderlet_bind_group_created: Arc<AtomicUsize>,

    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<SkyboxRenderPipeline>>>>,

    pub(crate) hdr_texture: Arc<RwLock<Texture>>,
    pub(crate) depth_texture: Arc<RwLock<Texture>>,
    pub(crate) msaa_render_target: Arc<RwLock<Option<wgpu::TextureView>>>,
    pub(crate) msaa_sample_count: Arc<AtomicU32>,
    pub(crate) clear_color_attachments: Arc<AtomicBool>,
    pub(crate) clear_depth_attachments: Arc<AtomicBool>,

    pub(crate) bloom: Bloom,

    pub(crate) tonemapping: Tonemapping,
    pub(crate) debug_overlay: DebugOverlay,
    pub(crate) background_color: Arc<RwLock<wgpu::Color>>,

    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) has_skybox: Arc<AtomicBool>,

    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) has_debug_overlay: Arc<AtomicBool>,

    pub(crate) stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,

    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,

    pub(crate) draw_calls: Arc<RwLock<DrawCalls>>,
}

impl AsRef<WgpuRuntime> for Stage {
    fn as_ref(&self) -> &WgpuRuntime {
        self.geometry.as_ref()
    }
}

impl Stage {
    pub fn device(&self) -> &wgpu::Device {
        &self.as_ref().device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.as_ref().queue
    }

    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    /// Access the geometry manager.
    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }

    /// Access the materials manager.
    pub fn materials(&self) -> &Materials {
        &self.materials
    }

    /// Access the lighting manager.
    pub fn lighting(&self) -> &Lighting {
        &self.lighting
    }

    pub fn builder(&self) -> RenderletBuilder<'_, ()> {
        RenderletBuilder::new(self)
    }

    /// Stage a [`Camera`] on the GPU.
    ///
    /// If the camera has not been set, this camera will be used
    /// automatically.
    pub fn new_camera(&self, camera: Camera) -> Hybrid<Camera> {
        self.geometry.new_camera(camera)
    }

    /// Set the given camera as the default one to use when rendering.
    pub fn use_camera(&self, camera: impl AsRef<Hybrid<Camera>>) {
        self.geometry().use_camera(camera);
    }

    /// Return the `Id` of the camera currently in use.
    pub fn used_camera_id(&self) -> Id<Camera> {
        self.geometry().descriptor().get().camera_id
    }

    /// Set the default camera `Id`.
    pub fn use_camera_id(&self, camera_id: Id<Camera>) {
        self.geometry()
            .descriptor()
            .modify(|desc| desc.camera_id = camera_id);
    }

    /// Stage a [`Transform`] on the GPU.
    pub fn new_transform(&self, transform: Transform) -> Hybrid<Transform> {
        self.geometry.new_transform(transform)
    }

    /// Stage a new [`Material`] on the GPU.
    pub fn new_material(&self, material: Material) -> Hybrid<Material> {
        self.materials.new_material(material)
    }

    /// Stage some vertex geometry data.
    pub fn new_vertices(&self, data: impl IntoIterator<Item = Vertex>) -> HybridArray<Vertex> {
        self.geometry.new_vertices(data)
    }

    /// Stage some vertex index data.
    pub fn new_indices(&self, data: impl IntoIterator<Item = u32>) -> HybridArray<u32> {
        self.geometry.new_indices(data)
    }

    /// Stage a new [`Renderlet`].
    ///
    /// The `Renderlet` should still be added to the draw list with
    /// [`Stage::add_renderlet`].
    pub fn new_renderlet(&self, renderlet: Renderlet) -> Hybrid<Renderlet> {
        self.geometry.new_renderlet(renderlet)
    }

    /// Stage a new analytical light.
    ///
    /// `T` must be one of:
    /// - [`DirectionalLightDescriptor`](crate::light::DirectionalLightDescriptor)
    /// - [`SpotLightDescriptor`](crate::light::SpotLightDescriptor)
    /// - [`PointLightDescriptor`](crate::light::PointLightDescriptor)
    pub fn new_analytical_light<T>(
        &self,
        light_descriptor: T,
        nested_transform: Option<NestedTransform>,
    ) -> AnalyticalLightBundle
    where
        T: Clone + Copy + SlabItem + Send + Sync,
        Light: From<Id<T>>,
        LightDetails: From<Hybrid<T>>,
    {
        self.lighting
            .new_analytical_light(light_descriptor, nested_transform)
    }

    /// Run all upkeep and commit all staged changes to the GPU.
    ///
    /// This is done implicitly in [`Stage::render`] and [`StageRendering::run`].
    ///
    /// This can be used after dropping [`Hybrid`] or [`Gpu`] resources to reclaim
    /// those resources on the GPU.
    #[must_use]
    pub fn commit(&self) -> StageCommitResult {
        let (materials_atlas_texture_was_recreated, materials_buffer) = self.materials.commit();
        if materials_atlas_texture_was_recreated {
            self.renderlet_bind_group.invalidate();
        }
        let geometry_buffer = self.geometry.commit();
        let lighting_buffer = self.lighting.commit();
        StageCommitResult {
            geometry_buffer,
            lighting_buffer,
            materials_buffer,
        }
    }

    fn renderlet_pipeline_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let geometry_slab = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let material_slab = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

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

        let atlas = wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2Array,
                multisampled: false,
            },
            count: None,
        };
        let atlas_sampler = wgpu::BindGroupLayoutEntry {
            binding: 3,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        };
        let (irradiance, irradiance_sampler) = cubemap_entry(4);
        let (prefilter, prefilter_sampler) = cubemap_entry(6);
        let (brdf, brdf_sampler) = image2d_entry(8);

        let LightingBindGroupLayoutEntries {
            light_slab,
            shadow_map_image,
            shadow_map_sampler,
        } = LightingBindGroupLayoutEntries::new(10);

        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("renderlet"),
            entries: &[
                geometry_slab,
                material_slab,
                atlas,
                atlas_sampler,
                irradiance,
                irradiance_sampler,
                prefilter,
                prefilter_sampler,
                brdf,
                brdf_sampler,
                light_slab,
                shadow_map_image,
                shadow_map_sampler,
            ],
        })
    }

    pub fn create_renderlet_pipeline(
        device: &wgpu::Device,
        fragment_color_format: wgpu::TextureFormat,
        multisample_count: u32,
    ) -> wgpu::RenderPipeline {
        log::trace!("creating stage render pipeline");
        let label = Some("renderlet");
        let vertex_linkage = crate::linkage::renderlet_vertex::linkage(device);
        let fragment_linkage = crate::linkage::renderlet_fragment::linkage(device);

        let bind_group_layout = Self::renderlet_pipeline_bindgroup_layout(device);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: Some(vertex_linkage.entry_point),
                buffers: &[],
                compilation_options: Default::default(),
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
                count: multisample_count,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: Some(fragment_linkage.entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format: fragment_color_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            multiview: None,
            cache: None,
        })
    }

    /// Create a new stage.
    pub fn new(ctx: &crate::Context) -> Self {
        let runtime = ctx.runtime();
        let device = &runtime.device;
        let resolution @ UVec2 { x: w, y: h } = ctx.get_size();
        let atlas_size = *ctx.atlas_size.read().unwrap();
        let geometry = Geometry::new(
            ctx,
            resolution,
            UVec2::new(atlas_size.width, atlas_size.height),
        );
        let materials = Materials::new(runtime, atlas_size);
        let multisample_count = 1;
        let hdr_texture = Arc::new(RwLock::new(Texture::create_hdr_texture(
            device,
            w,
            h,
            multisample_count,
        )));
        let depth_texture =
            Texture::create_depth_texture(device, w, h, multisample_count, Some("stage-depth"));
        let msaa_render_target = Default::default();
        // UNWRAP: safe because no other references at this point (created above^)
        let bloom = Bloom::new(ctx, &hdr_texture.read().unwrap());
        let tonemapping = Tonemapping::new(
            runtime,
            ctx.get_render_target().format().add_srgb_suffix(),
            &bloom.get_mix_texture(),
        );
        let stage_pipeline = Self::create_renderlet_pipeline(
            device,
            wgpu::TextureFormat::Rgba16Float,
            multisample_count,
        );
        let geometry_buffer = geometry.slab_allocator().commit();
        let lighting = Lighting::new(&geometry);

        Self {
            materials,
            draw_calls: Arc::new(RwLock::new(DrawCalls::new(
                ctx,
                true,
                &geometry_buffer,
                &depth_texture,
            ))),
            lighting,
            depth_texture: Arc::new(RwLock::new(depth_texture)),
            stage_slab_buffer: Arc::new(RwLock::new(geometry_buffer)),
            geometry,

            renderlet_pipeline: Arc::new(RwLock::new(stage_pipeline)),
            renderlet_bind_group: ManagedBindGroup::default(),
            renderlet_bind_group_created: Arc::new(0.into()),

            skybox: Arc::new(RwLock::new(Skybox::empty(runtime))),
            skybox_bindgroup: Default::default(),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            bloom,
            tonemapping,
            has_bloom: AtomicBool::from(true).into(),
            textures_bindgroup: Default::default(),
            debug_overlay: DebugOverlay::new(device, ctx.get_render_target().format()),
            has_debug_overlay: Arc::new(false.into()),
            hdr_texture,
            msaa_render_target,
            msaa_sample_count: Arc::new(multisample_count.into()),
            clear_color_attachments: Arc::new(true.into()),
            clear_depth_attachments: Arc::new(true.into()),
            background_color: Arc::new(RwLock::new(wgpu::Color::TRANSPARENT)),
        }
    }

    pub fn set_background_color(&self, color: impl Into<Vec4>) {
        let color = color.into();
        *self.background_color.write().unwrap() = wgpu::Color {
            r: color.x as f64,
            g: color.y as f64,
            b: color.z as f64,
            a: color.w as f64,
        };
    }

    pub fn with_background_color(self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    /// Set the MSAA multisample count.
    ///
    /// Set to `1` to disable MSAA. Setting to `0` will be treated the same as
    /// setting to `1`.
    pub fn set_msaa_sample_count(&self, multisample_count: u32) {
        let multisample_count = multisample_count.max(1);
        let prev_multisample_count = self
            .msaa_sample_count
            .swap(multisample_count, Ordering::Relaxed);
        if prev_multisample_count == multisample_count {
            log::warn!("set_multisample_count: multisample count is unchanged, noop");
            return;
        }

        log::debug!("setting multisample count to {multisample_count}");
        // UNWRAP: POP
        *self.renderlet_pipeline.write().unwrap() = Self::create_renderlet_pipeline(
            self.device(),
            wgpu::TextureFormat::Rgba16Float,
            multisample_count,
        );
        let size = self.get_size();
        // UNWRAP: POP
        *self.depth_texture.write().unwrap() = Texture::create_depth_texture(
            self.device(),
            size.x,
            size.y,
            multisample_count,
            Some("stage-depth"),
        );
        // UNWRAP: POP
        let format = self.hdr_texture.read().unwrap().texture.format();
        *self.msaa_render_target.write().unwrap() = if multisample_count == 1 {
            None
        } else {
            Some(create_msaa_textureview(
                self.device(),
                size.x,
                size.y,
                format,
                multisample_count,
            ))
        };

        // Invalidate the textures bindgroup - it must be recreated
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    /// Set the MSAA multisample count.
    ///
    /// Set to `1` to disable MSAA. Setting to `0` will be treated the same as
    /// setting to `1`.
    pub fn with_msaa_sample_count(self, multisample_count: u32) -> Self {
        self.set_msaa_sample_count(multisample_count);
        self
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn set_clear_color_attachments(&self, should_clear: bool) {
        self.clear_color_attachments
            .store(should_clear, Ordering::Relaxed);
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn with_clear_color_attachments(self, should_clear: bool) -> Self {
        self.set_clear_color_attachments(should_clear);
        self
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn set_clear_depth_attachments(&self, should_clear: bool) {
        self.clear_depth_attachments
            .store(should_clear, Ordering::Relaxed);
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn with_clear_depth_attachments(self, should_clear: bool) -> Self {
        self.set_clear_depth_attachments(should_clear);
        self
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&self, debug_mode: DebugChannel) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.debug_channel = debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(self, debug_mode: DebugChannel) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether to render the debug overlay.
    pub fn set_use_debug_overlay(&self, use_debug_overlay: bool) {
        self.has_debug_overlay
            .store(use_debug_overlay, std::sync::atomic::Ordering::Relaxed);
    }

    /// Set whether to render the debug overlay.
    pub fn with_debug_overlay(self, use_debug_overlay: bool) -> Self {
        self.set_use_debug_overlay(use_debug_overlay);
        self
    }

    /// Set whether to use frustum culling on GPU before drawing.
    ///
    /// This defaults to `true`.
    pub fn set_use_frustum_culling(&self, use_frustum_culling: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.perform_frustum_culling = use_frustum_culling);
    }

    /// Set whether to render the debug overlay.
    pub fn with_frustum_culling(self, use_frustum_culling: bool) -> Self {
        self.set_use_frustum_culling(use_frustum_culling);
        self
    }

    /// Set whether to use occlusion culling on GPU before drawing.
    ///
    /// This defaults to `false`.
    ///
    /// ## Warning
    ///
    /// Occlusion culling is a feature in development. YMMV.
    pub fn set_use_occlusion_culling(&self, use_occlusion_culling: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.perform_occlusion_culling = use_occlusion_culling);
    }

    /// Set whether to render the debug overlay.
    pub fn with_occlusion_culling(self, use_occlusion_culling: bool) -> Self {
        self.set_use_occlusion_culling(use_occlusion_culling);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&self, use_lighting: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.has_lighting = use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set whether to use vertex skinning.
    pub fn set_has_vertex_skinning(&self, use_skinning: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.has_skinning = use_skinning);
    }

    /// Set whether to use vertex skinning.
    pub fn with_vertex_skinning(self, use_skinning: bool) -> Self {
        self.set_has_vertex_skinning(use_skinning);
        self
    }

    pub fn get_size(&self) -> UVec2 {
        // UNWRAP: panic on purpose
        let hdr = self.hdr_texture.read().unwrap();
        let w = hdr.width();
        let h = hdr.height();
        UVec2::new(w, h)
    }

    pub fn set_size(&self, size: UVec2) {
        if size == self.get_size() {
            return;
        }

        self.geometry
            .descriptor()
            .modify(|cfg| cfg.resolution = size);
        let hdr_texture = Texture::create_hdr_texture(self.device(), size.x, size.y, 1);
        let sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        if let Some(msaa_view) = self.msaa_render_target.write().unwrap().as_mut() {
            *msaa_view = create_msaa_textureview(
                self.device(),
                size.x,
                size.y,
                hdr_texture.texture.format(),
                sample_count,
            );
        }

        // UNWRAP: panic on purpose
        *self.depth_texture.write().unwrap() = Texture::create_depth_texture(
            self.device(),
            size.x,
            size.y,
            sample_count,
            Some("stage-depth"),
        );
        self.bloom.set_hdr_texture(self.runtime(), &hdr_texture);
        self.tonemapping
            .set_hdr_texture(self.device(), &hdr_texture);
        *self.hdr_texture.write().unwrap() = hdr_texture;

        let _ = self.skybox_bindgroup.lock().unwrap().take();
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    pub fn with_size(self, size: UVec2) -> Self {
        self.set_size(size);
        self
    }

    /// Set the size of the atlas.
    ///
    /// This will cause a repacking.
    pub fn set_atlas_size(&self, size: wgpu::Extent3d) -> Result<(), StageError> {
        log::info!("resizing atlas to {size:?}");
        self.materials.atlas().resize(self.runtime(), size)?;
        Ok(())
    }

    /// Add images to the set of atlas images.
    ///
    /// This returns a vector of [`Hybrid<AtlasTexture>`], which
    /// is a descriptor of each image on the GPU. Dropping these entries
    /// will invalidate those images and cause the atlas to be repacked, and any raw
    /// GPU references to the underlying [`AtlasTexture`] will also be invalidated.
    ///     
    /// Adding an image can be quite expensive, as it requires creating a new texture
    /// array for the atlas and repacking all previous images. For that reason it is
    /// good to batch images to reduce the number of calls.
    pub fn add_images(
        &self,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasTexture>>, StageError> {
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let frames = self.materials.atlas().add_images(&images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();

        Ok(frames)
    }

    /// Clear all images from the atlas.
    ///
    /// ## WARNING
    /// This invalidates any previously staged [`AtlasTexture`]s.
    pub fn clear_images(&self) -> Result<(), StageError> {
        let none = Option::<AtlasImage>::None;
        let _ = self.set_images(none)?;
        Ok(())
    }

    /// Set the images to use for the atlas.
    ///
    /// Resets the atlas, packing it with the given images and returning a
    /// vector of the frames already staged.
    ///
    /// ## WARNING
    /// This invalidates any previously staged [`AtlasTexture`]s.
    pub fn set_images(
        &self,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasTexture>>, StageError> {
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let frames = self.materials.atlas().set_images(&images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();

        Ok(frames)
    }

    /// Set the skybox.
    pub fn set_skybox(&self, skybox: Skybox) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut guard = self.skybox.write().unwrap();
        *guard = skybox;
        self.has_skybox
            .store(true, std::sync::atomic::Ordering::Relaxed);
        *self.skybox_bindgroup.lock().unwrap() = None;
        *self.textures_bindgroup.lock().unwrap() = None;
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

    /// Set the amount of bloom that is mixed in with the input image.
    ///
    /// Defaults to `0.04`.
    pub fn set_bloom_mix_strength(&self, strength: f32) {
        self.bloom.set_mix_strength(strength);
    }

    pub fn with_bloom_mix_strength(self, strength: f32) -> Self {
        self.set_bloom_mix_strength(strength);
        self
    }

    /// Sets the bloom filter radius, in pixels.
    ///
    /// Default is `1.0`.
    pub fn set_bloom_filter_radius(&self, filter_radius: f32) {
        self.bloom.set_filter_radius(filter_radius);
    }

    /// Sets the bloom filter radius, in pixels.
    ///
    /// Default is `1.0`.
    pub fn with_bloom_filter_radius(self, filter_radius: f32) -> Self {
        self.set_bloom_filter_radius(filter_radius);
        self
    }

    /// Return the skybox render pipeline, creating it if necessary.
    fn get_skybox_pipeline_and_bindgroup(
        &self,
        geometry_slab_buffer: &wgpu::Buffer,
    ) -> (Arc<SkyboxRenderPipeline>, Arc<wgpu::BindGroup>) {
        let msaa_sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline_guard = self.skybox_pipeline.write().unwrap();
        let pipeline = if let Some(pipeline) = pipeline_guard.as_mut() {
            if pipeline.msaa_sample_count() != msaa_sample_count {
                *pipeline = Arc::new(crate::skybox::create_skybox_render_pipeline(
                    self.device(),
                    Texture::HDR_TEXTURE_FORMAT,
                    Some(msaa_sample_count),
                ));
            }
            pipeline.clone()
        } else {
            let pipeline = Arc::new(crate::skybox::create_skybox_render_pipeline(
                self.device(),
                Texture::HDR_TEXTURE_FORMAT,
                Some(msaa_sample_count),
            ));
            *pipeline_guard = Some(pipeline.clone());
            pipeline
        };
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.skybox_bindgroup.lock().unwrap();
        let bindgroup = if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let bg = Arc::new(crate::skybox::create_skybox_bindgroup(
                self.device(),
                geometry_slab_buffer,
                &self.skybox.read().unwrap().environment_cubemap,
            ));
            *bindgroup = Some(bg.clone());
            bg
        };
        (pipeline, bindgroup)
    }

    /// Adds a renderlet to the internal list of renderlets to be drawn each
    /// frame.
    ///
    /// If you drop the renderlet and no other references are kept, it will be
    /// removed automatically from the internal list and will cease to be
    /// drawn each frame.
    pub fn add_renderlet(&self, renderlet: &Hybrid<Renderlet>) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draw_calls.write().unwrap();
        draws.add_renderlet(renderlet);
    }

    /// Erase the given renderlet from the internal list of renderlets to be
    /// drawn each frame.
    pub fn remove_renderlet(&self, renderlet: &Hybrid<Renderlet>) {
        let mut draws = self.draw_calls.write().unwrap();
        draws.remove_renderlet(renderlet);
    }

    /// Re-order the renderlets.
    ///
    /// This determines the order in which they are drawn each frame.
    ///
    /// If the `order` iterator is missing any renderlet ids, those missing
    /// renderlets will be drawn _before_ the ordered ones, in no particular
    /// order.
    pub fn reorder_renderlets(&self, order: impl IntoIterator<Item = Id<Renderlet>>) {
        log::trace!("reordering renderlets");
        // UNWRAP: panic on purpose
        let mut guard = self.draw_calls.write().unwrap();
        guard.reorder_renderlets(order);
    }

    /// Iterator over all staged [`Renderlet`]s.
    ///
    /// This iterator returns `Renderlets` wrapped in `WeakHybrid`, because they
    /// are stored by weak references internally.
    ///
    /// You should have references of your own, but this is here as a convenience
    /// method, and is used internally.
    pub fn renderlets_iter(&self) -> impl Iterator<Item = WeakHybrid<Renderlet>> {
        // UNWRAP: panic on purpose
        let guard = self.draw_calls.read().unwrap();
        guard.renderlets_iter().collect::<Vec<_>>().into_iter()
    }

    /// Returns a clone of the current depth texture.
    pub fn get_depth_texture(&self) -> DepthTexture {
        DepthTexture {
            runtime: self.runtime().clone(),
            texture: self.depth_texture.read().unwrap().texture.clone(),
        }
    }

    pub fn new_skybox_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_path(path)?;
        Ok(Skybox::new(self.runtime(), hdr))
    }

    /// Create a new [`NestedTransform`].
    pub fn new_nested_transform(&self) -> NestedTransform {
        NestedTransform::new(self.geometry.slab_allocator())
    }

    /// Render the staged scene into the given view.
    pub fn render(&self, view: &wgpu::TextureView) {
        // UNWRAP: POP
        let background_color = *self.background_color.read().unwrap();
        // UNWRAP: POP
        let msaa_target = self.msaa_render_target.read().unwrap();
        let clear_colors = self.clear_color_attachments.load(Ordering::Relaxed);
        let hdr_texture = self.hdr_texture.read().unwrap();

        let mk_ops = |store| wgpu::Operations {
            load: if clear_colors {
                wgpu::LoadOp::Clear(background_color)
            } else {
                wgpu::LoadOp::Load
            },
            store,
        };
        let render_pass_color_attachment = if let Some(msaa_view) = msaa_target.as_ref() {
            wgpu::RenderPassColorAttachment {
                ops: mk_ops(wgpu::StoreOp::Discard),
                view: msaa_view,
                resolve_target: Some(&hdr_texture.view),
            }
        } else {
            wgpu::RenderPassColorAttachment {
                ops: mk_ops(wgpu::StoreOp::Store),
                view: &hdr_texture.view,
                resolve_target: None,
            }
        };

        let depth_texture = self.depth_texture.read().unwrap();
        let clear_depth = self.clear_depth_attachments.load(Ordering::Relaxed);
        let render_pass_depth_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: if clear_depth {
                    wgpu::LoadOp::Clear(1.0)
                } else {
                    wgpu::LoadOp::Load
                },
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let pipeline_guard = self.renderlet_pipeline.read().unwrap();
        let (_submission_index, maybe_indirect_buffer) = StageRendering {
            pipeline: &pipeline_guard,
            stage: self,
            color_attachment: render_pass_color_attachment,
            depth_stencil_attachment: render_pass_depth_attachment,
        }
        .run();

        // then render bloom
        if self.has_bloom.load(Ordering::Relaxed) {
            self.bloom.bloom(self.device(), self.queue());
        } else {
            // copy the input hdr texture to the bloom mix texture
            let mut encoder =
                self.device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("no bloom copy"),
                    });
            let bloom_mix_texture = self.bloom.get_mix_texture();
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.hdr_texture.read().unwrap().texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &bloom_mix_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: bloom_mix_texture.width(),
                    height: bloom_mix_texture.height(),
                    depth_or_array_layers: 1,
                },
            );
            self.queue().submit(std::iter::once(encoder.finish()));
        }

        // then render tonemapping
        self.tonemapping.render(self.device(), self.queue(), view);

        // then render the debug overlay
        if self.has_debug_overlay.load(Ordering::Relaxed) {
            if let Some(indirect_draw_buffer) = maybe_indirect_buffer {
                self.debug_overlay.render(
                    self.device(),
                    self.queue(),
                    view,
                    &self.stage_slab_buffer.read().unwrap(),
                    &indirect_draw_buffer,
                );
            }
        }
    }
}

/// Manages scene heirarchy on the [`Stage`].
///
/// Clones all reference the same nested transform.
///
/// Only available on CPU.
#[derive(Clone)]
pub struct NestedTransform<Ct: IsContainer = HybridContainer> {
    global_transform: Ct::Container<Transform>,
    local_transform: Arc<RwLock<Transform>>,
    children: Arc<RwLock<Vec<NestedTransform>>>,
    parent: Arc<RwLock<Option<NestedTransform>>>,
}

impl core::fmt::Debug for NestedTransform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let children = self
            .children
            .read()
            .unwrap()
            .iter()
            .map(|nt| nt.global_transform.id())
            .collect::<Vec<_>>();
        let parent = self
            .parent
            .read()
            .unwrap()
            .as_ref()
            .map(|nt| nt.global_transform.id());
        f.debug_struct("NestedTransform")
            .field("local_transform", &self.local_transform)
            .field("children", &children)
            .field("parent", &parent)
            .finish()
    }
}

impl NestedTransform<WeakContainer> {
    pub fn from_hybrid(hybrid: &NestedTransform) -> Self {
        Self {
            global_transform: WeakHybrid::from_hybrid(&hybrid.global_transform),
            local_transform: hybrid.local_transform.clone(),
            children: hybrid.children.clone(),
            parent: hybrid.parent.clone(),
        }
    }

    pub(crate) fn upgrade(&self) -> Option<NestedTransform> {
        Some(NestedTransform {
            global_transform: self.global_transform.upgrade()?,
            local_transform: self.local_transform.clone(),
            children: self.children.clone(),
            parent: self.parent.clone(),
        })
    }
}

impl NestedTransform {
    pub fn new(slab: &SlabAllocator<impl IsRuntime>) -> Self {
        let nested = NestedTransform {
            local_transform: Arc::new(RwLock::new(Transform::default())),
            global_transform: slab.new_value(Transform::default()),
            children: Default::default(),
            parent: Default::default(),
        };
        nested.mark_dirty();
        nested
    }

    /// Moves the inner `Gpu<Transform>` of the global transform to a different
    /// slab.
    ///
    /// This is used by the GLTF parser to move light's node transforms to the
    /// light slab after they are created, while keeping any geometry's node
    /// transforms untouched.
    pub(crate) fn move_gpu_to_slab(&mut self, slab: &SlabAllocator<impl IsRuntime>) {
        self.global_transform = slab.new_value(Transform::default());
        self.mark_dirty();
    }

    pub fn get_notifier_index(&self) -> usize {
        self.global_transform.notifier_index()
    }

    fn mark_dirty(&self) {
        self.global_transform.set(self.get_global_transform());
        for child in self.children.read().unwrap().iter() {
            child.mark_dirty();
        }
    }

    /// Modify the local transform.
    ///
    /// This automatically applies the local transform to the global transform.
    pub fn modify(&self, f: impl Fn(&mut Transform)) {
        {
            // UNWRAP: panic on purpose
            let mut local_guard = self.local_transform.write().unwrap();
            f(&mut local_guard);
        }
        self.mark_dirty()
    }

    /// Set the local transform.
    ///
    /// This automatically applies the local transform to the global transform.
    pub fn set(&self, transform: Transform) {
        self.modify(move |t| {
            *t = transform;
        });
    }

    pub fn get(&self) -> Transform {
        *self.local_transform.read().unwrap()
    }

    pub fn get_global_transform(&self) -> Transform {
        let maybe_parent_guard = self.parent.read().unwrap();
        let transform = self.get();
        let parent_transform = maybe_parent_guard
            .as_ref()
            .map(|parent| parent.get_global_transform())
            .unwrap_or_default();
        Transform::from(Mat4::from(parent_transform) * Mat4::from(transform))
    }

    pub fn global_transform_id(&self) -> Id<Transform> {
        self.global_transform.id()
    }

    pub fn add_child(&self, node: &NestedTransform) {
        *node.parent.write().unwrap() = Some(self.clone());
        node.mark_dirty();
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&self, node: &NestedTransform) {
        self.children.write().unwrap().retain_mut(|child| {
            if child.global_transform.id() == node.global_transform.id() {
                node.mark_dirty();
                let _ = node.parent.write().unwrap().take();
                false
            } else {
                true
            }
        });
    }

    pub fn parent(&self) -> Option<NestedTransform> {
        self.parent.read().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use craballoc::runtime::CpuRuntime;
    use crabslab::{Array, Id, Slab};
    use glam::{Mat4, Vec2, Vec3};

    use crate::{
        camera::Camera,
        geometry::GeometryDescriptor,
        stage::{cpu::SlabAllocator, NestedTransform, Renderlet, Vertex},
        transform::Transform,
        Context,
    };

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
    fn matrix_subtraction_sanity() {
        let m = Mat4::IDENTITY - Mat4::IDENTITY;
        assert_eq!(Mat4::ZERO, m);
    }

    #[test]
    fn can_global_transform_calculation() {
        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "This is just riff-raff, as it doesn't compile without the borrow."
        )]
        let slab = SlabAllocator::<CpuRuntime>::new(&CpuRuntime, ());
        // Setup a hierarchy of transforms
        log::info!("new");
        let root = NestedTransform::new(&slab);
        let child = NestedTransform::new(&slab);
        log::info!("set");
        child.set(Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            ..Default::default()
        });
        log::info!("grandchild");
        let grandchild = NestedTransform::new(&slab);
        grandchild.set(Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            ..Default::default()
        });

        log::info!("hierarchy");
        // Build the hierarchy
        root.add_child(&child);
        child.add_child(&grandchild);

        log::info!("get_global_transform");
        // Calculate global transforms
        let grandchild_global_transform = grandchild.get_global_transform();

        // Assert that the global transform is as expected
        assert_eq!(
            grandchild_global_transform.translation.x, 2.0,
            "Grandchild's global translation should   2.0 along the x-axis"
        );
    }

    #[test]
    fn can_msaa() {
        let ctx = Context::headless(100, 100);
        let stage = ctx
            .new_stage()
            .with_background_color([1.0, 1.0, 1.0, 1.0])
            .with_lighting(false);
        let _camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
        let _triangle_rez = stage
            .builder()
            .with_vertices([
                Vertex::default()
                    .with_position([10.0, 10.0, 0.0])
                    .with_color([0.0, 1.0, 1.0, 1.0]),
                Vertex::default()
                    .with_position([10.0, 90.0, 0.0])
                    .with_color([1.0, 1.0, 0.0, 1.0]),
                Vertex::default()
                    .with_position([90.0, 10.0, 0.0])
                    .with_color([1.0, 0.0, 1.0, 1.0]),
            ])
            .build();

        log::debug!("rendering without msaa");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "msaa/without.png",
            img,
            img_diff::DiffCfg {
                pixel_threshold: img_diff::LOW_PIXEL_THRESHOLD,
                ..Default::default()
            },
        );
        frame.present();
        log::debug!("  all good!");

        stage.set_msaa_sample_count(4);
        log::debug!("rendering with msaa");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "msaa/with.png",
            img,
            img_diff::DiffCfg {
                pixel_threshold: img_diff::LOW_PIXEL_THRESHOLD,
                ..Default::default()
            },
        );
        frame.present();
    }

    #[test]
    fn has_consistent_stage_renderlet_strong_count() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage();
        let r = stage.new_renderlet(Renderlet::default());
        assert_eq!(1, r.ref_count());

        stage.add_renderlet(&r);
        assert_eq!(1, r.ref_count());
    }

    #[test]
    /// Tests that the PBR descriptor is written to slot 0 of the geometry buffer,
    /// and that it contains what we think it contains.
    fn stage_geometry_desc_sanity() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage();
        let _ = stage.commit();

        let slab =
            futures_lite::future::block_on(stage.geometry().slab_allocator().read(..)).unwrap();
        let pbr_desc = slab.read_unchecked(Id::<GeometryDescriptor>::new(0));
        pretty_assertions::assert_eq!(stage.geometry().descriptor().get(), pbr_desc);
    }
}
