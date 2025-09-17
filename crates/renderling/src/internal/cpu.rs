//! Internal CPU stuff.
use std::sync::Arc;

use snafu::{OptionExt, ResultExt};

use crate::context::{
    CannotCreateAdaptorSnafu, CannotRequestDeviceSnafu, ContextError, IncompatibleSurfaceSnafu,
    RenderTarget, RenderTargetInner,
};

/// Create a new [`wgpu::Adapter`].
pub async fn adapter(
    instance: &wgpu::Instance,
    compatible_surface: Option<&wgpu::Surface<'_>>,
) -> Result<wgpu::Adapter, ContextError> {
    log::trace!(
        "creating adapter for a {} context",
        if compatible_surface.is_none() {
            "headless"
        } else {
            "surface-based"
        }
    );
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface,
            force_fallback_adapter: false,
        })
        .await
        .context(CannotCreateAdaptorSnafu)?;

    log::info!("Adapter selected: {:?}", adapter.get_info());
    let info = adapter.get_info();
    log::info!(
        "using adapter: '{}' backend:{:?} driver:'{}'",
        info.name,
        info.backend,
        info.driver
    );
    Ok(adapter)
}

/// Create a new [`wgpu::Device`].
pub async fn device(
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    let wanted_features = wgpu::Features::INDIRECT_FIRST_INSTANCE
        | wgpu::Features::MULTI_DRAW_INDIRECT
        //// when debugging rust-gpu shader miscompilation it's nice to have this
        //| wgpu::Features::SPIRV_SHADER_PASSTHROUGH
        // this one is a funny requirement, it seems it is needed if using storage buffers in
        // vertex shaders, even if those shaders are read-only
        | wgpu::Features::VERTEX_WRITABLE_STORAGE
        | wgpu::Features::CLEAR_TEXTURE;
    let supported_features = adapter.features();
    let required_features = wanted_features.intersection(supported_features);
    let unsupported_features = wanted_features.difference(supported_features);
    if !unsupported_features.is_empty() {
        log::error!("requested but unsupported features: {unsupported_features:#?}");
        log::warn!("requested and supported features: {supported_features:#?}");
    }
    let limits = adapter.limits();
    log::info!("adapter limits: {limits:#?}");
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_features,
            required_limits: adapter.limits(),
            label: None,
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        })
        .await
}

/// Create a new instance.
///
/// This is for internal use. It is not necessary to create your own `wgpu`
/// instance to use this library.
pub fn new_instance(backends: Option<wgpu::Backends>) -> wgpu::Instance {
    log::info!(
        "creating instance - available backends: {:#?}",
        wgpu::Instance::enabled_backend_features()
    );
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let backends = backends.unwrap_or(wgpu::Backends::PRIMARY);
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        let adapters = instance.enumerate_adapters(backends);
        log::trace!("available adapters: {adapters:#?}");
    }

    instance
}

/// Create a new suite of `wgpu` machinery using a window or canvas.
///
/// ## Note
/// This function is used internally.
pub async fn new_windowed_adapter_device_queue(
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
    window: impl Into<wgpu::SurfaceTarget<'static>>,
) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget), ContextError> {
    let surface = instance
        .create_surface(window)
        .map_err(|e| ContextError::CreateSurface { source: e })?;
    let adapter = adapter(instance, Some(&surface)).await?;
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
    log::info!("surface capabilities: {surface_caps:#?}");
    let mut surface_config = surface
        .get_default_config(&adapter, width, height)
        .context(IncompatibleSurfaceSnafu)?;
    surface_config.view_formats = view_fmts;
    let (device, queue) = device(&adapter).await.context(CannotRequestDeviceSnafu)?;
    surface.configure(&device, &surface_config);
    let target = RenderTarget(RenderTargetInner::Surface {
        surface,
        surface_config,
    });
    Ok((adapter, device, queue, target))
}

/// Create a new suite of `wgpu` machinery that renders to a texture.
///
/// ## Note
/// This function is used internally.
pub async fn new_headless_device_queue_and_target(
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue, RenderTarget), ContextError> {
    let adapter = adapter(instance, None).await?;
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
    let target = RenderTarget(RenderTargetInner::Texture { texture });
    Ok((adapter, device, queue, target))
}
