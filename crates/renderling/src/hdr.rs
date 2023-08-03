//! High definition rendering types and techniques.
use moongraph::*;
use renderling_shader::tonemapping::TonemapConstants;

use crate::{
    math::Vec4, DepthTexture, Device, Queue,
    RenderTarget, ScreenSize, Uniform, WgpuStateError, frame::FrameTextureView, BackgroundColor,
};

/// A texture, tonemapping pipeline and uniform used for high dynamic range
/// rendering.
///
/// See https://learnopengl.com/Advanced-Lighting/HDR.
pub struct HdrSurface {
    pub texture: crate::Texture,
    pub texture_bindgroup: wgpu::BindGroup,
    pub tonemapping_pipeline: wgpu::RenderPipeline,
    pub constants: Uniform<TonemapConstants>,
}

impl HdrSurface {
    pub fn create_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) -> crate::Texture {
        crate::Texture::new_with(
            &device,
            &queue,
            Some("HdrRenderBuffer"),
            Some(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING),
            Some({
                device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                })
            }),
            wgpu::TextureFormat::Rgba16Float,
            4,
            1,
            width,
            height,
            1,
            &[],
        )
    }

    pub fn create_texture_bindgroup(
        device: &wgpu::Device,
        texture: &crate::Texture,
    ) -> wgpu::BindGroup {
        let hdr_texture_layout = scene_hdr_surface_bindgroup_layout(&device);
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("HdrSurface texture bindgroup"),
            layout: &hdr_texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        })
    }
}

fn scene_hdr_surface_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("hdr buffer bindgroup"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    })
}

pub fn create_hdr_render_surface(
    (device, queue, size, target): (
        View<Device>,
        View<Queue>,
        View<ScreenSize>,
        View<RenderTarget>,
    ),
) -> Result<(HdrSurface,), WgpuStateError> {
    let (constants, constants_layout) = Uniform::new_and_layout(
        &device,
        TonemapConstants::default(),
        wgpu::BufferUsages::UNIFORM,
        wgpu::ShaderStages::FRAGMENT,
    );
    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };
    let texture = HdrSurface::create_texture(&device, &queue, size.width, size.height);
    let label = Some("hdr tonemapping");
    let vertex_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/vertex_tonemapping.spv"));
    let fragment_shader =
        device.create_shader_module(wgpu::include_spirv!("linkage/fragment_tonemapping.spv"));
    let hdr_texture_layout = scene_hdr_surface_bindgroup_layout(&device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&hdr_texture_layout, &constants_layout],
        push_constant_ranges: &[],
    });
    let tonemapping_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "vertex_tonemapping",
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
        depth_stencil: None,
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "fragment_tonemapping",
            targets: &[Some(wgpu::ColorTargetState {
                format: target.format(),
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    Ok((HdrSurface {
        texture_bindgroup: HdrSurface::create_texture_bindgroup(&device, &texture),
        texture,
        tonemapping_pipeline,
        constants,
    },))
}

/// Update the `HdrSurface` uniforms.
pub fn hdr_surface_update(
    (queue, mut hdr_surface): (View<Queue>, ViewMut<HdrSurface>),
) -> Result<(), WgpuStateError> {
    hdr_surface.constants.update(&queue);
    Ok(())
}

/// Conduct a clear pass on the window surface, the hdr surface and the depth
/// texture.
pub fn clear_surface_hdr_and_depth(
    (device, queue, frame, hdr, depth, color): (
        View<Device>,
        View<Queue>,
        View<FrameTextureView>,
        View<HdrSurface>,
        View<DepthTexture>,
        View<BackgroundColor>,
    ),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    let c: Vec4 = if frame.format.is_srgb() {
        color.0.powf(2.2)
    } else {
        color.0
    };
    let color = wgpu::Color {
        r: c.x.into(),
        g: c.y.into(),
        b: c.z.into(),
        a: c.w.into(),
    };
    crate::frame::conduct_clear_pass(
        &device,
        &queue,
        Some("clear_frame_and_depth"),
        vec![&frame.view, &hdr.texture.view],
        Some(&depth_view),
        color,
    );
    Ok(())
}
