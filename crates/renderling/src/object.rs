//! Renderable things with positions, transformations, meshes and materials.
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use async_channel::{unbounded, Receiver, Sender};
use glam::{Mat3, Mat4, Quat, Vec3};
use moongraph::{Read, Write};
use rustc_hash::{FxHashMap, FxHashSet};
use snafu::prelude::*;
use wgpu::util::DeviceExt;

use crate::{
    bank::Bank,
    linkage::{ObjectDraw, ShaderObject},
    resources::Id,
    CamerasNeedSorting, LocalTransform, Pipeline, Renderling, Shared, WorldTransform,
};

pub(crate) enum ObjUpdateCmd {
    // Update the given object's transform
    Transform {
        object_id: Id<Object>,
    },
    // Update the given object's mesh
    Mesh {
        object_id: Id<Object>,
        mesh: Arc<crate::Mesh>,
    },
    // Update the given object's mesh
    Material {
        object_id: Id<Object>,
    },
    // Destroy this object
    Destroy {
        object_id: Id<Object>,
    },
}

#[derive(Debug, Snafu)]
pub enum ObjectError {
    #[snafu(display("none"))]
    None
}

#[derive(Debug, Snafu)]
pub enum ObjectBuilderError {
    #[snafu(display("object builder is missing `mesh`"))]
    MissingMesh,
}

pub struct ObjectBuilder<'a> {
    pub(crate) inner: ObjectInner,
    pub(crate) mesh: Option<Arc<crate::Mesh>>,
    pub(crate) children: Vec<&'a Object>,
    pub(crate) generate_normal_matrix: bool,
    pub(crate) properties: FxHashMap<String, serde_json::Value>,
    pub(crate) renderer: &'a mut Renderling,
}

impl<'a> ObjectBuilder<'a> {
    pub fn with_mesh(mut self, mesh: impl Into<Arc<crate::Mesh>>) -> Self {
        self.mesh = Some(mesh.into());
        self
    }

    pub fn with_mesh_builder<Vertex: bytemuck::Pod>(
        self,
        mesh_builder: crate::MeshBuilder<Vertex>,
    ) -> Self {
        let mesh = mesh_builder.build(Some("object-builder-mesh"), self.renderer.get_device());
        self.with_mesh(mesh)
    }

    pub fn with_transform(mut self, t: crate::LocalTransform) -> Self {
        self.inner.local_transforms[0] = t;
        self
    }

    /// Add another local transform.
    ///
    /// This object will be rendered once with every transform using instancing.
    pub fn add_transform(mut self, t: crate::LocalTransform) -> Self {
        self.inner.local_transforms.push(t);
        self
    }

    pub fn with_position(mut self, p: Vec3) -> Self {
        self.inner.local_transforms[0].position = Vec3::new(p.x, p.y, p.z);
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.inner.local_transforms[0].rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.inner.local_transforms[0].scale = scale;
        self
    }

    pub fn with_material<T: crate::Material>(mut self, material: impl Into<Arc<T>>) -> Self {
        self.inner.material = Some(crate::AnyMaterial::new(material));
        self
    }

    pub fn with_child(mut self, child: &'a Object) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = &'a Object>) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    pub fn with_is_visible(mut self, is_visible: bool) -> Self {
        self.inner.is_visible = is_visible;
        self
    }

    pub fn with_generate_normal_matrix(mut self, should_generate_normal_matrix: bool) -> Self {
        self.generate_normal_matrix = should_generate_normal_matrix;
        self
    }

    pub fn with_property(mut self, name: impl Into<String>, property: serde_json::Value) -> Self {
        self.properties.insert(name.into(), property);
        self
    }

    pub fn build(self) -> Result<Object, ObjectBuilderError> {
        let ObjectBuilder {
            mesh,
            inner,
            children,
            generate_normal_matrix,
            properties: _,
            renderer,
        } = self;
        let material_uniform = inner
            .material
            .as_ref()
            .map(|mat| mat.create_material_uniform(renderer.get_device()));
        let position = inner.local_transforms[0].position;
        let objects = renderer.get_objects_mut();
        let update_queue = objects.update_queue();
        let inner = Shared::new(inner);
        let children = children
            .into_iter()
            .map(|child| {
                child.inner.write().parent = Some(ParentObject(inner.clone()));
                update_queue
                    .try_send(ObjUpdateCmd::Transform {
                        object_id: child.id,
                    })
                    .unwrap();
                ChildObject(child.id)
            })
            .collect::<Vec<_>>();
        inner.write().children = children;
        let instances = inner
            .read()
            .new_world_transforms_buffer(renderer.get_device(), self.generate_normal_matrix);

        let id = renderer.get_objects_mut().bank.insert_with({
            let inner = inner.clone();
            move |id| ObjectData {
                id: id.into(),
                mesh,
                material_uniform,
                instances,
                generate_normal_matrix,
                world_position: position,
                inner,
            }
        });

        Ok(Object {
            id: id.into(),
            inner,
            cmd: update_queue,
        })
    }
}

pub(crate) struct ParentObject(Shared<ObjectInner>);

pub(crate) struct ChildObject(Id<Object>);

/// Data shared between the library user and the renderling backend.
///
/// The data held in `ObjectInner` is data that the library user can change at
/// any time and data that has a downstream representation in `wgpu`, which is
/// created/modified in `Renderling::update`.
pub(crate) struct ObjectInner {
    pub(crate) material: Option<crate::AnyMaterial>,
    pub(crate) parent: Option<ParentObject>,
    pub(crate) children: Vec<ChildObject>,
    pub(crate) pipeline_membership: FxHashSet<Id<Pipeline>>,
    pub(crate) properties: FxHashMap<String, serde_json::Value>,
    pub(crate) is_visible: bool,
    pub(crate) local_transforms: Vec<crate::LocalTransform>,
}

impl Default for ObjectInner {
    fn default() -> Self {
        Self {
            material: Default::default(),
            parent: Default::default(),
            children: Default::default(),
            properties: Default::default(),
            pipeline_membership: Default::default(),
            is_visible: true,
            local_transforms: vec![LocalTransform::default()],
        }
    }
}

impl ObjectInner {
    fn model_matrix_to_vec(model: Mat4, generate_normal_matrix: bool) -> Vec<f32> {
        let mut m = model.as_ref().to_vec();
        if generate_normal_matrix {
            let normal = Mat3::from_mat4(model.inverse().transpose());
            let mut n = normal.as_ref().to_vec();
            m.append(&mut n);
        }
        m
    }

    /// Create a new instances buffer from a list of world transforms
    pub(crate) fn new_world_transforms_buffer(
        &self,
        device: &wgpu::Device,
        generate_normal_matrix: bool,
    ) -> crate::linkage::VertexBuffer {
        let ms: Vec<f32> = self
            .get_world_transforms()
            .flat_map(|t| Self::model_matrix_to_vec(Mat4::from(&t), generate_normal_matrix))
            .collect::<Vec<_>>();
        crate::linkage::VertexBuffer {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderling instance buffer"),
                contents: bytemuck::cast_slice(ms.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            len: self.local_transforms.len(),
        }
    }

    pub(crate) fn update_world_transforms_buffer(
        &self,
        queue: &wgpu::Queue,
        buffer: &crate::linkage::VertexBuffer,
        generate_normal_matrix: bool,
    ) {
        let ms: Vec<f32> = self
            .get_world_transforms()
            .flat_map(|t| Self::model_matrix_to_vec(Mat4::from(&t), generate_normal_matrix))
            .collect::<Vec<_>>();
        queue.write_buffer(&buffer.buffer, 0, bytemuck::cast_slice(ms.as_slice()));
    }

    /// Returns the world transform.
    pub(crate) fn get_parent_world_transform(&self) -> Option<WorldTransform> {
        let parent = self.parent.as_ref()?;
        let parent_inner = parent.0.read();
        let parent_tfrm = parent_inner
            .get_parent_world_transform()
            .unwrap_or_default();
        Some(
            parent_inner.local_transforms[0]
                .as_global()
                .append(&parent_tfrm),
        )
    }

    pub(crate) fn get_world_transforms(&self) -> impl Iterator<Item = WorldTransform> + '_ {
        let parent_tfrm = self.get_parent_world_transform().unwrap_or_default();
        self.local_transforms
            .iter()
            .map(move |t| t.as_global().append(&parent_tfrm))
    }
}

/// A library-user-facing display "object".
///
/// `Object`s are used as a handle to update graphical resources within the
/// renderling that was used to create it. To release the underlying resources
/// the object should be dropped.
#[derive(Clone)]
pub struct Object {
    pub(crate) id: Id<Object>,
    pub(crate) inner: Shared<ObjectInner>,
    pub(crate) cmd: Sender<ObjUpdateCmd>,
}

impl Drop for Object {
    // TODO: do the same drop treatment for cameras and lights
    fn drop(&mut self) {
        // the minimum count here is 2 because when the object is dropped there is 1
        // from the this object here and one stored in the renderer
        if self.inner.count() <= 2 {
            let _ = self
                .cmd
                .try_send(ObjUpdateCmd::Destroy { object_id: self.id });
        }
    }
}

impl Object {
    /// Update the local transform of this object.
    pub fn set_transform(&self, transform: LocalTransform) {
        let mut inner = self.inner.write();
        *inner.local_transforms.get_mut(0).unwrap() = transform;
        self.cmd
            .try_send(ObjUpdateCmd::Transform { object_id: self.id })
            .unwrap();
    }

    /// Update the local transform's scale of this object.
    pub fn set_scale(&self, scale: Vec3) {
        let mut inner = self.inner.write();
        inner.local_transforms.get_mut(0).unwrap().scale = scale;
        self.cmd
            .try_send(ObjUpdateCmd::Transform { object_id: self.id })
            .unwrap();
    }

    /// Update the local transform's rotation of this object.
    pub fn set_rotation(&self, rotation: Quat) {
        let mut inner = self.inner.write();
        inner.local_transforms.get_mut(0).unwrap().rotation = rotation;
        self.cmd
            .try_send(ObjUpdateCmd::Transform { object_id: self.id })
            .unwrap();
    }

    /// Update the local transform's scale of this object.
    pub fn set_position(&self, position: Vec3) {
        let mut inner = self.inner.write();
        inner.local_transforms.get_mut(0).unwrap().position = position;
        self.cmd
            .try_send(ObjUpdateCmd::Transform { object_id: self.id })
            .unwrap();
    }

    /// Returns whether this object is rendered by the pipeline with the given
    /// id.
    pub fn is_member_of_pipeline(&self, pipeline: &Id<Pipeline>) -> bool {
        self.inner.read().pipeline_membership.contains(pipeline)
    }

    /// Update the pipeline membership.
    pub fn add_to_pipeline(&self, pipeline: Id<Pipeline>) {
        let mut inner = self.inner.write();
        inner.pipeline_membership.insert(pipeline);
    }

    /// Update the pipeline membership.
    pub fn remove_from_pipeline(&self, pipeline: &Id<Pipeline>) {
        let mut inner = self.inner.write();
        inner.pipeline_membership.remove(pipeline);
    }

    /// Get the current local transformation of this object.
    pub fn get_transform(&self) -> LocalTransform {
        self.inner.read().local_transforms[0].clone()
    }

    /// Get all the instance transforms of this object.
    pub fn get_local_transforms(&self) -> Vec<LocalTransform> {
        self.inner.read().local_transforms.clone()
    }

    pub fn get_world_transform(&self) -> WorldTransform {
        self.inner.read().get_world_transforms().next().unwrap()
    }

    pub fn get_world_transforms(&self) -> Vec<WorldTransform> {
        self.inner.read().get_world_transforms().collect::<Vec<_>>()
    }

    /// Update the visibility of this object.
    pub fn set_visible(&self, is_visible: bool) {
        let mut inner = self.inner.write();
        if inner.is_visible != is_visible {
            inner.is_visible = is_visible;
        }
    }

    /// Update the mesh of this object.
    pub fn set_mesh(&self, mesh: impl Into<Arc<crate::Mesh>>) {
        self.cmd
            .try_send(ObjUpdateCmd::Mesh {
                object_id: self.id,
                mesh: mesh.into(),
            })
            .unwrap();
    }

    /// Update the material of this object.
    pub fn set_material<T: crate::Material>(&self, material: impl Into<Arc<T>>) {
        let mut inner = self.inner.write();
        inner.material = Some(crate::AnyMaterial::new(material));
        self.cmd
            .try_send(ObjUpdateCmd::Material { object_id: self.id })
            .unwrap();
    }

    /// Nest another object in this object.
    ///
    /// This has the effect of transforming the child object by this object's
    /// transform, until the child is removed with [`Object::remove_child`]
    /// or [`Object::detach_from_parent`].
    pub fn append_child(&self, child_object: &Object) {
        let mut parent = self.inner.write();
        parent.children.push(ChildObject(child_object.id));
        let mut child = child_object.inner.write();
        child.parent = Some(ParentObject(self.inner.clone()));
        self.cmd
            .try_send(ObjUpdateCmd::Transform {
                object_id: child_object.id,
            })
            .unwrap();
    }

    /// Un-nest another object from this object.
    ///
    /// This restores the child object's local transform as its global
    /// transform.
    pub fn remove_child(&self, child_object: &Object) {
        let mut parent = self.inner.write();
        parent.children.retain(|child| child.0 != child_object.id);
        let mut child = child_object.inner.write();
        child.parent = None;
        self.cmd
            .try_send(ObjUpdateCmd::Transform {
                object_id: child_object.id,
            })
            .unwrap();
    }

    /// Un-nest this object from its parent.
    ///
    /// This restores the object's local transform as its global transform.
    pub fn detach_from_parent(&self) {
        let mut inner = self.inner.write();
        if let Some(parent) = inner.parent.take() {
            parent.0.write().children.retain(|child| child.0 != self.id);
            self.cmd
                .try_send(ObjUpdateCmd::Transform { object_id: self.id })
                .unwrap();
        }
    }
}

/// Underlying data used by `wgpu` to render an object.
///
/// Contains `ObjectInner` as well as additional fields needed for rendering.
pub struct ObjectData {
    pub(crate) id: Id<Object>,
    pub(crate) mesh: Option<Arc<crate::Mesh>>,
    pub(crate) material_uniform: Option<crate::AnyMaterialUniform>,
    pub(crate) instances: crate::linkage::VertexBuffer,
    pub(crate) world_position: Vec3,
    pub(crate) generate_normal_matrix: bool,
    pub(crate) inner: Shared<ObjectInner>,
}

impl ObjectData {
    pub fn as_shader_object(&self) -> Option<ShaderObject<'_>> {
        let mesh = self.mesh.as_ref()?;
        let draw = mesh
            .index_buffer
            .as_ref()
            .map(|mb| ObjectDraw::Indexed {
                index_buffer: mb.buffer.slice(..),
                index_format: wgpu::IndexFormat::Uint16,
                index_range: 0..mb.len as u32,
                base_vertex: 0,
            })
            .unwrap_or_else(|| ObjectDraw::Default {
                vertex_range: 0..mesh.vertex_buffer.len as u32,
            });
        let object = ShaderObject {
            mesh_buffer: mesh.vertex_buffer.buffer.slice(..),
            instances: self.instances.buffer.slice(..),
            instances_range: 0..self.instances.len as u32,
            material: self.material_uniform.as_ref().map(|mu| mu.get_bindgroup()),
            name: None,
            draw,
        };
        Some(object)
    }

    pub fn is_member_of_pipeline(&self, pipeline: &Id<Pipeline>) -> bool {
        self.inner.read().pipeline_membership.contains(pipeline)
    }

    pub fn update_world_transform(&mut self, queue: &wgpu::Queue) {
        log::trace!("updating object {:?} world transform", self.id);
        let inner = self.inner.read();
        inner.update_world_transforms_buffer(&queue, &self.instances, self.generate_normal_matrix);
        let parent_tfrm = inner.get_parent_world_transform().unwrap_or_default();
        let parent_model_matrix = Mat4::from(&parent_tfrm);
        let p = inner.local_transforms[0].position;
        self.world_position = parent_model_matrix.project_point3(p);
    }
}

impl From<Id<ObjectData>> for Id<Object> {
    fn from(value: Id<ObjectData>) -> Self {
        Id::new(*value)
    }
}

impl From<Id<Object>> for Id<ObjectData> {
    fn from(value: Id<Object>) -> Self {
        Id::new(*value)
    }
}

/// All display objects on the "stage".
pub struct Objects {
    bank: Bank<ObjectData>,
    // queue/channel of updates from library userland objects to make before the next render
    pub(crate) object_update_queue: (Sender<ObjUpdateCmd>, Receiver<ObjUpdateCmd>),
}

impl Deref for Objects {
    type Target = Bank<ObjectData>;

    fn deref(&self) -> &Self::Target {
        &self.bank
    }
}

impl DerefMut for Objects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bank
    }
}

impl Default for Objects {
    fn default() -> Self {
        Self {
            bank: Default::default(),
            object_update_queue: unbounded(),
        }
    }
}

impl Objects {
    pub fn iter(&self) -> impl Iterator<Item = Option<&ObjectData>> + '_ {
        self.bank.iter()
    }

    pub fn iter_with_ids<'a>(
        &self,
        ids: impl IntoIterator<Item = &'a Id<Object>>,
    ) -> impl Iterator<Item = &ObjectData> {
        ids.into_iter().copied().filter_map(|id| {
            self.bank.get(&id.into())
        })
    }

    /// Update any object properties that have changed in userland.
    ///
    /// Returns whether the cameras need to have their objects resorted because
    /// of any updates.
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> bool {
        let mut should_sort = false;
        while let Ok(cmd) = self.object_update_queue.1.try_recv() {
            match cmd {
                ObjUpdateCmd::Transform { object_id } => {
                    if let Some(object) = self.bank.get_mut(&object_id.into()) {
                        object.update_world_transform(queue);
                        // this object's transform changed, so we should resort the cameras
                        should_sort = true;
                    }
                }
                ObjUpdateCmd::Mesh { object_id, mesh } => {
                    if let Some(object) = self.bank.get_mut(&object_id.into()) {
                        object.mesh = Some(mesh);
                        log::trace!("updated object {:?} mesh", object_id);
                        should_sort = true;
                    }
                }
                ObjUpdateCmd::Material { object_id } => {
                    if let Some(object) = self.bank.get_mut(&object_id.into()) {
                        log::trace!("updated object {:?} material", object_id);
                        let inner = object.inner.read();
                        object.material_uniform = inner
                            .material
                            .as_ref()
                            .map(|mat| mat.create_material_uniform(device));
                    }
                }
                ObjUpdateCmd::Destroy { object_id } => {
                    log::debug!("destroying {:?}", object_id);
                    self.bank.destroy(object_id.into());
                    should_sort = true;
                }
            }
        }
        should_sort
    }

    pub(crate) fn update_queue(&self) -> Sender<ObjUpdateCmd> {
        self.object_update_queue.0.clone()
    }

    pub fn objects_with_property<'a>(
        &'a self,
        name: impl AsRef<str> + 'a,
    ) -> impl Iterator<Item = &'a ObjectData> + 'a {
        self.bank.iter().filter_map(move |obj| {
            let obj = obj.as_ref()?;
            if obj.inner.read().properties.contains_key(name.as_ref()) {
                Some(*obj)
            } else {
                None
            }
        })
    }
}

#[derive(moongraph::Edges)]
pub struct ObjectUpdate {
    pub objects: Write<Objects>,
    pub device: Read<crate::Device>,
    pub queue: Read<crate::Queue>,
}

impl ObjectUpdate {
    /// Graph node that runs all object updates.
    pub fn run(mut self) -> Result<(CamerasNeedSorting,), ObjectError> {
        Ok((CamerasNeedSorting(
            self.objects.update(&self.device, &self.queue),
        ),))
    }
}

/// A render graph resource to use as the objects to render.
pub struct RenderObjects(pub Vec<Id<Object>>);
