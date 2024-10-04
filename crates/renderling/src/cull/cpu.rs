//! CPU side of compute culling.

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
            entry_point: linkage.entry_point,
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
            compute_pass.set_bind_group(0, bindgroup, &[]);
            compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
}

const OCCLUSION_LABEL: Option<&str> = Some("compute-occlusion-culling");

/// Computes occlusion culling on the GPU.
pub struct OcclusionCulling {
    pipeline: wgpu::ComputePipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    bindgroup: Option<wgpu::BindGroup>,
    depth_pyramid: Texture,
}

impl OcclusionCulling {
    fn new_bindgroup(
        slab_buffer: &wgpu::Buffer,
        indirect_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: OCCLUSION_LABEL,
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
            label: OCCLUSION_LABEL,
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
        let linkage = crate::linkage::compute_occlusion_culling::linkage(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: OCCLUSION_LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: OCCLUSION_LABEL,
                    bind_group_layouts: &[&bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &linkage.module,
            entry_point: linkage.entry_point,
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
            label: OCCLUSION_LABEL,
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: OCCLUSION_LABEL,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            let bindgroup = self.get_bindgroup(device, slab_buffer, indirect_draw_buffer);
            compute_pass.set_bind_group(0, bindgroup, &[]);
            compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
}
