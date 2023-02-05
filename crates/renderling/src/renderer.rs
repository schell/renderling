//! Builds the UI pipeline and manages resources.
use renderling_core::{
    light::{
        DirectionalLight as ShaderDirectionalLight, LightsUniform, PointLight as ShaderPointLight,
        SpotLight as ShaderSpotLight,
    },
    ObjectDraw, ShaderObject,
};
use rustc_hash::FxHashSet;
use snafu::prelude::*;
use std::{sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, RwLock,
}, marker::PhantomData};

use crate::{
    camera::*,
    light::{DirectionalLightInner, PointLightInner, SpotLightInner},
    resources::*,
    AnyMaterial, AnyMaterialUniform, AnyPipeline, LightUpdateCmd, Material, ObjUpdateCmd,
    ObjectBuilder, ObjectData, Pipeline, WorldTransform,
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
}

#[derive(Default)]
pub(crate) struct Scene {
    // Point lights
    pub point_lights: Vec<Shared<PointLightInner>>,
    // Spot lights
    pub spot_lights: Vec<Shared<SpotLightInner>>,
    // Directional lights
    pub directional_lights: Vec<Shared<DirectionalLightInner>>,
    // for creating camera ids
    camera_id_bank: BankOfIds,
    // for creating objects
    object_id_bank: BankOfIds,
    // all cameras, indexed by Id
    pub cameras: Vec<Option<CameraData>>,
    // all objects, indexed by Id
    pub objects: Vec<Option<ObjectData>>,
}

impl Scene {
    pub fn new_camera_id(&mut self) -> Id {
        let id = self.camera_id_bank.dequeue();
        if self.cameras.len() < id.0 + 1 {
            self.cameras.resize_with(id.0 + 1, || None);
        }
        id
    }

    pub fn get_camera_mut(&mut self, camera_id: &Id) -> Option<&mut CameraData> {
        let may_cam = self.cameras.get_mut(camera_id.0)?;
        may_cam.as_mut()
    }

    pub fn get_camera(&self, camera_id: &Id) -> Option<&CameraData> {
        let may_cam = self.cameras.get(camera_id.0)?;
        may_cam.as_ref()
    }

    pub fn destroy_camera(&mut self, camera_id: Id) {
        log::trace!("destroying and recycling camera {:?}", camera_id);
        self.cameras[camera_id.0] = None;
        for object in self.objects.iter_mut() {
            if let Some(object) = object.as_mut() {
                let _ = object.cameras.remove(&camera_id);
            }
        }
        self.camera_id_bank.recycle(camera_id);
    }

    pub fn new_object_id(&mut self) -> Id {
        let id = self.object_id_bank.dequeue();
        if self.objects.len() < id.0 + 1 {
            self.objects.resize_with(id.0 + 1, || None);
        }
        id
    }

    pub fn has_object(&self, object_id: &Id) -> bool {
        self.objects.get(object_id.0).map(Option::as_ref).flatten().is_some()
    }

    pub fn get_object_mut(&mut self, object_id: &Id) -> Option<&mut ObjectData> {
        let may_dat = self.objects.get_mut(object_id.0)?;
        may_dat.as_mut()
    }

    pub fn get_object(&self, object_id: &Id) -> Option<&ObjectData> {
        let may_dat = self.objects.get(object_id.0)?;
        may_dat.as_ref()
    }

    pub fn remove_object_from_camera(&mut self, object_id: &Id, camera_id: &Id) {
        if let Some(object) = self.get_object_mut(object_id) {
            object.cameras.remove(camera_id);
        }
    }

    pub fn add_object_to_camera(&mut self, object_id: &Id, camera_id: &Id) -> bool {
        if let Some(object) = self.get_object_mut(object_id) {
            let _ = object.cameras.insert(*camera_id);
            true
        } else {
            false
        }
    }

    pub fn destroy_object(&mut self, object_id: Id) {
        if let Some(_object) = self.objects[object_id.0].take() {
            self.object_id_bank.recycle(object_id);
        }
    }
}

/// A renderer typed by its pipeline.
///
/// `Renderling` manages GPU resources for cameras, materials and objects.
pub struct Renderling<P: Pipeline> {
    // queue/channel of updates from objects to make before the next render
    pub(crate) object_update_queue: (Sender<ObjUpdateCmd>, Receiver<ObjUpdateCmd>),
    // queue/channel of updates from cameras to make before the next render
    camera_update_queue: (Sender<CameraUpdateCmd>, Receiver<CameraUpdateCmd>),
    // queue/channel of updates from lights to make before the next render
    light_update_queue: (Sender<LightUpdateCmd>, Receiver<LightUpdateCmd>),

    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
    pub(crate) size: Arc<RwLock<(u32, u32)>>,

    // Built shader render pipeline
    pipeline: AnyPipeline,
    // default material to use when there is no other
    default_material: AnyMaterial,
    // default material bindgroup
    default_material_uniform: Option<AnyMaterialUniform>,
    // whether object meshes have a 3x3 normal matrix attribute (defaults to `true`)
    meshes_have_normal_matrix_attribute: bool,
    // lights uniform - optional because not all renderlings have lighting
    lights_uniform: Option<LightsUniform>,
    // The index of the camera's bindgroup
    camera_bindgroup_index: u32,
    // The index of the lights's bindgroup
    light_bindgroup_index: u32,
    // The index of the material's bindgroup
    material_bindgroup_index: u32,
    // The entire scene - all cameras and objects
    pub(crate) scene: Scene,

    _phantom: PhantomData<P>
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
            object_update_queue: channel(),
            camera_update_queue: channel(),
            light_update_queue: channel(),
            meshes_have_normal_matrix_attribute,
            default_material: AnyMaterial::new::<M>(material.into()),
            default_material_uniform: None,
            lights_uniform: None,
            scene: Default::default(),
            camera_bindgroup_index: 0,
            material_bindgroup_index: 1,
            light_bindgroup_index: 2,
            _phantom: PhantomData
        };
        // this will cause the next update to create (at least an empty) light uniform
        r.light_update_queue
            .0
            .send(LightUpdateCmd::DirectionalLights)
            .unwrap();
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
            update_tx: self.camera_update_queue.0.clone(),
            scene: &mut self.scene,
        }
    }

    /// Retrieves an iterator over all cameras.
    ///
    /// This will always have at least one camera in it.
    pub fn cameras(&self) -> impl Iterator<Item = Camera> + '_ {
        self.scene
            .cameras
            .iter()
            .enumerate()
            .filter_map(|(i, data)| {
                let data = data.as_ref()?;
                Some(Camera {
                    id: Id(i),
                    inner: data.inner.clone(),
                    cmd: self.camera_update_queue.0.clone(),
                })
            })
    }

    /// Retrieves the default camera.
    ///
    /// The default camera is the camera that renders first.
    /// The default camera comes first in the iterator returned by `Renderling::cameras`.
    /// The default camera is often the one that was created automatically when the renderling was
    /// created.
    pub fn default_camera(&self) -> Camera {
        // UNWRAP: having one default camera is an invariant of the system and we should panic otherwise
        self.cameras().next().unwrap()
    }

    /// Creates a new object builder.
    ///
    /// The builder can be used to customize the object befoure building it.
    ///
    /// If no camera is specified with `ObjectBuilder::with_camera`, the camera
    /// with first rendering priority will be selected, if available.
    ///
    /// If no material is provided, the renderling's default material will be used.
    ///
    /// If no transform is provided, the object will be positioned at the origin with
    /// no rotation and scale 1,1,1.
    pub fn new_object(&mut self) -> ObjectBuilder<'_> {
        ObjectBuilder {
            camera: Some(Id(0)),
            mesh: None,
            material: None,
            world_transform: WorldTransform::default(),
            world_transforms: vec![],
            generate_normal_matrix: self.meshes_have_normal_matrix_attribute,
            is_visible: true,
            update_tx: self.object_update_queue.0.clone(),
            device: &self.device,
            scene: &mut self.scene,
        }
    }

    pub fn new_point_light(&mut self) -> crate::PointLightBuilder<'_> {
        crate::PointLightBuilder::new(&mut self.scene, self.light_update_queue.0.clone())
    }

    pub fn new_spot_light(&mut self) -> crate::SpotLightBuilder<'_> {
        crate::SpotLightBuilder::new(&mut self.scene, self.light_update_queue.0.clone())
    }

    pub fn new_directional_light(&mut self) -> crate::DirectionalLightBuilder<'_> {
        crate::DirectionalLightBuilder::new(&mut self.scene, self.light_update_queue.0.clone())
    }

    /// Conduct all updates made from outside the renderling.
    ///
    /// This must be called in order to display any changes.
    pub fn update(&mut self) -> Result<(), RenderlingError> {
        let mut cameras_to_sort = FxHashSet::<Id>::default();
        while let Ok(cmd) = self.object_update_queue.1.try_recv() {
            match cmd {
                ObjUpdateCmd::RemoveFromCamera {
                    camera_id,
                    object_id,
                } => {
                    log::trace!("removed object {:?} from camera {:?}", object_id, camera_id);
                    self.scene.remove_object_from_camera(&object_id, &camera_id);
                }
                ObjUpdateCmd::AddToCamera {
                    camera_id,
                    object_id,
                } => {
                    if self.scene.add_object_to_camera(&object_id, &camera_id) {
                        log::trace!("added object {:?} to camera {:?}", object_id, camera_id);
                        cameras_to_sort.insert(camera_id);
                    }
                }
                ObjUpdateCmd::Transform { object_id } => {
                    if let Some(object) = self.scene.get_object_mut(&object_id) {
                        log::trace!("updated object {:?} transform", object_id);
                        object.inner.read().update_world_transforms_buffer(
                            &self.queue,
                            &object.instances,
                            self.meshes_have_normal_matrix_attribute,
                        );
                    }
                }
                ObjUpdateCmd::Mesh { object_id } => {
                    if let Some(object) = self.scene.get_object_mut(&object_id) {
                        log::trace!("updated object {:?} mesh", object_id);
                        object.mesh = object.inner.read().mesh.clone();
                    }
                }
                ObjUpdateCmd::Material { object_id } => {
                    if let Some(object) = self.scene.get_object_mut(&object_id) {
                        log::trace!("updated object {:?} material", object_id);
                        let inner = object.inner.read();
                        object.material_uniform = inner
                            .material
                            .as_ref()
                            .map(|mat| mat.create_material_uniform(&self.device));
                    }
                }
                ObjUpdateCmd::Destroy { object_id } => {
                    self.scene.destroy_object(object_id);
                }
            }
        }

        while let Ok(cmd) = self.camera_update_queue.1.try_recv() {
            match cmd {
                CameraUpdateCmd::Destroy { camera_id } => {
                    self.scene.destroy_camera(camera_id);
                }
                CameraUpdateCmd::Update { camera_id } => {
                    if let Some(camera_data) = self.scene.get_camera_mut(&camera_id) {
                        cameras_to_sort.insert(camera_id);
                        let mut inner = camera_data.inner.write();
                        let (buffer, bindgroup) = new_camera_uniform(&inner, &self.device);
                        inner.dirty_uniform = false;
                        camera_data.buffer = buffer;
                        camera_data.bindgroup = bindgroup;
                    }
                }
            }
        }

        //for camera_id in cameras_to_sort.into_iter() {
        //    if let Some(v) = self
        //        .scene
        //        .get_camera(&camera_id)
        //        .map(|data| data.inner.read().view.translation.vector)
        //    {
        //        let camera_position:Point3<f32> = Point3::new(v.x, v.y, v.z);
        //        let mut object_ids = self
        //            .scene
        //            .cameras_to_objects
        //            .remove(&camera_id)
        //            .unwrap_or(vec![])
        //            .into_iter()
        //            .filter_map(|id| self.scene.get_object(&id).map(|o| (id, o.position)))
        //            .collect::<Vec<_>>();
        //        object_ids.sort_by(|(a_id, a_p), (b_id, b_p)| {
        //            let a_d = nalgebra::distance(&a_p, &camera_position);
        //            let b_d = nalgebra::distance(&b_p, &camera_position);
        //            b_d.total_cmp(&a_d)
        //        });
        //        self.scene.cameras_to_objects.insert(camera_id, object_ids.into_iter().map(|(id, _)| id).collect());
        //    }
        //}

        // update lights
        let mut update_point_lights = false;
        let mut update_spot_lights = false;
        let mut update_directional_lights = false;
        while let Ok(cmd) = self.light_update_queue.1.try_recv() {
            update_point_lights = update_point_lights || cmd == LightUpdateCmd::PointLights;
            update_spot_lights = update_spot_lights || cmd == LightUpdateCmd::SpotLights;
            update_directional_lights =
                update_directional_lights || cmd == LightUpdateCmd::DirectionalLights;
        }

        if let Some(lights_uniform) = self.lights_uniform.as_ref() {
            if update_point_lights {
                log::trace!("updating point lights");
                lights_uniform.update_point_lights(&self.queue, self.get_point_lights());
            }
            if update_spot_lights {
                log::trace!("updating spot lights");
                lights_uniform.update_spot_lights(&self.queue, self.get_spot_lights());
            }
            if update_directional_lights {
                log::trace!("updating directional lights");
                lights_uniform
                    .update_directional_lights(&self.queue, self.get_directional_lights());
            }
        } else if update_point_lights || update_spot_lights || update_directional_lights {
            log::trace!("creating initial lights uniform");
            // create our lights uniform
            self.lights_uniform = Some(LightsUniform::new(
                &self.device,
                self.get_point_lights(),
                self.get_spot_lights(),
                self.get_directional_lights(),
            ));
        }

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

    fn get_point_lights(&self) -> Vec<ShaderPointLight> {
        self.scene.point_lights.iter().map(|l| l.read().0).collect()
    }

    fn get_spot_lights(&self) -> Vec<ShaderSpotLight> {
        self.scene.spot_lights.iter().map(|l| l.read().0).collect()
    }

    fn get_directional_lights(&self) -> Vec<ShaderDirectionalLight> {
        self.scene
            .directional_lights
            .iter()
            .map(|l| l.read().0)
            .collect()
    }

    /// Conduct a full render pass into the given textures using the given camera
    /// and objects.
    fn render_camera_objects<'a>(
        &'a self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera: &Id,
        objects: impl Iterator<Item = &'a ObjectData>,
    ) -> Result<(), RenderlingError> {
        log::trace!("render");
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Renderling encoder"),
            });

        let mut render_pass = renderling_core::begin_render_pass(
            &mut encoder,
            Some("renderling-pass"),
            self.pipeline.get_render_pipeline(),
            frame_texture_view,
            depth_texture_view,
        );

        let default_material_uniform = self
            .default_material_uniform
            .as_ref()
            .context(MissingDefaultMaterialSnafu)?;

        // bind the lights to our shader
        if let Some(lights_uniform) = self.lights_uniform.as_ref() {
            render_pass.set_bind_group(self.light_bindgroup_index, &lights_uniform.bindgroup, &[]);
        } else {
            log::warn!("no lights to bind");
        }
        let camera_data = self.scene.get_camera(camera).context(MissingCameraSnafu)?;
        log::trace!("rendering camera {:?}", camera);
        // bind the camera to our shader
        render_pass.set_bind_group(self.camera_bindgroup_index, &camera_data.bindgroup, &[]);

        // TODO: de-dupe this code with other render function
        for outer_object in objects {
            if let Some(object) = self
                .scene
                .get_object(&outer_object.id)
                .map(ObjectData::as_shader_object)
                .flatten()
            {
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
    /// Only renders using the given camera.
    pub fn render_camera(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
        camera_id: &Id,
    ) -> Result<(), RenderlingError> {
        let objects = self.scene.objects.iter().filter_map(|may_o| {
            let obj = may_o.as_ref()?;
            if obj.cameras.contains(camera_id) {
                Some(obj)
            } else {
                None
            }
        });
        self.render_camera_objects(frame_texture_view, depth_texture_view, camera_id, objects)
    }

    /// Conduct a full render pass into the given textures.
    ///
    /// Uses all cameras in the order they were created.
    pub fn render(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
    ) -> Result<(), RenderlingError> {
        for i in 0..self.scene.cameras.len() {
            let id = Id(i);
            self.render_camera(frame_texture_view, depth_texture_view, &id)?;
        }
        Ok(())
    }
}
