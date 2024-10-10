//! CPU side of compute culling.

use glam::UVec2;
use snafu::{OptionExt, Snafu};

use crate::{
    slab::{Hybrid, SlabAllocator},
    texture::Texture,
};

use super::PyramidDescriptor;

#[derive(Debug, Snafu)]
pub enum CullingError {
    #[snafu(display(
        "Texture is not a depth texture, expected '{:?}' but saw '{seen:?}'",
        Texture::DEPTH_FORMAT
    ))]
    NotADepthTexture { seen: wgpu::TextureFormat },

    #[snafu(display("Missing depth pyramid mip {index}"))]
    MissingMip { index: usize },
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
                // desc: PyramidDescriptor
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
                // current_mip: DepthPyramidImageMut
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::R32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
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
        mip_view: &wgpu::TextureView,
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
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(mip_view),
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
        mip_view: &wgpu::TextureView,
    ) {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::create_bindgroup(
                device,
                &self.bindgroup_layout,
                pyramid_desc_buffer,
                depth_texture,
                mip_view,
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

// pub struct DepthPyramid {
//     size: UVec2,
//     depth_and_top_pyramid_bindgroup: Option<wgpu::BindGroup>,
//     downsample_pipeline: wgpu::ComputePipeline,
//     bindgroup_layout: wgpu::BindGroupLayout,
//     slab: SlabAllocator<wgpu::Buffer>,
//     pyramid_cfg: Gpu<PyramidDescriptor>,
// }

// impl DepthPyramid {
//     pub fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
//         let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: DEPTH_PYRAMID_LABEL,
//             entries: &[
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: wgpu::ShaderStages::COMPUTE,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Storage { read_only: false },
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                     count: None,
//                 },
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: wgpu::ShaderStages::COMPUTE,
//                     ty: wgpu::BindingType::Texture {
//                         sample_type: wgpu::TextureSampleType::Float { filterable: false },
//                         view_dimension: wgpu::TextureViewDimension::D2,
//                         multisampled: sample_count > 1,
//                     },
//                     count: None,
//                 },
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 2,
//                     visibility: wgpu::ShaderStages::COMPUTE,
//                     ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
//                     count: None,
//                 },
//             ],
//         });

//         let slab = SlabAllocator::default();
//         let width_levels = size.x.ilog2();
//         let height_levels = size.y.ilog2();
//         let mip_level_count = width_levels.min(height_levels);
//         let mip_data = (0..mip_level_count)
//             .map(|mip_level| {
//                 let w = size.x >> mip_level;
//                 let h = size.y >> mip_level;
//                 slab.new_array(vec![0.0f32; w as usize * h as usize])
//                     .into_gpu_only()
//             })
//             .collect::<Vec<_>>();
//         let mips = slab
//             .new_array(mip_data.iter().map(|h| h.array()))
//             .into_gpu_only();
//         let mip_level = slab.new_value(0u32);
//         let pyramid_cfg = slab
//             .new_value(PyramidDescriptor { size, mip_level: 0 })
//             .into_gpu_only();

//         let linkage = crate::linkage::compute_downsample_depth_pyramid::linkage(device);
//         let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: DEPTH_PYRAMID_LABEL,
//             bind_group_layouts: &[&bindgroup_layout],
//             push_constant_ranges: &[],
//         });
//         let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//             label: DEPTH_PYRAMID_LABEL,
//             layout: Some(&pp_layout),
//             module: &linkage.module,
//             entry_point: Some(&linkage.entry_point),
//             compilation_options: wgpu::PipelineCompilationOptions::default(),
//             cache: None,
//         });

//         Self {
//             size,
//             downsample_pipeline: pipeline,
//             bindgroup_layout,
//             slab,
//             view_bindgroup: None,
//             mip_data,
//             mips,
//             mip_level,
//             pyramid_cfg,
//         }
//     }

//     pub fn invalidate(&mut self) {
//         self.view_bindgroup = None;
//     }

//     fn get_bindgroup(
//         &mut self,
//         device: &wgpu::Device,
//         slab_buffer: &wgpu::Buffer,
//         depth_texture: &Texture,
//     ) -> Arc<DepthViewAndBindgroup> {
//         fn create_downsample_bindgroup(
//             device: &wgpu::Device,
//             layout: &wgpu::BindGroupLayout,
//             slab_buffer: &wgpu::Buffer,
//             depth_texture: &Texture,
//         ) -> DepthViewAndBindgroup {
//             let view = depth_texture
//                 .texture
//                 .create_view(&wgpu::TextureViewDescriptor {
//                     label: Some("depth-pyramid-view"),
//                     format: Some(wgpu::TextureFormat::R32Float),
//                     dimension: Some(wgpu::TextureViewDimension::D2),
//                     aspect: wgpu::TextureAspect::All,
//                     ..Default::default()
//                 });
//             let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
//                 label: DEPTH_PYRAMID_LABEL,
//                 layout,
//                 entries: &[
//                     wgpu::BindGroupEntry {
//                         binding: 0,
//                         resource: wgpu::BindingResource::Buffer(
//                             slab_buffer.as_entire_buffer_binding(),
//                         ),
//                     },
//                     wgpu::BindGroupEntry {
//                         binding: 1,
//                         resource: wgpu::BindingResource::TextureView(&view),
//                     },
//                     wgpu::BindGroupEntry {
//                         binding: 2,
//                         resource: wgpu::BindingResource::Sampler(&depth_texture.sampler),
//                     },
//                 ],
//             });
//             DepthViewAndBindgroup { view, bindgroup }
//         }
//         if self.view_bindgroup.is_none() {
//             self.view_bindgroup = Some(
//                 create_downsample_bindgroup(
//                     device,
//                     &self.bindgroup_layout,
//                     slab_buffer,
//                     depth_texture,
//                 )
//                 .into(),
//             );
//         }
//         // UNWRAP: safe because we just set it
//         let bg = self.view_bindgroup.as_ref().unwrap();
//         bg.clone()
//     }

//     /// Generate mip maps.
//     ///
//     /// # Errs
//     /// Errors if the texture's format doesn't match the generator format.
//     pub fn downsample(
//         &mut self,
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         depth_texture: &Texture,
//     ) {
//         let (slab_buffer, slab_buffer_was_recreated) = self.slab.get_updated_buffer_and_check((
//             device,
//             queue,
//             Some("depth-pyramid-downsample upkeep"),
//             wgpu::BufferUsages::empty(),
//         ));

//         if slab_buffer_was_recreated {
//             self.invalidate();
//         }
//         let dvab = self.get_bindgroup(device, &slab_buffer, depth_texture);

//         // Now run through all mips downsampling one to the next
//         for i in 0..self.mip_data.len() {
//             self.mip_level.set(i as u32);
//             let _ = self.slab.upkeep((
//                 device,
//                 queue,
//                 Some("depth-pyramid-downsample-set-mip"),
//                 wgpu::BufferUsages::empty(),
//             ));

//             let mut encoder =
//                 device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

//             {
//                 let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
//                     label: Some(&format!("depth-pyramid-downsample-{i}")),
//                     ..Default::default()
//                 });

//                 compute_pass.set_pipeline(&self.downsample_pipeline);
//                 compute_pass.set_bind_group(0, Some(&dvab.bindgroup), &[]);
//                 compute_pass.dispatch_workgroups(self.size.x / 32 + 1, self.size.y / 32 + 1, 1);
//             }

//             queue.submit(std::iter::once(encoder.finish()));
//         }
//     }
// }

struct PyramidMip {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    bindgroup: Option<wgpu::BindGroup>,
}

impl PyramidMip {
    pub fn new(device: &wgpu::Device, mip_level: u32, size: UVec2) -> Self {
        let size = wgpu::Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        };
        let mip_string = format!("depth-pyramid-mip-{mip_level}");
        let label = Some(mip_string.as_str());
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label,
            ..Default::default()
        });
        Self {
            texture,
            view,
            bindgroup: None,
        }
    }
}

/// Computes occlusion culling on the GPU.
pub struct OcclusionCulling {
    sample_count: u32,
    compute_copy_depth: ComputeCopyDepth,
    depth_pyramid_desc: Hybrid<PyramidDescriptor>,
    slab: SlabAllocator<wgpu::Buffer>,
    mips: Vec<PyramidMip>,
    // pipeline: wgpu::ComputePipeline,
    // bindgroup_layout: wgpu::BindGroupLayout,
    // bindgroup: Option<wgpu::BindGroup>,
    // depth_pyramid: Option<DepthPyramid>,
}

impl OcclusionCulling {
    const LABEL: Option<&str> = Some("compute-occlusion-culling");

    // fn new_bindgroup(
    //     slab_buffer: &wgpu::Buffer,
    //     indirect_buffer: &wgpu::Buffer,
    //     layout: &wgpu::BindGroupLayout,
    //     device: &wgpu::Device,
    // ) -> wgpu::BindGroup {
    //     device.create_bind_group(&wgpu::BindGroupDescriptor {
    //         label: OCCLUSION_LABEL,
    //         layout,
    //         entries: &[
    //             wgpu::BindGroupEntry {
    //                 binding: 0,
    //                 resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
    //             },
    //             wgpu::BindGroupEntry {
    //                 binding: 1,
    //                 resource: wgpu::BindingResource::Buffer(
    //                     indirect_buffer.as_entire_buffer_binding(),
    //                 ),
    //             },
    //         ],
    //     })
    // }

    // fn create_bindgroup_layout_and_pipeline(
    //     device: &wgpu::Device,
    //     sample_count: u32,
    // ) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    //     let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    //         label: OCCLUSION_LABEL,
    //         entries: &[
    //             wgpu::BindGroupLayoutEntry {
    //                 binding: 0,
    //                 visibility: wgpu::ShaderStages::COMPUTE,
    //                 ty: wgpu::BindingType::Buffer {
    //                     ty: wgpu::BufferBindingType::Storage { read_only: false },
    //                     has_dynamic_offset: false,
    //                     min_binding_size: None,
    //                 },
    //                 count: None,
    //             },
    //             wgpu::BindGroupLayoutEntry {
    //                 binding: 1,
    //                 visibility: wgpu::ShaderStages::COMPUTE,
    //                 ty: wgpu::BindingType::Texture {
    //                     sample_type: wgpu::TextureSampleType::Depth,
    //                     view_dimension: wgpu::TextureViewDimension::D2,
    //                     multisampled: sample_count > 1,
    //                 },
    //                 count: None,
    //             },
    //             wgpu::BindGroupLayoutEntry {
    //                 binding: 2,
    //                 visibility: wgpu::ShaderStages::COMPUTE,
    //                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
    //                 count: None,
    //             },
    //         ],
    //     });
    //     let linkage = crate::linkage::compute_occlusion_culling::linkage(device);
    //     let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
    //         label: OCCLUSION_LABEL,
    //         layout: Some(
    //             &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    //                 label: OCCLUSION_LABEL,
    //                 bind_group_layouts: &[&bindgroup_layout],
    //                 push_constant_ranges: &[],
    //             }),
    //         ),
    //         module: &linkage.module,
    //         entry_point: Some(linkage.entry_point),
    //         compilation_options: wgpu::PipelineCompilationOptions::default(),
    //         cache: None,
    //     });
    //     (bindgroup_layout, pipeline)
    // }

    pub fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
        let default_sample_count = 1;
        let compute_copy_depth = ComputeCopyDepth::new(device, sample_count);
        let slab = SlabAllocator::default();
        let depth_pyramid_desc = slab.new_value(PyramidDescriptor { size, mip_level: 0 });
        // let (bindgroup_layout, pipeline) =
        //     Self::create_bindgroup_layout_and_pipeline(device, default_sample_count);
        Self {
            compute_copy_depth,
            sample_count: default_sample_count,
            slab,
            depth_pyramid_desc,
            mips: vec![],
        }
    }

    /// Invalidate the bindgroup.
    ///
    /// Call this if the depth texture is regenerated.
    pub fn invalidate(&mut self) {
        // self.bindgroup = None;
        self.compute_copy_depth.invalidate();
        self.mips = vec![];
    }

    // fn get_bindgroup(
    //     &mut self,
    //     device: &wgpu::Device,
    //     slab_buffer: &wgpu::Buffer,
    //     indirect_draw_buffer: &wgpu::Buffer,
    // ) -> &wgpu::BindGroup {
    //     if self.bindgroup.is_none() {
    //         self.bindgroup = Some(Self::new_bindgroup(
    //             slab_buffer,
    //             indirect_draw_buffer,
    //             &self.bindgroup_layout,
    //             device,
    //         ));
    //     }
    //     // UNWRAP: safe because we just set it
    //     self.bindgroup.as_ref().unwrap()
    // }

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
            self.invalidate();
            // let (bindgroup_layout, pipeline) =
            //     Self::create_bindgroup_layout_and_pipeline(device, sample_count);
            // self.bindgroup_layout = bindgroup_layout;
            // self.pipeline = pipeline;
        }

        let extent = depth_texture.texture.size();
        let size = UVec2::new(extent.width, extent.height);
        let mut depth_pyramid_desc = self.depth_pyramid_desc.get();
        if depth_pyramid_desc.size != size {
            self.invalidate();
            depth_pyramid_desc.size = size;
            self.depth_pyramid_desc.set(depth_pyramid_desc);
        }

        let (depth_pyramid_desc_buffer, should_invalidate) =
            self.slab.get_updated_buffer_and_check((
                device,
                queue,
                Self::LABEL,
                wgpu::BufferUsages::empty(),
            ));
        debug_assert_eq!(
            false, should_invalidate,
            "Depth pyramid descriptor buffer should never resize"
        );

        if self.mips.is_empty() {
            let width_levels = size.x.ilog2();
            let height_levels = size.y.ilog2();
            let mip_levels = width_levels.min(height_levels);
            for i in 0..mip_levels {
                let width = size.x >> i;
                let height = size.y >> i;
                let mip = PyramidMip::new(device, i, UVec2::new(width, height));
                self.mips.push(mip);
            }
        }
        let mip = self
            .mips
            .first()
            .context(MissingMipSnafu { index: 0usize })?;

        self.compute_copy_depth.run(
            device,
            queue,
            size,
            &depth_pyramid_desc_buffer,
            &depth_texture.view,
            &mip.view,
        );

        // if self.depth_pyramid.is_none() {
        //     log::info!("occlusion culling - generating depth pyramid");
        //     let extent = depth_texture.texture.size();
        //     let size = UVec2::new(extent.width, extent.height);
        //     let sample_count = depth_texture.texture.sample_count();
        //     self.depth_pyramid = Some(DepthPyramid::new(device, size, sample_count));
        // }
        // // UNWRAP: safe because we just set it ^
        // let depth_pyramid = self.depth_pyramid.as_mut().unwrap();
        // depth_pyramid.downsample(device, queue, depth_texture);

        // let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //     label: OCCLUSION_LABEL,
        // });
        // {
        //     let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        //         label: OCCLUSION_LABEL,
        //         timestamp_writes: None,
        //     });
        //     compute_pass.set_pipeline(&self.pipeline);
        //     let bindgroup = self.get_bindgroup(device, slab_buffer, indirect_draw_buffer);
        //     compute_pass.set_bind_group(0, bindgroup, &[]);
        //     compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        // }
        // queue.submit(Some(encoder.finish()));
        Ok(())
    }
}
