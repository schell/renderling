//! GPU staging area.
use core::ops::Deref;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use craballoc::prelude::*;
use crabslab::Id;
use glam::{Mat4, UVec2, Vec4};
use snafu::{ResultExt, Snafu};
use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock};

use crate::atlas::AtlasTexture;
use crate::camera::Camera;
use crate::geometry::{shader::GeometryDescriptor, MorphTarget, Vertex};
#[cfg(gltf)]
use crate::gltf::GltfDocument;
use crate::light::{DirectionalLight, IsLight, Light, PointLight, SpotLight};
use crate::material::Material;
use crate::pbr::brdf::BrdfLut;
use crate::pbr::ibl::Ibl;
use crate::primitive::Primitive;
use crate::{
    atlas::{AtlasError, AtlasImage, AtlasImageError},
    bindgroup::ManagedBindGroup,
    bloom::Bloom,
    camera::shader::CameraDescriptor,
    debug::DebugOverlay,
    draw::DrawCalls,
    geometry::{Geometry, Indices, MorphTargetWeights, MorphTargets, Skin, SkinJoint, Vertices},
    light::{
        AnalyticalLight, LightTiling, LightTilingConfig, Lighting, LightingBindGroupLayoutEntries,
        LightingError, ShadowMap,
    },
    material::Materials,
    pbr::debug::DebugChannel,
    skybox::{Skybox, SkyboxRenderPipeline},
    texture::{DepthTexture, Texture},
    tonemapping::Tonemapping,
    transform::{NestedTransform, Transform},
};

/// Enumeration of errors that may be the result of [`Stage`] functions.
#[derive(Debug, Snafu)]
pub enum StageError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },

    #[snafu(display("{source}"))]
    Lighting { source: LightingError },

    #[cfg(gltf)]
    #[snafu(display("{source}"))]
    Gltf { source: crate::gltf::StageGltfError },
}

impl From<AtlasError> for StageError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
    }
}

impl From<LightingError> for StageError {
    fn from(source: LightingError) -> Self {
        Self::Lighting { source }
    }
}

#[cfg(gltf)]
impl From<crate::gltf::StageGltfError> for StageError {
    fn from(source: crate::gltf::StageGltfError) -> Self {
        Self::Gltf { source }
    }
}

fn create_msaa_textureview(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("stage msaa render target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

/// Result of calling [`Stage::commit`].
pub struct StageCommitResult {
    pub(crate) geometry_buffer: SlabBuffer<wgpu::Buffer>,
    pub(crate) lighting_buffer: SlabBuffer<wgpu::Buffer>,
    pub(crate) materials_buffer: SlabBuffer<wgpu::Buffer>,
}

impl StageCommitResult {
    /// Timestamp of the most recently created buffer used by the stage.
    pub(crate) fn latest_creation_time(&self) -> usize {
        [
            &self.geometry_buffer,
            &self.materials_buffer,
            &self.lighting_buffer,
        ]
        .iter()
        .map(|buffer| buffer.creation_time())
        .max()
        .unwrap_or_default()
    }

    /// Whether or not the stage's bindgroups need to be invalidated as a result
    /// of the call to [`Stage::commit`] that produced this `StageCommitResult`.
    pub(crate) fn should_invalidate(&self, previous_creation_time: usize) -> bool {
        let mut should = false;
        if self.geometry_buffer.is_new_this_commit() {
            log::trace!("geometry buffer is new this frame");
            should = true;
        }
        if self.materials_buffer.is_new_this_commit() {
            log::trace!("materials buffer is new this frame");
            should = true;
        }
        if self.lighting_buffer.is_new_this_commit() {
            log::trace!("lighting buffer is new this frame");
            should = true;
        }
        let current = self.latest_creation_time();
        if current > previous_creation_time {
            log::trace!(
                "current latest buffer creation time {current} > previous {previous_creation_time}"
            );
            should = true;
        }
        should
    }
}

/// Bindgroup used to render primitives.
///
/// This is the bindgroup that occupies `descriptor_set = 0` in
/// [crate::primitive::shader::primitive_vertex] and
/// [crate::primitive::shader::primitive_fragment].
struct PrimitiveBindGroup<'a> {
    device: &'a wgpu::Device,
    layout: &'a wgpu::BindGroupLayout,
    geometry_buffer: &'a wgpu::Buffer,
    material_buffer: &'a wgpu::Buffer,
    light_buffer: &'a wgpu::Buffer,
    atlas_texture_view: &'a wgpu::TextureView,
    atlas_texture_sampler: &'a wgpu::Sampler,
    irradiance_texture_view: &'a wgpu::TextureView,
    irradiance_texture_sampler: &'a wgpu::Sampler,
    prefiltered_texture_view: &'a wgpu::TextureView,
    prefiltered_texture_sampler: &'a wgpu::Sampler,
    brdf_texture_view: &'a wgpu::TextureView,
    brdf_texture_sampler: &'a wgpu::Sampler,
    shadow_map_texture_view: &'a wgpu::TextureView,
    shadow_map_texture_sampler: &'a wgpu::Sampler,
}

impl PrimitiveBindGroup<'_> {
    pub fn create(self) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("primitive"),
            layout: self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.geometry_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.material_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(self.atlas_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(self.atlas_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(self.irradiance_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(self.irradiance_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(self.prefiltered_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::Sampler(self.prefiltered_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::TextureView(self.brdf_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 9,
                    resource: wgpu::BindingResource::Sampler(self.brdf_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 10,
                    resource: self.light_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 11,
                    resource: wgpu::BindingResource::TextureView(self.shadow_map_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 12,
                    resource: wgpu::BindingResource::Sampler(self.shadow_map_texture_sampler),
                },
            ],
        })
    }
}

/// Performs a rendering of an entire scene, given the resources at hand.
pub(crate) struct StageRendering<'a> {
    // TODO: include the rest of the needed paramaters from `stage`, and then remove `stage`
    pub stage: &'a Stage,
    pub pipeline: &'a wgpu::RenderPipeline,
    pub color_attachment: wgpu::RenderPassColorAttachment<'a>,
    pub depth_stencil_attachment: wgpu::RenderPassDepthStencilAttachment<'a>,
}

impl StageRendering<'_> {
    /// Run the stage rendering.
    ///
    /// Returns the queue submission index and the indirect draw buffer, if available.
    pub fn run(self) -> (wgpu::SubmissionIndex, Option<SlabBuffer<wgpu::Buffer>>) {
        let commit_result = self.stage.commit();
        let current_primitive_bind_group_creation_time = commit_result.latest_creation_time();
        log::trace!("current_primitive_bind_group_creation_time: {current_primitive_bind_group_creation_time}");
        let previous_primitive_bind_group_creation_time =
            self.stage.primitive_bind_group_created.swap(
                current_primitive_bind_group_creation_time,
                std::sync::atomic::Ordering::Relaxed,
            );
        let should_invalidate_primitive_bind_group =
            commit_result.should_invalidate(previous_primitive_bind_group_creation_time);
        log::trace!(
            "should_invalidate_primitive_bind_group: {should_invalidate_primitive_bind_group}"
        );
        let primitive_bind_group =
            self.stage
                .primitive_bind_group
                .get(should_invalidate_primitive_bind_group, || {
                    log::trace!("recreating primitive bind group");
                    let atlas_texture = self.stage.materials.atlas().get_texture();
                    let ibl = self.stage.ibl.read().unwrap();
                    let shadow_map = self.stage.lighting.shadow_map_atlas.get_texture();
                    PrimitiveBindGroup {
                        device: self.stage.device(),
                        layout: &Stage::primitive_pipeline_bindgroup_layout(self.stage.device()),
                        geometry_buffer: &commit_result.geometry_buffer,
                        material_buffer: &commit_result.materials_buffer,
                        light_buffer: &commit_result.lighting_buffer,
                        atlas_texture_view: &atlas_texture.view,
                        atlas_texture_sampler: &atlas_texture.sampler,
                        irradiance_texture_view: &ibl.irradiance_cubemap.view,
                        irradiance_texture_sampler: &ibl.irradiance_cubemap.sampler,
                        prefiltered_texture_view: &ibl.prefiltered_environment_cubemap.view,
                        prefiltered_texture_sampler: &ibl.prefiltered_environment_cubemap.sampler,
                        brdf_texture_view: &self.stage.brdf_lut.inner.view,
                        brdf_texture_sampler: &self.stage.brdf_lut.inner.sampler,
                        shadow_map_texture_view: &shadow_map.view,
                        shadow_map_texture_sampler: &shadow_map.sampler,
                    }
                    .create()
                });

        let mut draw_calls = self.stage.draw_calls.write().unwrap();
        let depth_texture = self.stage.depth_texture.read().unwrap();
        // UNWRAP: safe because we know the depth texture format will always match
        let maybe_indirect_buffer = draw_calls.pre_draw(&depth_texture).unwrap();

        log::trace!("rendering");
        let label = Some("stage render");

        let mut encoder = self
            .stage
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label,
                color_attachments: &[Some(self.color_attachment)],
                depth_stencil_attachment: Some(self.depth_stencil_attachment),
                ..Default::default()
            });

            render_pass.set_pipeline(self.pipeline);
            render_pass.set_bind_group(0, Some(primitive_bind_group.as_ref()), &[]);
            draw_calls.draw(&mut render_pass);

            let has_skybox = self.stage.has_skybox.load(Ordering::Relaxed);
            if has_skybox {
                let (pipeline, bindgroup) = self
                    .stage
                    .get_skybox_pipeline_and_bindgroup(&commit_result.geometry_buffer);
                render_pass.set_pipeline(&pipeline.pipeline);
                render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
                let camera_id = self.stage.geometry.descriptor().get().camera_id.inner();
                render_pass.draw(0..36, camera_id..camera_id + 1);
            }
        }
        let sindex = self.stage.queue().submit(std::iter::once(encoder.finish()));
        (sindex, maybe_indirect_buffer)
    }
}

/// Entrypoint for staging data on the GPU and interacting with lighting.
///
/// # Design
///
/// The `Stage` struct serves as the central hub for managing and staging data on the GPU.
/// It provides a consistent API for creating resources, applying effects, and customizing parameters.
///
/// The `Stage` uses a combination of `new_*`, `with_*`, `set_*`, and getter functions to facilitate
/// resource management and customization.
///
/// Resources are managed internally, requiring no additional lifecycle work from the user.
/// This design simplifies the process of resource management, allowing developers to focus on creating and rendering
/// their scenes without worrying about the underlying GPU resource management.
///
/// # Resources
///
/// The `Stage` is responsible for creating various resources and staging them on the GPU.
/// It handles the setup and management of the following resources:
///
/// * [`Camera`]: Manages the view and projection matrices for rendering scenes.
///   - [`Stage::new_camera`] creates a new [`Camera`].
///   - [`Stage::use_camera`] tells the `Stage` to use a camera.
/// * [`Transform`]: Represents the position, rotation, and scale of objects.
///   - [`Stage::new_transform`] creates a new [`Transform`].
/// * [`NestedTransform`]: Allows for hierarchical transformations, useful for complex object hierarchies.
///   - [`Stage::new_nested_transform`] creates a new [`NestedTransform`]
/// * [`Vertices`]: Manages vertex data for rendering meshes.
///   - [`Stage::new_vertices`]
/// * [`Indices`]: Manages index data for rendering meshes with indexed drawing.
///   - [`Stage::new_indices`]
/// * [`Primitive`]: Represents a drawable object in the scene.
///   - [`Stage::new_primitive`]
/// * [`GltfDocument`]: Handles loading and managing GLTF assets.
///   - [`Stage::load_gltf_document_from_path`] loads a new GLTF document from the local filesystem.
///   - [`Stage::load_gltf_document_from_bytes`] parses a new GLTF document from pre-loaded bytes.
/// * [`Skin`]: Animation and rigging information.
///   - [`Stage::new_skin`]
///
/// # Lighting effects
///
/// The `Stage` also manages various lighting effects, which enhance the visual quality of the scene:
///
/// * [`AnalyticalLight`]: Simulates a single light source, with three flavors:
///   - [`DirectionalLight`]: Represents sunlight or other distant light sources.
///   - [`PointLight`]: Represents a light source that emits light in all directions from a single point.
///   - [`SpotLight`]: Represents a light source that emits light in a cone shape.
/// * [`Skybox`]: Provides image-based lighting (IBL) for realistic environmental reflections and ambient lighting.
/// * [`Bloom`]: Adds a glow effect to bright areas of the scene, enhancing visual appeal.
/// * [`ShadowMap`]: Manages shadow mapping for realistic shadow rendering.
/// * [`LightTiling`]: Optimizes lighting calculations by dividing the scene into tiles for efficient processing.
///
/// # Note
///
/// Clones of [`Stage`] all point to the same underlying data.
#[derive(Clone)]
pub struct Stage {
    pub(crate) geometry: Geometry,
    pub(crate) materials: Materials,
    pub(crate) lighting: Lighting,

    pub(crate) primitive_pipeline: Arc<RwLock<wgpu::RenderPipeline>>,
    pub(crate) primitive_bind_group: ManagedBindGroup,
    pub(crate) primitive_bind_group_created: Arc<AtomicUsize>,

    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<SkyboxRenderPipeline>>>>,

    pub(crate) hdr_texture: Arc<RwLock<Texture>>,
    pub(crate) depth_texture: Arc<RwLock<Texture>>,
    pub(crate) msaa_render_target: Arc<RwLock<Option<wgpu::TextureView>>>,
    pub(crate) msaa_sample_count: Arc<AtomicU32>,
    pub(crate) clear_color_attachments: Arc<AtomicBool>,
    pub(crate) clear_depth_attachments: Arc<AtomicBool>,

    pub(crate) bloom: Bloom,

    pub(crate) tonemapping: Tonemapping,
    pub(crate) debug_overlay: DebugOverlay,
    pub(crate) background_color: Arc<RwLock<wgpu::Color>>,

    pub(crate) brdf_lut: BrdfLut,

    pub(crate) ibl: Arc<RwLock<Ibl>>,

    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    // TODO: remove Stage.has_skybox, replace with Skybox::is_empty
    pub(crate) has_skybox: Arc<AtomicBool>,

    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) has_debug_overlay: Arc<AtomicBool>,

    pub(crate) stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,

    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,

    pub(crate) draw_calls: Arc<RwLock<DrawCalls>>,
}

impl AsRef<WgpuRuntime> for Stage {
    fn as_ref(&self) -> &WgpuRuntime {
        self.geometry.as_ref()
    }
}

impl AsRef<Geometry> for Stage {
    fn as_ref(&self) -> &Geometry {
        &self.geometry
    }
}

impl AsRef<Materials> for Stage {
    fn as_ref(&self) -> &Materials {
        &self.materials
    }
}

impl AsRef<Lighting> for Stage {
    fn as_ref(&self) -> &Lighting {
        &self.lighting
    }
}

#[cfg(gltf)]
/// GLTF functions
impl Stage {
    pub fn load_gltf_document_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<GltfDocument, StageError> {
        use snafu::ResultExt;

        let (document, buffers, images) =
            gltf::import(&path).with_context(|_| crate::gltf::GltfSnafu {
                path: Some(path.as_ref().to_path_buf()),
            })?;
        GltfDocument::from_gltf(self, &document, buffers, images)
    }

    pub fn load_gltf_document_from_bytes(
        &self,
        bytes: impl AsRef<[u8]>,
    ) -> Result<GltfDocument, StageError> {
        let (document, buffers, images) =
            gltf::import_slice(bytes).context(crate::gltf::GltfSnafu { path: None })?;
        GltfDocument::from_gltf(self, &document, buffers, images)
    }
}

/// Geometry functions
impl Stage {
    /// Returns the vertices of a white unit cube.
    ///
    /// This is the mesh of every [`Primitive`] that has not had its vertices set.
    pub fn default_vertices(&self) -> &Vertices {
        self.geometry.default_vertices()
    }

    /// Stage a new [`Camera`] on the GPU.
    ///
    /// If no camera is currently in use on the [`Stage`] through
    /// [`Stage::use_camera`], this new camera will be used automatically.
    pub fn new_camera(&self) -> Camera {
        self.geometry.new_camera()
    }

    /// Use the given camera when rendering.
    pub fn use_camera(&self, camera: impl AsRef<Camera>) {
        self.geometry.use_camera(camera.as_ref());
    }

    /// Return the `Id` of the camera currently in use.
    pub fn used_camera_id(&self) -> Id<CameraDescriptor> {
        self.geometry.descriptor().get().camera_id
    }

    /// Set the default camera `Id`.
    pub fn use_camera_id(&self, camera_id: Id<CameraDescriptor>) {
        self.geometry
            .descriptor()
            .modify(|desc| desc.camera_id = camera_id);
    }

    /// Stage a [`Transform`] on the GPU.
    pub fn new_transform(&self) -> Transform {
        self.geometry.new_transform()
    }

    /// Stage some vertex geometry data.
    pub fn new_vertices(&self, data: impl IntoIterator<Item = Vertex>) -> Vertices {
        self.geometry.new_vertices(data)
    }

    /// Stage some vertex index data.
    pub fn new_indices(&self, data: impl IntoIterator<Item = u32>) -> Indices {
        self.geometry.new_indices(data)
    }

    /// Stage new morph targets.
    pub fn new_morph_targets(
        &self,
        data: impl IntoIterator<Item = Vec<MorphTarget>>,
    ) -> MorphTargets {
        self.geometry.new_morph_targets(data)
    }

    /// Stage new morph target weights.
    pub fn new_morph_target_weights(
        &self,
        data: impl IntoIterator<Item = f32>,
    ) -> MorphTargetWeights {
        self.geometry.new_morph_target_weights(data)
    }

    /// Stage a new skin.
    pub fn new_skin(
        &self,
        joints: impl IntoIterator<Item = impl Into<SkinJoint>>,
        inverse_bind_matrices: impl IntoIterator<Item = impl Into<Mat4>>,
    ) -> Skin {
        self.geometry.new_skin(joints, inverse_bind_matrices)
    }

    /// Stage a new [`Primitive`] on the GPU.
    ///
    /// The returned [`Primitive`] will automatically be added to this [`Stage`].
    ///
    /// The returned [`Primitive`] will have the stage's default [`Vertices`], which is an all-white
    /// unit cube.
    ///
    /// The returned [`Primitive`] uses the stage's default [`Material`], which is white and
    /// **does not** participate in lighting. To change this, first create a [`Material`] with
    /// [`Stage::new_material`] and then call [`Primitive::set_material`] with the new material.
    pub fn new_primitive(&self) -> Primitive {
        Primitive::new(self)
    }

    /// Returns a reference to the descriptor stored at the root of the
    /// geometry slab.
    pub fn geometry_descriptor(&self) -> &Hybrid<GeometryDescriptor> {
        self.geometry.descriptor()
    }
}

/// Materials methods.
impl Stage {
    /// Returns the default [`Material`].
    ///
    /// The default is an all-white matte material.
    pub fn default_material(&self) -> &Material {
        self.materials.default_material()
    }

    /// Stage a new [`Material`] on the GPU.
    ///
    /// The returned [`Material`] can be customized using the builder pattern.
    pub fn new_material(&self) -> Material {
        self.materials.new_material()
    }

    /// Set the size of the atlas.
    ///
    /// This will cause a repacking.
    pub fn set_atlas_size(&self, size: wgpu::Extent3d) -> Result<(), StageError> {
        log::info!("resizing atlas to {size:?}");
        self.materials.atlas().resize(self.runtime(), size)?;
        Ok(())
    }

    /// Add images to the set of atlas images.
    ///
    /// This returns a vector of [`Hybrid<AtlasTexture>`], which
    /// is a descriptor of each image on the GPU. Dropping these entries
    /// will invalidate those images and cause the atlas to be repacked, and any raw
    /// GPU references to the underlying [`AtlasTexture`] will also be invalidated.
    ///     
    /// Adding an image can be quite expensive, as it requires creating a new texture
    /// array for the atlas and repacking all previous images. For that reason it is
    /// good to batch images to reduce the number of calls.
    pub fn add_images(
        &self,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<AtlasTexture>, StageError> {
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let frames = self.materials.atlas().add_images(&images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();

        Ok(frames)
    }

    /// Clear all images from the atlas.
    ///
    /// ## WARNING
    /// This invalidates any previously staged [`AtlasTexture`]s.
    pub fn clear_images(&self) -> Result<(), StageError> {
        let none = Option::<AtlasImage>::None;
        let _ = self.set_images(none)?;
        Ok(())
    }

    /// Set the images to use for the atlas.
    ///
    /// Resets the atlas, packing it with the given images and returning a
    /// vector of the frames already staged.
    ///
    /// ## WARNING
    /// This invalidates any previously staged [`AtlasTexture`]s.
    pub fn set_images(
        &self,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<AtlasTexture>, StageError> {
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let frames = self.materials.atlas().set_images(&images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();

        Ok(frames)
    }
}

/// Lighting methods.
impl Stage {
    /// Stage a new directional light.
    pub fn new_directional_light(&self) -> AnalyticalLight<DirectionalLight> {
        self.lighting.new_directional_light()
    }

    /// Stage a new point light.
    pub fn new_point_light(&self) -> AnalyticalLight<PointLight> {
        self.lighting.new_point_light()
    }

    /// Stage a new spot light.
    pub fn new_spot_light(&self) -> AnalyticalLight<SpotLight> {
        self.lighting.new_spot_light()
    }

    /// Add an [`AnalyticalLight`] to the internal list of lights.
    ///
    /// This is called implicitly by `Stage::new_*_light`.
    ///
    /// This can be used to add the light back to the scene after using
    /// [`Stage::remove_light`].
    pub fn add_light<T>(&self, bundle: &AnalyticalLight<T>)
    where
        T: IsLight,
        Light: From<T>,
    {
        self.lighting.add_light(bundle)
    }

    /// Remove an [`AnalyticalLight`] from the internal list of lights.
    ///
    /// Use this to exclude a light from rendering, without dropping the light.
    ///
    /// After calling this function you can include the light again using [`Stage::add_light`].
    pub fn remove_light<T: IsLight>(&self, bundle: &AnalyticalLight<T>) {
        self.lighting.remove_light(bundle);
    }

    /// Enable shadow mapping for the given [`AnalyticalLight`], creating
    /// a new [`ShadowMap`].
    ///
    /// ## Tips for making a good shadow map
    ///
    /// 1. Make sure the map is big enough.
    ///    Using a big map can fix some peter panning issues, even before
    ///    playing with bias in the returned [`ShadowMap`].
    ///    The bigger the map, the cleaner the shadows will be. This can
    ///    also solve PCF problems.
    /// 2. Don't set PCF samples too high in the returned [`ShadowMap`], as
    ///    this can _cause_ peter panning.
    /// 3. Ensure the **znear** and **zfar** parameters make sense, as the
    ///    shadow map uses these to determine how much of the scene to cover.
    ///    If you find that shadows are cut off in a straight line, it's likely
    ///    `znear` or `zfar` needs adjustment.
    pub fn new_shadow_map<T>(
        &self,
        analytical_light: &AnalyticalLight<T>,
        // Size of the shadow map
        size: UVec2,
        // Distance to the near plane of the shadow map's frustum.
        //
        // Only objects within the shadow map's frustum will cast shadows.
        z_near: f32,
        // Distance to the far plane of the shadow map's frustum
        //
        // Only objects within the shadow map's frustum will cast shadows.
        z_far: f32,
    ) -> Result<ShadowMap, StageError>
    where
        T: IsLight,
        Light: From<T>,
    {
        Ok(self
            .lighting
            .new_shadow_map(analytical_light, size, z_near, z_far)?)
    }

    /// Enable light tiling, creating a new [`LightTiling`].
    pub fn new_light_tiling(&self, config: LightTilingConfig) -> LightTiling {
        let lighting = self.as_ref();
        let multisampled = self.get_msaa_sample_count() > 1;
        let depth_texture_size = self.get_depth_texture().size();
        LightTiling::new(
            lighting,
            multisampled,
            UVec2::new(depth_texture_size.width, depth_texture_size.height),
            config,
        )
    }
}

/// Skybox methods
impl Stage {
    /// Return the cached skybox render pipeline, creating it if necessary.
    fn get_skybox_pipeline_and_bindgroup(
        &self,
        geometry_slab_buffer: &wgpu::Buffer,
    ) -> (Arc<SkyboxRenderPipeline>, Arc<wgpu::BindGroup>) {
        let msaa_sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline_guard = self.skybox_pipeline.write().unwrap();
        let pipeline = if let Some(pipeline) = pipeline_guard.as_mut() {
            if pipeline.msaa_sample_count() != msaa_sample_count {
                *pipeline = Arc::new(crate::skybox::create_skybox_render_pipeline(
                    self.device(),
                    Texture::HDR_TEXTURE_FORMAT,
                    Some(msaa_sample_count),
                ));
            }
            pipeline.clone()
        } else {
            let pipeline = Arc::new(crate::skybox::create_skybox_render_pipeline(
                self.device(),
                Texture::HDR_TEXTURE_FORMAT,
                Some(msaa_sample_count),
            ));
            *pipeline_guard = Some(pipeline.clone());
            pipeline
        };
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.skybox_bindgroup.lock().unwrap();
        let bindgroup = if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let bg = Arc::new(crate::skybox::create_skybox_bindgroup(
                self.device(),
                geometry_slab_buffer,
                self.skybox.read().unwrap().environment_cubemap_texture(),
            ));
            *bindgroup = Some(bg.clone());
            bg
        };
        (pipeline, bindgroup)
    }

    /// Used the given [`Skybox`].
    ///
    /// To remove the currently used [`Skybox`], call [`Skybox::remove_skybox`].
    pub fn use_skybox(&self, skybox: &Skybox) -> &Self {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut guard = self.skybox.write().unwrap();
        *guard = skybox.clone();
        self.has_skybox
            .store(true, std::sync::atomic::Ordering::Relaxed);
        *self.skybox_bindgroup.lock().unwrap() = None;
        *self.textures_bindgroup.lock().unwrap() = None;
        self
    }

    /// Removes the currently used [`Skybox`].
    ///
    /// Returns the currently used [`Skybox`], if any.
    ///
    /// After calling this the [`Stage`] will not render with any [`Skybox`], until
    /// [`Skybox::use_skybox`] is called with another [`Skybox`].
    pub fn remove_skybox(&self) -> Option<Skybox> {
        let mut guard = self.skybox.write().unwrap();
        if guard.is_empty() {
            // Do nothing, the skybox is already empty
            None
        } else {
            let skybox = guard.clone();
            *guard = Skybox::empty(self.runtime());
            self.skybox_bindgroup.lock().unwrap().take();
            self.textures_bindgroup.lock().unwrap().take();
            Some(skybox)
        }
    }

    /// Returns a new [`Skybox`] using the HDR image at the given path, if possible.
    ///
    /// The returned [`Skybox`] must be **used** with [`Stage::use_skybox`].
    pub fn new_skybox_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_path(path)?;
        Ok(Skybox::new(self.runtime(), hdr))
    }

    /// Returns a new [`Skybox`] using the bytes of an HDR image, if possible.
    ///
    /// The returned [`Skybox`] must be **used** with [`Stage::use_skybox`].
    pub fn new_skybox_from_bytes(&self, bytes: &[u8]) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_bytes(bytes)?;
        Ok(Skybox::new(self.runtime(), hdr))
    }
}

/// Image based lighting methods
impl Stage {
    /// Crate a new [`Ibl`] from the given [`Skybox`].
    pub fn new_ibl(&self, skybox: &Skybox) -> Ibl {
        Ibl::new(self.runtime(), skybox)
    }

    /// Use the given image based lighting.
    ///
    /// Use [`Stage::new_ibl`] to create a new [`Ibl`].
    pub fn use_ibl(&self, ibl: &Ibl) -> &Self {
        let mut guard = self.ibl.write().unwrap();
        *guard = ibl.clone();
        self.primitive_bind_group.invalidate();
        self
    }

    /// Remove the current image based lighting from the stage and return it, if any.
    pub fn remove_ibl(&self) -> Option<Ibl> {
        let mut guard = self.ibl.write().unwrap();
        if guard.is_empty() {
            // Do nothing, we're already not using IBL
            None
        } else {
            let ibl = guard.clone();
            *guard = Ibl::new(self.runtime(), &Skybox::empty(self.runtime()));
            self.primitive_bind_group.invalidate();
            Some(ibl)
        }
    }
}

impl Stage {
    /// Returns the runtime.
    pub fn runtime(&self) -> &WgpuRuntime {
        self.as_ref()
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.runtime().device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.runtime().queue
    }

    /// Returns a reference to the [`BrdfLut`].
    ///
    /// This is used for creating skyboxes used in image based lighting.
    pub fn brdf_lut(&self) -> &BrdfLut {
        &self.brdf_lut
    }

    /// Sum the byte size of all used GPU memory.
    ///
    /// Adds together the byte size of all underlying slab buffers.
    ///
    /// ## Note
    /// This does not take into consideration staged data that has not yet
    /// been committed with either [`Stage::commit`] or [`Stage::render`].
    pub fn used_gpu_buffer_byte_size(&self) -> usize {
        let num_u32s = self.geometry.slab_allocator().len()
            + self.lighting.slab_allocator().len()
            + self.materials.slab_allocator().len()
            + self.bloom.slab_allocator().len()
            + self.tonemapping.slab_allocator().len()
            + self
                .draw_calls
                .read()
                .unwrap()
                .drawing_strategy()
                .as_indirect()
                .map(|draws| draws.slab_allocator().len())
                .unwrap_or_default();
        4 * num_u32s
    }

    pub fn hdr_texture(&self) -> impl Deref<Target = crate::texture::Texture> + '_ {
        self.hdr_texture.read().unwrap()
    }

    /// Run all upkeep and commit all staged changes to the GPU.
    ///
    /// This is done implicitly in [`Stage::render`].
    ///
    /// This can be used after dropping resources to reclaim those resources on the GPU.
    #[must_use]
    pub fn commit(&self) -> StageCommitResult {
        let (materials_atlas_texture_was_recreated, materials_buffer) = self.materials.commit();
        if materials_atlas_texture_was_recreated {
            self.primitive_bind_group.invalidate();
        }
        let geometry_buffer = self.geometry.commit();
        let lighting_buffer = self.lighting.commit();
        StageCommitResult {
            geometry_buffer,
            lighting_buffer,
            materials_buffer,
        }
    }

    fn primitive_pipeline_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let geometry_slab = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let material_slab = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        fn image2d_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
            let img = wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            };
            let sampler = wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            };
            (img, sampler)
        }

        fn cubemap_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
            let img = wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: false,
                },
                count: None,
            };
            let sampler = wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            };
            (img, sampler)
        }

        let atlas = wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2Array,
                multisampled: false,
            },
            count: None,
        };
        let atlas_sampler = wgpu::BindGroupLayoutEntry {
            binding: 3,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        };
        let (irradiance, irradiance_sampler) = cubemap_entry(4);
        let (prefilter, prefilter_sampler) = cubemap_entry(6);
        let (brdf, brdf_sampler) = image2d_entry(8);

        let LightingBindGroupLayoutEntries {
            light_slab,
            shadow_map_image,
            shadow_map_sampler,
        } = LightingBindGroupLayoutEntries::new(10);

        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("primitive"),
            entries: &[
                geometry_slab,
                material_slab,
                atlas,
                atlas_sampler,
                irradiance,
                irradiance_sampler,
                prefilter,
                prefilter_sampler,
                brdf,
                brdf_sampler,
                light_slab,
                shadow_map_image,
                shadow_map_sampler,
            ],
        })
    }

    pub fn create_primitive_pipeline(
        device: &wgpu::Device,
        fragment_color_format: wgpu::TextureFormat,
        multisample_count: u32,
    ) -> wgpu::RenderPipeline {
        log::trace!("creating stage render pipeline");
        let label = Some("primitive");
        let vertex_linkage = crate::linkage::primitive_vertex::linkage(device);
        let fragment_linkage = crate::linkage::primitive_fragment::linkage(device);

        let bind_group_layout = Self::primitive_pipeline_bindgroup_layout(device);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: Some(vertex_linkage.entry_point),
                buffers: &[],
                compilation_options: Default::default(),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                mask: !0,
                alpha_to_coverage_enabled: false,
                count: multisample_count,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: Some(fragment_linkage.entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format: fragment_color_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            multiview: None,
            cache: None,
        })
    }

    /// Create a new stage.
    pub fn new(ctx: &crate::context::Context) -> Self {
        let runtime = ctx.runtime();
        let device = &runtime.device;
        let resolution @ UVec2 { x: w, y: h } = ctx.get_size();
        let stage_config = *ctx.stage_config.read().unwrap();
        let geometry = Geometry::new(
            ctx,
            resolution,
            UVec2::new(
                stage_config.atlas_size.width,
                stage_config.atlas_size.height,
            ),
        );
        let materials = Materials::new(runtime, stage_config.atlas_size);
        let multisample_count = 1;
        let hdr_texture = Arc::new(RwLock::new(Texture::create_hdr_texture(
            device,
            w,
            h,
            multisample_count,
        )));
        let depth_texture =
            Texture::create_depth_texture(device, w, h, multisample_count, Some("stage-depth"));
        let msaa_render_target = Default::default();
        // UNWRAP: safe because no other references at this point (created above^)
        let bloom = Bloom::new(ctx, &hdr_texture.read().unwrap());
        let tonemapping = Tonemapping::new(
            runtime,
            ctx.get_render_target().format().add_srgb_suffix(),
            &bloom.get_mix_texture(),
        );
        let stage_pipeline = Self::create_primitive_pipeline(
            device,
            wgpu::TextureFormat::Rgba16Float,
            multisample_count,
        );
        let geometry_buffer = geometry.slab_allocator().commit();
        let lighting = Lighting::new(stage_config.shadow_map_atlas_size, &geometry);

        let brdf_lut = BrdfLut::new(runtime);
        let skybox = Skybox::empty(runtime);
        let ibl = Ibl::new(runtime, &skybox);

        Self {
            materials,
            draw_calls: Arc::new(RwLock::new(DrawCalls::new(
                ctx,
                ctx.get_use_direct_draw(),
                &geometry_buffer,
                &depth_texture,
            ))),
            lighting,
            depth_texture: Arc::new(RwLock::new(depth_texture)),
            stage_slab_buffer: Arc::new(RwLock::new(geometry_buffer)),
            geometry,

            primitive_pipeline: Arc::new(RwLock::new(stage_pipeline)),
            primitive_bind_group: ManagedBindGroup::default(),
            primitive_bind_group_created: Arc::new(0.into()),

            ibl: Arc::new(RwLock::new(ibl)),
            skybox: Arc::new(RwLock::new(skybox)),
            skybox_bindgroup: Default::default(),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            brdf_lut,
            bloom,
            tonemapping,
            has_bloom: AtomicBool::from(true).into(),
            textures_bindgroup: Default::default(),
            debug_overlay: DebugOverlay::new(device, ctx.get_render_target().format()),
            has_debug_overlay: Arc::new(false.into()),
            hdr_texture,
            msaa_render_target,
            msaa_sample_count: Arc::new(multisample_count.into()),
            clear_color_attachments: Arc::new(true.into()),
            clear_depth_attachments: Arc::new(true.into()),
            background_color: Arc::new(RwLock::new(wgpu::Color::TRANSPARENT)),
        }
    }

    pub fn set_background_color(&self, color: impl Into<Vec4>) {
        let color = color.into();
        *self.background_color.write().unwrap() = wgpu::Color {
            r: color.x as f64,
            g: color.y as f64,
            b: color.z as f64,
            a: color.w as f64,
        };
    }

    pub fn with_background_color(self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    /// Return the multisample count.
    pub fn get_msaa_sample_count(&self) -> u32 {
        self.msaa_sample_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Set the MSAA multisample count.
    ///
    /// Set to `1` to disable MSAA. Setting to `0` will be treated the same as
    /// setting to `1`.
    pub fn set_msaa_sample_count(&self, multisample_count: u32) {
        let multisample_count = multisample_count.max(1);
        let prev_multisample_count = self
            .msaa_sample_count
            .swap(multisample_count, Ordering::Relaxed);
        if prev_multisample_count == multisample_count {
            log::warn!("set_multisample_count: multisample count is unchanged, noop");
            return;
        }

        log::debug!("setting multisample count to {multisample_count}");
        // UNWRAP: POP
        *self.primitive_pipeline.write().unwrap() = Self::create_primitive_pipeline(
            self.device(),
            wgpu::TextureFormat::Rgba16Float,
            multisample_count,
        );
        let size = self.get_size();
        // UNWRAP: POP
        *self.depth_texture.write().unwrap() = Texture::create_depth_texture(
            self.device(),
            size.x,
            size.y,
            multisample_count,
            Some("stage-depth"),
        );
        // UNWRAP: POP
        let format = self.hdr_texture.read().unwrap().texture.format();
        *self.msaa_render_target.write().unwrap() = if multisample_count == 1 {
            None
        } else {
            Some(create_msaa_textureview(
                self.device(),
                size.x,
                size.y,
                format,
                multisample_count,
            ))
        };

        // Invalidate the textures bindgroup - it must be recreated
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    /// Set the MSAA multisample count.
    ///
    /// Set to `1` to disable MSAA. Setting to `0` will be treated the same as
    /// setting to `1`.
    pub fn with_msaa_sample_count(self, multisample_count: u32) -> Self {
        self.set_msaa_sample_count(multisample_count);
        self
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn set_clear_color_attachments(&self, should_clear: bool) {
        self.clear_color_attachments
            .store(should_clear, Ordering::Relaxed);
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn with_clear_color_attachments(self, should_clear: bool) -> Self {
        self.set_clear_color_attachments(should_clear);
        self
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn set_clear_depth_attachments(&self, should_clear: bool) {
        self.clear_depth_attachments
            .store(should_clear, Ordering::Relaxed);
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn with_clear_depth_attachments(self, should_clear: bool) -> Self {
        self.set_clear_depth_attachments(should_clear);
        self
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&self, debug_mode: DebugChannel) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.debug_channel = debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(self, debug_mode: DebugChannel) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether to render the debug overlay.
    pub fn set_use_debug_overlay(&self, use_debug_overlay: bool) {
        self.has_debug_overlay
            .store(use_debug_overlay, std::sync::atomic::Ordering::Relaxed);
    }

    /// Set whether to render the debug overlay.
    pub fn with_debug_overlay(self, use_debug_overlay: bool) -> Self {
        self.set_use_debug_overlay(use_debug_overlay);
        self
    }

    /// Set whether to use frustum culling on GPU before drawing.
    ///
    /// This defaults to `true`.
    pub fn set_use_frustum_culling(&self, use_frustum_culling: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.perform_frustum_culling = use_frustum_culling);
    }

    /// Set whether to render the debug overlay.
    pub fn with_frustum_culling(self, use_frustum_culling: bool) -> Self {
        self.set_use_frustum_culling(use_frustum_culling);
        self
    }

    /// Set whether to use occlusion culling on GPU before drawing.
    ///
    /// This defaults to `false`.
    ///
    /// ## Warning
    ///
    /// Occlusion culling is a feature in development. YMMV.
    pub fn set_use_occlusion_culling(&self, use_occlusion_culling: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.perform_occlusion_culling = use_occlusion_culling);
    }

    /// Set whether to render the debug overlay.
    pub fn with_occlusion_culling(self, use_occlusion_culling: bool) -> Self {
        self.set_use_occlusion_culling(use_occlusion_culling);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&self, use_lighting: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.has_lighting = use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set whether to use vertex skinning.
    pub fn set_has_vertex_skinning(&self, use_skinning: bool) {
        self.geometry
            .descriptor()
            .modify(|cfg| cfg.has_skinning = use_skinning);
    }

    /// Set whether to use vertex skinning.
    pub fn with_vertex_skinning(self, use_skinning: bool) -> Self {
        self.set_has_vertex_skinning(use_skinning);
        self
    }

    pub fn get_size(&self) -> UVec2 {
        // UNWRAP: panic on purpose
        let hdr = self.hdr_texture.read().unwrap();
        let w = hdr.width();
        let h = hdr.height();
        UVec2::new(w, h)
    }

    pub fn set_size(&self, size: UVec2) {
        if size == self.get_size() {
            return;
        }

        self.geometry
            .descriptor()
            .modify(|cfg| cfg.resolution = size);
        let hdr_texture = Texture::create_hdr_texture(self.device(), size.x, size.y, 1);
        let sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        if let Some(msaa_view) = self.msaa_render_target.write().unwrap().as_mut() {
            *msaa_view = create_msaa_textureview(
                self.device(),
                size.x,
                size.y,
                hdr_texture.texture.format(),
                sample_count,
            );
        }

        // UNWRAP: panic on purpose
        *self.depth_texture.write().unwrap() = Texture::create_depth_texture(
            self.device(),
            size.x,
            size.y,
            sample_count,
            Some("stage-depth"),
        );
        self.bloom.set_hdr_texture(self.runtime(), &hdr_texture);
        self.tonemapping
            .set_hdr_texture(self.device(), &hdr_texture);
        *self.hdr_texture.write().unwrap() = hdr_texture;

        let _ = self.skybox_bindgroup.lock().unwrap().take();
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    pub fn with_size(self, size: UVec2) -> Self {
        self.set_size(size);
        self
    }

    /// Turn the bloom effect on or off.
    pub fn set_has_bloom(&self, has_bloom: bool) {
        self.has_bloom
            .store(has_bloom, std::sync::atomic::Ordering::Relaxed);
    }

    /// Turn the bloom effect on or off.
    pub fn with_bloom(self, has_bloom: bool) -> Self {
        self.set_has_bloom(has_bloom);
        self
    }

    /// Set the amount of bloom that is mixed in with the input image.
    ///
    /// Defaults to `0.04`.
    pub fn set_bloom_mix_strength(&self, strength: f32) {
        self.bloom.set_mix_strength(strength);
    }

    pub fn with_bloom_mix_strength(self, strength: f32) -> Self {
        self.set_bloom_mix_strength(strength);
        self
    }

    /// Sets the bloom filter radius, in pixels.
    ///
    /// Default is `1.0`.
    pub fn set_bloom_filter_radius(&self, filter_radius: f32) {
        self.bloom.set_filter_radius(filter_radius);
    }

    /// Sets the bloom filter radius, in pixels.
    ///
    /// Default is `1.0`.
    pub fn with_bloom_filter_radius(self, filter_radius: f32) -> Self {
        self.set_bloom_filter_radius(filter_radius);
        self
    }

    /// Adds a primitive to the internal list of primitives to be drawn each
    /// frame.
    ///
    /// Returns the number of primitives added.
    ///
    /// If you drop the primitive and no other references are kept, it will be
    /// removed automatically from the internal list and will cease to be
    /// drawn each frame.
    pub fn add_primitive(&self, primitive: &Primitive) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draw_calls.write().unwrap();
        draws.add_primitive(primitive)
    }

    /// Erase the given primitive from the internal list of primitives to be
    /// drawn each frame.
    ///
    /// Returns the number of primitives added.
    pub fn remove_primitive(&self, primitive: &Primitive) -> usize {
        let mut draws = self.draw_calls.write().unwrap();
        draws.remove_primitive(primitive)
    }

    /// Sort the drawing order of primitives.
    ///
    /// This determines the order in which [`Primitive`]s are drawn each frame.
    pub fn sort_primitive(&self, f: impl Fn(&Primitive, &Primitive) -> std::cmp::Ordering) {
        // UNWRAP: panic on purpose
        let mut guard = self.draw_calls.write().unwrap();
        guard.sort_primitives(f);
    }

    /// Returns a clone of the current depth texture.
    pub fn get_depth_texture(&self) -> DepthTexture {
        DepthTexture {
            runtime: self.runtime().clone(),
            texture: self.depth_texture.read().unwrap().texture.clone(),
        }
    }

    /// Create a new [`NestedTransform`].
    pub fn new_nested_transform(&self) -> NestedTransform {
        NestedTransform::new(self.geometry.slab_allocator())
    }

    /// Render the staged scene into the given view.
    pub fn render(&self, view: &wgpu::TextureView) {
        // UNWRAP: POP
        let background_color = *self.background_color.read().unwrap();
        // UNWRAP: POP
        let msaa_target = self.msaa_render_target.read().unwrap();
        let clear_colors = self.clear_color_attachments.load(Ordering::Relaxed);
        let hdr_texture = self.hdr_texture.read().unwrap();

        let mk_ops = |store| wgpu::Operations {
            load: if clear_colors {
                wgpu::LoadOp::Clear(background_color)
            } else {
                wgpu::LoadOp::Load
            },
            store,
        };
        let render_pass_color_attachment = if let Some(msaa_view) = msaa_target.as_ref() {
            wgpu::RenderPassColorAttachment {
                ops: mk_ops(wgpu::StoreOp::Discard),
                view: msaa_view,
                resolve_target: Some(&hdr_texture.view),
                depth_slice: None,
            }
        } else {
            wgpu::RenderPassColorAttachment {
                ops: mk_ops(wgpu::StoreOp::Store),
                view: &hdr_texture.view,
                resolve_target: None,
                depth_slice: None,
            }
        };

        let depth_texture = self.depth_texture.read().unwrap();
        let clear_depth = self.clear_depth_attachments.load(Ordering::Relaxed);
        let render_pass_depth_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: if clear_depth {
                    wgpu::LoadOp::Clear(1.0)
                } else {
                    wgpu::LoadOp::Load
                },
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let pipeline_guard = self.primitive_pipeline.read().unwrap();
        let (_submission_index, maybe_indirect_buffer) = StageRendering {
            pipeline: &pipeline_guard,
            stage: self,
            color_attachment: render_pass_color_attachment,
            depth_stencil_attachment: render_pass_depth_attachment,
        }
        .run();

        // then render bloom
        if self.has_bloom.load(Ordering::Relaxed) {
            self.bloom.bloom(self.device(), self.queue());
        } else {
            // copy the input hdr texture to the bloom mix texture
            let mut encoder =
                self.device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("no bloom copy"),
                    });
            let bloom_mix_texture = self.bloom.get_mix_texture();
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.hdr_texture.read().unwrap().texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &bloom_mix_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: bloom_mix_texture.width(),
                    height: bloom_mix_texture.height(),
                    depth_or_array_layers: 1,
                },
            );
            self.queue().submit(std::iter::once(encoder.finish()));
        }

        // then render tonemapping
        self.tonemapping.render(self.device(), self.queue(), view);

        // then render the debug overlay
        if self.has_debug_overlay.load(Ordering::Relaxed) {
            if let Some(indirect_draw_buffer) = maybe_indirect_buffer {
                self.debug_overlay.render(
                    self.device(),
                    self.queue(),
                    view,
                    &self.stage_slab_buffer.read().unwrap(),
                    &indirect_draw_buffer,
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use craballoc::{runtime::CpuRuntime, slab::SlabAllocator};
    use crabslab::{Array, Id, Slab};
    use glam::{Mat4, Vec2, Vec3, Vec4};

    use crate::{
        context::Context,
        geometry::{shader::GeometryDescriptor, Geometry, Vertex},
        test::BlockOnFuture,
        transform::NestedTransform,
    };

    #[test]
    fn vertex_slab_roundtrip() {
        let initial_vertices = {
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(1.0, 0.0, 0.0))
                .with_uv0(Vec2::new(1.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, 1.0, 0.0))
                .with_uv0(Vec2::new(0.0, 1.0));
            let br = Vertex::default()
                .with_position(Vec3::new(1.0, 1.0, 0.0))
                .with_uv0(Vec2::splat(1.0));
            vec![tl, bl, br, tl, br, tr]
        };
        let mut slab = [0u32; 256];
        slab.write_indexed_slice(&initial_vertices, 0);
        let vertices = slab.read_vec(Array::<Vertex>::new(0, initial_vertices.len() as u32));
        pretty_assertions::assert_eq!(initial_vertices, vertices);
    }

    #[test]
    fn matrix_subtraction_sanity() {
        let m = Mat4::IDENTITY - Mat4::IDENTITY;
        assert_eq!(Mat4::ZERO, m);
    }

    #[test]
    fn can_global_transform_calculation() {
        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "This is just riff-raff, as it doesn't compile without the borrow."
        )]
        let slab = SlabAllocator::<CpuRuntime>::new(&CpuRuntime, "transform", ());
        // Setup a hierarchy of transforms
        let root = NestedTransform::new(&slab);
        let child = NestedTransform::new(&slab).with_local_translation(Vec3::new(1.0, 0.0, 0.0));
        let grandchild =
            NestedTransform::new(&slab).with_local_translation(Vec3::new(1.0, 0.0, 0.0));
        log::info!("hierarchy");
        // Build the hierarchy
        root.add_child(&child);
        child.add_child(&grandchild);

        log::info!("get_global_transform");
        // Calculate global transforms
        let grandchild_global_transform = grandchild.global_descriptor();

        // Assert that the global transform is as expected
        assert_eq!(
            grandchild_global_transform.translation.x, 2.0,
            "Grandchild's global translation should   2.0 along the x-axis"
        );
    }

    #[test]
    fn can_msaa() {
        let ctx = Context::headless(100, 100).block();
        let stage = ctx
            .new_stage()
            .with_background_color([1.0, 1.0, 1.0, 1.0])
            .with_lighting(false);
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let _triangle_rez = stage.new_primitive().with_vertices(
            stage.new_vertices([
                Vertex::default()
                    .with_position([10.0, 10.0, 0.0])
                    .with_color([0.0, 1.0, 1.0, 1.0]),
                Vertex::default()
                    .with_position([10.0, 90.0, 0.0])
                    .with_color([1.0, 1.0, 0.0, 1.0]),
                Vertex::default()
                    .with_position([90.0, 10.0, 0.0])
                    .with_color([1.0, 0.0, 1.0, 1.0]),
            ]),
        );

        log::debug!("rendering without msaa");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq_cfg(
            "msaa/without.png",
            img,
            img_diff::DiffCfg {
                pixel_threshold: img_diff::LOW_PIXEL_THRESHOLD,
                ..Default::default()
            },
        );
        frame.present();
        log::debug!("  all good!");

        stage.set_msaa_sample_count(4);
        log::debug!("rendering with msaa");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq_cfg(
            "msaa/with.png",
            img,
            img_diff::DiffCfg {
                pixel_threshold: img_diff::LOW_PIXEL_THRESHOLD,
                ..Default::default()
            },
        );
        frame.present();
    }

    #[test]
    /// Tests that the PBR descriptor is written to slot 0 of the geometry buffer,
    /// and that it contains what we think it contains.
    fn stage_geometry_desc_sanity() {
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage();
        let _ = stage.commit();

        let slab = futures_lite::future::block_on({
            let geometry: &Geometry = stage.as_ref();
            geometry.slab_allocator().read(..)
        })
        .unwrap();
        let pbr_desc = slab.read_unchecked(Id::<GeometryDescriptor>::new(0));
        pretty_assertions::assert_eq!(stage.geometry_descriptor().get(), pbr_desc);
    }

    #[test]
    fn slabbed_vertices_native() {
        let ctx = Context::headless(100, 100).block();
        let runtime = ctx.as_ref();

        // Create our geometry on the slab.
        let slab = SlabAllocator::new(
            runtime,
            "slabbed_isosceles_triangle",
            wgpu::BufferUsages::empty(),
        );

        let geometry = vec![
            (Vec3::new(0.5, -0.5, 0.0), Vec4::new(1.0, 0.0, 0.0, 1.0)),
            (Vec3::new(0.0, 0.5, 0.0), Vec4::new(0.0, 1.0, 0.0, 1.0)),
            (Vec3::new(-0.5, -0.5, 0.0), Vec4::new(0.0, 0.0, 1.0, 1.0)),
            (Vec3::new(-1.0, 1.0, 0.0), Vec4::new(1.0, 0.0, 0.0, 1.0)),
            (Vec3::new(-1.0, 0.0, 0.0), Vec4::new(0.0, 1.0, 0.0, 1.0)),
            (Vec3::new(0.0, 1.0, 0.0), Vec4::new(0.0, 0.0, 1.0, 1.0)),
        ];
        let vertices = slab.new_array(geometry);
        let array = slab.new_value(vertices.array());

        // Create a bindgroup for the slab so our shader can read out the types.
        let bindgroup_layout =
            runtime
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        let pipeline_layout =
            runtime
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bindgroup_layout],
                    push_constant_ranges: &[],
                });

        let vertex = crate::linkage::slabbed_vertices::linkage(&runtime.device);
        let fragment = crate::linkage::passthru_fragment::linkage(&runtime.device);
        let pipeline = runtime
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                cache: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    module: &vertex.module,
                    entry_point: Some(vertex.entry_point),
                    buffers: &[],
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
                    compilation_options: Default::default(),
                    module: &fragment.module,
                    entry_point: Some(fragment.entry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });
        let slab_buffer = slab.commit();

        let bindgroup = runtime
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bindgroup_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: slab_buffer.as_entire_binding(),
                }],
            });

        let frame = ctx.get_next_frame().unwrap();
        let mut encoder = runtime.device.create_command_encoder(&Default::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&pipeline);
            render_pass.set_bind_group(0, &bindgroup, &[]);
            let id = array.id().inner();
            render_pass.draw(0..vertices.len() as u32, id..id + 1);
        }
        runtime.queue.submit(std::iter::once(encoder.finish()));

        let img = frame
            .read_linear_image()
            .block()
            .expect("could not read frame");
        img_diff::assert_img_eq("tutorial/slabbed_isosceles_triangle.png", img);
    }
}
