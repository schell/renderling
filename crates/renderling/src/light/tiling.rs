//! Implementation of light tiling.
//!
//! For more info on what light tiling _is_, see
//! [this blog post](https://renderling.xyz/articles/live/light_tiling.html).
// TODO: Auto-generate more pipeline linkage like layout, bindgroups and pipeline itself.

use core::sync::atomic::AtomicUsize;
use std::sync::Arc;

use craballoc::{
    runtime::WgpuRuntime,
    slab::{SlabAllocator, SlabBuffer},
    value::{GpuArray, Hybrid},
};
use crabslab::Id;
use glam::UVec2;

use crate::bindgroup::ManagedBindGroup;

use super::{LightTile, LightTilingDescriptor};

pub struct LightTiling {
    // depth_pre_pass_pipeline: Arc<wgpu::RenderPipeline>,
    pub(crate) tiling_slab: SlabAllocator<WgpuRuntime>,
    pub(crate) tiling_descriptor: Hybrid<LightTilingDescriptor>,
    _tiles: GpuArray<LightTile>,
    bind_group_creation_time: Arc<AtomicUsize>,
    depth_texture_id: Arc<AtomicUsize>,
    compute_tiles_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    compute_tiles_bind_group: ManagedBindGroup,
    compute_tiles_pipeline: Arc<wgpu::ComputePipeline>,
}

impl LightTiling {
    fn create_compute_tiles_pipeline(
        device: &wgpu::Device,
        multisampled: bool,
    ) -> (
        wgpu::ComputePipeline,
        wgpu::PipelineLayout,
        wgpu::BindGroupLayout,
    ) {
        let label = Some("light-tiling-compute-tiles");
        let module = if multisampled {
            crate::linkage::light_tiling_compute_tiles_multisampled::linkage(device)
        } else {
            crate::linkage::light_tiling_compute_tiles::linkage(device)
        };
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[
                // Geometry slab
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Lighting slab
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Tiling slab
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Depth texture
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let compute_tiles_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label,
                layout: Some(&pipeline_layout),
                module: &module.module,
                entry_point: Some(module.entry_point),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });
        (compute_tiles_pipeline, pipeline_layout, bind_group_layout)
    }

    pub fn new(
        runtime: impl AsRef<WgpuRuntime>,
        multisampled: bool,
        depth_texture_size: UVec2,
        max_lights_per_tile: usize,
    ) -> Self {
        let runtime = runtime.as_ref();
        let (compute_tiles_pipeline, _, compute_tiles_bind_group_layout) =
            Self::create_compute_tiles_pipeline(&runtime.device, multisampled);
        let tiling_slab = SlabAllocator::new(runtime, "tiling", wgpu::BufferUsages::empty());
        let desc = LightTilingDescriptor {
            depth_texture_size,
            ..Default::default()
        };
        let tiling_descriptor = tiling_slab.new_value(desc);
        let tiled_size = desc.tile_dimensions();
        let mut tiles = Vec::new();
        for _ in 0..tiled_size.x * tiled_size.y {
            let lights = tiling_slab.new_array(vec![Id::NONE; max_lights_per_tile]);
            tiles.push(LightTile {
                lights_array: lights.array(),
                ..Default::default()
            });
        }
        let tiles = tiling_slab.new_array(tiles).into_gpu_only();
        tiling_descriptor.modify(|d| {
            d.tiles_array = tiles.array();
        });
        Self {
            tiling_slab,
            tiling_descriptor,
            _tiles: tiles,
            bind_group_creation_time: Default::default(),
            depth_texture_id: Default::default(),
            compute_tiles_bind_group_layout: compute_tiles_bind_group_layout.into(),
            compute_tiles_bind_group: Default::default(),
            compute_tiles_pipeline: compute_tiles_pipeline.into(),
        }
    }

    pub(crate) fn prepare(&self, depth_texture_size: UVec2) {
        self.tiling_descriptor.modify(|d| {
            d.depth_texture_size = depth_texture_size;
        });
    }

    pub fn run(
        &self,
        geometry_slab: &SlabBuffer<wgpu::Buffer>,
        lighting_slab: &SlabBuffer<wgpu::Buffer>,
        depth_texture: &crate::texture::Texture,
    ) {
        let depth_texture_size = depth_texture.size();
        self.prepare(depth_texture_size);

        let runtime = self.tiling_slab.runtime();
        let tiling_slab_buffer = self.tiling_slab.commit();
        let label = Some("light-tiling-compute-tiles");
        let mut encoder = runtime
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_tiles_pipeline);

            // UNWRAP: safe because we know there are elements
            let latest_buffer_creation = [
                tiling_slab_buffer.creation_time(),
                geometry_slab.creation_time(),
                lighting_slab.creation_time(),
            ]
            .into_iter()
            .max()
            .unwrap();
            let prev_buffer_creation = self
                .bind_group_creation_time
                .swap(latest_buffer_creation, std::sync::atomic::Ordering::Relaxed);
            let prev_depth_texture_id = self
                .depth_texture_id
                .swap(depth_texture.id(), std::sync::atomic::Ordering::Relaxed);
            let should_invalidate = tiling_slab_buffer.is_new_this_commit()
                || prev_buffer_creation < latest_buffer_creation
                || prev_depth_texture_id < depth_texture.id();
            let bind_group = self.compute_tiles_bind_group.get(should_invalidate, || {
                runtime
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label,
                        layout: &self.compute_tiles_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: geometry_slab.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: lighting_slab.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: tiling_slab_buffer.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::TextureView(&depth_texture.view),
                            },
                        ],
                    })
            });
            compute_pass.set_bind_group(0, bind_group.as_ref(), &[]);

            let x = depth_texture_size.x / LightTilingDescriptor::TILE_SIZE.x + 1;
            let y = depth_texture_size.y / LightTilingDescriptor::TILE_SIZE.y + 1;
            let z = 1;
            compute_pass.dispatch_workgroups(x, y, z);
        }
        runtime.queue.submit(Some(encoder.finish()));
    }

    #[cfg(test)]
    pub(crate) async fn read_images(
        &self,
    ) -> (image::GrayImage, image::GrayImage, image::GrayImage) {
        use crabslab::Slab;
        let size = self.tiling_descriptor.get().depth_texture_size / 16;
        let slab = self.tiling_slab.read(..).await.unwrap();
        log::info!("tiling slab size: {}", slab.len());
        let desc = slab.read(Id::<LightTilingDescriptor>::new(0));
        log::info!("desc: {desc:#?}");
        assert_eq!(size.x * size.y, desc.tiles_array.len() as u32);
        let (mins, maxs, lights) = slab
            .read_vec(desc.tiles_array)
            .into_iter()
            .map(|tile| {
                (
                    crate::math::scaled_u32_to_u8(tile.depth_min),
                    crate::math::scaled_u32_to_u8(tile.depth_max),
                    crate::math::scaled_f32_to_u8(
                        tile.next_light_index as f32 / tile.lights_array.len() as f32,
                    ),
                )
            })
            .fold(
                (vec![], vec![], vec![]),
                |(mut ays, mut bees, mut cees), (a, b, c)| {
                    ays.push(a);
                    bees.push(b);
                    cees.push(c);
                    (ays, bees, cees)
                },
            );
        let mins_img = image::GrayImage::from_vec(size.x, size.y, mins).unwrap();
        let maxs_img = image::GrayImage::from_vec(size.x, size.y, maxs).unwrap();
        let lights_img = image::GrayImage::from_vec(size.x, size.y, lights).unwrap();
        (mins_img, maxs_img, lights_img)
    }
}
