use moongraph::{GraphError, Move, View};

use crate::{frame::FrameTextureView, Device, HdrSurface, Queue};

/// Conducts the HDR tone mapping, writing the HDR surface texture to the (most
/// likely) sRGB window surface.
pub fn tonemapping(
    (device, queue, frame, hdr_frame): (
        View<Device>,
        View<Queue>,
        View<FrameTextureView>,
        View<HdrSurface>,
    ),
) -> Result<(), GraphError> {
    log::trace!("tonemapping");
    let label = Some("tonemapping");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: None,
    });
    render_pass.set_pipeline(&hdr_frame.tonemapping_pipeline);
    render_pass.set_bind_group(0, &hdr_frame.bindgroup, &[]);
    render_pass.draw(0..6, 0..1);
    drop(render_pass);

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}
