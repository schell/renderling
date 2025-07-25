//! Implementation of light tiling.
//!
//! For more info on what light tiling is, see
//! [this blog post](https://renderling.xyz/articles/live/light_tiling.html).

use core::sync::atomic::AtomicUsize;
use std::sync::Arc;

use craballoc::{
    runtime::WgpuRuntime,
    slab::{SlabAllocator, SlabBuffer},
    value::{GpuArrayContainer, Hybrid, HybridArrayContainer, IsContainer},
};
use crabslab::Id;
use glam::UVec2;

use crate::bindgroup::ManagedBindGroup;

use super::{LightTile, LightTilingDescriptor};

/// Shaders and resources for conducting light tiling.
///
/// This struct takes a container type variable in order to allow
/// tests to read and write [`LightTile`] values on the GPU.
///
/// For info on what light tiling is, see
/// <https://renderling.xyz/articles/live/light_tiling.html>.
pub struct LightTiling<Ct: IsContainer = GpuArrayContainer> {
    // TODO: maybe we don't need a tiling slab, and we could just run on the lighting slab?
    // Revisit this after light tiling works, as managing less slabs is better
    pub(crate) tiling_slab: SlabAllocator<WgpuRuntime>,
    pub(crate) tiling_descriptor: Hybrid<LightTilingDescriptor>,
    tiles: Ct::Container<LightTile>,
    /// Cache of the id of the Stage's depth texture.
    ///
    /// Used to invalidate our tiling bindgroup.
    depth_texture_id: Arc<AtomicUsize>,

    bindgroup: ManagedBindGroup,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    bindgroup_creation_time: Arc<AtomicUsize>,

    clear_tiles_pipeline: Arc<wgpu::ComputePipeline>,
    compute_min_max_depth_pipeline: Arc<wgpu::ComputePipeline>,
    compute_bins_pipeline: Arc<wgpu::ComputePipeline>,
}

const LABEL: Option<&'static str> = Some("light-tiling");

impl<Ct: IsContainer> LightTiling<Ct> {
    fn create_bindgroup_layout(device: &wgpu::Device, multisampled: bool) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: LABEL,
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
        })
    }

    fn create_clear_tiles_pipeline(
        device: &wgpu::Device,
        multisampled: bool,
    ) -> wgpu::ComputePipeline {
        const LABEL: Option<&'static str> = Some("light-tiling-clear-tiles");
        let module = crate::linkage::light_tiling_clear_tiles::linkage(device);
        let (pipeline_layout, _) = Self::create_layouts(device, multisampled);
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: LABEL,
            layout: Some(&pipeline_layout),
            module: &module.module,
            entry_point: Some(module.entry_point),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        })
    }

    fn create_compute_min_max_depth_pipeline(
        device: &wgpu::Device,
        multisampled: bool,
    ) -> wgpu::ComputePipeline {
        const LABEL: Option<&'static str> = Some("light-tiling-compute-min-max-depth");
        let module = crate::linkage::light_tiling_compute_tile_min_and_max_depth::linkage(device);
        let (pipeline_layout, _) = Self::create_layouts(device, multisampled);
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: LABEL,
            layout: Some(&pipeline_layout),
            module: &module.module,
            entry_point: Some(module.entry_point),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        })
    }

    fn create_compute_bins_pipeline(
        device: &wgpu::Device,
        multisampled: bool,
    ) -> wgpu::ComputePipeline {
        const LABEL: Option<&'static str> = Some("light-tiling-compute-bins");
        let module = crate::linkage::light_tiling_bin_lights::linkage(device);
        let (pipeline_layout, _) = Self::create_layouts(device, multisampled);
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: LABEL,
            layout: Some(&pipeline_layout),
            module: &module.module,
            entry_point: Some(module.entry_point),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        })
    }

    /// All pipelines share the same layout, so we do it here, once.
    fn create_layouts(
        device: &wgpu::Device,
        multisampled: bool,
    ) -> (wgpu::PipelineLayout, wgpu::BindGroupLayout) {
        let bindgroup_layout = Self::create_bindgroup_layout(device, multisampled);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: LABEL,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        (pipeline_layout, bindgroup_layout)
    }

    pub(crate) fn prepare(&self, depth_texture_size: UVec2) {
        self.tiling_descriptor.modify(|d| {
            d.depth_texture_size = depth_texture_size;
        });
    }

    pub(crate) fn clear_tiles(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bindgroup: &wgpu::BindGroup,
    ) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("light-tiling-clear-tiles"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.clear_tiles_pipeline);
        compute_pass.set_bind_group(0, bindgroup, &[]);

        let x = (LightTilingDescriptor::TILE_SIZE.x / 16) + 1;
        let y = (LightTilingDescriptor::TILE_SIZE.y / 16) + 1;
        let z = 1;
        compute_pass.dispatch_workgroups(x, y, z);
    }

    pub(crate) fn compute_min_max_depth(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bindgroup: &wgpu::BindGroup,
        depth_texture_size: UVec2,
    ) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("light-tiling-compute-min-max-depth"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.compute_min_max_depth_pipeline);
        compute_pass.set_bind_group(0, bindgroup, &[]);

        let x = (depth_texture_size.x / 16) + 1;
        let y = (depth_texture_size.y / 16) + 1;
        let z = 1;
        compute_pass.dispatch_workgroups(x, y, z);
    }

    pub(crate) fn compute_bins(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bindgroup: &wgpu::BindGroup,
        depth_texture_size: UVec2,
    ) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("light-tiling-compute-bins"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.compute_bins_pipeline);
        compute_pass.set_bind_group(0, bindgroup, &[]);

        let x = (depth_texture_size.x / 16) + 1;
        let y = (depth_texture_size.y / 16) + 1;
        let z = 1;
        compute_pass.dispatch_workgroups(x, y, z);
    }

    /// Get the bindgroup.
    ///
    /// This commits the tiling slab.
    pub fn get_bindgroup(
        &self,
        device: &wgpu::Device,
        geometry_slab: &SlabBuffer<wgpu::Buffer>,
        lighting_slab: &SlabBuffer<wgpu::Buffer>,
        depth_texture: &crate::texture::Texture,
    ) -> Arc<wgpu::BindGroup> {
        let tiling_slab_buffer = self.tiling_slab.commit();
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
            .bindgroup_creation_time
            .swap(latest_buffer_creation, std::sync::atomic::Ordering::Relaxed);
        let prev_depth_texture_id = self
            .depth_texture_id
            .swap(depth_texture.id(), std::sync::atomic::Ordering::Relaxed);
        let should_invalidate = tiling_slab_buffer.is_new_this_commit()
            || prev_buffer_creation < latest_buffer_creation
            || prev_depth_texture_id < depth_texture.id();
        self.bindgroup.get(should_invalidate, || {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("light-tiling"),
                layout: &self.bindgroup_layout,
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
        })
    }

    /// Run light tiling, resulting in edits to the tiling slab.
    pub fn run(
        &self,
        geometry_slab: &SlabBuffer<wgpu::Buffer>,
        lighting_slab: &SlabBuffer<wgpu::Buffer>,
        depth_texture: &crate::texture::Texture,
    ) {
        let depth_texture_size = depth_texture.size();
        self.prepare(depth_texture_size);

        let runtime = self.tiling_slab.runtime();
        let label = Some("light-tiling-run");
        let bindgroup =
            self.get_bindgroup(&runtime.device, geometry_slab, lighting_slab, depth_texture);

        let mut encoder = runtime
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            self.clear_tiles(&mut encoder, bindgroup.as_ref());
            self.compute_min_max_depth(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            self.compute_bins(&mut encoder, bindgroup.as_ref(), depth_texture_size);
        }
        runtime.queue.submit(Some(encoder.finish()));
    }

    pub fn tiles(&self) -> &Ct::Container<LightTile> {
        &self.tiles
    }

    #[cfg(test)]
    /// Returns a tuple containing an image of depth mins, depth maximums and number of lights.
    pub(crate) async fn read_images(
        &self,
    ) -> (image::GrayImage, image::GrayImage, image::GrayImage) {
        use crabslab::Slab;

        let tile_dimensions = self.tiling_descriptor.get().tile_dimensions();
        let slab = self.tiling_slab.read(..).await.unwrap();
        log::info!("tiling slab length: {}", slab.len());
        let desc = slab.read(Id::<LightTilingDescriptor>::new(0));
        log::info!("light-tiling-descriptor: {desc:#?}");
        assert_eq!(
            tile_dimensions.x * tile_dimensions.y,
            desc.tiles_array.len() as u32,
            "LightTilingDescriptor's tiles array is borked: {:?}",
            desc.tiles_array
        );
        let (mins, maxs, lights) = slab
            .read_vec(desc.tiles_array)
            .into_iter()
            .enumerate()
            .map(|(i, tile)| {
                assert!(tile.depth_min <= tile.depth_max);
                log::info!(
                    "read_images-light-{i}-next_light_index: {}",
                    tile.next_light_index
                );
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
        let mins_img =
            image::GrayImage::from_vec(tile_dimensions.x, tile_dimensions.y, mins).unwrap();
        let maxs_img =
            image::GrayImage::from_vec(tile_dimensions.x, tile_dimensions.y, maxs).unwrap();
        let lights_img =
            image::GrayImage::from_vec(tile_dimensions.x, tile_dimensions.y, lights).unwrap();
        (mins_img, maxs_img, lights_img)
    }
}

impl LightTiling<HybridArrayContainer> {
    /// Creates a new [`LightTiling`] struct with a [`HybridArray`] of tiles.
    pub fn new_hybrid(
        runtime: impl AsRef<WgpuRuntime>,
        multisampled: bool,
        depth_texture_size: UVec2,
        max_lights_per_tile: usize,
    ) -> Self {
        let runtime = runtime.as_ref();

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
        let tiles = tiling_slab.new_array(tiles);
        tiling_descriptor.modify(|d| {
            d.tiles_array = tiles.array();
        });
        let clear_tiles_pipeline = Arc::new(Self::create_clear_tiles_pipeline(
            &runtime.device,
            multisampled,
        ));
        let compute_min_max_depth_pipeline = Arc::new(Self::create_compute_min_max_depth_pipeline(
            &runtime.device,
            multisampled,
        ));
        let compute_bins_pipeline = Arc::new(Self::create_compute_bins_pipeline(
            &runtime.device,
            multisampled,
        ));
        let bindgroup_layout =
            Arc::new(Self::create_bindgroup_layout(&runtime.device, multisampled));

        Self {
            tiling_slab,
            tiling_descriptor,
            tiles,
            // The inner bindgroup is created on-demand
            bindgroup: ManagedBindGroup::default(),
            bindgroup_creation_time: Default::default(),
            bindgroup_layout,
            depth_texture_id: Default::default(),
            clear_tiles_pipeline,
            compute_min_max_depth_pipeline,
            compute_bins_pipeline,
        }
    }
}

impl LightTiling {
    /// Creates a new [`LightTiling`] struct.
    pub fn new(
        runtime: impl AsRef<WgpuRuntime>,
        multisampled: bool,
        depth_texture_size: UVec2,
        max_lights_per_tile: usize,
    ) -> Self {
        // Note to self, I wish we had `fmap` here.
        let LightTiling {
            tiling_slab,
            tiling_descriptor,
            tiles,
            bindgroup_creation_time,
            depth_texture_id,
            bindgroup_layout,
            bindgroup,
            clear_tiles_pipeline,
            compute_min_max_depth_pipeline,
            compute_bins_pipeline,
        } = LightTiling::new_hybrid(
            runtime,
            multisampled,
            depth_texture_size,
            max_lights_per_tile,
        );
        Self {
            tiling_slab,
            tiling_descriptor,
            tiles: tiles.into_gpu_only(),
            depth_texture_id,
            bindgroup,
            bindgroup_layout,
            bindgroup_creation_time,
            clear_tiles_pipeline,
            compute_min_max_depth_pipeline,
            compute_bins_pipeline,
        }
    }
}
