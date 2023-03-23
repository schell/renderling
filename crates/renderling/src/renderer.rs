//! Builds the UI pipeline and manages resources.
use moongraph::Graph;
use snafu::prelude::*;
use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use crate::linkage::ObjectDraw;
use crate::{
    camera::*, resources::*, AnyMaterial, AnyMaterialUniform, AnyPipeline, Lights, Material,
    Object, ObjectBuilder, Objects, Pipeline, Transform, WgpuState,
};

#[derive(Debug, Snafu)]
pub enum RenderlingError {
    #[snafu(display("wgpu stat is missing the current frame"))]
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
}

/// Full-fledged renderer.
pub struct Renderer {
    graph: Graph,
}

impl Renderer {
    /// Create a new full-fledged renderer.
    pub fn new(gpu: WgpuState) -> Self {
        let graph = Graph::default().with_resource(gpu);
        Renderer { graph }
    }

    pub fn gpu(&self) -> &WgpuState {
        // UNWRAP: safe because we always have the gpu
        self.graph.get_resource().unwrap().unwrap()
    }

    pub fn gpu_mut(&mut self) -> &mut WgpuState {
        // UNWRAP: safe because we always have the gpu
        self.graph.get_resource_mut().unwrap().unwrap()
    }
}

/// A renderer typed by its pipeline.
///
/// `Renderling` manages GPU resources for cameras, materials and objects.
pub struct Renderling<P: Pipeline> {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
    pub(crate) size: Arc<RwLock<(u32, u32)>>,

    // Lighting
    pub(crate) lights: Lights,
    pub(crate) cameras: Cameras,
    pub(crate) objects: Objects,
    // Built shader render pipeline
    pipeline: AnyPipeline,
    // default material to use when there is no other
    default_material: AnyMaterial,
    // default material bindgroup
    default_material_uniform: Option<AnyMaterialUniform>,
    // whether object meshes have a 3x3 normal matrix attribute (defaults to `true`)
    meshes_have_normal_matrix_attribute: bool,
    // The index of the camera's bindgroup
    camera_bindgroup_index: u32,
    // The index of the lights's bindgroup
    light_bindgroup_index: u32,
    // The index of the material's bindgroup
    material_bindgroup_index: u32,

    _phantom: PhantomData<P>,
}

impl<P: Pipeline> Renderling<P> {
    pub fn new<M: Material>(
        wgpu_state: &crate::WgpuState,
        pipeline: impl Into<Arc<P>>,
        material: impl Into<Arc<M>>,
        meshes_have_normal_matrix_attribute: bool,
    ) -> Self {
        let r = Self {
            device: wgpu_state.device.clone(),
            queue: wgpu_state.queue.clone(),
            size: wgpu_state.size.clone(),
            pipeline: AnyPipeline::new::<P>(pipeline.into()),
            cameras: Cameras::default(),
            objects: Objects::default(),
            lights: Lights::default(),
            meshes_have_normal_matrix_attribute,
            default_material: AnyMaterial::new::<M>(material.into()),
            default_material_uniform: None,
            camera_bindgroup_index: 0,
            material_bindgroup_index: 1,
            light_bindgroup_index: 2,
            _phantom: PhantomData,
        };
        r
    }

    /// Create a new camera builder.
    pub fn new_camera(&mut self) -> CameraBuilder<'_> {
        let (width, height) = *self.size.read().unwrap();
        CameraBuilder {
            inner: CameraInner::new_perspective(width as f32, height as f32),
            width: width as f32,
            height: height as f32,
            device: &self.device,
            update_tx: self.cameras.camera_update_queue.0.clone(),
            cameras: &mut self.cameras,
        }
    }

    /// Retrieves an iterator over all cameras.
    ///
    /// This will always have at least one camera in it.
    pub fn cameras(&self) -> impl Iterator<Item = Camera> + '_ {
        self.cameras.iter().enumerate().filter_map(|(i, data)| {
            let data = data.as_ref()?;
            Some(Camera {
                id: Id::new(i),
                inner: data.inner.clone(),
                cmd: self.cameras.camera_update_queue.0.clone(),
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
    /// The builder can be used to customize the object befoure building it.
    ///
    /// If no material is provided, the renderling's default material will be
    /// used.
    ///
    /// If no transform is provided, the object will be positioned at the origin
    /// with no rotation and scale 1,1,1.
    pub fn new_object(&mut self) -> ObjectBuilder<'_> {
        ObjectBuilder {
            mesh: None,
            material: None,
            children: vec![],
            local_transform: Transform::default(),
            local_transforms: vec![],
            generate_normal_matrix: self.meshes_have_normal_matrix_attribute,
            is_visible: true,
            update_tx: self.objects.object_update_queue.0.clone(),
            device: &self.device,
            objects: &mut self.objects,
        }
    }

    pub fn new_point_light(&mut self) -> crate::PointLightBuilder {
        crate::PointLightBuilder::new(&mut self.lights)
    }

    pub fn new_spot_light(&mut self) -> crate::SpotLightBuilder {
        crate::SpotLightBuilder::new(&mut self.lights)
    }

    pub fn new_directional_light(&mut self) -> crate::DirectionalLightBuilder {
        crate::DirectionalLightBuilder::new(&mut self.lights)
    }

    /// Conduct all updates made from outside the renderling.
    ///
    /// This must be called in order to display any changes.
    pub fn update(&mut self) -> Result<(), RenderlingError> {
        // lights, camera, objects!
        self.lights.update(&self.device, &self.queue);
        let camera_objects_need_sorting = self.objects.update(&self.device, &self.queue);
        self.cameras
            .update(&self.device, camera_objects_need_sorting, &self.objects);

        // update default material
        if self.default_material_uniform.is_none() {
            log::trace!("updating default material");
            self.default_material_uniform =
                Some(self.default_material.create_material_uniform(&self.device));
        }

        Ok(())
    }

    pub fn set_default_material<T: Material>(&mut self, material: impl Into<Arc<T>>) {
        self.default_material_uniform = None;
        self.default_material = AnyMaterial::new(material);
    }

    pub fn get_object_ids_sorted_by_distance_to_camera(&self, camera: &Camera) -> &Vec<Id<Object>> {
        self.cameras
            .get_object_ids_sorted_by_distance_to_camera(camera)
    }

    /// Conduct a full render pass into the given textures using the given
    /// camera and objects.
    pub fn render_object_ids<'a>(
        &'a self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera: &Id<Camera>,
        objects: impl Iterator<Item = &'a Id<Object>>,
    ) -> Result<(), RenderlingError> {
        log::trace!("render");
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render_camera_objects"),
            });

        let mut render_pass = crate::linkage::begin_render_pass(
            &mut encoder,
            Some("render_camera_objects"),
            self.pipeline.get_render_pipeline(),
            frame_texture_view,
            depth_texture_view,
        );

        let default_material_uniform = self
            .default_material_uniform
            .as_ref()
            .context(MissingDefaultMaterialSnafu)?;

        // bind the lights to our shader
        if let Some(lights_uniform) = self.lights.uniform() {
            render_pass.set_bind_group(self.light_bindgroup_index, &lights_uniform.bindgroup, &[]);
        } else {
            log::warn!("no lights to bind");
        }
        let camera_data = self
            .cameras
            .get(&Id::new(**camera))
            .context(MissingCameraSnafu)?;
        log::trace!("rendering camera {:?}", camera);
        // bind the camera to our shader
        render_pass.set_bind_group(self.camera_bindgroup_index, &camera_data.bindgroup, &[]);

        for object_id in objects {
            let object = if let Some(object) = self.objects.get(&Id::new(**object_id)) {
                if !object.inner.read().is_visible {
                    continue;
                }
                object
            } else {
                continue;
            };
            if let Some(object) = object.as_shader_object() {
                let material = object
                    .material
                    .unwrap_or(default_material_uniform.get_bindgroup());
                // bind the object's material to our shader
                render_pass.set_bind_group(self.material_bindgroup_index, material, &[]);

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
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    /// Conduct a full render pass into the given textures.
    ///
    /// Renders all objects with the given camera.
    pub fn render_objects<'a>(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera: &Camera,
        objects: impl Iterator<Item = &'a Object>,
    ) -> Result<(), RenderlingError> {
        self.render_object_ids(
            frame_texture_view,
            depth_texture_view,
            &camera.id,
            objects.map(|o| &o.id),
        )
    }

    /// Conduct a full render pass into the given textures.
    ///
    /// Renders all objects with the given camera.
    pub fn render_camera(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera: &Camera,
    ) -> Result<(), RenderlingError> {
        let object_ids = self
            .cameras
            .get_object_ids_sorted_by_distance_to_camera(camera);
        self.render_object_ids(
            frame_texture_view,
            depth_texture_view,
            &camera.id,
            object_ids.iter(),
        )
    }

    /// Conduct a full render pass into the given textures.
    ///
    /// Uses the first camera to render, if available.
    /// Errs if no cameras have been created.
    pub fn render(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
    ) -> Result<(), RenderlingError> {
        let camera = self.cameras().next().context(MissingCameraSnafu)?;
        self.render_camera(frame_texture_view, depth_texture_view, &camera)
    }
}
