//! Various render nodes.
//!
//! A render node is simply a function whose input parameters are all
//! [`Edges`](moongraph::Edges), and whose result is
//! [`NodeResults`](moongraph::NodeResults).
//!
//! Render nodes are registered using [`Renderer::add_node`] or
//! [`Renderer::with_node`].
//!
//! See `moongraph`'s [`Node`] documentation for more info.
use std::{ops::Deref, sync::Arc};

use moongraph::*;
use snafu::prelude::*;

use crate::{
    linkage::ObjectDraw, BackgroundColor, BufferDimensions, Camera, Cameras, DepthTexture, Device,
    Frame, Id, ObjectData, Objects, CopiedTextureBuffer, Queue, RenderTarget, ScreenSize,
    UiPipeline, WgpuStateError, ForwardPipeline, Lights,
};

fn default_frame_texture_view(frame_texture: &wgpu::Texture) -> wgpu::TextureView {
    frame_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("WgpuState::default_frame_texture_view"),
        format: None,
        dimension: None,
        aspect: wgpu::TextureAspect::All,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
    })
}

pub struct FrameTextureView(Arc<wgpu::TextureView>);

impl Deref for FrameTextureView {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Create the next screen frame texture, frame texture view and depth texture.
pub fn create_frame(
    render_target: Read<RenderTarget>,
) -> Result<(Frame, FrameTextureView), WgpuStateError> {
    let frame = render_target.get_current_frame()?;
    let frame_view = default_frame_texture_view(frame.texture());
    Ok((frame, FrameTextureView(frame_view.into())))
}

/// Conduct a clear pass on the global frame and depth textures.
pub fn clear_frame_and_depth(
    (device, queue, frame_view, depth, color): (
        Read<Device>,
        Read<Queue>,
        Read<FrameTextureView>,
        Read<DepthTexture>,
        Read<BackgroundColor>,
    ),
) -> Result<(), WgpuStateError> {
    let depth_view = &depth.view;
    let [r, g, b, a] = color.0.to_array();
    let color = wgpu::Color {
        r: r.into(),
        g: g.into(),
        b: b.into(),
        a: a.into(),
    };
    crate::linkage::conduct_clear_pass(
        &device,
        &queue,
        Some("clear_frame_and_depth"),
        Some(&frame_view),
        Some(&depth_view),
        color,
    );
    Ok(())
}

/// Conduct a full render pass into the given textures using the given
/// camera and objects.
///
/// Helper for building render nodes that render objects w/ a particular
/// pipeline.
pub fn render_objects<'a>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipeline: &wgpu::RenderPipeline,
    frame_texture_view: &wgpu::TextureView,
    depth_texture_view: &wgpu::TextureView,
    default_material_uniform: &wgpu::BindGroup,
    material_bindgroup_index: u32,
    lights_uniform: Option<&wgpu::BindGroup>,
    light_bindgroup_index: Option<u32>,
    camera_uniform: &wgpu::BindGroup,
    camera_bindgroup_index: u32,
    objects: impl IntoIterator<Item = &'a ObjectData>,
) {
    log::trace!("render");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("render_camera_objects"),
    });

    let mut render_pass = crate::linkage::begin_render_pass(
        &mut encoder,
        Some("render_camera_objects"),
        pipeline,
        frame_texture_view,
        depth_texture_view,
    );

    // bind the lights to our shader
    if let Some(lights_uniform) = lights_uniform {
        render_pass.set_bind_group(light_bindgroup_index.unwrap(), lights_uniform, &[]);
    } else {
        log::warn!("no lights to bind");
    }
    // bind the camera to our shader
    render_pass.set_bind_group(camera_bindgroup_index, camera_uniform, &[]);

    for object in objects {
        if !object.inner.read().is_visible {
            continue;
        }
        if let Some(object) = object.as_shader_object() {
            let material = object.material.unwrap_or(default_material_uniform);
            // bind the object's material to our shader
            render_pass.set_bind_group(material_bindgroup_index, material, &[]);

            render_pass.set_vertex_buffer(0, object.mesh_buffer);
            render_pass.set_vertex_buffer(1, object.instances);
            // draw
            match &object.draw {
                ObjectDraw::Indexed {
                    index_buffer,
                    index_range,
                    base_vertex,
                    index_format,
                } => {
                    render_pass.set_index_buffer(*index_buffer, *index_format);
                    render_pass.draw_indexed(
                        index_range.clone(),
                        *base_vertex,
                        object.instances_range.clone(),
                    );
                }
                ObjectDraw::Default { vertex_range } => {
                    render_pass.draw(vertex_range.clone(), object.instances_range.clone());
                }
            }
        }
    }

    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
}

/// A buffer holding a copy of the last frame's buffer/texture.
pub struct PostRenderBuffer(pub CopiedTextureBuffer);

/// Render node that copies the current frame into a buffer.
#[derive(Edges)]
pub struct PostRenderBufferCreate {
    device: Read<Device>,
    queue: Read<Queue>,
    size: Read<ScreenSize>,
    frame: Read<Frame>,
}

impl PostRenderBufferCreate {
    /// Copies the current frame into a `PostRenderBuffer` resource.
    ///
    /// If rendering to a window surface, this should be called after rendering,
    /// before presentation.
    pub fn create(self) -> Result<(PostRenderBuffer,), WgpuStateError> {
        let ScreenSize { width, height } = *self.size;
        let dimensions = BufferDimensions::new(width as usize, height as usize);
        // The output buffer lets us retrieve the self as an array
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("screen capture buffer"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("post render screen capture encoder"),
            });
        let texture = self.frame.texture();
        // Copy the data from the surface texture to the buffer
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(
                        std::num::NonZeroU32::new(dimensions.padded_bytes_per_row as u32).unwrap(),
                    ),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: dimensions.width as u32,
                height: dimensions.height as u32,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok((PostRenderBuffer(CopiedTextureBuffer { dimensions, buffer }),))
    }
}

/// Consume and present the screen frame to the screen.
pub fn present(frame: Move<Frame>) -> Result<(), WgpuStateError> {
    let frame = frame.into();
    frame.present();
    Ok(())
}

#[derive(Debug, Snafu)]
pub enum RenderNodeError {
    #[snafu(display("Missing {id:?}"))]
    MissingCamera { id: Id<Camera> },
}

/// A render graph resource to use as the camera to render with.
pub struct UiRenderCamera(pub Id<Camera>);

pub fn ui_render(
    (device, queue, ui_pipeline, frame, depth_texture, rcam, cameras, objects): (
        Read<Device>,
        Read<Queue>,
        Read<UiPipeline>,
        Read<FrameTextureView>,
        Read<DepthTexture>,
        Read<UiRenderCamera>,
        Read<Cameras>,
        Read<Objects>,
    ),
) -> Result<(), RenderNodeError> {
    let camera_id = rcam.0.into();
    let camera_data = cameras
        .get(&camera_id)
        .with_context(|| MissingCameraSnafu { id: camera_id })?;
    let camera_objects = cameras.get_object_ids_sorted_by_distance_to_camera(&camera_id);
    let object_data = objects.iter_with_ids(camera_objects);
    render_objects(
        &device,
        &queue,
        &ui_pipeline.pipeline,
        &frame,
        &depth_texture.view,
        ui_pipeline.default_material_uniform.get_bindgroup(),
        ui_pipeline.bindgroup_index_config.material_bindgroup_index,
        None,
        None,
        &camera_data.bindgroup,
        ui_pipeline.bindgroup_index_config.camera_bindgroup_index,
        object_data,
    );
    Ok(())
}

/// A render graph resource to use as the camera to render with.
pub struct ForwardRenderCamera(pub Id<Camera>);

pub fn forward_render(
    (device, queue, pipeline, frame, depth_texture, rcam, lights, cameras, objects): (
        Read<Device>,
        Read<Queue>,
        Read<ForwardPipeline>,
        Read<FrameTextureView>,
        Read<DepthTexture>,
        Read<ForwardRenderCamera>,
        Read<Lights>,
        Read<Cameras>,
        Read<Objects>,
    ),
) -> Result<(), RenderNodeError> {
    let camera_id = rcam.0.into();
    let camera_data = cameras
        .get(&camera_id)
        .with_context(|| MissingCameraSnafu { id: camera_id })?;
    let object_data = objects.iter().flatten();
    render_objects(
        &device,
        &queue,
        &pipeline.pipeline,
        &frame,
        &depth_texture.view,
        pipeline.default_material_uniform.get_bindgroup(),
        pipeline.bindgroup_index_config.material_bindgroup_index,
        lights.uniform().map(|uniform| &uniform.bindgroup),
        Some(pipeline.bindgroup_index_config.light_bindgroup_index),
        &camera_data.bindgroup,
        pipeline.bindgroup_index_config.camera_bindgroup_index,
        object_data,
    );
    Ok(())
}
