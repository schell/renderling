//! Entity builder.

use glam::{Quat, Vec3};
use renderling_shader::scene::{GpuEntity, GpuVertex};

use crate::SceneBuilder;

pub struct EntityBuilder<'a> {
    pub(crate) scene: &'a mut SceneBuilder,
    pub(crate) entity: GpuEntity,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.entity.visible = if is_visible { 1 } else { 0 };
        self
    }

    pub fn with_meshlet(mut self, vertices: impl IntoIterator<Item = GpuVertex>) -> Self {
        let (start, len) = self.scene.add_meshlet(vertices);
        self.entity.mesh_first_vertex = start;
        self.entity.mesh_vertex_count = len;
        self
    }

    pub fn with_starting_vertex_and_count(mut self, first_vertex: u32, count: u32) -> Self {
        self.entity.mesh_first_vertex = first_vertex;
        self.entity.mesh_vertex_count = count;
        self
    }

    pub fn with_position(mut self, position: impl Into<Vec3>) -> Self {
        self.entity.position = position.into().extend(0.0);
        self
    }

    pub fn with_scale(mut self, scale: impl Into<Vec3>) -> Self {
        self.entity.scale = scale.into().extend(0.0);
        self
    }

    pub fn with_rotation(mut self, rotation: impl Into<Quat>) -> Self {
        self.entity.rotation = rotation.into();
        self
    }

    pub fn with_material(mut self, material_id: u32) -> Self {
        self.entity.material = material_id;
        self
    }

    pub fn with_parent(mut self, parent: &GpuEntity) -> Self {
        if let Some(parent) = self.scene.entities.get_mut(parent.id as usize) {
            self.entity.parent = parent.id;
        } else {
            log::error!("no such parent entity {}", parent.id);
        }
        self
    }

    pub fn build(self) -> GpuEntity {
        let EntityBuilder { scene, mut entity } = self;
        entity.id = scene.entities.len() as u32;
        scene.entities.push(entity.clone());
        entity
    }
}
