//! Renderable things with positions, transformations, meshes and materials.
use std::sync::{mpsc::Sender, Arc};

use nalgebra::{Matrix4, Point3, UnitQuaternion, Vector3};
use renderling_core::{ObjectDraw, ShaderObject};
use snafu::prelude::*;
use wgpu::util::DeviceExt;

use crate::resources::Id;

pub(crate) enum ObjUpdateCmd {
    // remove the object from the camera's objects
    RemoveFromCamera {
        camera_id: Id,
        object_id: Id,
    },
    // add the object to the camera's list of objects
    AddToCamera {
        camera_id: Id,
        object_id: Id,
    },
    // update the given object's transform
    Transform {
        object_id: Id,
        camera_id: Option<Id>,
    },
    // update the given object's mesh
    Mesh {
        object_id: Id,
        camera_id: Option<Id>,
    },
    // update the given object's mesh
    Material {
        object_id: Id,
        camera_id: Option<Id>,
    },
}

#[derive(Debug, Snafu)]
pub enum ObjectBuilderError {
    #[snafu(display("object builder is missing `mesh`"))]
    MissingMesh,
}

pub struct ObjectBuilder<'a> {
    pub(crate) camera: Option<Id>,
    pub(crate) mesh: Option<Arc<crate::Mesh>>,
    pub(crate) material: Option<crate::AnyMaterial>,
    pub(crate) world_transform: crate::WorldTransform,
    pub(crate) world_transforms: Vec<crate::WorldTransform>,
    pub(crate) generate_normal_matrix: bool,
    pub(crate) is_visible: bool,
    pub(crate) update_tx: Sender<ObjUpdateCmd>,
    pub(crate) device: &'a wgpu::Device,
    pub(crate) scene: &'a mut crate::Scene,
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

    pub fn add_world_transform(mut self, wt: crate::WorldTransform) -> Self {
        self.world_transforms.push(wt);
        self
    }

    pub fn with_position(mut self, p: Point3<f32>) -> Self {
        self.world_transform.translate = Vector3::new(p.x, p.y, p.z);
        self
    }

    pub fn with_rotation(mut self, rotation: UnitQuaternion<f32>) -> Self {
        self.world_transform.rotate = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.world_transform.scale = scale;
        self
    }

    pub fn with_material<T: crate::Material>(mut self, material: impl Into<Arc<T>>) -> Self {
        self.material = Some(crate::AnyMaterial::new(material));
        self
    }

    pub fn with_is_visible(mut self, is_visible: bool) -> Self {
        self.is_visible = is_visible;
        self
    }

    pub fn build(self) -> Result<Object, ObjectBuilderError> {
        //let id = self.data.object_id_bank.dequeue();
        let material_uniform = self
            .material
            .as_ref()
            .map(|mat| mat.create_material_uniform(self.device));
        let mesh = self.mesh.context(MissingMeshSnafu)?;
        let position = Point3::from(self.world_transform.translate.xyz());
        let world_transforms = std::iter::once(self.world_transform)
            .chain(self.world_transforms)
            .collect::<Vec<_>>();
        let is_visible = self.camera.is_some() && self.is_visible;
        let inner = ObjectInner {
            camera: self.camera.clone(),
            mesh: mesh.clone(),
            material: self.material,
            world_transforms,
            is_visible: self.is_visible,
        };
        let instances = inner.new_world_transforms_buffer(self.device, self.generate_normal_matrix);
        let obj_inner = crate::Shared::new(inner);
        let id = self.scene.new_object_id();
        let obj_data = ObjectData {
            id,
            mesh,
            material_uniform,
            instances,
            position,
            inner: obj_inner.clone(),
        };

        if is_visible {
            let camera_id = self.camera.as_ref().unwrap();
            let objects = self.scene.visible_objects.entry(*camera_id).or_default();
            objects.push(obj_data);
        } else {
            let _ = self.scene.invisible_objects.insert(obj_data.id, obj_data);
        }

        let object = Object {
            id,
            inner: obj_inner,
            cmd: self.update_tx,
        };

        Ok(object)
    }
}

/// Data shared between the library user and the renderling backend.
///
/// The data held in `ObjectInner` is data that the library user can change at any time
/// and data that has a downstream representation in `wgpu`, which is created/modified
/// in `Renderling::update`.
pub(crate) struct ObjectInner {
    pub(crate) mesh: Arc<crate::Mesh>,
    pub(crate) material: Option<crate::AnyMaterial>,
    pub(crate) world_transforms: Vec<crate::WorldTransform>,
    pub(crate) camera: Option<Id>,
    pub(crate) is_visible: bool,
}

impl ObjectInner {
    fn model_matrix_to_vec<M: Into<Matrix4<f32>>>(model: M, generate_normal_matrix: bool) -> Vec<f32> {
        let model = model.into();
        let mut m = model.as_slice().to_vec();
        if generate_normal_matrix {
            let normal = model.try_inverse().unwrap().transpose().fixed_resize::<3, 3>(0.0);
            let mut n = normal.as_slice().to_vec();
            m.append(&mut n);
        }
        m
    }

    /// Create a new instances buffer from a list of world transforms
    pub(crate) fn new_world_transforms_buffer(&self, device: &wgpu::Device,
                                              generate_normal_matrix: bool,
    ) -> crate::VertexBuffer {
        let ms: Vec<f32> = self
            .world_transforms
            .iter()
            .flat_map(|t| Self::model_matrix_to_vec(t, generate_normal_matrix))
            .collect::<Vec<_>>();
        crate::VertexBuffer {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Renderling instance buffer"),
                contents: bytemuck::cast_slice(ms.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            len: self.world_transforms.len(),
        }
    }

    pub(crate) fn update_world_transforms_buffer(
        &self,
        queue: &wgpu::Queue,
        buffer: &crate::VertexBuffer,
        generate_normal_matrix: bool,
    ) {
        let ms: Vec<f32> = self
            .world_transforms
            .iter()
            .flat_map(|t| Self::model_matrix_to_vec(t, generate_normal_matrix))
            .collect::<Vec<_>>();

        queue.write_buffer(&buffer.buffer, 0, bytemuck::cast_slice(ms.as_slice()));
    }
}

/// A library-user-facing display "object".
///
/// `Object`s are used as a handle to update graphical resources within the
/// renderling that was used to create it. To release the underlying resources
/// the object should be dropped.
pub struct Object {
    pub(crate) id: Id,
    pub(crate) inner: crate::Shared<ObjectInner>,
    pub(crate) cmd: Sender<ObjUpdateCmd>,
}

impl Object {
    /// Associate this object with the given camera.
    ///
    /// This will have the effect that the object will be drawn with this camera on
    /// the next frame.
    pub fn set_camera(&self, camera: &crate::Camera) {
        let new_camera_id = camera.id;
        let object_id = self.id;
        if let Some(old_camera) = std::mem::replace(&mut self.inner.write().camera, Some(camera.id)) {
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

    /// Update the transform of this object.
    pub fn set_transform(&self, transform: crate::WorldTransform) {
        let mut inner = self.inner.write();
        *inner.world_transforms.get_mut(0).unwrap() = transform;
        self.cmd
            .send(ObjUpdateCmd::Transform {
                object_id: self.id,
                camera_id: if inner.is_visible {
                    inner.camera.as_ref().copied()
                } else {
                    None
                },
            })
            .unwrap();
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
        inner.mesh = mesh;
        self.cmd
            .send(ObjUpdateCmd::Mesh {
                object_id: self.id,
                camera_id: inner.camera.as_ref().copied(),
            })
            .unwrap();
    }

    /// Update the material of this object.
    pub fn set_material<T: crate::Material>(&self, material: impl Into<Arc<T>>) {
        let mut inner = self.inner.write();
        inner.material = Some(crate::AnyMaterial::new(material));
        self.cmd
            .send(ObjUpdateCmd::Material {
                object_id: self.id,
                camera_id: inner.camera.as_ref().copied(),
            })
            .unwrap();
    }
}

/// Underlying data used by `wgpu` to render an object.
pub(crate) struct ObjectData {
    pub(crate) id: Id,
    pub(crate) mesh: Arc<crate::Mesh>,
    pub(crate) material_uniform: Option<crate::AnyMaterialUniform>,
    pub(crate) instances: crate::VertexBuffer,
    pub(crate) position: Point3<f32>,
    pub(crate) inner: crate::Shared<ObjectInner>,
}

impl<'a> From<&'a ObjectData> for ShaderObject<'a> {
    fn from(value: &'a ObjectData) -> Self {
        let draw = value
            .mesh
            .index_buffer
            .as_ref()
            .map(|mb| ObjectDraw::Indexed {
                index_buffer: mb.buffer.slice(..),
                index_format: wgpu::IndexFormat::Uint16,
                index_range: 0..mb.len as u32,
                base_vertex: 0,
            })
            .unwrap_or_else(|| ObjectDraw::Default {
                vertex_range: 0..value.mesh.vertex_buffer.len as u32,
            });
        let object = ShaderObject {
            mesh_buffer: value.mesh.vertex_buffer.buffer.slice(..),
            instances: value.instances.buffer.slice(..),
            instances_range: 0..value.instances.len as u32,
            material: value.material_uniform.as_ref().map(|mu| mu.get_bindgroup()),
            name: None,
            draw,
        };
        object
    }
}
