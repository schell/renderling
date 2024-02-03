//! Frame creation and clearing.
//!
//! Contains graph nodes for creating and clearing frames, as well as a
//! `PostRenderBuffer` resource that holds a copy of the last frame's buffer.
use std::{ops::Deref, sync::Arc};

use moongraph::*;

use crate::{
    math::Vec4, BackgroundColor, CopiedTextureBuffer, DepthTexture, Device, Frame, Queue,
    RenderTarget, ScreenSize, WgpuStateError,
};

pub fn default_frame_texture_view(
    frame_texture: &wgpu::Texture,
) -> (wgpu::TextureView, wgpu::TextureFormat) {
    let format = frame_texture.format().add_srgb_suffix();
    let view = frame_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("WgpuState::default_frame_texture_view"),
        format: Some(format),
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    });
    (view, format)
}

pub struct FrameTextureView {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}

impl Deref for FrameTextureView {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

/// Create the next screen frame texture, frame texture view and depth texture.
pub fn create_frame(
    render_target: View<RenderTarget>,
) -> Result<(Frame, FrameTextureView), WgpuStateError> {
    let frame = render_target.get_current_frame()?;
    let (frame_view, format) = default_frame_texture_view(frame.texture());
    Ok((
        frame,
        FrameTextureView {
            view: frame_view.into(),
            format,
        },
    ))
}

/// Perform a clearing render pass on a frame and/or a depth texture.
pub fn conduct_clear_pass(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: Option<&str>,
    frame_views: Vec<&wgpu::TextureView>,
    depth_view: Option<&wgpu::TextureView>,
    clear_color: wgpu::Color,
) {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("renderling clear pass"),
    });

    let frame_views = frame_views
        .into_iter()
        .map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })
        })
        .collect::<Vec<_>>();
    let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &frame_views,
        depth_stencil_attachment: depth_view.map(|view| wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    });
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
}

/// Render graph node to conduct a clear pass on the global frame and depth
/// textures.
pub fn clear_frame_and_depth(
    (device, queue, frame_view, depth, color): (
        View<Device>,
        View<Queue>,
        View<FrameTextureView>,
        View<DepthTexture>,
        View<BackgroundColor>,
    ),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    let color: Vec4 = if frame_view.format.is_srgb() {
        color.0.powf(2.2)
    } else {
        color.0
    };
    let [r, g, b, a] = color.to_array();
    let color = wgpu::Color {
        r: r.into(),
        g: g.into(),
        b: b.into(),
        a: a.into(),
    };
    let frames = vec![frame_view.view.as_ref()];
    conduct_clear_pass(
        &device,
        &queue,
        Some("clear_frame_and_depth"),
        frames,
        Some(&depth_view),
        color,
    );
    Ok(())
}

/// Conduct a clear pass on **only the depth texture**.
pub fn clear_depth(
    (device, queue, depth): (View<Device>, View<Queue>, View<DepthTexture>),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    conduct_clear_pass(
        &device,
        &queue,
        Some("clear_depth"),
        vec![],
        Some(&depth_view),
        Default::default(),
    );
    Ok(())
}

// pub fn create_hdr_render_buffer

/// A buffer holding a copy of the last frame's buffer/texture.
pub struct PostRenderBuffer(pub CopiedTextureBuffer);

/// Render node that copies the current frame into a buffer.
#[derive(Edges)]
pub struct PostRenderBufferCreate {
    device: View<Device>,
    queue: View<Queue>,
    size: View<ScreenSize>,
    frame: View<Frame>,
}

/// Copies the current frame into a `PostRenderBuffer` resource.
///
/// If rendering to a window surface, this should be called after rendering,
/// before presentation.
pub fn copy_frame_to_post(
    create: PostRenderBufferCreate,
) -> Result<(PostRenderBuffer,), WgpuStateError> {
    let ScreenSize { width, height } = *create.size;
    let copied_texture_buffer =
        create
            .frame
            .copy_to_buffer(&create.device, &create.queue, width, height);
    Ok((PostRenderBuffer(copied_texture_buffer),))
}

/// Consume and present the screen frame to the screen.
pub fn present(frame: Move<Frame>) -> Result<(), WgpuStateError> {
    let frame = frame.into();
    frame.present();
    Ok(())
}
