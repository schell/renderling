//! The CPU side of [`renderling_shader::scene`] module.
use std::sync::Arc;

use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use image::RgbaImage;
use moongraph::{IsGraphNode, Read, Write};
use snafu::prelude::*;
use wgpu::util::DeviceExt;

pub use renderling_shader::scene::*;

use crate::{
    node::FrameTextureView, Atlas, DepthTexture, Device, GlyphCache, GpuArray, Queue, RenderTarget,
    Renderling,
};

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

    #[snafu(display("Missing entity {id}"))]
    MissingEntity { id: u32 },
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

/// A builder for a spot light.
pub struct GpuSpotLightBuilder<'a> {
    inner: &'a mut GpuLight,
}

impl<'a> GpuSpotLightBuilder<'a> {
    pub fn new(lights: &'a mut Vec<GpuLight>) -> GpuSpotLightBuilder<'a> {
        let inner = GpuLight {
            light_type: GpuLight::SPOT_LIGHT,
            ..Default::default()
        };
        let index = lights.len();
        lights.push(inner);
        let white = Vec4::splat(1.0);
        Self {
            inner: &mut lights[index],
        }
        .with_cutoff(std::f32::consts::PI / 3.0, std::f32::consts::PI / 2.0)
        .with_attenuation(1.0, 0.014, 0.007)
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_ambient_color(white)
        .with_diffuse_color(white)
        .with_specular_color(white)
    }

    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.inner.position = position.into().extend(1.0);
        self
    }

    pub fn with_direction(mut self, direction: impl Into<Vec3>) -> Self {
        self.inner.direction = direction.into().extend(1.0);
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.attenuation = Vec4::new(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_cutoff(mut self, inner: f32, outer: f32) -> Self {
        self.inner.inner_cutoff = inner;
        self.inner.outer_cutoff = outer;
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.specular_color = color.into();
        self
    }

    pub fn build(self) -> GpuLight {
        self.inner.clone()
    }
}

/// A builder for a directional light.
///
/// Directional lights illuminate all geometry from a certain direction,
/// without attenuation.
///
/// This is like the sun, or the moon.
pub struct GpuDirectionalLightBuilder<'a> {
    inner: &'a mut GpuLight,
}

impl<'a> GpuDirectionalLightBuilder<'a> {
    pub fn new(lights: &'a mut Vec<GpuLight>) -> GpuDirectionalLightBuilder<'a> {
        let inner = GpuLight {
            light_type: GpuLight::DIRECTIONAL_LIGHT,
            ..Default::default()
        };
        let index = lights.len();
        lights.push(inner);
        Self {
            inner: &mut lights[index],
        }
        .with_direction(Vec3::new(0.0, -1.0, 0.0))
        .with_ambient_color(Vec4::splat(1.0))
        .with_diffuse_color(Vec4::splat(1.0))
        .with_specular_color(Vec4::splat(1.0))
    }

    pub fn with_direction(mut self, direction: impl Into<Vec3>) -> Self {
        self.inner.direction = direction.into().extend(1.0);
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.specular_color = color.into();
        self
    }

    pub fn build(self) -> GpuLight {
        *self.inner
    }
}

pub struct GpuPointLightBuilder<'a> {
    inner: &'a mut GpuLight,
}

impl<'a> GpuPointLightBuilder<'a> {
    pub fn new(lights: &mut Vec<GpuLight>) -> GpuPointLightBuilder<'_> {
        let inner = GpuLight {
            light_type: GpuLight::POINT_LIGHT,
            ..Default::default()
        };
        let index = lights.len();
        lights.push(inner);
        let white = Vec4::splat(1.0);
        GpuPointLightBuilder {
            inner: &mut lights[index],
        }
        .with_attenuation(1.0, 0.14, 0.07)
        .with_ambient_color(white)
        .with_diffuse_color(white)
        .with_specular_color(white)
    }

    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.inner.position = position.into().extend(0.0);
        self
    }

    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.inner.attenuation = Vec4::new(constant, linear, quadratic, 0.0);
        self
    }

    pub fn with_ambient_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.ambient_color = color.into();
        self
    }

    pub fn with_diffuse_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.diffuse_color = color.into();
        self
    }

    pub fn with_specular_color(mut self, color: impl Into<Vec4>) -> Self {
        self.inner.specular_color = color.into();
        self
    }

    pub fn build(self) -> GpuLight {
        *self.inner
    }
}

pub struct PhongMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: GpuMaterial,
}

impl<'a> PhongMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let mut material = GpuMaterial::default();
        material.lighting_model = LightingModel::PHONG_LIGHTING;
        Self { builder, material }
    }

    pub fn with_diffuse_texture(mut self, texture_id: u32) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_specular_texture(mut self, texture_id: u32) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn build(self) -> u32 {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        id as u32
    }
}

pub struct UnlitMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: GpuMaterial,
}

impl<'a> UnlitMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let material = GpuMaterial::default();
        Self { builder, material }
    }

    pub fn with_base_color(mut self, color: impl Into<Vec4>) -> Self {
        self.material.factor0 = color.into();
        self
    }

    pub fn with_texture0(mut self, texture_id: u32) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_texture2(mut self, texture_id: u32) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn build(self) -> u32 {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        id as u32
    }
}

pub struct PbrMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: GpuMaterial,
}

impl<'a> PbrMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let mut material = GpuMaterial::default();
        material.lighting_model = LightingModel::PBR_LIGHTING;
        Self { builder, material }
    }

    pub fn with_base_color_factor(mut self, factor: impl Into<Vec4>) -> Self {
        self.material.factor0 = factor.into();
        self
    }

    pub fn with_base_color_texture(mut self, texture_id: u32) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_metallic_factor(mut self, metallic: f32) -> Self {
        self.material.factor1.y = metallic;
        self
    }

    pub fn with_roughness_factor(mut self, roughness: f32) -> Self {
        self.material.factor1.z = roughness;
        self
    }

    pub fn with_metallic_roughness_texture(mut self, texture_id: u32) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn build(self) -> u32 {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        id as u32
    }
}

pub struct EntityBuilder<'a> {
    scene: &'a mut SceneBuilder,
    entity: GpuEntity,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.entity.visible = if is_visible { 1 } else { 0 };
        self
    }

    pub fn with_meshlet(mut self, vertices: impl IntoIterator<Item = GpuVertex>) -> Self {
        let (start, len) = self.scene.add_meshlet(vertices);
        self.entity.mesh_first_vertex = start;
        self.entity.mesh_vertex_count = len;
        self
    }

    pub fn with_starting_vertex_and_count(mut self, first_vertex: u32, count: u32) -> Self {
        self.entity.mesh_first_vertex = first_vertex;
        self.entity.mesh_vertex_count = count;
        self
    }

    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.entity.position = position.into().extend(0.0);
        self
    }

    pub fn with_scale(mut self, scale: impl Into<Vec3>) -> Self {
        self.entity.scale = scale.into().extend(0.0);
        self
    }

    pub fn with_rotation(mut self, rotation: impl Into<Quat>) -> Self {
        self.entity.rotation = rotation.into();
        self
    }

    pub fn with_material(mut self, material_id: u32) -> Self {
        self.entity.material = material_id;
        self
    }

    pub fn with_parent(mut self, parent: &GpuEntity) -> Self {
        if let Some(parent) = self.scene.entities.get_mut(parent.id as usize) {
            self.entity.parent = parent.id;
        } else {
            log::error!("no such parent entity {}", parent.id);
        }
        self
    }

    pub fn build(self) -> GpuEntity {
        let EntityBuilder { scene, mut entity } = self;
        entity.id = scene.entities.len() as u32;
        scene.entities.push(entity.clone());
        entity
    }
}

/// Sets up the scene using different build phases.
#[derive(Clone, Debug)]
pub struct SceneBuilder {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
    pub(crate) projection: Mat4,
    pub(crate) view: Mat4,
    pub(crate) images: Vec<RgbaImage>,
    pub(crate) textures: Vec<(usize, TextureAddressMode, TextureAddressMode)>,
    pub(crate) materials: Vec<GpuMaterial>,
    pub(crate) vertices: Vec<GpuVertex>,
    pub(crate) entities: Vec<GpuEntity>,
    pub(crate) lights: Vec<GpuLight>,
}

impl SceneBuilder {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device,
            queue,
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

    /// Add an image and return a texture id for it.
    pub fn add_image_texture_mode(
        &mut self,
        img: RgbaImage,
        mode_s: TextureAddressMode,
        mode_t: TextureAddressMode,
    ) -> u32 {
        let image_index = self.images.len();
        self.images.push(img);
        self.add_texture(image_index, mode_s, mode_t)
    }

    /// Add an image and return a texture id for it.
    ///
    /// The texture sampling mode defaults to "repeat".
    pub fn add_image_texture(&mut self, img: RgbaImage) -> u32 {
        self.add_image_texture_mode(
            img,
            TextureAddressMode::CLAMP_TO_EDGE,
            TextureAddressMode::CLAMP_TO_EDGE,
        )
    }

    /// Add an image, without adding an explicit texture.
    ///
    /// Returns the index of the image.
    pub fn add_image(&mut self, img: RgbaImage) -> usize {
        let index = self.images.len();
        self.images.push(img);
        index
    }

    /// Add a texture referencing an image.
    pub fn add_texture(
        &mut self,
        image_index: usize,
        mode_s: TextureAddressMode,
        mode_t: TextureAddressMode,
    ) -> u32 {
        let texture_id = self.textures.len();
        self.textures.push((image_index, mode_s, mode_t));
        texture_id as u32
    }

    pub fn with_camera(mut self, projection: Mat4, view: Mat4) -> Self {
        self.set_camera(projection, view);
        self
    }

    pub fn set_camera(&mut self, projection: Mat4, view: Mat4) {
        self.projection = projection;
        self.view = view;
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

    pub fn get_material(&self, material_id: u32) -> Option<GpuMaterial> {
        self.materials.get(material_id as usize).copied()
    }

    pub fn entities(&self) -> &[GpuEntity] {
        &self.entities
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

    pub fn new_phong_material(&mut self) -> PhongMaterialBuilder<'_> {
        PhongMaterialBuilder::new(self)
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

    pub fn update_entity(&mut self, entity: GpuEntity) -> Result<(), SceneError> {
        let here = self
            .entities
            .get_mut(entity.id as usize)
            .context(MissingEntitySnafu { id: entity.id })?;
        *here = entity;
        Ok(())
    }

    pub fn build(self) -> Result<Scene, SceneError> {
        Scene::new(self)
    }

    #[cfg(feature = "text")]
    /// Creates a new text scene on the GPU.
    pub fn build_text_scene(self, glyph_cache: &GlyphCache) -> Result<Scene, SceneError> {
        Scene::new_text_scene(self, glyph_cache)
    }
}

pub struct Scene {
    pub vertices: GpuArray<GpuVertex>,
    pub entities: GpuArray<GpuEntity>,
    pub lights: GpuArray<GpuLight>,
    pub materials: GpuArray<GpuMaterial>,
    pub textures: GpuArray<GpuTexture>,
    pub constants: wgpu::Buffer,
    pub indirect_draws: GpuArray<DrawIndirect>,
    constants_update: Option<GpuConstants>,
    cull_bindgroup: wgpu::BindGroup,
    render_buffers_bindgroup: wgpu::BindGroup,
    render_atlas_bindgroup: wgpu::BindGroup,
    pub atlas: Atlas,
}

impl Scene {
    #[cfg(feature = "text")]
    /// Creates a new scene on the GPU.
    pub fn new_text_scene(
        mut scene_builder: SceneBuilder,
        glyph_cache: &GlyphCache,
    ) -> Result<Self, SceneError> {
        snafu::ensure!(
            scene_builder.images.is_empty(),
            TextSceneSingletonSnafu { name: "image" }
        );

        if scene_builder.textures.is_empty() {
            scene_builder.textures.push(crate::TEXT_TEXTURE);
        } else {
            snafu::ensure!(
                scene_builder.textures.len() == 1
                    && scene_builder.textures[0] == crate::TEXT_TEXTURE,
                TextSceneSingletonSnafu { name: "texture" }
            );
        }

        if scene_builder.materials.is_empty() {
            scene_builder.materials.push(crate::TEXT_MATERIAL);
        } else {
            snafu::ensure!(
                scene_builder.materials.len() == 1
                    && scene_builder.materials[0] == crate::TEXT_MATERIAL,
                TextSceneSingletonSnafu { name: "material" }
            );
        }

        let (w, h) = glyph_cache.brush.texture_dimensions();
        let cache = glyph_cache.cache.as_ref().context(MissingCacheSnafu)?;
        let mut atlas = Atlas::new_with_texture(cache.texture.clone(), w, h);
        atlas.rects = vec![crunch::Rect {
            x: 0,
            y: 0,
            w: w as usize,
            h: h as usize,
        }];
        Self::new_with_atlas(scene_builder, atlas)
    }

    /// Creates a new scene with the given atlas.
    pub fn new(mut scene_builder: SceneBuilder) -> Result<Self, SceneError> {
        let atlas = Atlas::pack(
            &scene_builder.device,
            &scene_builder.queue,
            std::mem::take(&mut scene_builder.images),
        )
        .context(AtlasSnafu)?;
        Self::new_with_atlas(scene_builder, atlas)
    }

    /// Creates a new scene on the GPU.
    pub fn new_with_atlas(scene_builder: SceneBuilder, atlas: Atlas) -> Result<Self, SceneError> {
        let SceneBuilder {
            device,
            queue,
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
        for (image_index, address_mode_s, address_mode_t) in frames {
            let (offset_px, size_px) = atlas
                .get_frame(image_index)
                .context(MissingImageSnafu { index: image_index })?;
            textures.push(GpuTexture {
                offset_px,
                size_px,
                address_mode_s,
                address_mode_t,
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
        let constants = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene::new constants"),
            contents: bytemuck::cast_slice(&[GpuConstants {
                camera_projection: projection,
                camera_pos: view.inverse().transform_point3(Vec3::ZERO).extend(0.0),
                camera_view: view,
                atlas_size: atlas.size,
                padding: Vec2::ZERO,
            }]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

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
                    resource: constants.as_entire_binding(),
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
            constants_update: None,
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
            constants: _,
            render_buffers_bindgroup: _,
            render_atlas_bindgroup: _,
            indirect_draws: _,
            cull_bindgroup: _,
            atlas: _,
            vertices,
            entities,
            materials,
            textures,
            constants_update: camera_update,
            lights,
        } = self;
        vertices.update(queue);
        entities.update(queue);
        materials.update(queue);
        textures.update(queue);
        lights.update(queue);
        if let Some(camera) = camera_update.take() {
            queue.write_buffer(&self.constants, 0, bytemuck::cast_slice(&[camera]));
        }
        queue.submit(std::iter::empty());
    }

    /// Set the camera.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn set_camera(&mut self, proj: Mat4, view: Mat4) {
        self.constants_update = Some(GpuConstants {
            camera_projection: proj,
            camera_pos: view.inverse().transform_point3(Vec3::ZERO).extend(0.0),
            camera_view: view,
            atlas_size: self.atlas.size,
            padding: Vec2::ZERO,
        });
    }

    /// Update/set an entity.
    pub fn update_entity(&mut self, entity: GpuEntity) -> Result<(), SceneError> {
        let (i, n) = self
            .entities
            .overwrite(entity.id as usize, std::iter::once(entity))
            .context(BufferSnafu)?;
        debug_assert_eq!((entity.id as usize, 1), (i, n));
        Ok(())
    }

    /// Return a reference to the inner texture Atlas.
    // TODO: remove this as the `atlas` field is public now
    pub fn atlas(&self) -> &Atlas {
        &self.atlas
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
            cull_mode: Some(wgpu::Face::Back),
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

pub struct SceneRenderPipeline(wgpu::RenderPipeline);

pub struct SceneComputeCullPipeline(wgpu::ComputePipeline);

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

fn scene_render(
    (device, queue, scene, pipeline, frame, depth): (
        Read<Device>,
        Read<Queue>,
        Read<Scene>,
        Read<SceneRenderPipeline>,
        Read<FrameTextureView>,
        Read<DepthTexture>,
    ),
) -> Result<(), SceneError> {
    let label = Some("scene render");
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
    render_pass.set_pipeline(&pipeline.0);
    render_pass.set_bind_group(0, &scene.render_buffers_bindgroup, &[]);
    render_pass.set_bind_group(1, &scene.render_atlas_bindgroup, &[]);
    // TODO: use RenderPass::multi_draw_indirect_count after atomics are added to
    // naga's spirv frontend @see
    render_pass.multi_draw_indirect(
        scene.indirect_draws.get_buffer(),
        0,
        scene.entities.len() as u32,
    );
    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

pub fn setup_scene_render_graph(scene: Scene, r: &mut Renderling, with_screen_capture: bool) {
    r.graph.add_resource(scene);

    let pipeline = SceneRenderPipeline(
        r.graph
            .visit(|(device, target): (Read<Device>, Read<RenderTarget>)| {
                create_scene_render_pipeline(&device, target.format())
            })
            .unwrap(),
    );
    r.graph.add_resource(pipeline);

    r.graph.add_node(
        crate::node::create_frame
            .into_node()
            .with_name("create_frame"),
    );
    r.graph.add_node(
        crate::node::clear_frame_and_depth
            .into_node()
            .with_name("clear_frame_and_depth"),
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
