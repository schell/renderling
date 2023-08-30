//! Build GPU scenes from the CPU.
use std::sync::Arc;

use glam::{Mat4, Vec3};
use moongraph::{Move, View, ViewMut};
use renderling_shader::{debug::DebugChannel, GpuToggles};
use snafu::prelude::*;

pub use renderling_shader::{pbr::PbrMaterial, scene::*};

use crate::{
    bloom::BloomResult, frame::FrameTextureView, hdr::HdrSurface, Atlas, BufferArray, DepthTexture,
    Device, HasWgpuBuffer, Id, Queue, Skybox, SkyboxRenderPipeline, Uniform,
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
#[derive(Debug)]
pub struct SceneBuilder {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub debug_channel: DebugChannel,
    pub projection: Mat4,
    pub view: Mat4,
    pub images: Vec<SceneImage>,
    pub skybox_image: Option<SceneImage>,
    pub skybox: Option<Skybox>,
    pub textures: Vec<TextureParams>,
    pub materials: Vec<PbrMaterial>,
    pub vertices: Vec<GpuVertex>,
    pub entities: Vec<GpuEntity>,
    pub lights: Vec<GpuLight>,
    pub use_lighting: bool,
}

impl SceneBuilder {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device,
            queue,
            debug_channel: DebugChannel::None.into(),
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            images: vec![],
            skybox_image: None,
            skybox: None,
            vertices: vec![],
            textures: vec![],
            materials: vec![],
            entities: vec![],
            lights: vec![],
            use_lighting: true,
        }
    }

    /// Set whether or not to use **ANY** lighting.
    /// If set to false, this will overide model and material settings.
    pub fn set_use_lighting(&mut self, use_lighting: bool) {
        self.use_lighting = use_lighting;
    }

    /// Set whether or not to use **ANY** lighting.
    /// If set to false, this will overide model and material settings.
    pub fn with_lighting(mut self, use_lighting: bool) -> Self {
        self.set_use_lighting(use_lighting);
        self
    }

    /// Add a material.
    pub fn add_material(&mut self, material: PbrMaterial) -> Id<PbrMaterial> {
        let id = self.materials.len();
        self.materials.push(material);
        Id::new(id as u32)
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

    /// Add a skybox image from bytes.
    ///
    /// Currently only one skybox image is supported, so this function returns
    /// nothing.
    ///
    /// ## Panics
    /// This function panics if there is an error decoding the HDR image data.
    pub fn add_skybox_image_from_bytes(&mut self, hdr_data: &[u8]) {
        self.skybox_image = Some(SceneImage::from_hdr_bytes(hdr_data).unwrap());
    }

    /// Add a skybox image from bytes.
    ///
    /// Currently only one skybox image is supported, so this function returns
    /// nothing.
    pub fn with_skybox_image_from_bytes(mut self, hdr_data: &[u8]) -> Self {
        self.add_skybox_image_from_bytes(hdr_data);
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Add a skybox image from path.
    ///
    /// Not available on WASM.
    ///
    /// Currently only one skybox image is supported, so this function returns
    /// nothing.
    ///
    /// ## Panics
    /// This function panics if there is an error decoding the HDR image data.
    pub fn add_skybox_image_from_path(&mut self, hdr_path: impl AsRef<std::path::Path>) {
        self.skybox_image = Some(SceneImage::from_hdr_path(hdr_path).unwrap());
    }

    /// Add a pre-made [`Skybox`].
    ///
    /// This overrides any path or image given to use for the skybox.
    pub fn add_skybox(&mut self, skybox: Skybox) {
        self.skybox = Some(skybox);
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Add a skybox image from path.
    ///
    /// Not available on WASM.
    ///
    /// Currently only one skybox image is supported, so this function returns
    /// nothing.
    pub fn with_skybox_image_from_path(mut self, hdr_path: impl AsRef<std::path::Path>) -> Self {
        self.add_skybox_image_from_path(hdr_path);
        self
    }

    /// Add a pre-made [`Skybox`].
    ///
    /// This overrides any path or image given to use for the skybox.
    pub fn with_skybox(mut self, skybox: Skybox) -> Self {
        self.add_skybox(skybox);
        self
    }

    pub fn with_camera(mut self, projection: Mat4, view: Mat4) -> Self {
        self.set_camera(projection, view);
        self
    }

    pub fn set_camera(&mut self, projection: Mat4, view: Mat4) {
        self.projection = projection;
        self.view = view;
    }

    pub fn set_debug_channel(&mut self, channel: DebugChannel) {
        self.debug_channel = channel.into();
    }

    pub fn with_debug_channel(mut self, channel: DebugChannel) -> Self {
        self.set_debug_channel(channel);
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

    #[cfg(all(feature = "gltf", not(target_arch = "wasm32")))]
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
            gltf::import(path).map_err(|source| gltf_support::GltfLoaderError::Gltf {
                source,
                cwd: std::env::current_dir()
                    .map(|p| format!("{}", p.display()))
                    .unwrap_or("?".to_string()),
            })?;
        gltf_support::GltfLoader::load(self, document, buffers, images)
    }

    #[cfg(feature = "gltf")]
    /// Load the gltf file of the given bytes.
    ///
    /// The first material in `self` will be used as the default material for
    /// the gltf or none if not available.
    pub fn gltf_load_bytes(
        &mut self,
        bytes: Vec<u8>,
    ) -> Result<gltf_support::GltfLoader, gltf_support::GltfLoaderError> {
        let (document, buffers, images) =
            gltf::import_slice(&bytes).map_err(|source| gltf_support::GltfLoaderError::Gltf {
                source,
                cwd: "{bytes}".to_string(),
            })?;
        gltf_support::GltfLoader::load(self, document, buffers, images)
    }

    pub fn build(self) -> Result<Scene, SceneError> {
        Scene::new(self)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type MutableBufferArray<T> = BufferArray<T>;
#[cfg(target_arch = "wasm32")]
pub type MutableBufferArray<T> = BufferArray<T, crate::CpuAndGpuBuffer>;

pub struct Scene {
    pub vertices: BufferArray<GpuVertex>,
    pub entities: MutableBufferArray<GpuEntity>,
    pub lights: BufferArray<GpuLight>,
    pub materials: BufferArray<PbrMaterial>,
    pub textures: BufferArray<GpuTexture>,
    pub indirect_draws: MutableBufferArray<DrawIndirect>,
    pub constants: Uniform<GpuConstants>,
    pub skybox: Skybox,
    pub atlas: Atlas,
    skybox_update: Option<Option<SceneImage>>,
    cull_bindgroup: wgpu::BindGroup,
    render_buffers_bindgroup: wgpu::BindGroup,
    render_atlas_bindgroup: wgpu::BindGroup,
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
            debug_channel,
            projection,
            view,
            images,
            skybox_image,
            skybox,
            textures,
            materials,
            vertices,
            entities,
            lights,
            use_lighting,
        } = scene_builder;
        snafu::ensure!(images.is_empty(), AlreadyPackedAtlasSnafu);
        let debug_mode = debug_channel.into();
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
        let textures = BufferArray::new(&device, &textures, textures.len(), scene_render_usage());
        let vertices = BufferArray::new(&device, &vertices, vertices.len(), scene_render_usage());
        let draws = entities
            .iter()
            .map(|_| DrawIndirect::default())
            .collect::<Vec<_>>();
        let indirect_draws: MutableBufferArray<_> =
            BufferArray::new(&device, &draws, draws.len(), scene_indirect_usage());
        let entities: MutableBufferArray<_> =
            BufferArray::new(&device, &entities, entities.len(), scene_render_usage());
        let lights = BufferArray::new(&device, &lights, lights.len(), scene_render_usage());
        let materials =
            BufferArray::new(&device, &materials, materials.len(), scene_render_usage());
        let constants = Uniform::new(
            &device,
            GpuConstants {
                camera_projection: projection,
                camera_pos: view.inverse().transform_point3(Vec3::ZERO).extend(0.0),
                camera_view: view,
                atlas_size: atlas.size,
                debug_mode,
                toggles: {
                    let mut toggles = GpuToggles::default();
                    toggles.set_has_skybox(skybox_image.is_some() || skybox.is_some());
                    toggles.set_use_lighting(use_lighting);
                    toggles
                },
            },
            wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        );
        let skybox = if let Some(mut skybox) = skybox {
            log::trace!("scene has skybox");
            skybox.environment_bindgroup = crate::skybox::create_skybox_bindgroup(
                &device,
                &constants,
                &skybox.environment_cubemap,
            );
            skybox
        } else {
            if let Some(skybox_img) = skybox_image {
                log::trace!("scene will build a skybox");
                Skybox::new(&device, &queue, &constants, skybox_img)
            } else {
                log::trace!("skybox is empty");
                Skybox::empty(&device, &queue, &constants)
            }
        };

        let cull_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new cull_bindgroup"),
            layout: &scene_draw_indirect_bindgroup_layout(&device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: indirect_draws.buffer.get_wgpu_buffer().as_entire_binding(),
            }],
        });

        // TODO: rename this or regroup the bindgroups
        let render_buffers_bindgroup = create_scene_buffers_bindgroup(
            &device,
            &constants,
            &vertices,
            &entities.buffer.get_wgpu_buffer(),
            &lights,
            &materials,
            &skybox,
        );

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
            skybox,
            skybox_update: None,
        };
        scene.update(&device, &queue);
        Ok(scene)
    }

    /// Update the scene.
    ///
    /// This uploads changed data to the GPU and submits the queue.
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let Self {
            constants,
            render_buffers_bindgroup,
            render_atlas_bindgroup: _,
            indirect_draws: _,
            cull_bindgroup: _,
            atlas: _,
            skybox,
            skybox_update,
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
        match skybox_update.take() {
            None => {
                // no update, do nothing
            }
            Some(None) => {
                // skybox should be "removed"
                constants.toggles.set_has_skybox(false);
            }
            Some(Some(img)) => {
                // skybox should change image
                log::trace!("skybox changed");
                constants.toggles.set_has_skybox(true);
                *skybox = Skybox::new(device, queue, constants, img);
                // we also have to create a new render buffers bindgroup because irradiance is
                // part of that
                *render_buffers_bindgroup = create_scene_buffers_bindgroup(
                    device,
                    constants,
                    vertices,
                    entities.buffer.get_wgpu_buffer(),
                    lights,
                    materials,
                    &self.skybox,
                );
            }
        }
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

    pub fn get_debug_channel(&self) -> DebugChannel {
        self.constants.debug_mode.into()
    }

    pub fn set_debug_channel(&mut self, channel: DebugChannel) {
        let current: DebugChannel = self.constants.debug_mode.into();
        if current != channel {
            log::debug!("setting debug mode from '{current:?}' to '{channel:?}'");
            self.constants.debug_mode = channel.into();
        }
    }

    /// Queues an update to the skybox.
    ///
    /// This will not update any GPU resources until [`Scene::update`] is
    /// called.
    pub fn set_skybox_img(&mut self, may_img: Option<SceneImage>) {
        self.skybox_update = Some(may_img);
    }
}

pub fn create_scene_buffers_bindgroup(
    device: &wgpu::Device,
    constants: &Uniform<GpuConstants>,
    vertices: &BufferArray<GpuVertex>,
    entities: &wgpu::Buffer,
    lights: &BufferArray<GpuLight>,
    materials: &BufferArray<PbrMaterial>,
    skybox: &Skybox,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                resource: entities.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: lights.get_buffer().as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: materials.get_buffer().as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::TextureView(&skybox.irradiance_cubemap.view),
            },
            wgpu::BindGroupEntry {
                binding: 6,
                resource: wgpu::BindingResource::Sampler(&skybox.irradiance_cubemap.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 7,
                resource: wgpu::BindingResource::TextureView(
                    &skybox.prefiltered_environment_cubemap.view,
                ),
            },
            wgpu::BindGroupEntry {
                binding: 8,
                resource: wgpu::BindingResource::Sampler(
                    &skybox.prefiltered_environment_cubemap.sampler,
                ),
            },
            wgpu::BindGroupEntry {
                binding: 9,
                resource: wgpu::BindingResource::TextureView(&skybox.brdf_lut.view),
            },
            wgpu::BindGroupEntry {
                binding: 10,
                resource: wgpu::BindingResource::Sampler(&skybox.brdf_lut.sampler),
            },
        ],
    })
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
    entries.extend([
        wgpu::BindGroupLayoutEntry {
            binding: 5,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::Cube,
                multisampled: false,
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 6,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 7,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::Cube,
                multisampled: false,
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 8,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 9,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 10,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
    ]);
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene buffers and lighting"),
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
            targets: &[
                Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }),
                Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }),
            ],
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

pub fn scene_update(
    (device, queue, mut scene): (View<Device>, View<Queue>, ViewMut<Scene>),
) -> Result<(), SceneError> {
    scene.update(&device, &queue);
    Ok(())
}

pub fn scene_cull(
    (device, queue, scene, pipeline): (
        View<Device>,
        View<Queue>,
        View<Scene>,
        View<SceneComputeCullPipeline>,
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

pub fn skybox_render(
    (device, queue, scene, pipeline, hdr_frame, depth): (
        View<Device>,
        View<Queue>,
        View<Scene>,
        View<SkyboxRenderPipeline>,
        View<HdrSurface>,
        View<DepthTexture>,
    ),
) -> Result<(), SceneError> {
    if !scene.constants.toggles.get_has_skybox() {
        return Ok(());
    }
    let label = Some("skybox render");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &hdr_frame.hdr_texture.view,
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
    render_pass.set_bind_group(0, &scene.skybox.environment_bindgroup, &[]);
    render_pass.draw(0..36, 0..1);
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

pub fn scene_render(
    (device, queue, scene, pipeline, hdr_frame, depth): (
        View<Device>,
        View<Queue>,
        View<Scene>,
        View<SceneRenderPipeline>,
        View<HdrSurface>,
        View<DepthTexture>,
    ),
) -> Result<(), SceneError> {
    let label = Some("scene hdr render");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &hdr_frame.color_attachments(),
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
    // We should use multi_draw_indirect_count after (actually) culling
    #[cfg(not(target_arch = "wasm32"))]
    render_pass.multi_draw_indirect(
        scene.indirect_draws.get_buffer(),
        0,
        scene.entities.len() as u32,
    );
    #[cfg(target_arch = "wasm32")]
    {
        let indirect_buffer = scene
            .indirect_draws
            .buffer
            .cpu_buffer
            .read::<DrawIndirect>(0, scene.indirect_draws.len())
            .context(BufferSnafu)?;
        for indirect in indirect_buffer.into_iter() {
            if indirect.instance_count == 0 || indirect.vertex_count == 0 {
                continue;
            }
            let id = indirect.base_instance;
            render_pass.draw(0..indirect.vertex_count, id..id + 1);
        }
    }

    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

/// Conducts the HDR tone mapping, writing the HDR surface texture to the (most
/// likely) sRGB window surface.
pub fn scene_tonemapping(
    (device, queue, frame, hdr_frame, bloom_result): (
        View<Device>,
        View<Queue>,
        View<FrameTextureView>,
        View<HdrSurface>,
        Move<BloomResult>,
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
        depth_stencil_attachment: None,
    });
    render_pass.set_pipeline(&hdr_frame.tonemapping_pipeline);
    render_pass.set_bind_group(0, &hdr_frame.texture_bindgroup, &[]);
    render_pass.set_bind_group(1, hdr_frame.constants.bindgroup(), &[]);
    let bloom_bg = bloom_result
        .0
        .as_deref()
        .unwrap_or(&hdr_frame.no_bloom_bindgroup);
    render_pass.set_bind_group(2, bloom_bg, &[]);
    render_pass.draw(0..6, 0..1);
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}
