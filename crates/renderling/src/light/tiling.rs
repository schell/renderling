//! Implementation of light tiling.
//!
//! For more info on what light tiling is, see
//! [this blog post](https://renderling.xyz/articles/live/light_tiling.html).

use core::sync::atomic::AtomicUsize;
use std::sync::Arc;

use craballoc::{
    slab::SlabBuffer,
    value::{GpuArrayContainer, Hybrid, HybridArrayContainer, IsContainer},
};
use crabslab::Id;
use glam::{UVec2, UVec3};

use crate::{
    bindgroup::ManagedBindGroup,
    light::{LightTile, LightTilingDescriptor, Lighting},
    stage::Stage,
};

/// Shaders and resources for conducting light tiling.
///
/// This struct takes a container type variable in order to allow
/// tests to read and write [`LightTile`] values on the GPU.
///
/// For info on what light tiling is, see
/// <https://renderling.xyz/articles/live/light_tiling.html>.
pub struct LightTiling<Ct: IsContainer = GpuArrayContainer> {
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
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Depth texture
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
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

    pub(crate) fn prepare(&self, lighting: &Lighting, depth_texture_size: UVec2) {
        self.tiling_descriptor.modify(|d| {
            d.depth_texture_size = depth_texture_size;
        });
        lighting.lighting_descriptor.modify(|desc| {
            desc.light_tiling_descriptor_id = self.tiling_descriptor.id();
        });
    }

    pub(crate) fn clear_tiles(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bindgroup: &wgpu::BindGroup,
        depth_texture_size: UVec2,
    ) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("light-tiling-clear-tiles"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.clear_tiles_pipeline);
        compute_pass.set_bind_group(0, bindgroup, &[]);

        let tile_size = self.tiling_descriptor.get().tile_size;
        let dims_f32 = depth_texture_size.as_vec2() / tile_size as f32;
        let workgroups = (dims_f32 / 16.0).ceil().as_uvec2();
        let x = workgroups.x;
        let y = workgroups.y;
        let z = 1;
        compute_pass.dispatch_workgroups(x, y, z);
    }

    const WORKGROUP_SIZE: UVec3 = UVec3::new(16, 16, 1);

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

        let x = (depth_texture_size.x / Self::WORKGROUP_SIZE.x) + 1;
        let y = (depth_texture_size.y / Self::WORKGROUP_SIZE.y) + 1;
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

        let tile_size = self.tiling_descriptor.get().tile_size;
        let x = (depth_texture_size.x / tile_size) + 1;
        let y = (depth_texture_size.y / tile_size) + 1;
        let z = 1;
        compute_pass.dispatch_workgroups(x, y, z);
    }

    /// Get the bindgroup.
    pub fn get_bindgroup(
        &self,
        device: &wgpu::Device,
        geometry_slab: &SlabBuffer<wgpu::Buffer>,
        lighting_slab: &SlabBuffer<wgpu::Buffer>,
        depth_texture: &crate::texture::Texture,
    ) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we know there are elements
        let latest_buffer_creation = [geometry_slab.creation_time(), lighting_slab.creation_time()]
            .into_iter()
            .max()
            .unwrap();
        let prev_buffer_creation = self
            .bindgroup_creation_time
            .swap(latest_buffer_creation, std::sync::atomic::Ordering::Relaxed);
        let prev_depth_texture_id = self
            .depth_texture_id
            .swap(depth_texture.id(), std::sync::atomic::Ordering::Relaxed);
        let should_invalidate = prev_buffer_creation < latest_buffer_creation
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
                        resource: wgpu::BindingResource::TextureView(&depth_texture.view),
                    },
                ],
            })
        })
    }

    /// Set the minimum illuminance, in lux, to determine if a light illuminates a tile.
    pub fn set_minimum_illuminance(&self, minimum_illuminance_lux: f32) {
        self.tiling_descriptor.modify(|desc| {
            desc.minimum_illuminance_lux = minimum_illuminance_lux;
        });
    }

    /// Run light tiling, resulting in edits to the lighting slab.
    pub fn run(&self, stage: &Stage) {
        let depth_texture = stage.depth_texture.read().unwrap();
        let depth_texture_size = depth_texture.size();
        let lighting = stage.as_ref();
        self.prepare(lighting, depth_texture_size);

        let light_slab = &lighting.light_slab;
        let geometry_slab = &lighting.geometry_slab;
        let runtime = light_slab.runtime();
        let label = Some("light-tiling-run");
        let bindgroup = self.get_bindgroup(
            &runtime.device,
            &geometry_slab.commit(),
            &light_slab.commit(),
            &depth_texture,
        );

        let mut encoder = runtime
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            self.clear_tiles(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            self.compute_min_max_depth(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            self.compute_bins(&mut encoder, bindgroup.as_ref(), depth_texture_size);
        }
        runtime.queue.submit(Some(encoder.finish()));
    }

    pub fn tiles(&self) -> &Ct::Container<LightTile> {
        &self.tiles
    }

    #[cfg(test)]
    /// Read the tiles from the light slab.
    pub(crate) async fn read_tiles(&self, lighting: &Lighting) -> Vec<LightTile> {
        lighting
            .light_slab
            .read_array(self.tiling_descriptor.get().tiles_array)
            .await
            .unwrap()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn read_tile(&self, lighting: &Lighting, tile_coord: UVec2) -> LightTile {
        let desc = self.tiling_descriptor.get();
        let tile_index = tile_coord.y * desc.tile_grid_size().x + tile_coord.x;
        let tile_id = desc.tiles_array.at(tile_index as usize);
        futures_lite::future::block_on(lighting.light_slab.read_one(tile_id)).unwrap()
    }

    #[cfg(test)]
    /// Returns a tuple containing an image of depth mins, depth maximums and number of lights.
    pub(crate) async fn read_images(
        &self,
        lighting: &Lighting,
    ) -> (image::GrayImage, image::GrayImage, image::GrayImage) {
        use crabslab::Slab;

        use crate::light::dequantize_depth_u32_to_f32;

        let tile_dimensions = self.tiling_descriptor.get().tile_grid_size();
        let slab = lighting.light_slab.read(..).await.unwrap();
        let tiling_descriptor_id_in_lighting = lighting
            .lighting_descriptor
            .get()
            .light_tiling_descriptor_id;
        let tiling_descriptor_id = self.tiling_descriptor.id();
        assert_eq!(tiling_descriptor_id_in_lighting, tiling_descriptor_id);
        let desc = slab.read(
            lighting
                .lighting_descriptor
                .get()
                .light_tiling_descriptor_id,
        );
        let should_be_len = tile_dimensions.x * tile_dimensions.y;
        if should_be_len != desc.tiles_array.len() as u32 {
            log::error!(
                "LightTilingDescriptor's tiles array is borked: {:?}\n\
                   expected {should_be_len} tiles\n\
                   tile_dimensions: {tile_dimensions}",
                desc.tiles_array,
            );
        }
        let mut mins_img = image::GrayImage::new(tile_dimensions.x, tile_dimensions.y);
        let mut maxs_img = image::GrayImage::new(tile_dimensions.x, tile_dimensions.y);
        let mut lights_img = image::GrayImage::new(tile_dimensions.x, tile_dimensions.y);
        slab.read_vec(desc.tiles_array)
            .into_iter()
            .enumerate()
            .for_each(|(i, tile)| {
                let x = i as u32 % tile_dimensions.x;
                let y = i as u32 / tile_dimensions.x;
                let min = dequantize_depth_u32_to_f32(tile.depth_min);
                let max = dequantize_depth_u32_to_f32(tile.depth_max);

                mins_img.get_pixel_mut(x, y).0[0] = crate::math::scaled_f32_to_u8(min);
                maxs_img.get_pixel_mut(x, y).0[0] = crate::math::scaled_f32_to_u8(max);
                lights_img.get_pixel_mut(x, y).0[0] = crate::math::scaled_f32_to_u8(
                    tile.next_light_index as f32 / tile.lights_array.len() as f32,
                );
            });

        (mins_img, maxs_img, lights_img)
    }
}

/// Parameters for tuning light tiling.
#[derive(Debug, Clone, Copy)]
pub struct LightTilingConfig {
    /// The size of each tile, in pixels.
    ///
    /// Default is `16`.
    pub tile_size: u32,
    /// The maximum number of lights per tile.
    ///
    /// Default is `32`.
    pub max_lights_per_tile: u32,
    /// The minimum illuminance, in lux.
    ///
    /// Used to determine the radius of illumination of a light,
    /// which is then used to determine if a light illuminates a tile.
    ///
    /// * Moonlight: < 1 lux.
    ///   - Full moon on a clear night: 0.25 lux.
    ///   - Quarter moon: 0.01 lux
    ///   - Starlight overcast moonless night sky: 0.0001 lux.
    /// * General indoor lighting: Around 100 to 300 lux.                                   
    /// * Office lighting: Typically around 300 to 500 lux.                                 
    /// * Reading or task lighting: Around 500 to 750 lux.                                  
    /// * Detailed work (e.g., drafting, surgery): 1000 lux or more.
    ///
    /// Default is `0.1`.
    pub minimum_illuminance: f32,
}

impl Default for LightTilingConfig {
    fn default() -> Self {
        LightTilingConfig {
            tile_size: 16,
            max_lights_per_tile: 32,
            minimum_illuminance: 0.1,
        }
    }
}

impl LightTiling<HybridArrayContainer> {
    /// Creates a new [`LightTiling`] struct with a [`HybridArray`] of tiles.
    pub fn new_hybrid(
        lighting: &Lighting,
        multisampled: bool,
        depth_texture_size: UVec2,
        config: LightTilingConfig,
    ) -> Self {
        log::trace!("creating LightTiling");
        let lighting_slab = lighting.slab_allocator();
        let runtime = lighting_slab.runtime();
        let desc = LightTilingDescriptor {
            depth_texture_size,
            tile_size: config.tile_size,
            minimum_illuminance_lux: config.minimum_illuminance,
            ..Default::default()
        };
        let tiling_descriptor = lighting_slab.new_value(desc);
        lighting.lighting_descriptor.modify(|desc| {
            desc.light_tiling_descriptor_id = tiling_descriptor.id();
        });
        log::trace!("created tiling descriptor: {tiling_descriptor:#?}");
        let tiled_size = desc.tile_grid_size();
        log::trace!("  grid size: {tiled_size}");
        let mut tiles = Vec::new();
        for _ in 0..tiled_size.x * tiled_size.y {
            let lights =
                lighting_slab.new_array(vec![Id::NONE; config.max_lights_per_tile as usize]);
            tiles.push(LightTile {
                lights_array: lights.array(),
                ..Default::default()
            });
        }
        let tiles = lighting_slab.new_array(tiles);
        tiling_descriptor.modify(|d| {
            let tiles_array = tiles.array();
            log::trace!("  setting tiles array: {tiles_array:?}");
            d.tiles_array = tiles_array;
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
        lighting: &Lighting,
        multisampled: bool,
        depth_texture_size: UVec2,
        config: LightTilingConfig,
    ) -> Self {
        // Note to self, I wish we had `fmap` here.
        let LightTiling {
            tiling_descriptor,
            tiles,
            bindgroup_creation_time,
            depth_texture_id,
            bindgroup_layout,
            bindgroup,
            clear_tiles_pipeline,
            compute_min_max_depth_pipeline,
            compute_bins_pipeline,
        } = LightTiling::new_hybrid(lighting, multisampled, depth_texture_size, config);
        Self {
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
