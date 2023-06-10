//! Build GPU scenes from the CPU.
use std::sync::Arc;

use glam::{Mat4, Vec2, Vec3};
use moongraph::{IsGraphNode, Read, Write};
use renderling_shader::GpuToggles;
use snafu::prelude::*;

pub use renderling_shader::scene::*;

use crate::{
    node::{self, FrameTextureView, HdrSurface},
    Atlas, DepthTexture, Device, GpuArray, Queue, Renderling, Uniform,
};

mod entity;
pub use entity::*;

mod img;
pub use img::*;

mod light;
pub use light::*;

#[cfg(feature = "gltf")]
mod gltf_support;
#[cfg(feature = "gltf")]
pub use gltf_support::*;

mod material;
pub use material::*;

#[derive(Debug, Snafu)]
pub enum SceneError {
    #[snafu(display("{source}"))]
    Buffer { source: crate::BufferError },

    #[snafu(display("Invalid indirect draw count"))]
    InvalidIndirectCount,

    #[snafu(display("{source}"))]
    Atlas { source: crate::AtlasError },

    #[snafu(display("Missing texture {id}"))]
    MissingTexture { id: u32 },

    #[snafu(display("Missing image {index}"))]
    MissingImage { index: usize },

    #[snafu(display("All atlases are used."))]
    ExhaustedAtlases,

    #[snafu(display("Images cannot be packed. The texture atlas has already been packed."))]
    AlreadyPackedAtlas,

    #[snafu(display(
        "The scene builder has {name}s, but a text scene only contains one {name} which is \
         automatically included."
    ))]
    TextSceneSingleton { name: &'static str },

    #[snafu(display("Missing the glyph.cache"))]
    MissingCache,

    #[snafu(display("Missing entity {id:?}"))]
    MissingEntity { id: Id<GpuEntity> },
}

pub(crate) fn scene_render_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
}

pub(crate) fn scene_indirect_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::STORAGE
        | wgpu::BufferUsages::COPY_DST
        | wgpu::BufferUsages::COPY_SRC
        | wgpu::BufferUsages::INDIRECT
}

/// Helps build textures, storing texture parameters before images are packed
/// into the atlas.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureParams {
    // The index of the image this texture references.
    pub image_index: usize,
    // The wrapping in S.
    pub mode_s: TextureAddressMode,
    // The wrapping in T.
    pub mode_t: TextureAddressMode,
}

impl Default for TextureParams {
    fn default() -> Self {
        Self {
            image_index: Default::default(),
            mode_s: Default::default(),
            mode_t: Default::default(),
        }
    }
}

/// Sets up the scene using different build phases.
#[derive(Clone, Debug)]
pub struct SceneBuilder {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub toggles: GpuToggles,
    pub debug_mode: DebugMode,
    pub projection: Mat4,
    pub view: Mat4,
    pub images: Vec<SceneImage>,
    pub textures: Vec<TextureParams>,
    pub materials: Vec<GpuMaterial>,
    pub vertices: Vec<GpuVertex>,
    pub entities: Vec<GpuEntity>,
    pub lights: Vec<GpuLight>,
}

impl SceneBuilder {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device,
            queue,
            toggles: GpuToggles::default(),
            debug_mode: DebugMode::NONE,
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            images: vec![],
            vertices: vec![],
            textures: vec![],
            materials: vec![],
            entities: vec![],
            lights: vec![],
        }
    }

    pub fn set_gamma_correction(&mut self, gamma_correction_on: bool) {
        self.toggles.set_gamma_correct(gamma_correction_on);
    }

    pub fn with_gamma_correction(mut self, gamma_correction_on: bool) -> Self {
        self.set_gamma_correction(gamma_correction_on);
        self
    }

    /// Add an image and return a texture id for it.
    pub fn add_image_texture_mode(
        &mut self,
        img: impl Into<SceneImage>,
        mode_s: TextureAddressMode,
        mode_t: TextureAddressMode,
    ) -> Id<GpuTexture> {
        let image_index = self.images.len();
        self.images.push(img.into());
        self.add_texture(TextureParams {
            image_index,
            mode_s,
            mode_t,
        })
    }

    /// Add an image and return a texture id for it.
    ///
    /// The texture sampling mode defaults to "repeat".
    pub fn add_image_texture(&mut self, img: impl Into<SceneImage>) -> Id<GpuTexture> {
        self.add_image_texture_mode(
            img,
            TextureAddressMode::CLAMP_TO_EDGE,
            TextureAddressMode::CLAMP_TO_EDGE,
        )
    }

    /// Add an image, without adding an explicit texture.
    ///
    /// Returns the index of the image.
    pub fn add_image(&mut self, img: impl Into<SceneImage>) -> usize {
        let index = self.images.len();
        self.images.push(img.into());
        index
    }

    /// Add a texture referencing an image.
    pub fn add_texture(&mut self, params: TextureParams) -> Id<GpuTexture> {
        let texture_id = self.textures.len();
        self.textures.push(params);
        Id::new(texture_id as u32)
    }

    pub fn with_camera(mut self, projection: Mat4, view: Mat4) -> Self {
        self.set_camera(projection, view);
        self
    }

    pub fn set_camera(&mut self, projection: Mat4, view: Mat4) {
        self.projection = projection;
        self.view = view;
    }

    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        self.debug_mode = debug_mode;
    }

    pub fn with_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    pub fn new_directional_light(&mut self) -> GpuDirectionalLightBuilder<'_> {
        GpuDirectionalLightBuilder::new(&mut self.lights)
    }

    pub fn new_spot_light(&mut self) -> GpuSpotLightBuilder<'_> {
        GpuSpotLightBuilder::new(&mut self.lights)
    }

    pub fn new_point_light(&mut self) -> GpuPointLightBuilder<'_> {
        GpuPointLightBuilder::new(&mut self.lights)
    }

    pub fn add_material(&mut self, material: GpuMaterial) -> u32 {
        let id = self.materials.len();
        self.materials.push(material);
        id as u32
    }

    /// Add a meshlet.
    ///
    /// Returns the index of the first vertex of the newly created meshlet and
    /// the vertex count.
    pub fn add_meshlet(&mut self, vertices: impl IntoIterator<Item = GpuVertex>) -> (u32, u32) {
        let vertices = vertices.into_iter().collect::<Vec<_>>();
        let start = self.vertices.len();
        let len = vertices.len();
        self.vertices.extend(vertices);
        (start as u32, len as u32)
    }

    pub fn new_unlit_material(&mut self) -> UnlitMaterialBuilder<'_> {
        UnlitMaterialBuilder::new(self)
    }

    pub fn new_pbr_material(&mut self) -> PbrMaterialBuilder<'_> {
        PbrMaterialBuilder::new(self)
    }

    pub fn new_entity(&mut self) -> EntityBuilder<'_> {
        EntityBuilder {
            scene: self,
            entity: GpuEntity::default(),
        }
    }

    pub fn get_image_for_texture_id_mut(
        &mut self,
        tex_id: &Id<GpuTexture>,
    ) -> Option<&mut SceneImage> {
        let texture = self.textures.get(tex_id.index())?;
        self.images.get_mut(texture.image_index)
    }

    #[cfg(feature = "gltf")]
    /// Load the gltf file at the given path.
    ///
    /// The first material in `self` will be used as the default material for
    /// the gltf or none if not available.
    pub fn gltf_load(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<gltf_support::GltfLoader, gltf_support::GltfLoaderError> {
        log::trace!("loading gltf file '{}'", path.as_ref().display());
        let (document, buffers, images) =
            gltf::import(path).map_err(|source| gltf_support::GltfLoaderError::Gltf { source })?;
        gltf_support::GltfLoader::load(self, document, buffers, images)
    }

    pub fn build(self) -> Result<Scene, SceneError> {
        Scene::new(self)
    }
}

pub struct Scene {
    pub vertices: GpuArray<GpuVertex>,
    pub entities: GpuArray<GpuEntity>,
    pub lights: GpuArray<GpuLight>,
    pub materials: GpuArray<GpuMaterial>,
    pub textures: GpuArray<GpuTexture>,
    pub indirect_draws: GpuArray<DrawIndirect>,
    pub constants: Uniform<GpuConstants>,
    cull_bindgroup: wgpu::BindGroup,
    render_buffers_bindgroup: wgpu::BindGroup,
    render_atlas_bindgroup: wgpu::BindGroup,
    pub atlas: Atlas,
}

impl Scene {
    /// Creates a new scene with the given atlas.
    pub fn new(mut scene_builder: SceneBuilder) -> Result<Self, SceneError> {
        let atlas = Atlas::pack(
            &scene_builder.device,
            &scene_builder.queue,
            std::mem::take(&mut scene_builder.images),
        )
        .context(AtlasSnafu)?;
        log::trace!("atlas created");
        Self::new_with_atlas(scene_builder, atlas)
    }

    /// Creates a new scene on the GPU.
    pub fn new_with_atlas(scene_builder: SceneBuilder, atlas: Atlas) -> Result<Self, SceneError> {
        let SceneBuilder {
            device,
            queue,
            toggles,
            debug_mode,
            projection,
            view,
            images,
            textures,
            materials,
            vertices,
            entities,
            lights,
        } = scene_builder;
        snafu::ensure!(images.is_empty(), AlreadyPackedAtlasSnafu);

        let frames = textures.into_iter();
        let mut textures = vec![];
        for TextureParams {
            image_index,
            mode_s,
            mode_t,
        } in frames
        {
            let (offset_px, size_px) = atlas
                .get_frame(image_index)
                .context(MissingImageSnafu { index: image_index })?;
            textures.push(GpuTexture {
                offset_px,
                size_px,
                modes: {
                    let mut modes = TextureModes::default();
                    modes.set_wrap_s(mode_s);
                    modes.set_wrap_t(mode_t);
                    modes
                },
                ..Default::default()
            });
        }
        let textures = GpuArray::new(&device, &textures, textures.len(), scene_render_usage());
        let vertices = GpuArray::new(&device, &vertices, vertices.len(), scene_render_usage());
        let draws = entities
            .iter()
            .map(|_| DrawIndirect::default())
            .collect::<Vec<_>>();
        let indirect_draws = GpuArray::new(&device, &draws, draws.len(), scene_indirect_usage());
        let entities = GpuArray::new(&device, &entities, entities.len(), scene_render_usage());
        let lights = GpuArray::new(&device, &lights, lights.len(), scene_render_usage());
        let materials = GpuArray::new(&device, &materials, materials.len(), scene_render_usage());
        let constants = Uniform::new(
            &device,
            GpuConstants {
                camera_projection: projection,
                camera_pos: view.inverse().transform_point3(Vec3::ZERO).extend(0.0),
                camera_view: view,
                atlas_size: atlas.size,
                debug_mode,
                toggles,
            },
            wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        );

        let cull_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new cull_bindgroup"),
            layout: &scene_draw_indirect_bindgroup_layout(&device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: indirect_draws.get_buffer().as_entire_binding(),
            }],
        });

        let render_buffers_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new render_buffers_bindgroup"),
            layout: &scene_buffers_bindgroup_layout(&device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: constants.buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: vertices.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: entities.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: lights.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: materials.get_buffer().as_entire_binding(),
                },
            ],
        });

        let render_atlas_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new render_atlas_bindgroup"),
            layout: &scene_atlas_bindgroup_layout(&device),
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
                    resource: textures.get_buffer().as_entire_binding(),
                },
            ],
        });

        let mut scene = Self {
            render_buffers_bindgroup,
            render_atlas_bindgroup,
            vertices,
            textures,
            materials,
            entities,
            constants,
            indirect_draws,
            cull_bindgroup,
            atlas,
            lights,
        };
        scene.update(&queue);
        Ok(scene)
    }

    /// Update the scene.
    ///
    /// This uploads changed data to the GPU and submits the queue.
    pub fn update(&mut self, queue: &wgpu::Queue) {
        let Self {
            constants,
            render_buffers_bindgroup: _,
            render_atlas_bindgroup: _,
            indirect_draws: _,
            cull_bindgroup: _,
            atlas: _,
            vertices,
            entities,
            materials,
            textures,
            lights,
        } = self;
        vertices.update(queue);
        entities.update(queue);
        materials.update(queue);
        textures.update(queue);
        lights.update(queue);
        constants.update(queue);
        queue.submit(std::iter::empty());
    }

    /// Set the camera.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn set_camera(&mut self, proj: Mat4, view: Mat4) {
        if self.constants.camera_projection != proj || self.constants.camera_view != view {
            self.constants.camera_projection = proj;
            self.constants.camera_view = view;
            self.constants.camera_pos = view.inverse().transform_point3(Vec3::ZERO).extend(0.0);
        }
    }

    /// Set the camera projection.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn set_camera_projection(&mut self, proj: Mat4) {
        let view = self.constants.camera_view;
        self.set_camera(proj, view);
    }

    /// Set the camera view.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn set_camera_view(&mut self, view: Mat4) {
        let proj = self.constants.camera_projection;
        self.set_camera(proj, view);
    }

    /// Update/set an entity.
    pub fn update_entity(&mut self, entity: GpuEntity) -> Result<(), SceneError> {
        let (i, n) = self
            .entities
            .overwrite(entity.id.index(), std::iter::once(entity))
            .context(BufferSnafu)?;
        debug_assert_eq!((entity.id.index(), 1), (i, n));
        Ok(())
    }

    pub fn get_debug_mode(&self) -> DebugMode {
        self.constants.debug_mode
    }

    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        if self.constants.debug_mode != debug_mode {
            log::debug!(
                "setting debug mode from '{}' to '{debug_mode}'",
                self.constants.debug_mode
            );
            self.constants.debug_mode = debug_mode;
        }
    }
}

pub fn scene_buffers_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let visibility =
        wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE;
    let cfg = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    };
    let mut entries = vec![cfg];
    let buffers = (1..5)
        .map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect::<Vec<_>>();
    entries.extend(buffers);
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene buffers"),
        entries: &entries,
    })
}

pub fn scene_atlas_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene atlas"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

pub fn scene_draw_indirect_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene compute cull indirect"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

pub fn create_scene_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    log::trace!("creating scene render pipeline with format '{format:?}'");
    let label = Some("scene render pipeline");
    let vertex_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/main_vertex_scene.spv"));
    let fragment_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/main_fragment_scene.spv"));
    let scene_buffers_layout = scene_buffers_bindgroup_layout(device);
    let scene_atlas_layout = scene_atlas_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&scene_buffers_layout, &scene_atlas_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "main_vertex_scene",
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
            entry_point: "main_fragment_scene",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    pipeline
}

pub fn create_scene_compute_cull_pipeline(device: &wgpu::Device) -> wgpu::ComputePipeline {
    let label = Some("scene compute cull pipeline");
    let shader_crate =
        device.create_shader_module(wgpu::include_spirv!("linkage/compute_cull_entities.spv"));
    let scene_buffers_layout = scene_buffers_bindgroup_layout(device);
    let indirect_buffers_layout = scene_draw_indirect_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&scene_buffers_layout, &indirect_buffers_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label,
        layout: Some(&layout),
        module: &shader_crate,
        entry_point: "compute_cull_entities",
    });
    pipeline
}

pub struct SceneRenderPipeline(pub wgpu::RenderPipeline);

pub struct SceneComputeCullPipeline(pub wgpu::ComputePipeline);

pub fn scene_update((queue, mut scene): (Read<Queue>, Write<Scene>)) -> Result<(), SceneError> {
    scene.update(&queue);
    Ok(())
}

pub fn scene_cull(
    (device, queue, scene, pipeline): (
        Read<Device>,
        Read<Queue>,
        Read<Scene>,
        Read<SceneComputeCullPipeline>,
    ),
) -> Result<(), SceneError> {
    let label = Some("scene cull");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label });
    compute_pass.set_pipeline(&pipeline.0);
    compute_pass.set_bind_group(0, &scene.render_buffers_bindgroup, &[]);
    compute_pass.set_bind_group(1, &scene.cull_bindgroup, &[]);
    let num_entities = scene.entities.capacity();
    let groups = num_entities / 32 + 1;
    compute_pass.dispatch_workgroups(groups as u32, 1, 1);
    drop(compute_pass);
    queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

pub fn scene_render(
    (device, queue, scene, pipeline, hdr_frame, depth): (
        Read<Device>,
        Read<Queue>,
        Read<Scene>,
        Read<SceneRenderPipeline>,
        Read<HdrSurface>,
        Read<DepthTexture>,
    ),
) -> Result<(), SceneError> {
    let label = Some("scene hdr render");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &hdr_frame.texture.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            }),
            stencil_ops: None,
        }),
    });
    render_pass.set_pipeline(&pipeline.0);
    render_pass.set_bind_group(0, &scene.render_buffers_bindgroup, &[]);
    render_pass.set_bind_group(1, &scene.render_atlas_bindgroup, &[]);
    // TODO: use RenderPass::multi_draw_indirect_count after atomics are added to
    // naga's spirv frontend
    render_pass.multi_draw_indirect(
        scene.indirect_draws.get_buffer(),
        0,
        scene.entities.len() as u32,
    );
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

/// Conducts the HDR tone mapping, writing the HDR surface texture to the (most
/// likely) sRGB window surface.
pub fn scene_tonemapping(
    (device, queue, frame, hdr_frame, depth): (
        Read<Device>,
        Read<Queue>,
        Read<FrameTextureView>,
        Read<HdrSurface>,
        Read<DepthTexture>,
    ),
) -> Result<(), SceneError> {
    let label = Some("scene tonemapping");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            }),
            stencil_ops: None,
        }),
    });
    render_pass.set_pipeline(&hdr_frame.pipeline);
    render_pass.set_bind_group(0, &hdr_frame.texture_bindgroup, &[]);
    render_pass.set_bind_group(1, hdr_frame.constants.bindgroup(), &[]);
    render_pass.draw(0..6, 0..1);
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

pub fn setup_scene_render_graph(scene: Scene, r: &mut Renderling, with_screen_capture: bool) {
    r.graph.add_resource(scene);

    let (hdr_surface,) = r
        .graph
        .visit(node::create_hdr_render_surface)
        .unwrap()
        .unwrap();
    let pipeline = SceneRenderPipeline({
        let device = r.get_device();
        create_scene_render_pipeline(&device, hdr_surface.texture.texture.format())
    });
    r.graph.add_resource(pipeline);
    r.graph.add_resource(hdr_surface);

    r.graph
        .add_node(node::create_frame.into_node().with_name("create_frame"));
    r.graph.add_node(
        node::clear_surface_hdr_and_depth
            .into_node()
            .with_name("clear_hdr_frame_and_depth"),
    );

    r.graph.add_node(
        node::hdr_surface_update
            .into_node()
            .with_name("hdr_surface_update"),
    );
    r.graph
        .add_node(scene_update.into_node().with_name("scene_update"));

    let pipeline = SceneComputeCullPipeline(
        r.graph
            .visit(|device: Read<Device>| create_scene_compute_cull_pipeline(&device))
            .unwrap(),
    );
    r.graph.add_resource(pipeline);

    r.graph.add_node(
        scene_cull
            .into_node()
            .with_name("scene_cull")
            .run_after("scene_update"),
    );
    r.graph.add_barrier();

    r.graph
        .add_node(scene_render.into_node().with_name("scene_render"));
    r.graph.add_node(
        scene_tonemapping
            .into_node()
            .with_name("scene_tonemapping")
            .run_after("scene_render")
            .run_before("present"),
    );
    r.graph
        .add_node(crate::node::present.into_node().with_name("present"));
    if with_screen_capture {
        r.graph.add_node(
            crate::node::PostRenderBufferCreate::create
                .into_node()
                .with_name("copy_frame_to_post")
                .run_after("scene_render")
                .run_before("present"),
        );
    }
}
