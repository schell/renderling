//! The CPU side of [`renderling_shader::scene`] module.
use std::{any::Any, marker::PhantomData};

use async_channel::{Receiver, Sender};
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use image::{EncodableLayout, RgbaImage};
use moongraph::{IsGraphNode, Read, Write};
use snafu::prelude::*;
use wgpu::util::DeviceExt;

pub use renderling_shader::scene::*;

use crate::{
    linkage, node::FrameTextureView, DepthTexture, Device, Queue, RenderTarget, Renderling, Texture,
};

#[derive(Debug, Snafu)]
pub enum SceneError {
    #[snafu(display(
        "{name} has run out of capacity. Capacity is {capacity} but the operation requires \
         {required}."
    ))]
    NoCapacity {
        name: &'static str,
        capacity: usize,
        required: usize,
    },

    #[snafu(display("Out of bounds, index is {index} but length is {length}."))]
    OutOfBounds { index: usize, length: usize },

    #[snafu(display("Invalid indirect draw count"))]
    InvalidIndirectCount,

    #[snafu(display("Cannot pack textures"))]
    CannotPackTextures,

    #[snafu(display("Missing texture {id}"))]
    MissingTexture { id: u32 },

    #[snafu(display("All atlases are used."))]
    ExhaustedAtlases,
}

fn scene_render_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
}

fn scene_indirect_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::STORAGE
        | wgpu::BufferUsages::COPY_DST
        | wgpu::BufferUsages::COPY_SRC
        | wgpu::BufferUsages::INDIRECT
}

/// Read a vector from the GPU corresponding to the given range.
///
/// This creates an output buffer, creates an encoder, submits the queue and
/// then maps the output buffer and polls the device.
pub fn read_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    start: usize,
    length: usize,
) -> Result<Vec<T>, SceneError> {
    log::trace!(
        "reading {length} {} starting at index {start}",
        std::any::type_name::<T>()
    );
    let output_buffer_size = (length * std::mem::size_of::<T>()) as u64;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("GpuArray output buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(
        buffer,
        (start * std::mem::size_of::<T>()) as u64,
        &output_buffer,
        0,
        output_buffer_size,
    );
    queue.submit(std::iter::once(encoder.finish()));

    let buffer_slice = output_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let items = bytemuck::cast_slice::<u8, T>(&buffer_slice.get_mapped_range()).to_vec();
    output_buffer.unmap();
    Ok(items)
}

fn gpu_storage_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    label: Option<&str>,
    contents: &[T],
    capacity: usize,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let mut contents = contents.to_vec();
    contents.resize_with(capacity, T::zeroed);
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        usage,
        contents: bytemuck::cast_slice(contents.as_slice()),
    });
    buffer
}

pub struct GpuBuffer {
    // the gpu-side buffer
    pub buffer: wgpu::Buffer,
    // the number of elements stored in the buffer
    len: usize,
    // the total number of elements that can be stored in the buffer
    capacity: usize,
}

/// An array of `T` elements living on the GPU, backed by a storage buffer.
pub struct GpuArray<T: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: GpuBuffer,
    updates: (Sender<(usize, Vec<T>)>, Receiver<(usize, Vec<T>)>),
    _phantom: PhantomData<T>,
}

impl<T: Any + Clone + Copy + bytemuck::Pod + bytemuck::Zeroable> GpuArray<T> {
    /// Create a new buffer of [`GpuMeshVertex`] on the GPU.
    pub fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let capacity = capacity.max(1);
        GpuArray {
            buffer: GpuBuffer {
                buffer: gpu_storage_buffer(
                    device,
                    Some("GpuArray::new"),
                    &contents,
                    capacity,
                    usage,
                ),
                len: 0,
                capacity,
            },
            updates: async_channel::unbounded(),
            _phantom: PhantomData,
        }
    }

    /// Push items onto the end of the array.
    ///
    /// Returns the index of the first item and the number of new items.
    ///
    /// Errs if the array has no capacity for the items.
    pub fn extend(
        &mut self,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(usize, usize), SceneError> {
        let items = items.into_iter().collect::<Vec<_>>();
        let items_len = items.len();
        let required = self.buffer.len + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity,
            NoCapacitySnafu {
                name: std::any::type_name::<Self>(),
                capacity: self.buffer.capacity,
                required
            }
        );
        let start = self.buffer.len;
        self.updates.0.try_send((start, items)).unwrap();
        self.buffer.len += items_len;
        Ok((start, items_len))
    }

    /// Push an item onto the end of the array.
    ///
    /// Returns the index of the item and `1`.
    ///
    /// Errs if the array has no capacity for the item.
    pub fn push(&mut self, item: T) -> Result<(usize, usize), SceneError> {
        self.extend(vec![item])
    }

    /// Overwrite a portion of items in the array.
    ///
    /// Returns the starting index and the length.
    ///
    /// Errs if the array has no capacity for the items.
    pub fn overwrite(
        &mut self,
        start_index: usize,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(usize, usize), SceneError> {
        let items = items.into_iter().collect::<Vec<_>>();
        let items_len = items.len();
        let required = start_index + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity,
            NoCapacitySnafu {
                name: std::any::type_name::<Self>(),
                capacity: self.buffer.capacity,
                required
            }
        );
        self.updates.0.try_send((start_index, items)).unwrap();
        self.buffer.len = self.buffer.len.max(start_index + items_len);
        Ok((start_index, items_len))
    }

    /// Return the length of the array.
    pub fn len(&self) -> usize {
        self.buffer.len
    }

    /// Return the capacity of the array.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity
    }

    /// Update the buffer on the GPU side.
    ///
    /// This array won't be fully updated on the GPU side until the queue has
    /// been submitted.
    pub fn update(&self, queue: &wgpu::Queue) {
        while let Ok((starting_index, items)) = self.updates.1.try_recv() {
            log::trace!(
                "{} updating {} items",
                std::any::type_name::<Self>(),
                items.len()
            );
            queue.write_buffer(
                &self.buffer.buffer,
                (starting_index * std::mem::size_of::<T>()) as u64,
                bytemuck::cast_slice(items.as_slice()),
            );
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer.buffer
    }

    /// Read a vector from the GPU corresponding to the given range.
    ///
    /// This creates an output buffer, creates an encoder, submits the queue and
    /// then maps the output buffer and polls the device.
    pub fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        length: usize,
    ) -> Result<Vec<T>, SceneError> {
        read_buffer(device, queue, self.get_buffer(), start, length)
    }
}

/// Like a GpuArray but with only one element.
pub struct Gpu<T: bytemuck::Pod + bytemuck::Zeroable> {
    inner: GpuArray<T>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> Gpu<T> {
    pub fn new(device: &wgpu::Device, contents: T, usage: wgpu::BufferUsages) -> Self {
        Self {
            inner: GpuArray::<T>::new(device, &[contents], 1, usage),
        }
    }

    pub fn set(&mut self, t: T) -> Result<(), SceneError> {
        let (start, len) = self.inner.overwrite(0, vec![t])?;
        debug_assert_eq!((0, 1), (start, len));
        Ok(())
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.inner.update(queue)
    }

    pub fn read(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<T, SceneError> {
        match self.inner.read(device, queue, 0, 1)?.as_slice() {
            &[t] => Ok(t),
            _ => Err(SceneError::InvalidIndirectCount),
        }
    }
}

pub struct Atlas {
    pub texture: Texture,
    pub rects: Vec<crunch::Rect>,
    pub size: Vec2,
}

impl Atlas {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) -> Self {
        log::trace!("creating new atlas with dimensions {width} {height}");
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("atlas texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let img = RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));
        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            img.as_bytes(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            size,
        );

        let gpu_texture = crate::Texture::from_wgpu_tex(texture, device);

        Atlas {
            texture: gpu_texture,
            rects: vec![crunch::Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
            }],
            size: Vec2::new(width as f32, height as f32),
        }
    }

    /// Packs the atlas with the list of images.
    ///
    /// Returns a vector of ids that determine the locations of the given images
    /// within the atlas.
    ///
    /// This invalidates any pointers to previous textures in this atlas.
    pub fn pack(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: impl IntoIterator<Item = RgbaImage>,
    ) -> Result<Vec<u32>, SceneError> {
        // TODO: we don't actually need this white_pixel because the shader sets the
        // uv colors to white if the texture index is `0`. Just make sure we don't hand
        // out a `0` id.
        let white_pixel = RgbaImage::from_pixel(1, 1, image::Rgba([255, 255, 255, 255]));
        let images = std::iter::once(white_pixel)
            .chain(images.into_iter())
            .collect::<Vec<_>>();
        let len = images.len();
        let crunch::PackedItems { w, h, items } = crunch::pack_into_po2(
            8192,
            images.iter().enumerate().map(|(i, img)| {
                let w = img.width();
                let h = img.height();
                crunch::Item::new(i, w as usize, h as usize, crunch::Rotation::None)
            }),
        )
        .ok()
        .context(CannotPackTexturesSnafu)?;

        *self = Atlas::new(device, queue, w as u32, h as u32);
        self.rects = vec![crunch::Rect::default(); len];

        for crunch::PackedItem { data: i, rect } in items.into_iter() {
            let img = &images[i];
            queue.write_texture(
                wgpu::ImageCopyTextureBase {
                    texture: &self.texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: rect.x as u32,
                        y: rect.y as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                img.as_bytes(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * img.width()),
                    rows_per_image: std::num::NonZeroU32::new(img.height()),
                },
                wgpu::Extent3d {
                    width: img.width(),
                    height: img.height(),
                    depth_or_array_layers: 1,
                },
            );

            self.rects[i] = rect;
        }

        Ok((1..len as u32).collect())
    }

    pub fn images(&self) -> Vec<(u32, crunch::Rect)> {
        (0u32..).zip(self.rects.iter().copied()).collect()
    }

    pub fn transform_uvs(uv: Vec2, rect: crunch::Rect, atlas_size: Vec2) -> Vec2 {
        let crunch::Rect { x, y, w, h } = rect;
        let img_origin = Vec2::new(x as f32, y as f32);
        let img_size = Vec2::new(w as f32, h as f32);
        // convert the uv from normalized into pixel locations in the original image
        let uv_img_pixels = uv * img_size;
        // convert those into pixel locations in the atlas image
        let uv_atlas_pixels = img_origin + uv_img_pixels;
        // normalize the uvs by the atlas size
        let uv_atlas_normalized = uv_atlas_pixels / atlas_size;
        uv_atlas_normalized
    }
}

/// A builder for a spot light.
pub struct GpuSpotLightBuilder<'a> {
    inner: GpuLight,
    scene: &'a mut Scene,
}

impl<'a> GpuSpotLightBuilder<'a> {
    pub fn new(scene: &'a mut Scene) -> GpuSpotLightBuilder<'a> {
        let inner = GpuLight {
            light_type: GpuLight::SPOT_LIGHT,
            ..Default::default()
        };
        let white = Vec4::splat(1.0);
        Self { inner, scene }
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
        let _ = self.scene.lights.push(self.inner);
        self.inner
    }
}

/// A builder for a directional light.
///
/// Directional lights illuminate all geometry from a certain direction,
/// without attenuation.
///
/// This is like the sun, or the moon.
pub struct GpuDirectionalLightBuilder<'a> {
    inner: GpuLight,
    scene: &'a mut Scene,
}

impl<'a> GpuDirectionalLightBuilder<'a> {
    pub fn new(scene: &'a mut Scene) -> GpuDirectionalLightBuilder<'a> {
        let inner = GpuLight {
            light_type: GpuLight::DIRECTIONAL_LIGHT,
            ..Default::default()
        };
        Self { inner, scene }
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
        let _ = self.scene.lights.push(self.inner);
        self.inner
    }
}

pub struct GpuPointLightBuilder<'a> {
    inner: GpuLight,
    scene: &'a mut Scene,
}

impl<'a> GpuPointLightBuilder<'a> {
    pub fn new(scene: &mut Scene) -> GpuPointLightBuilder<'_> {
        let inner = GpuLight {
            light_type: GpuLight::POINT_LIGHT,
            ..Default::default()
        };
        let white = Vec4::splat(1.0);
        GpuPointLightBuilder { inner, scene }
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
        self.inner.ambient_color= color.into();
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
        let _ = self.scene.lights.push(self.inner);
        self.inner
    }
}

/// The parameters of the scene.
#[derive(Clone, Copy)]
pub struct SceneConfig {
    // the maximum number of vertices in the scene
    pub max_vertices: usize,
    // the maximum number of transforms in the scene
    pub max_transforms: usize,
    // the maximum number of entities in the scene
    pub max_entities: usize,
    // the naxmum number of lights in the scene
    pub max_lights: usize,
}

impl Default for SceneConfig {
    fn default() -> Self {
        // These are all very hand-wavey guesses.
        let megabyte = 10usize.pow(6);
        let mb128 = 128 * megabyte;
        Self {
            max_vertices: mb128 / std::mem::size_of::<GpuVertex>(),
            max_transforms: mb128 / std::mem::size_of::<Mat4>(),
            max_entities: mb128 / std::mem::size_of::<GpuEntity>(),
            max_lights: 64,
        }
    }
}

pub struct Scene {
    next_entity: u32,
    pub vertices: GpuArray<GpuVertex>,
    pub transforms: GpuArray<Mat4>,
    pub entities: GpuArray<GpuEntity>,
    pub lights: GpuArray<GpuLight>,
    // TODO: change the camera to a uniform to save storage buffers
    pub camera: wgpu::Buffer,
    pub indirect_draws: GpuArray<DrawIndirect>,
    camera_update: Option<GpuCamera>,
    cull_bindgroup: wgpu::BindGroup,
    render_buffers_bindgroup: wgpu::BindGroup,
    render_atlas_bindgroup: wgpu::BindGroup,
    atlas: Atlas,
}

impl Scene {
    /// Graph helper to create a new scene on the GPU.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, scene_config: SceneConfig) -> Self {
        let SceneConfig {
            max_vertices,
            max_transforms,
            max_entities,
            max_lights,
        } = scene_config;
        let vertices = GpuArray::<GpuVertex>::new(&device, &[], max_vertices, scene_render_usage());
        let transforms = GpuArray::<Mat4>::new(&device, &[], max_transforms, scene_render_usage());
        let entities_contents = vec![GpuEntity::default(); max_entities];
        let entities = GpuArray::<GpuEntity>::new(
            &device,
            &entities_contents,
            max_entities,
            scene_render_usage(),
        );
        let lights = GpuArray::<GpuLight>::new(&device, &[], max_lights, scene_render_usage());
        let atlas = Atlas::new(device, queue, 1, 1);
        let indirect_draws =
            GpuArray::<DrawIndirect>::new(&device, &[], max_entities, scene_indirect_usage());
        let camera = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene::new camera"),
            contents: bytemuck::cast_slice(&[GpuCamera::default()]),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let cull_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new cull_bindgroup"),
            layout: &scene_draw_indirect_bindgroup_layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: indirect_draws.get_buffer().as_entire_binding(),
            }],
        });

        let render_buffers_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new render_buffers_bindgroup"),
            layout: &scene_vertex_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: vertices.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: transforms.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: entities.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: lights.get_buffer().as_entire_binding(),
                },
            ],
        });

        let render_atlas_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new render_atlas_bindgroup"),
            layout: &scene_fragment_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&atlas.texture.sampler),
                },
            ],
        });

        Self {
            next_entity: 0,
            render_buffers_bindgroup,
            render_atlas_bindgroup,
            vertices,
            transforms,
            entities,
            camera_update: None,
            camera,
            indirect_draws,
            cull_bindgroup,
            atlas,
            lights,
        }
    }

    /// Update the scene.
    ///
    /// This uploads changed data to the GPU and submits the queue.
    pub fn update(&mut self, queue: &wgpu::Queue) {
        let Self {
            next_entity: _,
            camera: _,
            render_buffers_bindgroup: _,
            render_atlas_bindgroup: _,
            indirect_draws: _,
            cull_bindgroup: _,
            atlas: _,
            vertices,
            transforms,
            entities,
            camera_update,
            lights,
        } = self;
        vertices.update(queue);
        transforms.update(queue);
        entities.update(queue);
        lights.update(queue);
        if let Some(camera) = camera_update.take() {
            queue.write_buffer(&self.camera, 0, bytemuck::cast_slice(&[camera]));
        }
        queue.submit(std::iter::empty());
    }

    /// Set the camera.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn set_camera(&mut self, camera: GpuCamera) {
        self.camera_update = Some(camera);
    }

    /// Set a transform.
    pub fn set_transform(&mut self, id: u32, transform: Mat4) -> Result<(), SceneError> {
        let (i, n) = self
            .transforms
            .overwrite(id as usize, std::iter::once(transform))?;
        debug_assert_eq!((id as usize, 1), (i, n));
        Ok(())
    }

    /// Update/set an entity.
    pub fn update_entity(&mut self, entity: GpuEntity) -> Result<(), SceneError> {
        let (i, n) = self
            .entities
            .overwrite(entity.id as usize, std::iter::once(entity))?;
        debug_assert_eq!((entity.id as usize, 1), (i, n));
        Ok(())
    }

    /// Load a new mesh.
    ///
    /// Returns the index of the first vertex of the newly created meshlet and
    /// the vertex count.
    ///
    /// The data is not uploaded to the GPU until [`Scene::update`] has been
    /// called.
    pub fn new_meshlet(
        &mut self,
        vertices: impl IntoIterator<Item = GpuVertex>,
        texture0_id: Option<u32>,
        texture1_id: Option<u32>,
    ) -> Result<(u32, u32), SceneError> {
        let id0 = texture0_id.unwrap_or_default();
        let id1 = texture1_id.unwrap_or_default();
        let vertices = {
            let rect0 = self
                .atlas
                .rects
                .get(id0 as usize)
                .context(MissingTextureSnafu { id: id0 })?;
            let rect1 = self
                .atlas
                .rects
                .get(id1 as usize)
                .context(MissingTextureSnafu { id: id1 })?;
            let size = self.atlas.size;
            vertices
                .into_iter()
                .map(|mut v| {
                    let uv0 = Atlas::transform_uvs(v.uv.xy(), *rect0, size);
                    let uv1 = Atlas::transform_uvs(v.uv.zw(), *rect1, size);
                    v.uv = Vec4::new(uv0.x, uv0.y, uv1.x, uv1.y);
                    v
                })
                .collect::<Vec<_>>()
        };
        let (start, len) = self.vertices.extend(vertices)?;
        Ok((start as u32, len as u32))
    }

    /// Load a new transform.
    ///
    /// Returns the index of the transform in the GPU transform buffer.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn new_transform(&mut self, transform: Mat4) -> Result<usize, SceneError> {
        Ok(self.transforms.push(transform)?.0)
    }

    /// Load an atlas full of images.
    ///
    /// Returns the ids to use to associate meshlets with the images.
    pub fn load_images(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: impl IntoIterator<Item = RgbaImage>,
    ) -> Result<Vec<u32>, SceneError> {
        let ids = self.atlas.pack(device, queue, images)?;
        self.render_atlas_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::load_images render_atlas_bindgroup"),
            layout: &scene_fragment_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.atlas.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.atlas.texture.sampler),
                },
            ],
        });
        Ok(ids)
    }

    pub fn new_entity(&mut self) -> EntityBuilder<'_> {
        EntityBuilder {
            scene: self,
            meshlet: Err((0, 0)),
            transform: None,
            textures: [0, 0],
            lighting: LightingModel::NO_LIGHTING,
        }
    }

    pub fn new_directional_light(&mut self) -> GpuDirectionalLightBuilder<'_> {
        GpuDirectionalLightBuilder::new(self)
    }

    pub fn new_spot_light(&mut self) -> GpuSpotLightBuilder<'_> {
        GpuSpotLightBuilder::new(self)
    }

    pub fn new_point_light(&mut self) -> GpuPointLightBuilder<'_> {
        GpuPointLightBuilder::new(self)
    }

    /// Return a reference to the inner texture Atlas.
    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }
}

pub struct EntityBuilder<'a> {
    scene: &'a mut Scene,
    meshlet: Result<Vec<GpuVertex>, (u32, u32)>,
    transform: Option<Mat4>,
    textures: [u32; 2],
    lighting: u32,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_meshlet(mut self, vertices: impl IntoIterator<Item = GpuVertex>) -> Self {
        self.meshlet = Ok(vertices.into_iter().collect());
        self
    }

    pub fn with_starting_vertex_and_count(mut self, first_vertex: u32, count: u32) -> Self {
        self.meshlet = Err((first_vertex, count));
        self
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn with_texture_ids(mut self, id0: Option<u32>, id1: Option<u32>) -> Self {
        self.textures = [id0.unwrap_or_default(), id1.unwrap_or_default()];
        self
    }

    pub fn with_lighting_model(mut self, lighting: u32) -> Self {
        self.lighting = lighting;
        self
    }

    pub fn build(self) -> Result<GpuEntity, SceneError> {
        let EntityBuilder {
            scene,
            meshlet,
            transform,
            textures,
            lighting,
        } = self;
        let mut entity = GpuEntity {
            id: scene.next_entity,
            ..Default::default()
        };
        scene.next_entity += 1;

        let model_matrix = transform.unwrap_or_else(|| Mat4::IDENTITY);
        let model_matrix_id = scene.new_transform(model_matrix)?;
        let normal_matrix = model_matrix.inverse().transpose();
        let normal_matrix_id = scene.new_transform(normal_matrix)?;
        entity.model_matrix = model_matrix_id as u32;
        entity.normal_matrix = normal_matrix_id as u32;
        entity.lighting = lighting;
        entity.texture0 = textures[0];
        entity.texture1 = textures[1];

        let (start, len) = match meshlet {
            Ok(vertices) => scene.new_meshlet(vertices, Some(textures[0]), Some(textures[1]))?,
            Err(start_len) => start_len,
        };
        entity.mesh_first_vertex = start;
        entity.mesh_vertex_count = len;

        let (id, _) = scene.entities.push(entity.clone())?;
        debug_assert_eq!(entity.id, id as u32);

        Ok(entity)
    }
}

pub fn scene_vertex_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let entries = (0..5)
        .map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX
                | wgpu::ShaderStages::FRAGMENT
                | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene render vertex"),
        entries: &entries,
    })
}

pub fn scene_fragment_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene render fragment"),
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
    let shader_crate = linkage::shader_crate(device);
    let scene_buffers_layout = scene_vertex_bindgroup_layout(device);
    let scene_atlas_layout = scene_fragment_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&scene_buffers_layout, &scene_atlas_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader_crate,
            entry_point: "main_vertex_scene",
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
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
            module: &shader_crate,
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
    let shader_crate = linkage::shader_crate(device);
    let scene_buffers_layout = scene_vertex_bindgroup_layout(device);
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
    let num_entities = scene.next_entity;
    let groups = num_entities / 32 + 1;
    compute_pass.dispatch_workgroups(groups, 1, 1);
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

pub fn setup_scene_render_graph(scene: Scene, r: &mut Renderling) {
    r.add_resource(scene);

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
}
