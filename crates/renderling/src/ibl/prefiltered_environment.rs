//! Pipeline for creating a prefiltered environment map from an existing
//! environment cubemap.

use crate::Uniform;
use renderling_shader::stage::GpuConstants;

pub fn create_pipeline_and_bindgroup(
    device: &wgpu::Device,
    constants: &Uniform<GpuConstants>,
    roughness: &Uniform<f32>,
    environment_texture: &crate::Texture,
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let label = Some("prefiltered environment");
    let bindgroup_layout_desc = wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    };
    let bg_layout = device.create_bind_group_layout(&bindgroup_layout_desc);
    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bg_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    constants.buffer().as_entire_buffer_binding(),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(
                    roughness.buffer().as_entire_buffer_binding(),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&environment_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&environment_texture.sampler),
            },
        ],
    });
    let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    let vertex_shader = device.create_shader_module(wgpu::include_spirv!(
        "../linkage/convolution-vertex_prefilter_environment_cubemap.spv"
    ));
    let fragment_shader = device.create_shader_module(wgpu::include_spirv!(
        "../linkage/convolution-fragment_prefilter_environment_cubemap.spv"
    ));
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("prefiltered environment"),
        layout: Some(&pp_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "convolution::vertex_prefilter_environment_cubemap",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 3 * std::mem::size_of::<f32>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x3
                ],
            }],
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            mask: !0,
            alpha_to_coverage_enabled: false,
            count: 1,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "convolution::fragment_prefilter_environment_cubemap",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    (pipeline, bindgroup)
}
