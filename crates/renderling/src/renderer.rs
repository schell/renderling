//! Builds the UI pipeline and manages resources.
use glam::Vec4;
use moongraph::TypeKey;
use snafu::prelude::*;
use std::{ops::Deref, sync::Arc};

use crate::{
    hdr::HdrSurface, CreateSurfaceFn, Graph, RenderTarget, Scene, SceneBuilder, Stage,
    TextureError, UiDrawObject, UiScene, UiSceneBuilder, View, ViewMut, WgpuStateError,
};

#[derive(Debug, Snafu)]
pub enum RenderlingError {
    #[snafu(display("{}", source))]
    Texture { source: crate::TextureError },

    #[snafu(display("missing the target frame - call WgpuState::prepare_target_frame first"))]
    MissingTargetFrame,

    #[snafu(display("could not map buffer"))]
    CouldNotMapBuffer { source: wgpu::BufferAsyncError },

    #[snafu(display("could not convert image buffer"))]
    CouldNotConvertImageBuffer,

    #[snafu(display("missing the current frame"))]
    RenderTargetMissingFrame,

    #[snafu(display(
        "missing a material uniform, have you called Renderling::update at least once?"
    ))]
    MissingDefaultMaterial,

    #[snafu(display("missing a camera, this should not have happened"))]
    MissingCamera,

    #[cfg(feature = "gltf")]
    #[snafu(display("gltf import failed: {}", source))]
    GltfImport { source: gltf::Error },

    //#[snafu(display("could not create scene: {}", source))]
    // Scene { source: crate::GltfError },
    #[snafu(display("missing resource"))]
    Resource { key: TypeKey },

    #[snafu(display("{source}"))]
    Graph { source: moongraph::GraphError },

    //#[snafu(display("{source}"))]
    // Lights { source: crate::light::LightsError },

    //#[snafu(display("{source}"))]
    // Object { source: crate::object::ObjectError },
    #[snafu(display(
        "Missing PostRenderBuffer resource. Ensure a node that creates PostRenderBuffer (like \
         PostRenderbufferCreate) is present in the graph: {source}"
    ))]
    MissingPostRenderBuffer { source: moongraph::GraphError },

    #[snafu(display("Timeout while waiting for a screengrab"))]
    ScreenGrabTimeout { source: TextureError },

    #[snafu(display("{source}"))]
    State { source: WgpuStateError },
}

impl From<WgpuStateError> for RenderlingError {
    fn from(source: WgpuStateError) -> Self {
        Self::State { source }
    }
}

/// A thread-safe, clonable wrapper around `wgpu::Device`.
#[derive(Clone)]
pub struct Device(pub Arc<wgpu::Device>);

impl Deref for Device {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A thread-safe, clonable wrapper around `wgpu::Queue`.
#[derive(Clone)]
pub struct Queue(pub Arc<wgpu::Queue>);

impl Deref for Queue {
    type Target = wgpu::Queue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A thread-safe, clonable wrapper around `wgpu::Adapter`.
#[derive(Clone)]
pub struct Adapter(pub Arc<wgpu::Adapter>);

impl Deref for Adapter {
    type Target = wgpu::Adapter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A type for the screen/frame size.
#[derive(Clone, Copy, Debug)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

/// The global depth texture.
pub struct DepthTexture(crate::Texture);

impl Deref for DepthTexture {
    type Target = crate::Texture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The global background color.
pub struct BackgroundColor(pub Vec4);

/// A helper struct for configuring calls to `Renderling::setup_render_graph`.
pub struct RenderGraphConfig {
    pub scene: Option<Scene>,
    pub ui: Option<UiScene>,
    pub objs: Vec<UiDrawObject>,
    // Whether or not to use the screen capture node.
    // You probably want this to be `true` if you are rendering headless.
    pub with_screen_capture: bool,
    // Whether or not to use bloom filter in post-processing.
    pub with_bloom: bool,
}

impl Default for RenderGraphConfig {
    fn default() -> Self {
        Self {
            scene: Default::default(),
            ui: Default::default(),
            objs: Default::default(),
            with_screen_capture: false,
            with_bloom: true,
        }
    }
}

/// A graph-based renderer that manages GPU resources for cameras, materials and
/// meshes.
pub struct Renderling {
    // The inner render graph
    pub graph: Graph,
}

impl Renderling {
    pub fn new(
        target: RenderTarget,
        depth_texture: crate::Texture,
        adapter: impl Into<Arc<wgpu::Adapter>>,
        device: impl Into<Arc<wgpu::Device>>,
        queue: impl Into<Arc<wgpu::Queue>>,
        (width, height): (u32, u32),
    ) -> Self {
        Self {
            graph: Graph::default()
                .with_resource(target)
                .with_resource(DepthTexture(depth_texture))
                .with_resource(Adapter(adapter.into()))
                .with_resource(Device(device.into()))
                .with_resource(Queue(queue.into()))
                .with_resource(ScreenSize { width, height })
                .with_resource(BackgroundColor(Vec4::splat(0.0))),
        }
    }

    pub async fn try_new_headless(width: u32, height: u32) -> Result<Self, RenderlingError> {
        let size = (width, height);
        let (adapter, device, queue, target) = crate::state::new_adapter_device_queue_and_target(
            width,
            height,
            None as Option<CreateSurfaceFn>,
        )
        .await?;
        let depth_texture = crate::Texture::create_depth_texture(&device, width, height);
        Ok(Self::new(
            target,
            depth_texture,
            adapter,
            device,
            queue,
            size,
        ))
    }

    #[cfg(feature = "raw-window-handle")]
    pub async fn try_from_raw_window<W>(
        width: u32,
        height: u32,
        window: &W,
    ) -> Result<Self, RenderlingError>
    where
        W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    {
        let size = (width, height);
        let (adapter, device, queue, target) = crate::state::new_adapter_device_queue_and_target(
            width,
            height,
            Some(Box::new(|instance: &wgpu::Instance| {
                unsafe { instance.create_surface(window) }
                    .map_err(|e| WgpuStateError::CreateSurface { source: e })
            }) as crate::CreateSurfaceFn),
        )
        .await?;
        let depth_texture = crate::Texture::create_depth_texture(&device, width, height);

        Ok(Self::new(
            target,
            depth_texture,
            adapter,
            device,
            queue,
            size,
        ))
    }

    #[cfg(feature = "raw-window-handle")]
    pub async fn from_window_async(window: &winit::window::Window) -> Self {
        let inner_size = window.inner_size();
        Self::try_from_raw_window(inner_size.width, inner_size.height, window)
            .await
            .unwrap()
    }

    #[cfg(all(feature = "raw-window-handle", not(target_arch = "wasm32")))]
    pub fn try_from_raw_window_handle(
        window_handle: &(impl raw_window_handle::HasRawWindowHandle
              + raw_window_handle::HasRawDisplayHandle),
        width: u32,
        height: u32,
    ) -> Result<Self, RenderlingError> {
        futures_lite::future::block_on(Self::try_from_raw_window(width, height, window_handle))
    }

    #[cfg(all(
        feature = "winit",
        feature = "raw-window-handle",
        not(target_arch = "wasm32")
    ))]
    pub fn try_from_window(window: &winit::window::Window) -> Result<Self, RenderlingError> {
        let inner_size = window.inner_size();
        Self::try_from_raw_window_handle(window, inner_size.width, inner_size.height)
    }

    /// Create a new headless renderer.
    ///
    /// ## Panics
    /// This function will panic if an adapter cannot be found. For example this would
    /// happen on machines without a GPU.
    pub fn headless(width: u32, height: u32) -> Self {
        futures_lite::future::block_on(Self::try_new_headless(width, height)).unwrap()
    }

    pub fn set_background_color(&mut self, color: impl Into<Vec4>) {
        self.graph.add_resource(BackgroundColor(color.into()));
    }

    pub fn with_background_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    pub fn get_screen_size(&self) -> (u32, u32) {
        // UNWRAP: safe because invariant - Renderer always has ScreenSize
        let ScreenSize { width, height } =
            self.graph.get_resource::<ScreenSize>().unwrap().unwrap();
        (*width, *height)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // UNWRAP: safe because invariant
        self.graph
            .visit(
                |(device, mut screen_size, mut target, mut depth_texture): (
                    View<Device>,
                    ViewMut<ScreenSize>,
                    ViewMut<RenderTarget>,
                    ViewMut<DepthTexture>,
                )| {
                    *screen_size = ScreenSize { width, height };
                    target.resize(width, height, &device.0);
                    depth_texture.0 = crate::Texture::create_depth_texture(&device, width, height);
                },
            )
            .unwrap();
        // The renderer doesn't _always_ have an HrdSurface, so we don't unwrap this
        // one.
        let _ = self.graph.visit(
            |(device, queue, mut hdr): (View<Device>, View<Queue>, ViewMut<HdrSurface>)| {
                hdr.hdr_texture = HdrSurface::create_texture(&device, &queue, width, height);
                hdr.texture_bindgroup =
                    HdrSurface::create_texture_bindgroup(&device, &hdr.hdr_texture);
            },
        );
    }

    pub fn create_texture<P>(
        &self,
        label: Option<&str>,
        img: &image::ImageBuffer<P, Vec<u8>>,
    ) -> Result<crate::Texture, RenderlingError>
    where
        P: image::PixelWithColorType,
        image::ImageBuffer<P, Vec<u8>>: image::GenericImage + std::ops::Deref<Target = [u8]>,
    {
        let name = label.unwrap_or("unknown");
        let device = self
            .graph
            .get_resource::<Device>()
            .context(GraphSnafu)?
            .context(ResourceSnafu {
                key: TypeKey::new::<Device>(),
            })?;
        let queue = self
            .graph
            .get_resource::<Queue>()
            .context(GraphSnafu)?
            .context(ResourceSnafu {
                key: TypeKey::new::<Queue>(),
            })?;
        crate::Texture::from_image_buffer(
            device,
            queue,
            img,
            Some(&format!("Renderling::create_texture {}", name)),
            None,
            None,
        )
        .context(TextureSnafu)
    }

    pub fn texture_from_wgpu_tex(
        &self,
        texture: impl Into<Arc<wgpu::Texture>>,
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> crate::Texture {
        crate::Texture::from_wgpu_tex(self.get_device(), texture, sampler, None)
    }

    pub fn get_device(&self) -> &wgpu::Device {
        // UNWRAP: safe because invariant - Renderer always has Device
        &self.graph.get_resource::<Device>().unwrap().unwrap().0
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        // UNWRAP: safe because invariant - Renderer always has Queue
        &self.graph.get_resource::<Queue>().unwrap().unwrap().0
    }

    pub fn get_adapter(&self) -> &wgpu::Adapter {
        // UNWRAP: safe because invariant - Renderer always has Adapter
        &self.graph.get_resource::<Adapter>().unwrap().unwrap().0
    }

    /// Returns a the adapter in an owned wrapper.
    pub fn get_adapter_owned(&self) -> crate::Adapter {
        // UNWRAP: safe because invariant - Renderer always has Adapter
        self.graph
            .get_resource::<Adapter>()
            .unwrap()
            .unwrap()
            .clone()
    }

    /// Returns a pair of the device and queue in an owned wrapper.
    pub fn get_device_and_queue_owned(&self) -> (crate::Device, crate::Queue) {
        // UNWRAP: safe because we always have device and queue
        let device = self
            .graph
            .get_resource::<crate::Device>()
            .unwrap()
            .unwrap()
            .clone();
        let queue = self
            .graph
            .get_resource::<crate::Queue>()
            .unwrap()
            .unwrap()
            .clone();
        (device, queue)
    }

    pub fn get_render_target(&self) -> &RenderTarget {
        // UNWRAP: safe because we always have a render target, or we need to panic
        self.graph.get_resource().unwrap().unwrap()
    }

    pub fn new_stage(&self) -> Stage {
        let (device, queue) = self.get_device_and_queue_owned();
        Stage::new(device, queue)
    }

    pub fn new_scene(&self) -> SceneBuilder {
        let (device, queue) = self.get_device_and_queue_owned();
        SceneBuilder::new(device.0, queue.0)
    }

    pub fn empty_scene(&self) -> Scene {
        self.new_scene().build().unwrap()
    }

    pub fn new_ui_scene(&self) -> UiSceneBuilder<'_> {
        let (device, _) = self.get_device_and_queue_owned();
        let queue = self.get_queue();
        UiSceneBuilder::new(device.0.clone(), queue)
    }

    pub fn empty_ui_scene(&self) -> UiScene {
        self.new_ui_scene().build()
    }

    #[cfg(feature = "text")]
    /// Create a new `GlyphCache` used to cache text rendering info.
    pub fn new_glyph_cache(&self, fonts: Vec<crate::FontArc>) -> crate::GlyphCache {
        crate::GlyphCache::new(fonts)
    }

    /// Sets up the render graph with the given scenes and objects.
    ///
    /// The scenes and objects may be "visited" later, or even retrieved.
    pub fn setup_render_graph(&mut self, config: RenderGraphConfig) {
        let RenderGraphConfig {
            scene,
            ui,
            objs,
            with_screen_capture,
            with_bloom,
        } = config;
        let scene = scene.unwrap_or_else(|| self.empty_scene());
        let ui = ui.unwrap_or_else(|| self.empty_ui_scene());
        crate::setup_render_graph(self, scene, ui, objs, with_screen_capture, with_bloom)
    }

    /// Render into an image.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// For this call to succeed, the `PostRenderBufferCreate::create` node must
    /// be present in the graph.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub fn render_image(&mut self) -> Result<image::RgbaImage, RenderlingError> {
        use crate::frame::PostRenderBuffer;

        self.render()?;
        let buffer = self
            .graph
            .remove_resource::<PostRenderBuffer>()
            .context(MissingPostRenderBufferSnafu)?
            .context(ResourceSnafu {
                key: TypeKey::new::<PostRenderBuffer>(),
            })?;
        let device = self.get_device();
        let img = buffer.0.into_rgba(device).context(TextureSnafu)?;
        Ok(img)
    }

    /// Run the render graph.
    pub fn render(&mut self) -> Result<(), RenderlingError> {
        self.graph.run().context(GraphSnafu)
    }
}
