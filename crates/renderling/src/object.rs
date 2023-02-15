//! Renderable things with positions, transformations, meshes and materials.
use std::sync::{mpsc::Sender, Arc};

use glam::{Mat3, Mat4, Quat, Vec3};
use rustc_hash::FxHashSet;
use snafu::prelude::*;
use wgpu::util::DeviceExt;

use crate::{
    linkage::{ObjectDraw, ShaderObject},
    resources::Id,
    Camera, LocalTransform, Shared, WorldTransform,
};

pub(crate) enum ObjUpdateCmd {
    // remove the object from the camera's objects
    RemoveFromCamera {
        camera_id: Id<Camera>,
        object_id: Id<Object>,
    },
    // add the object to the camera's list of objects
    AddToCamera {
        camera_id: Id<Camera>,
        object_id: Id<Object>,
    },
    // update the given object's transform
    Transform {
        object_id: Id<Object>,
    },
    // update the given object's mesh
    Mesh {
        object_id: Id<Object>,
    },
    // update the given object's mesh
    Material {
        object_id: Id<Object>,
    },
    // Destroy this object
    Destroy {
        object_id: Id<Object>,
    },
}

#[derive(Debug, Snafu)]
pub enum ObjectBuilderError {
    #[snafu(display("object builder is missing `mesh`"))]
    MissingMesh,
}

pub struct ObjectBuilder<'a> {
    pub(crate) camera: Option<Id<Camera>>,
    pub(crate) mesh: Option<Arc<crate::Mesh>>,
    pub(crate) material: Option<crate::AnyMaterial>,
    pub(crate) local_transform: crate::LocalTransform,
    pub(crate) local_transforms: Vec<crate::LocalTransform>,
    pub(crate) children: Vec<&'a Object>,
    pub(crate) generate_normal_matrix: bool,
    pub(crate) is_visible: bool,
    pub(crate) update_tx: Sender<ObjUpdateCmd>,
    pub(crate) device: &'a wgpu::Device,
    pub(crate) scene: &'a mut crate::Stage,
}

impl<'a> ObjectBuilder<'a> {
    pub fn with_camera(mut self, camera: &crate::Camera) -> Self {
        self.camera = Some(camera.id);
        self
    }

    pub fn with_mesh(mut self, mesh: impl Into<Arc<crate::Mesh>>) -> Self {
        self.mesh = Some(mesh.into());
        self
    }

    pub fn with_mesh_builder<Vertex: bytemuck::Pod>(
        self,
        mesh_builder: crate::MeshBuilder<Vertex>,
    ) -> Self {
        let mesh = mesh_builder.build(Some("object-builder-mesh"), self.device);
        self.with_mesh(mesh)
    }

    pub fn with_transform(mut self, t: crate::LocalTransform) -> Self {
        self.local_transform = t;
        self
    }

    /// Add another local transform.
    ///
    /// This object will be rendered once with every transform using instancing.
    pub fn add_transform(mut self, t: crate::LocalTransform) -> Self {
        self.local_transforms.push(t);
        self
    }

    pub fn with_position(mut self, p: Vec3) -> Self {
        self.local_transform.translate = Vec3::new(p.x, p.y, p.z);
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.local_transform.rotate = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.local_transform.scale = scale;
        self
    }

    pub fn with_material<T: crate::Material>(mut self, material: impl Into<Arc<T>>) -> Self {
        self.material = Some(crate::AnyMaterial::new(material));
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
        self.is_visible = is_visible;
        self
    }

    pub fn build(self) -> Result<Object, ObjectBuilderError> {
        let material_uniform = self
            .material
            .as_ref()
            .map(|mat| mat.create_material_uniform(self.device));
        let position = self.local_transform.translate;
        let local_transforms = std::iter::once(self.local_transform)
            .chain(self.local_transforms)
            .collect::<Vec<_>>();
        let is_visible = self.camera.is_some() && self.is_visible;
        let id = self.scene.new_object_id();
        let inner = ObjectInner {
            camera: self.camera.clone(),
            // parent is set to `Some` when/if the parent is built, or updated
            parent: None,
            children: vec![],
            mesh: self.mesh.clone(),
            material: self.material,
            local_transforms,
            is_visible: self.is_visible,
        };
        let inner = Shared::new(inner);
        let mut children = vec![];
        for child in self.children.into_iter() {
            child.inner.write().parent = Some(ParentObject(inner.clone()));
            children.push(ChildObject(child.id));
            self.update_tx
                .send(ObjUpdateCmd::Transform {
                    object_id: child.id,
                })
                .unwrap();
        }
        inner.write().children = children;
        let instances = inner
            .read()
            .new_world_transforms_buffer(self.device, self.generate_normal_matrix);
        let obj_data = ObjectData {
            id,
            mesh: self.mesh,
            material_uniform,
            instances,
            world_position: position,
            cameras: FxHashSet::from_iter(self.camera.clone()),
            inner: inner.clone(),
        };

        self.scene.objects[id.0] = Some(obj_data);
        if is_visible {
            let camera_id = self.camera.as_ref().unwrap();
            self.scene.add_object_to_camera(&id, camera_id);
        }

        let object = Object {
            id,
            inner,
            cmd: self.update_tx,
        };

        Ok(object)
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
    pub(crate) mesh: Option<Arc<crate::Mesh>>,
    pub(crate) material: Option<crate::AnyMaterial>,
    pub(crate) camera: Option<Id<Camera>>,
    pub(crate) parent: Option<ParentObject>,
    pub(crate) children: Vec<ChildObject>,
    pub(crate) is_visible: bool,
    pub(crate) local_transforms: Vec<crate::LocalTransform>,
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
    fn drop(&mut self) {
        if self.inner.count() <= 1 {
            let _ = self.cmd.send(ObjUpdateCmd::Destroy { object_id: self.id });
        }
    }
}

impl Object {
    /// Associate this object with the given camera.
    ///
    /// This will have the effect that the object will be drawn with this camera
    /// on the next frame.
    pub fn set_camera(&self, camera: &crate::Camera) {
        let new_camera_id = camera.id;
        let object_id = self.id;
        if let Some(old_camera) = std::mem::replace(&mut self.inner.write().camera, Some(camera.id))
        {
            self.cmd
                .send(ObjUpdateCmd::RemoveFromCamera {
                    camera_id: old_camera,
                    object_id,
                })
                .unwrap();
        }
        self.cmd
            .send(ObjUpdateCmd::AddToCamera {
                camera_id: new_camera_id,
                object_id,
            })
            .unwrap();
    }

    /// Update the local transform of this object.
    pub fn set_transform(&self, transform: LocalTransform) {
        let mut inner = self.inner.write();
        *inner.local_transforms.get_mut(0).unwrap() = transform;
        self.cmd
            .send(ObjUpdateCmd::Transform { object_id: self.id })
            .unwrap();
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
            if is_visible {
                if let Some(camera) = inner.camera.as_ref() {
                    self.cmd
                        .send(ObjUpdateCmd::AddToCamera {
                            camera_id: *camera,
                            object_id: self.id,
                        })
                        .unwrap();
                }
            } else {
                if let Some(camera) = inner.camera.as_ref() {
                    self.cmd
                        .send(ObjUpdateCmd::RemoveFromCamera {
                            camera_id: *camera,
                            object_id: self.id,
                        })
                        .unwrap();
                }
            }
        }
    }

    /// Update the mesh of this object.
    pub fn set_mesh(&self, mesh: Arc<crate::Mesh>) {
        let mut inner = self.inner.write();
        inner.mesh = Some(mesh);
        self.cmd
            .send(ObjUpdateCmd::Mesh { object_id: self.id })
            .unwrap();
    }

    /// Update the material of this object.
    pub fn set_material<T: crate::Material>(&self, material: impl Into<Arc<T>>) {
        let mut inner = self.inner.write();
        inner.material = Some(crate::AnyMaterial::new(material));
        self.cmd
            .send(ObjUpdateCmd::Material { object_id: self.id })
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
            .send(ObjUpdateCmd::Transform {
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
            .send(ObjUpdateCmd::Transform {
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
                .send(ObjUpdateCmd::Transform { object_id: self.id })
                .unwrap();
        }
    }
}

/// Underlying data used by `wgpu` to render an object.
pub(crate) struct ObjectData {
    pub(crate) id: Id<Object>,
    pub(crate) mesh: Option<Arc<crate::Mesh>>,
    pub(crate) material_uniform: Option<crate::AnyMaterialUniform>,
    pub(crate) instances: crate::linkage::VertexBuffer,
    pub(crate) world_position: Vec3,
    pub(crate) cameras: FxHashSet<Id<Camera>>,
    pub(crate) inner: Shared<ObjectInner>,
}

impl ObjectData {
    pub(crate) fn as_shader_object(&self) -> Option<ShaderObject<'_>> {
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

    pub(crate) fn update_world_transform(
        &mut self,
        queue: &wgpu::Queue,
        generate_normal_matrix: bool,
    ) {
        log::trace!("updating object {:?} world transform", self.id);
        let inner = self.inner.read();
        inner.update_world_transforms_buffer(&queue, &self.instances, generate_normal_matrix);
        let parent_tfrm = inner.get_parent_world_transform().unwrap_or_default();
        let parent_model_matrix = Mat4::from(&parent_tfrm);
        let p = inner.local_transforms[0].translate;
        self.world_position = parent_model_matrix.project_point3(p);
    }
}
