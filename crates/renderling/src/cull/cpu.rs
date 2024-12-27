//! CPU side of compute culling.

use crabslab::{Array, Slab};
use glam::UVec2;
use snafu::{OptionExt, Snafu};
use std::sync::Arc;

use crate::{
    slab::{GpuArray, Hybrid, SlabAllocator, SlabAllocatorError},
    texture::Texture,
};

use super::DepthPyramidDescriptor;

#[derive(Debug, Snafu)]
pub enum CullingError {
    #[snafu(display(
        "Texture is not a depth texture, expected '{:?}' but saw '{seen:?}'",
        Texture::DEPTH_FORMAT
    ))]
    NotADepthTexture { seen: wgpu::TextureFormat },

    #[snafu(display("Missing depth pyramid mip {index}"))]
    MissingMip { index: usize },

    #[snafu(display("{source}"))]
    SlabError { source: SlabAllocatorError },

    #[snafu(display("Could not read mip {index}"))]
    ReadMip { index: usize },
}

impl From<SlabAllocatorError> for CullingError {
    fn from(source: SlabAllocatorError) -> Self {
        CullingError::SlabError { source }
    }
}

/// Computes frustum and occlusion culling on the GPU.
pub struct ComputeCulling {
    pipeline: wgpu::ComputePipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    bindgroup: Option<wgpu::BindGroup>,
    pub(crate) compute_depth_pyramid: ComputeDepthPyramid,
}

impl ComputeCulling {
    const LABEL: Option<&'static str> = Some("compute-culling");

    fn new_bindgroup(
        stage_slab_buffer: &wgpu::Buffer,
        hzb_slab_buffer: &wgpu::Buffer,
        indirect_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        stage_slab_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        hzb_slab_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        indirect_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    pub fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
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
            ],
        });
        let linkage = crate::linkage::compute_culling::linkage(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Self::LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Self::LABEL,
                    bind_group_layouts: &[&bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &linkage.module,
            entry_point: Some(linkage.entry_point),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        Self {
            pipeline,
            bindgroup_layout,
            bindgroup: None,
            compute_depth_pyramid: ComputeDepthPyramid::new(device, size, sample_count),
        }
    }

    pub fn invalidate_bindgroup(&mut self) {
        self.bindgroup = None;
    }

    fn get_bindgroup(
        &mut self,
        device: &wgpu::Device,
        stage_slab_buffer: &wgpu::Buffer,
        hzb_slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
    ) -> &wgpu::BindGroup {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::new_bindgroup(
                stage_slab_buffer,
                hzb_slab_buffer,
                indirect_draw_buffer,
                &self.bindgroup_layout,
                device,
            ));
        }
        // UNWRAP: safe because we just set it
        self.bindgroup.as_ref().unwrap()
    }

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        stage_slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
        indirect_draw_count: u32,
        depth_texture: &Texture,
    ) -> Result<(), CullingError> {
        // Compute the depth pyramid from last frame's depth buffer
        self.compute_depth_pyramid
            .run(device, queue, depth_texture)?;
        let (hzb_buffer, invalidate) = self
            .compute_depth_pyramid
            .depth_pyramid
            .slab
            .get_updated_buffer_and_check((
                device,
                queue,
                Self::LABEL,
                wgpu::BufferUsages::empty(),
            ));
        if invalidate {
            self.invalidate_bindgroup();
        }
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Self::LABEL,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            let bindgroup =
                self.get_bindgroup(device, stage_slab_buffer, &hzb_buffer, indirect_draw_buffer);
            compute_pass.set_bind_group(0, Some(bindgroup), &[]);
            compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        Ok(())
    }
}

pub struct DepthPyramid {
    slab: SlabAllocator<wgpu::Buffer>,
    desc: Hybrid<DepthPyramidDescriptor>,
    mip: GpuArray<Array<f32>>,
    mip_data: Vec<GpuArray<f32>>,
}

impl DepthPyramid {
    const LABEL: Option<&'static str> = Some("depth-pyramid");

    fn allocate(
        size: UVec2,
        desc: &Hybrid<DepthPyramidDescriptor>,
        slab: &SlabAllocator<wgpu::Buffer>,
    ) -> (Vec<GpuArray<f32>>, GpuArray<Array<f32>>) {
        let mip_levels = size.min_element().ilog2();
        let mip_data = (0..mip_levels)
            .map(|i| {
                let width = size.x >> i;
                let height = size.y >> i;
                slab.new_array(vec![0f32; (width * height) as usize])
                    .into_gpu_only()
            })
            .collect::<Vec<_>>();
        let mip = slab.new_array(mip_data.iter().map(|m| m.array()));
        desc.set(DepthPyramidDescriptor {
            size,
            mip_level: 0,
            mip: mip.array(),
        });
        (mip_data, mip.into_gpu_only())
    }

    pub fn new(size: UVec2) -> Self {
        let slab = SlabAllocator::default();
        let desc = slab.new_value(DepthPyramidDescriptor::default());
        let (mip_data, mip) = Self::allocate(size, &desc, &slab);

        Self {
            slab,
            desc,
            mip_data,
            mip,
        }
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: UVec2,
    ) -> (Arc<wgpu::Buffer>, bool) {
        log::info!("resizing depth pyramid to {size}");
        let mip = self.slab.new_array(vec![]);
        self.mip_data = vec![];
        self.desc.modify(|desc| desc.mip = mip.array());
        self.mip = mip.into_gpu_only();

        // Reclaim the dropped buffer slots
        let (_, should_invalidate_a) = self.slab.get_updated_buffer_and_check((
            device,
            queue,
            Self::LABEL,
            wgpu::BufferUsages::empty(),
        ));

        // Reallocate
        let (mip_data, mip) = Self::allocate(size, &self.desc, &self.slab);
        self.mip_data = mip_data;
        self.mip = mip;

        // Run upkeep one more time to sync the resize
        let (buffer, should_invalidate_b) = self.slab.get_updated_buffer_and_check((
            device,
            queue,
            Self::LABEL,
            wgpu::BufferUsages::empty(),
        ));
        (buffer, should_invalidate_a || should_invalidate_b)
    }

    pub fn size(&self) -> UVec2 {
        self.desc.get().size
    }

    pub async fn read_images(
        &self,
        ctx: &crate::Context,
    ) -> Result<Vec<image::GrayImage>, CullingError> {
        let size = self.size();
        let slab_data = self.slab.read(ctx, Self::LABEL, 0..).await?;
        let mut images = vec![];
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for (i, mip) in self.mip_data.iter().enumerate() {
            let depth_data: Vec<u8> = slab_data
                .read_vec(mip.array())
                .into_iter()
                .map(|depth: f32| {
                    if i == 0 {
                        min = min.min(depth);
                        max = max.max(depth);
                    }
                    crate::color::f32_to_u8(depth)
                })
                .collect();
            log::info!("min: {min}");
            log::info!("max: {max}");
            let width = size.x >> i;
            let height = size.y >> i;
            let image = image::GrayImage::from_raw(width, height, depth_data)
                .context(ReadMipSnafu { index: i })?;
            images.push(image);
        }
        Ok(images)
    }
}

/// Copies the depth texture to the top of the depth pyramid.
struct ComputeCopyDepth {
    pipeline: wgpu::ComputePipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    sample_count: u32,
    bindgroup: Option<wgpu::BindGroup>,
}

impl ComputeCopyDepth {
    const LABEL: Option<&'static str> = Some("compute-occlusion-copy-depth-to-pyramid");

    fn create_bindgroup_layout(device: &wgpu::Device, sample_count: u32) -> wgpu::BindGroupLayout {
        if sample_count > 1 {
            log::info!(
                "creating bindgroup layout with {sample_count} multisampled depth for {}",
                Self::LABEL.unwrap()
            );
        } else {
            log::info!(
                "creating bindgroup layout without multisampling for {}",
                Self::LABEL.unwrap()
            );
        }
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
                // slab
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // previous_mip: DepthPyramidImage
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: sample_count > 1,
                    },
                    count: None,
                },
            ],
        })
    }

    fn create_pipeline(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        multisampled: bool,
    ) -> wgpu::ComputePipeline {
        let linkage = if multisampled {
            log::info!("creating multisampled shader for {}", Self::LABEL.unwrap());
            crate::linkage::compute_copy_depth_to_pyramid_multisampled::linkage(device)
        } else {
            log::info!(
                "creating shader without multisampling for {}",
                Self::LABEL.unwrap()
            );
            crate::linkage::compute_copy_depth_to_pyramid::linkage(device)
        };
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Self::LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Self::LABEL,
                    bind_group_layouts: &[bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &linkage.module,
            entry_point: Some(linkage.entry_point),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        })
    }

    fn create_bindgroup(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        pyramid_desc_buffer: &wgpu::Buffer,
        depth_texture_view: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        pyramid_desc_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(depth_texture_view),
                },
            ],
        })
    }

    pub fn new(device: &wgpu::Device, sample_count: u32) -> Self {
        let bindgroup_layout = Self::create_bindgroup_layout(device, sample_count);
        let pipeline = Self::create_pipeline(device, &bindgroup_layout, sample_count > 1);
        Self {
            pipeline,
            bindgroup: None,
            bindgroup_layout,
            sample_count,
        }
    }

    pub fn invalidate(&mut self) {
        self.bindgroup = None;
    }

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pyramid: &DepthPyramid,
        depth_texture: &Texture,
    ) {
        let size = pyramid.desc.modify(|desc| {
            desc.mip_level = 0;
            desc.size
        });
        let (slab_buffer, slab_buffer_is_new) = pyramid.slab.get_updated_buffer_and_check((
            device,
            queue,
            Self::LABEL,
            wgpu::BufferUsages::empty(),
        ));

        if slab_buffer_is_new {
            self.bindgroup = None;
        }

        let sample_count = depth_texture.texture.sample_count();
        let sample_count_mismatch = sample_count != self.sample_count;

        if sample_count_mismatch {
            log::info!(
                "sample count changed from {} to {}, updating {} bindgroup layout and pipeline",
                self.sample_count,
                sample_count,
                Self::LABEL.unwrap()
            );
            self.sample_count = sample_count;
            self.bindgroup_layout = Self::create_bindgroup_layout(device, sample_count);
            self.pipeline = Self::create_pipeline(device, &self.bindgroup_layout, sample_count > 1);
            self.bindgroup = None;
        }

        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::create_bindgroup(
                device,
                &self.bindgroup_layout,
                &slab_buffer,
                &depth_texture.view,
            ));
        }
        // UNWRAP: safe because we just set it above^
        let bindgroup = self.bindgroup.as_ref().unwrap();
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Self::LABEL,
                ..Default::default()
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, Some(bindgroup), &[]);
            let x = size.x / 32 + 1;
            let y = size.y / 32 + 1;
            let z = 1;
            compute_pass.dispatch_workgroups(x, y, z);
        }
        queue.submit(Some(encoder.finish()));
    }
}

/// Downsamples the depth texture from one mip to the next.
struct ComputeDownsampleDepth {
    pipeline: wgpu::ComputePipeline,
    bindgroup: Option<wgpu::BindGroup>,
    bindgroup_layout: wgpu::BindGroupLayout,
}

impl ComputeDownsampleDepth {
    const LABEL: Option<&'static str> = Some("compute-occlusion-downsample-depth");

    fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
                // slab
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }

    fn create_pipeline(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::ComputePipeline {
        let linkage = crate::linkage::compute_downsample_depth_pyramid::linkage(device);
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Self::LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Self::LABEL,
                    bind_group_layouts: &[bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &linkage.module,
            entry_point: Some(linkage.entry_point),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        })
    }

    fn create_bindgroup(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        pyramid_desc_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Self::LABEL,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    pyramid_desc_buffer.as_entire_buffer_binding(),
                ),
            }],
        })
    }

    pub fn new(device: &wgpu::Device) -> Self {
        let bindgroup_layout = Self::create_bindgroup_layout(device);
        let pipeline = Self::create_pipeline(device, &bindgroup_layout);
        Self {
            pipeline,
            bindgroup: None,
            bindgroup_layout,
        }
    }

    pub fn invalidate(&mut self) {
        self.bindgroup = None;
    }

    pub fn run(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, pyramid: &DepthPyramid) {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::create_bindgroup(
                device,
                &self.bindgroup_layout,
                &pyramid.slab.get_updated_buffer((
                    device,
                    queue,
                    Self::LABEL,
                    wgpu::BufferUsages::empty(),
                )),
            ));
        }
        for i in 1..pyramid.mip_data.len() {
            log::trace!("downsampling to mip {i}");
            // UNWRAP: safe because we just set it above^
            let bindgroup = self.bindgroup.as_ref().unwrap();
            // Update the mip_level we're operating on.
            let size = pyramid.desc.modify(|desc| {
                desc.mip_level = i as u32;
                desc.size
            });
            // Sync the change.
            let (_, should_invalidate) = pyramid.slab.get_updated_buffer_and_check((
                device,
                queue,
                Self::LABEL,
                wgpu::BufferUsages::empty(),
            ));
            debug_assert!(!should_invalidate, "pyramid slab should never resize here");

            let mut encoder = device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });
            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Self::LABEL,
                    ..Default::default()
                });
                compute_pass.set_pipeline(&self.pipeline);
                compute_pass.set_bind_group(0, Some(bindgroup), &[]);
                let w = size.x >> i;
                let h = size.y >> i;
                let x = w / 32 + 1;
                let y = h / 32 + 1;
                let z = 1;
                compute_pass.dispatch_workgroups(x, y, z);
            }
            queue.submit(Some(encoder.finish()));
        }
    }
}

/// Computes occlusion culling on the GPU.
pub struct ComputeDepthPyramid {
    sample_count: u32,
    pub(crate) depth_pyramid: DepthPyramid,
    compute_copy_depth: ComputeCopyDepth,
    compute_downsample_depth: ComputeDownsampleDepth,
}

impl ComputeDepthPyramid {
    const LABEL: Option<&'static str> = Some("compute-depth-pyramid");

    pub fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
        let depth_pyramid = DepthPyramid::new(size);
        let compute_copy_depth = ComputeCopyDepth::new(device, sample_count);
        let compute_downsample_depth = ComputeDownsampleDepth::new(device);
        Self {
            depth_pyramid,
            compute_copy_depth,
            compute_downsample_depth,
            sample_count,
        }
    }

    /// Invalidate the bindgroup.
    ///
    /// Call this if the depth texture is regenerated.
    pub fn invalidate(&mut self) {
        self.compute_copy_depth.invalidate();
    }

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        depth_texture: &Texture,
    ) -> Result<(), CullingError> {
        let sample_count = depth_texture.texture.sample_count();
        if sample_count != self.sample_count {
            log::warn!(
                "sample_count changed from {} to {sample_count}, invalidating",
                self.sample_count
            );
            self.compute_copy_depth.invalidate();
            self.sample_count = sample_count;
        }

        let extent = depth_texture.texture.size();
        let size = UVec2::new(extent.width, extent.height);
        let (_, should_invalidate) = if size != self.depth_pyramid.size() {
            log::warn!("depth texture size changed, invalidating");
            self.compute_copy_depth.invalidate();
            self.compute_downsample_depth.invalidate();
            self.depth_pyramid.resize(device, queue, size)
        } else {
            self.depth_pyramid.slab.get_updated_buffer_and_check((
                device,
                queue,
                Self::LABEL,
                wgpu::BufferUsages::empty(),
            ))
        };
        if should_invalidate {
            self.compute_copy_depth.invalidate();
            self.compute_downsample_depth.invalidate();
        }

        self.compute_copy_depth
            .run(device, queue, &self.depth_pyramid, depth_texture);

        self.compute_downsample_depth
            .run(device, queue, &self.depth_pyramid);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        bvol::BoundingSphere, cull::DepthPyramidDescriptor, draw::DrawIndirectArgs,
        math::hex_to_vec4, prelude::*,
    };
    use crabslab::GrowableSlab;
    use glam::{Mat4, Quat, UVec2, UVec3, Vec2, Vec3, Vec4};

    #[test]
    fn occlusion_culling_sanity() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
        let camera_position = Vec3::new(0.0, 9.0, 9.0);
        let camera = stage.new_value(Camera::new(
            Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 1.0, 24.0),
            Mat4::look_at_rh(camera_position, Vec3::ZERO, Vec3::Y),
        ));
        let geometry = stage.new_array(crate::test::gpu_cube_vertices());
        let transform = stage.new_value(Transform {
            scale: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -std::f32::consts::FRAC_PI_4),
            ..Default::default()
        });
        let cube = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            transform_id: transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&cube);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        frame.present();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::save("cull/pyramid/frame.png", img);
        frame.present();

        let depth_texture = stage.get_depth_texture();
        let depth_img = depth_texture.read_image().unwrap();
        img_diff::save("cull/pyramid/depth.png", depth_img);

        let pyramid_images = futures_lite::future::block_on(
            stage
                .draw_calls
                .read()
                .unwrap()
                .drawing_strategy
                .as_indirect()
                .unwrap()
                .read_hzb_images(&ctx),
        )
        .unwrap();
        for (i, img) in pyramid_images.into_iter().enumerate() {
            img_diff::save(&format!("cull/pyramid/mip_{i}.png"), img);
        }
    }

    #[test]
    fn depth_pyramid_descriptor_sanity() {
        let mut slab = vec![];
        let size = UVec2::new(64, 32);
        let mip_levels = size.min_element().ilog2();
        let desc_id = slab.allocate::<DepthPyramidDescriptor>();
        let mips_array = slab.allocate_array::<Array<f32>>(mip_levels as usize);
        let mip_data_arrays = (0..mip_levels)
            .map(|i| {
                let w = size.x >> i;
                let h = size.y >> i;
                let len = (w * h) as usize;
                log::info!("allocating {len} f32s for mip {i} of size {w}x{h}");
                let array = slab.allocate_array::<f32>(len);
                let mut data: Vec<f32> = vec![];
                for _y in 0..h {
                    for x in 0..w {
                        data.push(x as f32);
                    }
                }
                slab.write_array(array, &data);
                array
            })
            .collect::<Vec<_>>();
        slab.write_array(mips_array, &mip_data_arrays);
        let desc = DepthPyramidDescriptor {
            size: UVec2::new(64, 32),
            mip_level: 0,
            mip: mips_array,
        };
        slab.write(desc_id, &desc);

        // Test that `id_of_depth` returns the correct value.
        for mip_level in 0..mip_levels {
            let size = desc.size_at(mip_level);
            log::info!("mip {mip_level} is size {size}");
            for y in 0..size.y {
                for x in 0..size.x {
                    let id = desc.id_of_depth(mip_level, UVec2::new(x, y), &slab);
                    let depth = slab.read(id);
                    assert_eq!(x as f32, depth, "depth should be x value");
                }
            }
        }
    }

    #[test]
    fn occlusion_culling_debugging() {
        let ctx = Context::headless(128, 128);
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_bloom(false)
            .with_background_color(Vec4::splat(1.0));
        let camera = {
            let fovy = std::f32::consts::FRAC_PI_4;
            let aspect = 1.0;
            let znear = 0.1;
            let zfar = 100.0;
            let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
            // Camera is looking straight down Z, towards the origin with Y up
            let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 10.0), Vec3::ZERO, Vec3::Y);
            stage.new_value(Camera::new(projection, view))
        };

        let save_render = |s: &str| {
            let frame = ctx.get_next_frame().unwrap();
            stage.render(&frame.view());
            let img = frame.read_image().unwrap();
            img_diff::save(&format!("cull/debugging_{s}.png"), img);
            frame.present();
        };

        // A hashmap to hold renderlet ids to their names.
        let mut names = HashMap::<Id<Renderlet>, String>::default();

        // Add four yellow cubes in each corner
        let _ycubes = [
            (Vec2::new(-1.0, 1.0), "top_left"),
            (Vec2::new(1.0, 1.0), "top_right"),
            (Vec2::new(1.0, -1.0), "bottom_right"),
            (Vec2::new(-1.0, -1.0), "bottom_left"),
        ]
        .map(|(offset, suffix)| {
            let yellow = hex_to_vec4(0xFFE6A5FF);
            let transform = stage.new_value(Transform {
                // move it back behind the purple cube
                translation: (offset * 10.0).extend(-20.0),
                // scale it up since it's a unit cube
                scale: Vec3::splat(2.0),
                ..Default::default()
            });
            let vertices = stage.new_array(crate::math::unit_cube().into_iter().map(|(p, n)| {
                Vertex::default()
                    .with_position(p)
                    .with_normal(n)
                    .with_color(yellow)
            }));
            let renderlet = stage.new_value(Renderlet {
                camera_id: camera.id(),
                vertices_array: vertices.array(),
                transform_id: transform.id(),
                bounds: BoundingSphere::new(Vec3::ZERO, Vec3::splat(0.5).length()),
                ..Default::default()
            });
            stage.add_renderlet(&renderlet);
            names.insert(renderlet.id(), format!("yellow_cube_{suffix}"));
            (renderlet, transform, vertices)
        });

        save_render("0_yellow_cubes");

        // We'll add a golden floor
        let _floor = {
            let golden = hex_to_vec4(0xFFBF61FF);
            let transform = stage.new_value(Transform {
                // flip it so it's facing up, like a floor should be
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                // move it down and back a bit
                translation: Vec3::new(0.0, -5.0, -10.0),
                // scale it up, since it's a unit quad
                scale: Vec3::new(100.0, 100.0, 1.0),
            });
            let vertices = stage.new_array(
                crate::math::UNIT_QUAD_CCW
                    .map(|p| Vertex::default().with_position(p).with_color(golden)),
            );
            let renderlet = stage.new_value(Renderlet {
                camera_id: camera.id(),
                vertices_array: vertices.array(),
                transform_id: transform.id(),
                bounds: BoundingSphere::new(Vec3::ZERO, Vec2::splat(0.5).length()),
                ..Default::default()
            });
            stage.add_renderlet(&renderlet);
            names.insert(renderlet.id(), "floor".into());
            (renderlet, transform, vertices)
        };

        save_render("1_floor");

        // Add a green cube
        let _gcube = {
            let green = hex_to_vec4(0x8ABFA3FF);
            let transform = stage.new_value(Transform {
                // move it back behind the purple cube
                translation: Vec3::new(0.0, 0.0, -10.0),
                // scale it up since it's a unit cube
                scale: Vec3::splat(5.0),
                ..Default::default()
            });
            let vertices = stage.new_array(crate::math::unit_cube().into_iter().map(|(p, n)| {
                Vertex::default()
                    .with_position(p)
                    .with_normal(n)
                    .with_color(green)
            }));
            let renderlet = stage.new_value(Renderlet {
                camera_id: camera.id(),
                vertices_array: vertices.array(),
                transform_id: transform.id(),
                bounds: BoundingSphere::new(Vec3::ZERO, Vec3::splat(0.5).length()),
                ..Default::default()
            });
            stage.add_renderlet(&renderlet);
            names.insert(renderlet.id(), "green_cube".into());
            (renderlet, transform, vertices)
        };

        save_render("2_green_cube");

        // And a purple cube
        let _pcube = {
            let purple = hex_to_vec4(0x605678FF);
            let transform = stage.new_value(Transform {
                // move it back a bit
                translation: Vec3::new(0.0, 0.0, -3.0),
                // scale it up since it's a unit cube
                scale: Vec3::splat(5.0),
                ..Default::default()
            });
            let vertices = stage.new_array(crate::math::unit_cube().into_iter().map(|(p, n)| {
                Vertex::default()
                    .with_position(p)
                    .with_normal(n)
                    .with_color(purple)
            }));
            let renderlet = stage.new_value(Renderlet {
                camera_id: camera.id(),
                vertices_array: vertices.array(),
                transform_id: transform.id(),
                bounds: BoundingSphere::new(Vec3::ZERO, Vec3::splat(0.5).length()),
                ..Default::default()
            });
            stage.add_renderlet(&renderlet);
            names.insert(renderlet.id(), "purple_cube".into());
            (renderlet, transform, vertices)
        };

        // Do two renders, because depth pyramid operates on depth data one frame
        // behind
        save_render("3_purple_cube");
        save_render("3_purple_cube");

        // save the normalized depth image
        let mut depth_img = stage.get_depth_texture().read_image().unwrap();
        img_diff::normalize_gray_img(&mut depth_img);
        img_diff::save("cull/debugging_4_depth.png", depth_img);

        // save the normalized pyramid images
        let pyramid_images = futures_lite::future::block_on(
            stage
                .draw_calls
                .read()
                .unwrap()
                .drawing_strategy
                .as_indirect()
                .unwrap()
                .read_hzb_images(&ctx),
        )
        .unwrap();
        for (i, mut img) in pyramid_images.into_iter().enumerate() {
            img_diff::normalize_gray_img(&mut img);
            img_diff::save(&format!("cull/debugging_pyramid_mip_{i}.png"), img);
        }

        // The stage's slab, which contains the `Renderlet`s and their `BoundingSphere`s
        let stage_slab =
            futures_lite::future::block_on(stage.read(&ctx, Some("read stage"), ..)).unwrap();
        let draw_calls = stage.draw_calls.read().unwrap();
        let indirect_draws = draw_calls.drawing_strategy.as_indirect().unwrap();
        // The HZB slab, which contains a `DepthPyramidDescriptor` at index 0, and all the
        // pyramid's mips
        let depth_pyramid_slab = futures_lite::future::block_on(
            indirect_draws
                .compute_culling
                .compute_depth_pyramid
                .depth_pyramid
                .slab
                .read(&ctx, Some("read hzb desc"), ..),
        )
        .unwrap();
        // The indirect draw buffer
        let mut args_slab =
            futures_lite::future::block_on(indirect_draws.slab.read(&ctx, Some("read args"), ..))
                .unwrap();
        let args: &mut [DrawIndirectArgs] = bytemuck::cast_slice_mut(&mut args_slab);
        // Number of `DrawIndirectArgs` in the `args` buffer.
        let num_draw_calls = draw_calls.draw_count();

        // Print our names so we know what we're working with
        let mut pnames = names.iter().collect::<Vec<_>>();
        pnames.sort();
        for (id, name) in pnames.into_iter() {
            log::info!("id: {id:?}, name: {name}");
        }

        for i in 0..num_draw_calls as u32 {
            let renderlet_id = Id::<Renderlet>::new(args[i as usize].first_instance);
            let name = names.get(&renderlet_id).unwrap();
            if name != "green_cube" {
                continue;
            }
            log::info!("");
            log::info!("name: {name}");
            crate::cull::compute_culling(
                &stage_slab,
                &depth_pyramid_slab,
                args,
                UVec3::new(i, 0, 0),
            );
        }
    }
}
