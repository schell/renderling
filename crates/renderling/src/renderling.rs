//! Builds the UI pipeline and manages resources.
use nalgebra::{Matrix4, Point3, UnitQuaternion, Vector3};
use snafu::prelude::*;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, RwLock,
};
use wgpu::util::DeviceExt;

pub use renderling_ui::{
    begin_render_pass, create_camera_buffer_bindgroup, create_pipeline,
    create_ui_material_bindgroup, render_object, Camera as ShaderCamera, Object as ShaderObject,
    ObjectDraw, Vertex, ViewProjection,
};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{camera::*, resources::*, MeshBuilder, WorldTransform};

#[derive(Debug, Snafu)]
pub enum RenderlingError {
    #[snafu(display("wgpu stat is missing the current frame"))]
    RenderTargetMissingFrame,

    #[snafu(display("default_material field is `None`"))]
    MissingDefaultMaterial,
}

/// Create a new camera uniform
fn new_camera_uniform(
    inner: &CameraInner,
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::BindGroup) {
    let (projection, view) = inner.as_projection_and_view();
    let viewproj = ViewProjection {
        projection: projection.into(),
        view: view.into(),
    };
    create_camera_buffer_bindgroup(device, viewproj, "new_camera_uniform")
}

pub struct CameraBuilder<'a> {
    width: f32,
    height: f32,
    device: &'a wgpu::Device,
    update_tx: Sender<CameraUpdateCmd>,
    scene: &'a mut Scene,
    inner: CameraInner,
}

impl<'a> CameraBuilder<'a> {
    /// Create an orthographic 2d camera with a projection where the x axis
    /// increases to the right, the y axis increases down and z increases
    /// out of the screen towards the viewer.
    pub fn with_projection_ortho2d(mut self) -> Self {
        self.inner = CameraInner::new_ortho2d(self.width, self.height);
        self
    }

    /// Create a perspective 3d camera positioned at 0,12,20 looking at the origin.
    pub fn with_projection_perspective(mut self) -> Self {
        self.inner = CameraInner::new_perspective(self.width, self.height);
        self
    }

    pub fn build(self) -> Camera {
        let id = self.scene.new_camera_id();
        let CameraBuilder {
            device,
            update_tx: cmd,
            scene,
            width: _,
            height: _,
            inner,
        } = self;
        let (buffer, bindgroup) = new_camera_uniform(&inner, device);
        let inner = Shared::new(inner);
        let camera_data = CameraData {
            buffer,
            bindgroup,
            inner: inner.clone(),
        };
        let camera = Camera { id, cmd, inner };
        scene.cameras.push((id, camera_data));
        camera
    }
}

enum ObjUpdateCmd {
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
    camera: Option<Camera>,
    mesh: Option<Arc<crate::Mesh>>,
    material: Option<crate::AnyMaterial>,
    world_transform: WorldTransform,
    world_transforms: Vec<WorldTransform>,
    is_visible: bool,
    update_tx: Sender<ObjUpdateCmd>,
    device: &'a wgpu::Device,
    scene: &'a mut Scene,
}

impl<'a> ObjectBuilder<'a> {
    pub fn with_camera(mut self, camera: &Camera) -> Self {
        self.camera = Some(camera.clone());
        self
    }

    pub fn with_mesh(mut self, mesh: Arc<crate::Mesh>) -> Self {
        self.mesh = Some(mesh);
        self
    }

    pub fn with_mesh_builder(self, mesh_builder: MeshBuilder<Vertex>) -> Self {
        let mesh = mesh_builder.build(Some("object_builder_mesh"), self.device);
        self.with_mesh(Arc::new(mesh))
    }

    pub fn add_world_transform(mut self, wt: WorldTransform) -> Self {
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
        let material_bindgroup = self
            .material
            .as_ref()
            .map(|mat| mat.create_bindgroup(self.device));
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
        let instances = inner.new_world_transforms_buffer(self.device);
        let obj_inner = Shared::new(inner);
        let id = self.scene.new_object_id();
        let obj_data = ObjectData {
            id,
            mesh,
            material_bindgroup,
            instances,
            position,
            inner: obj_inner.clone(),
        };

        if is_visible {
            let camera_id = self.camera.as_ref().unwrap().id;
            let objects = self.scene.visible_objects.entry(camera_id).or_default();
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
struct ObjectInner {
    mesh: Arc<crate::Mesh>,
    material: Option<crate::AnyMaterial>,
    world_transforms: Vec<WorldTransform>,
    camera: Option<Camera>,
    is_visible: bool,
}

impl ObjectInner {
    /// Create a new instances buffer from a list of world transforms
    fn new_world_transforms_buffer(&self, device: &wgpu::Device) -> crate::VertexBuffer {
        let ms: Vec<f32> = self
            .world_transforms
            .iter()
            .flat_map(|m| Matrix4::from(m).as_slice().to_vec())
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

    fn update_world_transforms_buffer(&self, queue: &wgpu::Queue, buffer: &crate::VertexBuffer) {
        let ms: Vec<f32> = self
            .world_transforms
            .iter()
            .flat_map(|m| Matrix4::from(m).as_slice().to_vec())
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
    id: Id,
    inner: Shared<ObjectInner>,
    cmd: Sender<ObjUpdateCmd>,
}

impl Object {
    /// Associate this object with the given camera.
    ///
    /// This will have the effect that the object will be drawn with this camera on
    /// the next frame.
    pub fn set_camera(&self, camera: Camera) {
        let new_camera_id = camera.id;
        let object_id = self.id;
        if let Some(old_camera) = std::mem::replace(&mut self.inner.write().camera, Some(camera)) {
            self.cmd
                .send(ObjUpdateCmd::RemoveFromCamera {
                    camera_id: old_camera.id,
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
    pub fn set_transform(&self, transform: WorldTransform) {
        let mut inner = self.inner.write();
        *inner.world_transforms.get_mut(0).unwrap() = transform;
        self.cmd
            .send(ObjUpdateCmd::Transform {
                object_id: self.id,
                camera_id: if inner.is_visible {
                    inner.camera.as_ref().map(|c| c.id)
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
                            camera_id: camera.id,
                            object_id: self.id,
                        })
                        .unwrap();
                }
            } else {
                if let Some(camera) = inner.camera.as_ref() {
                    self.cmd
                        .send(ObjUpdateCmd::RemoveFromCamera {
                            camera_id: camera.id,
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
                camera_id: inner.camera.as_ref().map(|c| c.id),
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
                camera_id: inner.camera.as_ref().map(|c| c.id),
            })
            .unwrap();
    }
}

/// Underlying data used by `wgpu` to render an object.
struct ObjectData {
    id: Id,
    mesh: Arc<crate::Mesh>,
    material_bindgroup: Option<wgpu::BindGroup>,
    instances: crate::VertexBuffer,
    position: Point3<f32>,
    inner: Shared<ObjectInner>,
}

impl<'a> From<&'a ObjectData> for ShaderObject<'a, ()> {
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
            material: value.material_bindgroup.as_ref(),
            name: None,
            draw,
            extra: (),
        };
        object
    }
}

#[derive(Default)]
struct Scene {
    // for creating camera ids
    camera_id_bank: BankOfIds,
    // for creating objects
    object_id_bank: BankOfIds,
    // all cameras, in their intended render order
    cameras: Vec<(Id, CameraData)>,
    // invisible objects keyed by their object id
    invisible_objects: FxHashMap<Id, ObjectData>,
    // all visible objects collated by their camera's id, in render order
    visible_objects: FxHashMap<Id, Vec<ObjectData>>,
}

impl Scene {
    fn new_camera_id(&self) -> Id {
        self.camera_id_bank.dequeue()
    }

    fn new_object_id(&self) -> Id {
        self.object_id_bank.dequeue()
    }

    fn get_camera_mut(&mut self, camera_id: Id) -> Option<&mut CameraData> {
        self.cameras.iter_mut().find_map(|c| {
            if c.0 == camera_id {
                Some(&mut c.1)
            } else {
                None
            }
        })
    }

    fn find_object_data_mut(
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

    fn remove_object_from_camera(&mut self, object_id: &Id, camera_id: &Id) {
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

    fn add_object_to_camera(&mut self, object_id: &Id, camera_id: &Id) -> bool {
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
    // link to the global wgpu state
    //wgpu_state: Arc<crate::WgpuState>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    size: Arc<RwLock<(u32, u32)>>,
    // Built shader render pipeline
    pipeline: crate::AnyPipeline,
    // default material to use when there is no other
    pub default_material: Option<crate::AnyMaterial>,
    pub default_material_bindgroup: Option<wgpu::BindGroup>,
    // The entire scene - all cameras and objects
    scene: Scene,
}

impl Renderling {
    pub fn new<T: crate::Pipeline>(
        wgpu_state: &crate::WgpuState,
        pipeline: impl Into<Arc<T>>,
    ) -> Self {
        Self {
            device: wgpu_state.device.clone(),
            queue: wgpu_state.queue.clone(),
            size: wgpu_state.size.clone(),
            pipeline: crate::AnyPipeline::new::<T>(pipeline.into()),
            object_update_queue: channel(),
            camera_update_queue: channel(),
            default_material: None,
            default_material_bindgroup: None,
            scene: Default::default(),
        }
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
            is_visible: true,
            update_tx: self.object_update_queue.0.clone(),
            device: &self.device,
            scene: &mut self.scene,
        }
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
                    self.scene.remove_object_from_camera(&object_id, &camera_id);
                }
                ObjUpdateCmd::AddToCamera {
                    camera_id,
                    object_id,
                } => {
                    if self.scene.add_object_to_camera(&object_id, &camera_id) {
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
                        object
                            .inner
                            .read()
                            .update_world_transforms_buffer(&self.queue, &object.instances);
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
                        let inner = object.inner.read();
                        object.material_bindgroup = inner
                            .material
                            .as_ref()
                            .map(|mat| mat.create_bindgroup(&self.device));
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

        Ok(())
    }

    /// Conduct a full render pass into the given textures.
    pub fn render(
        &self,
        frame_texture_view: &wgpu::TextureView,
        depth_texture_view: &wgpu::TextureView,
    ) -> Result<(), RenderlingError> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Renderling encoder"),
            });

        let mut render_pass =
            self.pipeline
                .begin_render_pass(&mut encoder, frame_texture_view, depth_texture_view);

        let default_material_bindgroup = self
            .default_material_bindgroup
            .as_ref()
            .context(MissingDefaultMaterialSnafu)?;

        // render
        for (camera_id, camera_data) in self.scene.cameras.iter() {
            // bind the camera to our shader uniform
            render_pass.set_bind_group(0, &camera_data.bindgroup, &[]);

            if let Some(visible_objects) = self.scene.visible_objects.get(camera_id) {
                for object in visible_objects.iter().map(ShaderObject::from) {
                    render_object(&mut render_pass, object, default_material_bindgroup)
                }
            }
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use nalgebra::{Perspective3, Point2};

    use crate::{renderling::*, MeshBuilder, UiColorBlend, UiMaterial, WgpuState};

    fn right_tri_builder() -> MeshBuilder<Vertex> {
        MeshBuilder::default().with_vertices(vec![
            Vertex::default()
                .with_position(0.0, 0.0, 0.5)
                .with_color(0.0, 1.0, 1.0, 1.0),
            Vertex::default()
                .with_position(100.0, 0.0, 0.5)
                .with_color(1.0, 0.0, 1.0, 1.0),
            Vertex::default()
                .with_position(0.0, 100.0, 0.5)
                .with_color(1.0, 1.0, 0.0, 1.0),
        ])
    }

    struct CmyTri {
        gpu: WgpuState,
        ui: Renderling,
        _cam: Camera,
        tri: Object,
    }

    fn cmy_triangle_setup() -> CmyTri {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::WHITE;

        let mut ui: Renderling = gpu.new_ui_renderling();
        let cam = ui.new_camera().with_projection_ortho2d().build();
        let tri = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(right_tri_builder())
            .build()
            .unwrap();
        CmyTri {
            gpu,
            ui,
            _cam: cam,
            tri,
        }
    }

    #[test]
    fn cmy_triangle() {
        let mut c = cmy_triangle_setup();
        let (frame, depth) = c.gpu.next_frame().unwrap();
        c.gpu.clear(&frame, Some(&depth));
        c.ui.update().unwrap();
        c.ui.render(&frame, &depth).unwrap();
        let img = c.gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_triangle", "../../img/cmy_triangle.png", img).unwrap();
    }

    #[test]
    fn cmy_triangle_update_transform() {
        let mut c = cmy_triangle_setup();
        let (frame, depth) = c.gpu.next_frame().unwrap();
        c.gpu.clear(&frame, Some(&depth));
        c.ui.update().unwrap();
        c.ui.render(&frame, &depth).unwrap();
        c.gpu.present().unwrap();

        let (frame, depth) = c.gpu.next_frame().unwrap();
        c.gpu.clear(&frame, Some(&depth));
        c.tri.set_transform(
            WorldTransform::default()
                .with_position(Point3::new(100.0, 0.0, 0.0))
                .with_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::z_axis(),
                    std::f32::consts::FRAC_PI_2,
                ))
                .with_scale(Vector3::new(0.5, 0.5, 1.0)),
        );
        c.ui.update().unwrap();
        c.ui.render(&frame, &depth).unwrap();

        let img = c.gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_triangle_update_transform",
            "../../img/cmy_triangle_update_transform.png",
            img,
        )
        .unwrap();
    }

    /// Points around the unit cube.
    ///
    ///    yb          1_____2     _____
    ///    |           /    /|    /|    |  (same box, left and front sides removed)
    ///    |___x     0/___3/ |   /7|____|6
    ///   /    g      |    | /   | /    /
    /// z/r           |____|/   4|/____/5
    pub fn unit_points() -> [Point3<f32>; 8] {
        let p0 = Point3::from([-0.5, 0.5, 0.5]);
        let p1 = Point3::from([-0.5, 0.5, -0.5]);
        let p2 = Point3::from([0.5, 0.5, -0.5]);
        let p3 = Point3::from([0.5, 0.5, 0.5]);

        let p4 = Point3::from([-0.5, -0.5, 0.5]);
        let p7 = Point3::from([-0.5, -0.5, -0.5]);
        let p6 = Point3::from([0.5, -0.5, -0.5]);
        let p5 = Point3::from([0.5, -0.5, 0.5]);

        [p0, p1, p2, p3, p4, p5, p6, p7]
    }

    /// Points around a pyramid height=1 with the base around the origin.
    ///
    ///    yb
    ///    |               *top
    ///    |___x       tl_____tr
    ///   /    g        /    /
    /// z/r          bl/____/br
    fn pyramid_points() -> [Point3<f32>; 5] {
        let tl = Point3::new(-0.5, -0.5, -0.5);
        let tr = Point3::new(0.5, -0.5, -0.5);
        let br = Point3::new(0.5, -0.5, 0.5);
        let bl = Point3::new(-0.5, -0.5, 0.5);
        let top = Point3::new(0.0, 0.5, 0.0);
        [tl, tr, br, bl, top]
    }

    fn pyramid_indices() -> [u16; 18] {
        let (tl, tr, br, bl, top) = (0, 1, 2, 3, 4);
        [
            // bottom
            tl, bl, br, tl, br, tr, // front
            br, bl, top, // left
            bl, tl, top, // back
            tl, tr, top, // right
            tr, br, top,
        ]
    }

    fn cmy_vertex(p: Point3<f32>) -> Vertex {
        let r: f32 = p.z + 0.5;
        let g: f32 = p.x + 0.5;
        let b: f32 = p.y + 0.5;
        Vertex::default()
            .with_position(p.x.min(1.0), p.y.min(1.0), p.z.min(1.0))
            .with_color(r, g, b, 1.0)
    }

    fn cube_builder() -> MeshBuilder<Vertex> {
        let vertices = unit_points();
        let indices: [u16; 12 * 3] = [
            0, 1, 2, 0, 2, 3, // top
            0, 3, 4, 4, 3, 5, // front
            3, 2, 6, 3, 6, 5, // right
            1, 0, 7, 7, 0, 4, // left
            4, 5, 6, 4, 6, 7, // bottom
            2, 1, 7, 2, 7, 6, // back
        ];
        MeshBuilder::default()
            .with_vertices(vertices.map(cmy_vertex))
            .with_indices(indices)
    }

    fn pyramid_builder() -> MeshBuilder<Vertex> {
        let vertices = pyramid_points();
        let indices = pyramid_indices();
        MeshBuilder::default()
            .with_vertices(vertices.map(cmy_vertex))
            .with_indices(indices)
    }

    #[test]
    fn cmy_cube() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::WHITE;

        let mut ui: Renderling = gpu.new_ui_renderling();

        // test updating the camera by starting with ortho2d
        let cam = ui.new_camera().with_projection_ortho2d().build();
        cam.look_at(Point3::new(0.0, 12.0, 20.0), Point3::origin(), Vector3::y());
        cam.set_projection(Projection::Perspective(Perspective3::new(
            1.0,
            std::f32::consts::PI / 4.0,
            0.1,
            100.0,
        )));

        let _cube = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_scale(Vector3::new(6.0, 6.0, 6.0))
            .with_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                -std::f32::consts::FRAC_PI_4,
            ))
            .build()
            .unwrap();

        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq("cmy_cube", "../../img/cmy_cube.png", img).unwrap();
    }

    #[test]
    fn cmy_cube_visible() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::WHITE;

        let mut ui: Renderling = gpu.new_ui_renderling();

        let cam = ui.new_camera().with_projection_perspective().build();

        let _cube_one = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_position(Point3::new(-4.0, 0.0, 0.0))
            .with_scale(Vector3::new(6.0, 6.0, 6.0))
            .with_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                -std::f32::consts::FRAC_PI_4,
            ))
            .build()
            .unwrap();

        let cube_two = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_position(Point3::new(4.0, 0.0, 0.0))
            .with_scale(Vector3::new(6.0, 6.0, 6.0))
            .with_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                std::f32::consts::FRAC_PI_4,
            ))
            .build()
            .unwrap();

        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_visible_before.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_before",
            "../../img/cmy_cube_visible_before.png",
            img,
        )
        .unwrap();

        cube_two.set_visible(false);
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_visible_after.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_after",
            "../../img/cmy_cube_visible_after.png",
            img,
        )
        .unwrap();

        cube_two.set_visible(true);
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_visible_before_again",
            "../../img/cmy_cube_visible_before.png",
            img,
        )
        .unwrap();
    }

    #[test]
    fn cmy_cube_remesh() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::TRANSPARENT;
        let mut ui: Renderling = gpu.new_ui_renderling();
        let cam = ui.new_camera().with_projection_perspective().build();
        let cube = ui
            .new_object()
            .with_camera(&cam)
            .with_mesh_builder(cube_builder())
            .with_scale(Vector3::new(10.0, 10.0, 10.0))
            .build()
            .unwrap();
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_remesh_before.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_remesh_before",
            "../../img/cmy_cube_remesh_before.png",
            img,
        )
        .unwrap();

        let pyramid_mesh = pyramid_builder().build(Some("pyramid mesh"), &gpu.device);
        cube.set_mesh(Arc::new(pyramid_mesh));
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_remesh_after.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_remesh_after",
            "../../img/cmy_cube_remesh_after.png",
            img,
        )
        .unwrap();
    }

    #[test]
    fn cmy_cube_material() {
        let mut gpu = WgpuState::headless(100, 100).unwrap();
        gpu.default_background_color = wgpu::Color::TRANSPARENT;
        let mut ui: Renderling = gpu.new_ui_renderling();
        let cam = ui.new_camera().with_projection_perspective().build();
        let png = image::open("../../img/sandstone.png").unwrap();
        let tex = gpu
            .create_texture(Some("sandstone_material"), &png.to_rgba8())
            .unwrap();
        let material = UiMaterial {
            diffuse_texture: tex,
            color_blend: UiColorBlend::UvOnly,
        };
        let builder = MeshBuilder::default().with_vertices({
            let p: [Point3<f32>; 8] = unit_points();
            let tl = Point2::from([0.0, 0.0]);
            let tr = Point2::from([1.0, 0.0]);
            let bl = Point2::from([0.0, 1.0]);
            let br = Point2::from([1.0, 1.0]);

            vec![
                // top
                Vertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[1].x, p[1].y, p[1].z)
                    .with_uv(tl.x, tl.y),
                Vertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(br.x, br.y),
                // bottom
                Vertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(tl.x, tl.y),
                Vertex::default()
                    .with_position(p[6].x, p[6].y, p[6].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[6].x, p[6].y, p[6].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[7].x, p[7].y, p[7].z)
                    .with_uv(br.x, br.y),
                // left
                Vertex::default()
                    .with_position(p[7].x, p[7].y, p[7].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[1].x, p[1].y, p[1].z)
                    .with_uv(tl.x, tl.y),
                Vertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[7].x, p[7].y, p[7].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(br.x, br.y),
                // right
                Vertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(tl.x, tl.y),
                Vertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[2].x, p[2].y, p[2].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[6].x, p[6].y, p[6].z)
                    .with_uv(br.x, br.y),
                // front
                Vertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[0].x, p[0].y, p[0].z)
                    .with_uv(tl.x, tl.y),
                Vertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[4].x, p[4].y, p[4].z)
                    .with_uv(bl.x, bl.y),
                Vertex::default()
                    .with_position(p[3].x, p[3].y, p[3].z)
                    .with_uv(tr.x, tr.y),
                Vertex::default()
                    .with_position(p[5].x, p[5].y, p[5].z)
                    .with_uv(br.x, br.y),
            ]
        });
        let cube = ui
            .new_object()
            .with_camera(&cam)
            .with_material(material)
            .with_mesh_builder(builder)
            .with_scale(Vector3::new(10.0, 10.0, 10.0))
            .build()
            .unwrap();
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_material_before.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_material_before",
            "../../img/cmy_cube_material_before.png",
            img,
        )
        .unwrap();

        let png = image::open("../../img/dirt.jpg").unwrap();
        let tex = gpu
            .create_texture(Some("dirt_material"), &png.to_rgba8())
            .unwrap();
        let material = UiMaterial {
            diffuse_texture: tex,
            color_blend: UiColorBlend::UvOnly,
        };
        cube.set_material(material);
        let (frame, depth) = gpu.next_frame().unwrap();
        gpu.clear(&frame, Some(&depth));
        ui.update().unwrap();
        ui.render(&frame, &depth).unwrap();

        let img = gpu.grab_frame_image().unwrap();
        //img.save_with_format(
        //    "../../img/cmy_cube_material_after.png",
        //    image::ImageFormat::Png,
        //)
        //.unwrap();
        crate::img_diff::assert_img_eq(
            "cmy_cube_material_after",
            "../../img/cmy_cube_material_after.png",
            img,
        )
        .unwrap();
    }
}
