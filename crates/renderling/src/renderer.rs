//! Builds the UI pipeline and manages resources.
use futures_lite::future::FutureExt;
use glam::Vec4;
use moongraph::{Edges, Function, Graph, Node, Read, TypeKey, Write};
use snafu::prelude::*;
use std::{any::Any, ops::Deref, sync::Arc};

use crate::{
    camera::*, node::UiRenderCamera, resources::*, ForwardPipeline, Lights, Object, ObjectBuilder,
    ObjectInner, Objects, Pipelines, PostRenderBuffer, RenderTarget, WgpuStateError,
};

pub use moongraph::IsGraphNode;

pub type RenderNode = Node<Function, TypeKey>;

#[derive(Debug, Snafu)]
pub enum RenderlingError {
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

    #[snafu(display("could not create scene: {}", source))]
    Scene { source: crate::GltfError },

    #[snafu(display("missing resource"))]
    Resource,

    #[snafu(display("{source}"))]
    Graph { source: moongraph::GraphError },

    #[snafu(display("{source}"))]
    Lights { source: crate::light::LightsError },

    #[snafu(display("{source}"))]
    Object { source: crate::object::ObjectError },

    #[snafu(display(
        "Missing PostRenderBuffer resource. Ensure a node that creates PostRenderBuffer (like \
         PostRenderbufferCreate) is present in the graph: {source}"
    ))]
    MissingPostRenderBuffer { source: moongraph::GraphError },

    #[snafu(display("Timeout while waiting for a screengrab"))]
    ScreenGrabTimeout { source: WgpuStateError },
}

/// A thread-safe wrapper around `wgpu::Device`.
pub struct Device(pub Arc<wgpu::Device>);

impl Deref for Device {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A thread-safe wrapper around `wgpu::Queue`.
pub struct Queue(pub Arc<wgpu::Queue>);

impl Deref for Queue {
    type Target = wgpu::Queue;

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

/// A graph-based renderer that manages GPU resources for cameras, materials and
/// meshes.
// TODO: Think about adding a phantom type variable to Renderling, and then using adjoint types that
// include specific functionality:
// ```rust
// pub struct Renderling<T> {
//     ...
// }
// impl<T: Has<ForwardRenderNode>> Renderling<T> {
//     pub fn forward_render(&self) -> Result<(), RenderlingError> {...}
// }
// ```
pub struct Renderling {
    pub graph: Graph,
}

impl Renderling {
    pub fn new(
        target: RenderTarget,
        depth_texture: crate::Texture,
        device: wgpu::Device,
        queue: wgpu::Queue,
        (width, height): (u32, u32),
    ) -> Self {
        Self {
            graph: Graph::default()
                .with_resource(target)
                .with_resource(DepthTexture(depth_texture))
                .with_resource(Device(device.into()))
                .with_resource(Queue(queue.into()))
                .with_resource(ScreenSize { width, height })
                .with_resource(Lights::default())
                .with_resource(Cameras::default())
                .with_resource(Objects::default())
                .with_resource(Pipelines::default())
                .with_resource(BackgroundColor(Vec4::splat(0.0))),
        }
    }

    pub async fn try_new_headless(width: u32, height: u32) -> Result<Self, RenderlingError> {
        let size = (width, height);

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = if cfg!(target_arch = "wasm32") {
            wgpu::Backends::all()
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .context(CannotCreateAdaptorSnafu)?;
        let limits = if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
        } else {
            wgpu::Limits::default()
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits,
                    label: None,
                },
                None, // Trace path
            )
            .await
            .context(CannotRequestDeviceSnafu)?;

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
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };
        let texture = Arc::new(device.create_texture(&texture_desc));
        let depth_texture = crate::Texture::create_depth_texture(&device, width, height);
        let target = RenderTarget::Texture { texture };

        Ok(Self::new(target, depth_texture, device, queue, size))
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

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let backends = if cfg!(target_arch = "wasm32") {
            wgpu::Backends::all()
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });
        let surface = unsafe { instance.create_surface(window) }.context(CreateSurfaceSnafu)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context(CannotCreateAdaptorSnafu)?;
        let limits = if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
        } else {
            wgpu::Limits::default()
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits,
                    label: None,
                },
                None, // Trace path
            )
            .await
            .context(CannotRequestDeviceSnafu)?;

        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .context(IncompatibleSurfaceSnafu)?;
        surface.configure(&device, &surface_config);
        let target = RenderTarget::Surface {
            surface,
            surface_config,
        };
        let depth_texture = crate::Texture::create_depth_texture(&device, width, height);

        Ok(Self::new(target, depth_texture, device, queue, size))
    }

    #[cfg(feature = "raw-window-handle")]
    pub fn try_from_raw_window_handle(
        window_handle: &(impl raw_window_handle::HasRawWindowHandle
              + raw_window_handle::HasRawDisplayHandle),
        width: u32,
        height: u32,
    ) -> Result<Self, RenderlingError> {
        futures_lite::future::block_on(Self::try_from_raw_window(width, height, window_handle))
    }

    #[cfg(feature = "winit")]
    pub fn try_from_window(window: &winit::window::Window) -> Result<Self, RenderlingError> {
        let inner_size = window.inner_size();
        Self::try_from_raw_window_handle(window, inner_size.width, inner_size.height)
    }

    pub fn headless(width: u32, height: u32) -> Result<Self, RenderlingError> {
        futures_lite::future::block_on(Self::try_new_headless(width, height))
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
                    Read<Device>,
                    Write<ScreenSize>,
                    Write<RenderTarget>,
                    Write<DepthTexture>,
                )| {
                    *screen_size = ScreenSize { width, height };
                    target.resize(width, height, &device.0);
                    depth_texture.0 = crate::Texture::create_depth_texture(&device, width, height);
                },
            )
            .unwrap();
    }

    #[cfg(feature = "image")]
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
            .context(ResourceSnafu)?;
        let queue = self
            .graph
            .get_resource::<Queue>()
            .context(GraphSnafu)?
            .context(ResourceSnafu)?;
        crate::Texture::from_image_buffer(
            device,
            queue,
            img,
            Some(&format!("Renderling::create_texture {}", name)),
            None,
        )
        .context(TextureSnafu)
    }

    pub fn get_device(&self) -> &wgpu::Device {
        // UNWRAP: safe because invariant - Renderer always has Device
        &self.graph.get_resource::<Device>().unwrap().unwrap().0
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        // UNWRAP: safe because invariant - Renderer always has Queue
        &self.graph.get_resource::<Queue>().unwrap().unwrap().0
    }

    pub fn get_cameras(&self) -> &Cameras {
        // UNWRAP: safe because invariant - Renderer always has Cameras
        self.graph.get_resource::<Cameras>().unwrap().unwrap()
    }

    pub fn get_cameras_mut(&mut self) -> &mut Cameras {
        // UNWRAP: safe because invariant - Renderer always has Cameras
        self.graph.get_resource_mut::<Cameras>().unwrap().unwrap()
    }

    pub fn get_objects(&self) -> &Objects {
        // UNWRAP: safe because invariant - Renderer always has Objects
        self.graph.get_resource::<Objects>().unwrap().unwrap()
    }

    pub fn get_objects_mut(&mut self) -> &mut Objects {
        // UNWRAP: safe because invariant - Renderer always has Objects
        self.graph.get_resource_mut::<Objects>().unwrap().unwrap()
    }

    /// Create a new camera builder.
    pub fn new_camera(&mut self) -> CameraBuilder<'_> {
        let (width, height) = self.get_screen_size();
        CameraBuilder {
            inner: CameraInner::new_perspective(width as f32, height as f32),
            width: width as f32,
            height: height as f32,
            renderer: self,
        }
    }

    /// Retrieves an iterator over all cameras.
    ///
    /// This will always have at least one camera in it.
    pub fn cameras(&self) -> impl Iterator<Item = Camera> + '_ {
        // UNWRAP: safe because Renderer always has the Cameras object
        let cameras = self.graph.get_resource::<Cameras>().unwrap().unwrap();
        cameras.iter().enumerate().filter_map(|(i, data)| {
            let data = data.as_ref()?;
            Some(Camera {
                id: Id::new(i),
                inner: data.inner.clone(),
                cmd: cameras.update_queue(),
            })
        })
    }

    /// Retrieves the default camera.
    ///
    /// The default camera comes first in the iterator returned by
    /// `Renderling::cameras`. The default camera is the one that was
    /// created first after the renderling was created.
    pub fn default_camera(&self) -> Camera {
        // UNWRAP: having one default camera is an invariant of the system and we should
        // panic otherwise
        self.cameras().next().unwrap()
    }

    /// Creates a new object builder.
    ///
    /// The builder can be used to customize the object before building it.
    ///
    /// If no material is provided, the renderling's default material will be
    /// used.
    ///
    /// If no transform is provided, the object will be positioned at the origin
    /// with no rotation and scale 1,1,1.
    pub fn new_object(&mut self) -> ObjectBuilder<'_> {
        let objects = self.get_objects_mut();
        ObjectBuilder {
            mesh: None,
            children: vec![],
            generate_normal_matrix: false,
            properties: Default::default(),
            inner: ObjectInner::default(),
            renderer: self,
        }
    }

    pub fn get_lights(&self) -> &Lights {
        // UNWRAP: safe because invariant - Renderer always has Lights
        self.graph.get_resource::<Lights>().unwrap().unwrap()
    }

    pub fn get_lights_mut(&mut self) -> &mut Lights {
        // UNWRAP: safe because invariant - Renderer always has Lights
        self.graph.get_resource_mut::<Lights>().unwrap().unwrap()
    }

    pub fn new_point_light(&mut self) -> crate::PointLightBuilder {
        crate::PointLightBuilder::new(&mut self.get_lights_mut())
    }

    pub fn new_spot_light(&mut self) -> crate::SpotLightBuilder {
        crate::SpotLightBuilder::new(&mut self.get_lights_mut())
    }

    pub fn new_directional_light(&mut self) -> crate::DirectionalLightBuilder {
        crate::DirectionalLightBuilder::new(&mut self.get_lights_mut())
    }

    /// Add a render node to the render graph.
    pub fn add_node(&mut self, node: RenderNode) {
        self.graph.add_node(node);
    }

    pub fn with_node(mut self, node: RenderNode) -> Self {
        self.add_node(node);
        self
    }

    /// Add a barrier to the render graph.
    ///
    /// All nodes added after the barrier will run after nodes added before the
    /// barrier.
    pub fn with_barrier(mut self) -> Self {
        self.add_barrier();
        self
    }

    /// Add a barrier to the render graph.
    ///
    /// All nodes added after the barrier will run after nodes added before the
    /// barrier.
    pub fn add_barrier(&mut self) {
        self.graph.add_barrier();
    }

    /// Add a resource to the render graph.
    pub fn add_resource<T: Any + Send + Sync>(&mut self, resource: T) {
        self.graph.add_resource(resource);
    }

    /// Add a resource to the render graph.
    pub fn with_resource<T: Any + Send + Sync>(mut self, resource: T) -> Self {
        self.add_resource(resource);
        self
    }

    #[cfg(feature = "ui")]
    /// Set the graph to the default user interface rendering configuration.
    pub fn with_default_ui_render_graph(mut self) -> Self {
        self.set_background_color(Vec4::splat(1.0));
        self.add_node(
            crate::ObjectUpdate::run
                .into_node()
                .with_name("object_update"),
        );
        self.add_node(
            crate::CameraUpdate::run
                .into_node()
                .with_name("camera_update"),
        );
        self.add_barrier();
        self.add_node(
            crate::node::create_frame
                .into_node()
                .with_name("create_frame"),
        );
        self.add_node(
            crate::node::clear_frame_and_depth
                .into_node()
                .with_name("clear_frame_and_depth"),
        );
        self.add_barrier();

        let ui_pipeline = self.graph.visit(crate::UiPipelineCreator::create).unwrap();
        self.with_resource(ui_pipeline)
            .with_node(crate::node::ui_render.into_node().with_name("ui_render"))
            .with_node(crate::node::present.into_node().with_name("present_frame"))
    }

    #[cfg(feature = "forward")]
    /// Set the graph to a default forward rendering configuration.
    ///
    /// This renderer uses blinn-phong shading.
    pub fn with_default_forward_render_graph(self) -> Self {
        self.with_forward_render_graph(crate::ForwardPipelineCreator::create)
    }

    #[cfg(feature = "forward")]
    /// Set the graph to a forward rendering configuration.
    pub fn with_forward_render_graph<T: Edges>(
        mut self,
        create: impl FnOnce(T) -> ForwardPipeline,
    ) -> Self {
        self.set_background_color(Vec4::splat(1.0));
        self.add_node(
            crate::LightUpdate::run
                .into_node()
                .with_name("light_update"),
        );
        self.add_node(
            crate::ObjectUpdate::run
                .into_node()
                .with_name("object_update"),
        );
        self.add_node(
            crate::CameraUpdate::run
                .into_node()
                .with_name("camera_update"),
        );
        self.add_barrier();
        self.add_node(
            crate::node::create_frame
                .into_node()
                .with_name("create_frame"),
        );
        self.add_node(
            crate::node::clear_frame_and_depth
                .into_node()
                .with_name("clear_frame_and_depth"),
        );
        self.add_barrier();

        let forward_pipeline = self.graph.visit(create).unwrap();
        self.with_resource(forward_pipeline)
            .with_node(
                crate::node::forward_render
                    .into_node()
                    .with_name("forward_render"),
            )
            .with_node(crate::node::present.into_node().with_name("present_frame"))
    }

    #[cfg(feature = "gltf")]
    pub fn new_gltf_loader(&self) -> crate::gltf_support::GltfLoader {
        // UNWRAP: safe because device and queue are _always_ available (if not we should panic)
        let device = self
            .graph
            .get_resource::<Device>()
            .unwrap()
            .unwrap()
            .0
            .clone();
        let queue = self
            .graph
            .get_resource::<Queue>()
            .unwrap()
            .unwrap()
            .0
            .clone();
        crate::gltf_support::GltfLoader::new(device, queue)
    }

    #[cfg(feature = "image")]
    /// Render into an image.
    ///
    /// This should be called after rendering, before presentation.
    /// Good for getting headless screen grabs.
    ///
    /// For this call to succeed, the `PostRenderBufferCreate::create` node must be
    /// present in the graph.
    ///
    /// ## Note
    /// This operation can take a long time, depending on how big the screen is.
    pub fn render_image(&mut self) -> Result<image::RgbaImage, RenderlingError> {
        self.render()?;
        let buffer = self
            .graph
            .remove_resource::<PostRenderBuffer>()
            .context(MissingPostRenderBufferSnafu)?
            .context(ResourceSnafu)?;
        let device = self.get_device();
        let img = futures_lite::future::block_on(async move {
            async { buffer.convert_to_rgba().await }
                .or(async {
                    loop {
                        device.poll(wgpu::Maintain::Poll);
                        futures_lite::future::yield_now().await;
                    }
                })
                .await
        })
        .with_context(|_| ScreenGrabTimeoutSnafu)?;
        Ok(img)
    }

    /// Run the render graph.
    pub fn render(&mut self) -> Result<(), RenderlingError> {
        self.graph.run().context(GraphSnafu)
    }
}
