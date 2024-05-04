//! `wgpu` state management.
//!
//! Contains stuff for initializing [`Renderling`]s and doing GPU things like
//! building pipelines, etc.
use snafu::prelude::*;
use std::sync::Arc;

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
}

pub enum RenderTarget {
    Surface {
        surface: wgpu::Surface<'static>,
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

    pub fn is_headless(&self) -> bool {
        match self {
            RenderTarget::Surface { .. } => false,
            RenderTarget::Texture { .. } => true,
        }
    }
}

fn limits(adapter: &wgpu::Adapter) -> wgpu::Limits {
    if cfg!(target_arch = "wasm32") {
        wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
    } else {
        wgpu::Limits::default()
    }
}

async fn adapter<'window>(
    instance: &wgpu::Instance,
    compatible_surface: Option<&wgpu::Surface<'window>>,
) -> Result<wgpu::Adapter, WgpuStateError> {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface,
            force_fallback_adapter: false,
        })
        .await
        .context(CannotCreateAdaptorSnafu)?;
    let info = adapter.get_info();
    log::trace!(
        "using adapter: '{}' backend:{:?} driver:'{}'",
        info.name,
        info.backend,
        info.driver
    );
    Ok(adapter)
}

async fn device(
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::INDIRECT_FIRST_INSTANCE
                        | wgpu::Features::MULTI_DRAW_INDIRECT
                        // this one is a funny requirement, it seems it is needed if using storage buffers in
                        // vertex shaders, even if those shaders are read-only
                        | wgpu::Features::VERTEX_WRITABLE_STORAGE, //| wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                //// when debugging rust-gpu shader miscompilation it's nice to have this
                //| wgpu::Features::SPIRV_SHADER_PASSTHROUGH
                required_limits: limits(&adapter),
                label: None,
            },
            None, // Trace path
        )
        .await
}

pub fn new_default_instance() -> wgpu::Instance {
    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        // TODO: change wgpu backend bit to just PRIMARY
        backends: wgpu::Backends::all(),
        ..Default::default()
    })
}

pub async fn new_windowed_adapter_device_queue(
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
    window: impl Into<wgpu::SurfaceTarget<'static>>,
) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget), WgpuStateError> {
    let surface = instance
        .create_surface(window)
        .map_err(|e| WgpuStateError::CreateSurface { source: e })?;
    let adapter = adapter(&instance, Some(&surface)).await?;
    let surface_caps = surface.get_capabilities(&adapter);
    let fmt = if surface_caps
        .formats
        .contains(&wgpu::TextureFormat::Rgba8UnormSrgb)
    {
        wgpu::TextureFormat::Rgba8UnormSrgb
    } else {
        surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0])
    };
    let view_fmts = if fmt.is_srgb() {
        vec![]
    } else {
        vec![fmt.add_srgb_suffix()]
    };
    log::debug!("surface capabilities: {surface_caps:#?}");
    let mut surface_config = surface
        .get_default_config(&adapter, width, height)
        .context(IncompatibleSurfaceSnafu)?;
    surface_config.view_formats = view_fmts;
    let (device, queue) = device(&adapter).await.context(CannotRequestDeviceSnafu)?;
    surface.configure(&device, &surface_config);
    let target = RenderTarget::Surface {
        surface,
        surface_config,
    };
    Ok((adapter, device, queue, target))
}

pub async fn new_headless_device_queue_and_target(
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget), WgpuStateError> {
    let adapter = adapter(&instance, None).await?;
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
    let (device, queue) = device(&adapter).await.context(CannotRequestDeviceSnafu)?;
    let texture = Arc::new(device.create_texture(&texture_desc));
    let target = RenderTarget::Texture { texture };
    Ok((adapter, device, queue, target))
}
