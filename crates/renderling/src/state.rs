//! `wgpu` state management.
//!
//! Contains stuff for initializing [`Renderling`]s and doing GPU things like
//! building pipelines, etc.
use snafu::prelude::*;
use std::sync::Arc;

use crate::{BufferDimensions, CopiedTextureBuffer};

#[derive(Debug, Snafu)]
pub enum WgpuStateError {
    #[snafu(display("cannot create adaptor"))]
    CannotCreateAdaptor,

    #[snafu(display("cannot request device: {}", source))]
    CannotRequestDevice { source: wgpu::RequestDeviceError },

    #[snafu(display("surface is incompatible with adapter"))]
    IncompatibleSurface,

    #[snafu(display("could not create surface: {}", source))]
    CreateSurface { source: wgpu::CreateSurfaceError },

    #[snafu(display("missing surface texture: {}", source))]
    MissingSurfaceTexture { source: wgpu::SurfaceError },

    #[snafu(display("{}", source))]
    Texture { source: crate::TextureError },

    #[snafu(display("missing the target frame - call WgpuState::prepare_target_frame first"))]
    MissingTargetFrame,

    #[snafu(display("could not map buffer"))]
    CouldNotMapBuffer { source: wgpu::BufferAsyncError },

    #[snafu(display("could not convert image buffer"))]
    CouldNotConvertImageBuffer,
}

pub enum RenderTarget {
    Surface {
        surface: wgpu::Surface,
        surface_config: wgpu::SurfaceConfiguration,
    },
    Texture {
        texture: Arc<wgpu::Texture>,
    },
}

impl From<wgpu::Texture> for RenderTarget {
    fn from(value: wgpu::Texture) -> Self {
        RenderTarget::Texture {
            texture: Arc::new(value),
        }
    }
}

impl RenderTarget {
    pub fn resize(&mut self, width: u32, height: u32, device: &wgpu::Device) {
        match self {
            RenderTarget::Surface {
                surface,
                surface_config,
            } => {
                surface_config.width = width;
                surface_config.height = height;
                surface.configure(&device, &surface_config);
            }
            RenderTarget::Texture { texture } => {
                let usage = texture.usage();
                let format = texture.format();
                let texture_desc = wgpu::TextureDescriptor {
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    label: Some("RenderTarget texture"),
                    view_formats: &[],
                };
                *texture = Arc::new(device.create_texture(&texture_desc));
            }
        }
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        match self {
            RenderTarget::Surface { surface_config, .. } => surface_config.format,
            RenderTarget::Texture { texture } => texture.format(),
        }
    }

    /// Get the current render target frame.
    ///
    /// Errs if the render target is a surface and there was an error getting
    /// the next swapchain texture.
    pub fn get_current_frame(&self) -> Result<Frame, WgpuStateError> {
        match self {
            RenderTarget::Surface { surface, .. } => {
                let surface_texture = surface
                    .get_current_texture()
                    .context(MissingSurfaceTextureSnafu)?;
                Ok(Frame::Surface(surface_texture))
            }
            RenderTarget::Texture { texture, .. } => Ok(Frame::Texture(texture.clone())),
        }
    }
}

/// Abstracts over window and texture render targets.
///
/// Either a [`SurfaceTexture`] or a [`Texture`].
pub enum Frame {
    Surface(wgpu::SurfaceTexture),
    Texture(Arc<wgpu::Texture>),
}

impl Frame {
    /// Returns the underlying texture of this target.
    pub fn texture(&self) -> &wgpu::Texture {
        match self {
            Frame::Surface(s) => &s.texture,
            Frame::Texture(t) => &t,
        }
    }

    pub fn copy_to_buffer(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) -> CopiedTextureBuffer {
        let dimensions = BufferDimensions::new(4, 1, width as usize, height as usize);
        // The output buffer lets us retrieve the self as an array
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("RenderTarget::copy_to_buffer"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("post render screen capture encoder"),
        });
        let texture = self.texture();
        // Copy the data from the surface texture to the buffer
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(dimensions.padded_bytes_per_row as u32),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: dimensions.width as u32,
                height: dimensions.height as u32,
                depth_or_array_layers: 1,
            },
        );

        queue.submit(std::iter::once(encoder.finish()));

        CopiedTextureBuffer {
            dimensions,
            buffer,
            format: texture.format(),
        }
    }

    /// If self is `TargetFrame::Surface` this presents the surface frame.
    ///
    /// If self is a `TargetFrame::Texture` this is a noop.
    pub fn present(self) {
        match self {
            Frame::Surface(s) => s.present(),
            Frame::Texture(_) => {}
        }
    }
}

pub type CreateSurfaceFn<'a> =
    Box<dyn FnOnce(&wgpu::Instance) -> Result<wgpu::Surface, WgpuStateError> + 'a>;

pub async fn new_adapter_device_queue_and_target<'a>(
    width: u32,
    height: u32,
    create_surface: Option<
        impl FnOnce(&wgpu::Instance) -> Result<wgpu::Surface, WgpuStateError> + 'a,
    >,
) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget) {
    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let backends = wgpu::Backends::all();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        dx12_shader_compiler: wgpu::Dx12Compiler::default(),
    });

    fn limits(adapter: &wgpu::Adapter) -> wgpu::Limits {
        if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
        } else {
            wgpu::Limits::default()
        }
    }

    async fn adapter(
        instance: &wgpu::Instance,
        compatible_surface: Option<&wgpu::Surface>,
    ) -> wgpu::Adapter {
        instance.enumerate_adapters(wgpu::Backends::all()).for_each(|adapter| {
            log::trace!("adapter: {:#?}", adapter.get_info());
        });
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface,
                force_fallback_adapter: false,
            })
            .await
            .unwrap()
    }

    async fn device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    // TODO: WASM: Review device features
                    features: wgpu::Features::INDIRECT_FIRST_INSTANCE
                        | wgpu::Features::MULTI_DRAW_INDIRECT
                        // this one is a funny requirement, it seems it is needed if using storage buffers in
                        // vertex shaders, even if those shaders are read-only
                        | wgpu::Features::VERTEX_WRITABLE_STORAGE
                        | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                        | wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
                    limits: limits(&adapter),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap()
    }

    if let Some(create_surface) = create_surface {
        let surface = (create_surface)(&instance).unwrap();
        let adapter = adapter(&instance, Some(&surface)).await;
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        let (device, queue) = device(&adapter).await;
        surface.configure(&device, &surface_config);
        let target = RenderTarget::Surface {
            surface,
            surface_config,
        };
        (adapter, device, queue, target)
    } else {
        let adapter = adapter(&instance, None).await;
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
            view_formats: &[],
        };
        let (device, queue) = device(&adapter).await;
        let texture = Arc::new(device.create_texture(&texture_desc));
        let target = RenderTarget::Texture { texture };
        (adapter, device, queue, target)
    }
}
