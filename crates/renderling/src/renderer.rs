//! Builds the UI pipeline and manages resources.
use nalgebra::Point3;
use renderling_core::{
    light::{
        DirectionalLight as ShaderDirectionalLight, LightsUniform, PointLight as ShaderPointLight,
        SpotLight as ShaderSpotLight,
    },
    ObjectDraw, ShaderObject,
};
use rustc_hash::{FxHashMap, FxHashSet};
use snafu::prelude::*;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, RwLock,
};

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
}

#[derive(Default)]
pub(crate) struct Scene {
    // Point lights
    pub(crate) point_lights: Vec<Shared<PointLightInner>>,
    // Spot lights
    pub(crate) spot_lights: Vec<Shared<SpotLightInner>>,
    // Directional lights
    pub(crate) directional_lights: Vec<Shared<DirectionalLightInner>>,
    // for creating camera ids
    camera_id_bank: BankOfIds,
    // for creating objects
    object_id_bank: BankOfIds,
    // all cameras, in their intended render order
    pub(crate) cameras: Vec<(Id, CameraData)>,
    // invisible objects keyed by their object id
    pub(crate) invisible_objects: FxHashMap<Id, ObjectData>,
    // all visible objects collated by their camera's id, in render order
    pub(crate) visible_objects: FxHashMap<Id, Vec<ObjectData>>,
}

impl Scene {
    pub(crate) fn new_camera_id(&self) -> Id {
        self.camera_id_bank.dequeue()
    }

    pub(crate) fn new_object_id(&self) -> Id {
        self.object_id_bank.dequeue()
    }

    pub(crate) fn get_camera_mut(&mut self, camera_id: Id) -> Option<&mut CameraData> {
        self.cameras.iter_mut().find_map(|c| {
            if c.0 == camera_id {
                Some(&mut c.1)
            } else {
                None
            }
        })
    }

    pub(crate) fn find_object_data_mut(
        &mut self,
        object_id: &Id,
        camera_id: Option<&Id>,
    ) -> Option<&mut ObjectData> {
        if let Some(camera_id) = camera_id {
            let objects = self.visible_objects.get_mut(camera_id)?;
            objects.iter_mut().find(|o| o.id == *object_id)
        } else {
            self.invisible_objects.get_mut(object_id)
        }
    }

    pub(crate) fn remove_object_from_camera(&mut self, object_id: &Id, camera_id: &Id) {
        if let Some(objects) = self.visible_objects.get_mut(&camera_id) {
            if let Some(object_index) =
                objects.iter().enumerate().find_map(
                    |(i, o)| {
                        if o.id == *object_id {
                            Some(i)
                        } else {
                            None
                        }
                    },
                )
            {
                let o = objects.remove(object_index);
                self.invisible_objects.insert(*object_id, o);
            }
        }
    }

    pub(crate) fn add_object_to_camera(&mut self, object_id: &Id, camera_id: &Id) -> bool {
        if let Some(object) = self.invisible_objects.remove(object_id) {
            if let Some(objects) = self.visible_objects.get_mut(camera_id) {
                objects.push(object);
                return true;
            }
        }
        false
    }
}

pub struct Renderling {
    // queue/channel of updates from objects to make before the next render
    object_update_queue: (Sender<ObjUpdateCmd>, Receiver<ObjUpdateCmd>),
    // queue/channel of updates from cameras to make before the next render
    camera_update_queue: (Sender<CameraUpdateCmd>, Receiver<CameraUpdateCmd>),
    // queue/channel of updates from lights to make before the next render
    light_update_queue: (Sender<LightUpdateCmd>, Receiver<LightUpdateCmd>),

    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    size: Arc<RwLock<(u32, u32)>>,

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
    scene: Scene,
}

impl Renderling {
    pub fn new<P: Pipeline, M: Material>(
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
        };
        // this will cause the next update to create (at least an empty) light uniform
        r.light_update_queue.0.send(LightUpdateCmd::DirectionalLights).unwrap();
        r
    }

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

    pub fn new_object(&mut self) -> ObjectBuilder<'_> {
        ObjectBuilder {
            camera: None,
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
                ObjUpdateCmd::Transform {
                    object_id,
                    camera_id,
                } => {
                    if let Some(object) = self
                        .scene
                        .find_object_data_mut(&object_id, camera_id.as_ref())
                    {
                        log::trace!("updated object {:?} transform", object_id);
                        object.inner.read().update_world_transforms_buffer(
                            &self.queue,
                            &object.instances,
                            self.meshes_have_normal_matrix_attribute,
                        );
                    }
                }
                ObjUpdateCmd::Mesh {
                    object_id,
                    camera_id,
                } => {
                    if let Some(object) = self
                        .scene
                        .find_object_data_mut(&object_id, camera_id.as_ref())
                    {
                        log::trace!("updated object {:?} mesh", object_id);
                        object.mesh = object.inner.read().mesh.clone();
                    }
                }
                ObjUpdateCmd::Material {
                    object_id,
                    camera_id,
                } => {
                    if let Some(object) = self
                        .scene
                        .find_object_data_mut(&object_id, camera_id.as_ref())
                    {
                        log::trace!("updated object {:?} material", object_id);
                        let inner = object.inner.read();
                        object.material_uniform = inner
                            .material
                            .as_ref()
                            .map(|mat| mat.create_material_uniform(&self.device));
                    }
                }
            }
        }

        while let Ok(cmd) = self.camera_update_queue.1.try_recv() {
            match cmd {
                CameraUpdateCmd { camera_id } => {
                    if let Some(camera_data) = self.scene.get_camera_mut(camera_id) {
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

        for camera_id in cameras_to_sort.into_iter() {
            if let Some((_, camera_data)) = self.scene.cameras.get(*camera_id) {
                if let Some(objects) = self.scene.visible_objects.get_mut(&camera_id) {
                    let camera_position =
                        Point3::from(camera_data.inner.read().view.translation.vector);
                    objects.sort_by(|a, b| {
                        let a_d = nalgebra::distance(&a.position, &camera_position);
                        let b_d = nalgebra::distance(&b.position, &camera_position);
                        b_d.total_cmp(&a_d)
                    });
                }
            }
        }

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

    /// Conduct a full render pass into the given textures.
    pub fn render(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
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
        // over each camera
        for (camera_id, camera_data) in self.scene.cameras.iter() {
            log::trace!("rendering camera {:?}", camera_id);
            // bind the camera to our shader
            render_pass.set_bind_group(self.camera_bindgroup_index, &camera_data.bindgroup, &[]);

            if let Some(visible_objects) = self.scene.visible_objects.get(camera_id) {
                log::trace!("rendering {} objects on camera {:?}", visible_objects.len(), camera_id);
                for object in visible_objects.iter().map(ShaderObject::from) {
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
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
