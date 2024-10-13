//! CPU side of compute culling.

use crabslab::{Array, Slab};
use glam::UVec2;
use snafu::{OptionExt, Snafu};
use std::sync::Arc;

use crate::{
    camera::Camera,
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

const FRUSTUM_LABEL: Option<&str> = Some("compute-frustum-culling");

/// Computes furstum culling on the GPU.
pub struct FrustumCulling {
    pipeline: wgpu::ComputePipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    bindgroup: Option<wgpu::BindGroup>,
}

impl FrustumCulling {
    fn new_bindgroup(
        slab_buffer: &wgpu::Buffer,
        indirect_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: FRUSTUM_LABEL,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        indirect_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    pub fn new(device: &wgpu::Device) -> Self {
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: FRUSTUM_LABEL,
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
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let linkage = crate::linkage::compute_frustum_culling::linkage(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: FRUSTUM_LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: FRUSTUM_LABEL,
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
        }
    }

    pub fn invalidate_bindgroup(&mut self) {
        self.bindgroup = None;
    }

    fn get_bindgroup(
        &mut self,
        device: &wgpu::Device,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
    ) -> &wgpu::BindGroup {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::new_bindgroup(
                slab_buffer,
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
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
        indirect_draw_count: u32,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: FRUSTUM_LABEL,
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: FRUSTUM_LABEL,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            let bindgroup = self.get_bindgroup(device, slab_buffer, indirect_draw_buffer);
            compute_pass.set_bind_group(0, Some(bindgroup), &[]);
            compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
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
        let width_levels = size.x.ilog2();
        let height_levels = size.y.ilog2();
        let mip_levels = width_levels.min(height_levels);
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
        camera: &Camera,
    ) -> Result<Vec<image::DynamicImage>, CullingError> {
        let size = self.size();
        let slab_data = self
            .slab
            .read(ctx.get_device(), ctx.get_queue(), Self::LABEL, 0..)
            .await?;
        let mut images = vec![];
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for (i, mip) in self.mip_data.iter().enumerate() {
            let depth_data: Vec<f32> = slab_data
                .read_vec(mip.array())
                .into_iter()
                .map(|depth| {
                    if i == 0 {
                        min = min.min(depth);
                        max = max.max(depth);
                    }
                    camera.linearize_depth_value(depth)
                    // depth
                })
                .collect();
            log::info!("min: {min}");
            log::info!("max: {max}");
            let width = size.x >> i;
            let height = size.y >> i;
            let image: image::ImageBuffer<image::Luma<f32>, Vec<f32>> =
                image::ImageBuffer::from_raw(width, height, depth_data)
                    .context(ReadMipSnafu { index: i })?;
            images.push(image::DynamicImage::from(image));
        }
        Ok(images)
    }
}

/// Copies the depth texture to the top of the depth pyramid.
struct ComputeCopyDepth {
    pipeline: wgpu::ComputePipeline,
    bindgroup: Option<wgpu::BindGroup>,
    bindgroup_layout: wgpu::BindGroupLayout,
}

impl ComputeCopyDepth {
    const LABEL: Option<&'static str> = Some("compute-occlusion-copy-depth-to-pyramid");

    fn create_bindgroup_layout(device: &wgpu::Device, sample_count: u32) -> wgpu::BindGroupLayout {
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
    ) -> wgpu::ComputePipeline {
        let linkage = crate::linkage::compute_copy_depth_to_pyramid::linkage(device);
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

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: UVec2,
        pyramid_desc_buffer: &wgpu::Buffer,
        depth_texture: &wgpu::TextureView,
    ) {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::create_bindgroup(
                device,
                &self.bindgroup_layout,
                pyramid_desc_buffer,
                depth_texture,
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

/// Computes occlusion culling on the GPU.
pub struct OcclusionCulling {
    sample_count: u32,
    pub(crate) depth_pyramid: DepthPyramid,
    compute_copy_depth: ComputeCopyDepth,
}

impl OcclusionCulling {
    const LABEL: Option<&'static str> = Some("compute-occlusion-culling");

    pub fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
        let depth_pyramid = DepthPyramid::new(size);
        let compute_copy_depth = ComputeCopyDepth::new(device, sample_count);
        Self {
            depth_pyramid,
            compute_copy_depth,
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
        // slab_buffer: &wgpu::Buffer,
        // indirect_draw_buffer: &wgpu::Buffer,
        // indirect_draw_count: u32,
        depth_texture: &Texture,
    ) -> Result<(), CullingError> {
        let sample_count = depth_texture.texture.sample_count();
        if sample_count != self.sample_count {
            log::warn!("sample_count changed, invalidating");
            self.invalidate();
            // let (bindgroup_layout, pipeline) =
            //     Self::create_bindgroup_layout_and_pipeline(device,
            // sample_count); self.bindgroup_layout =
            // bindgroup_layout; self.pipeline = pipeline;
        }

        let extent = depth_texture.texture.size();
        let size = UVec2::new(extent.width, extent.height);
        let (depth_pyramid_buffer, should_invalidate) = if size != self.depth_pyramid.size() {
            log::warn!("depth texture size changed, invalidating");
            self.invalidate();
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
        }

        self.compute_copy_depth.run(
            device,
            queue,
            size,
            &depth_pyramid_buffer,
            &depth_texture.view,
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use glam::{Mat4, Quat, Vec3, Vec4};


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
                .occlusion_culling
                .depth_pyramid
                .read_images(&ctx, &camera.get()),
        )
        .unwrap();
        for (i, img) in pyramid_images.into_iter().enumerate() {
            img_diff::save(&format!("cull/pyramid/mip_{i}.png"), img);
        }
    }
}
