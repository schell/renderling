//! Entity builder.
use glam::{Quat, Vec3};
use renderling_shader::scene::{GpuEntity, GpuMaterial, GpuVertex, Id};

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

    pub fn with_is_skin(mut self, is_skin: bool) -> Self {
        self.entity.info.set_is_skin(is_skin);
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

    pub fn with_num_morph_targets(mut self, num_morph_targets: u8) -> Self {
        self.entity.info.set_num_morph_targets(num_morph_targets);
        self
    }

    pub fn with_morph_targets_have_positions(mut self, has: bool) -> Self {
        self.entity.info.set_morph_targets_have_positions(has);
        self
    }

    pub fn with_morph_targets_have_normals(mut self, has: bool) -> Self {
        self.entity.info.set_morph_targets_have_normals(has);
        self
    }

    pub fn with_morph_targets_have_tangents(mut self, has: bool) -> Self {
        self.entity.info.set_morph_targets_have_tangents(has);
        self
    }

    pub fn with_morph_target_weights(mut self, weights: impl IntoIterator<Item = f32>) -> Self {
        for (i, weight) in weights.into_iter().take(8).enumerate() {
            self.entity.morph_targets_weights[i] = weight;
        }
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

    pub fn with_material(mut self, material_id: Id<GpuMaterial>) -> Self {
        self.entity.material = material_id;
        self
    }

    pub fn with_parent(mut self, parent: impl Into<Id<GpuEntity>>) -> Self {
        self.entity.parent = parent.into();
        self
    }

    pub fn build(self) -> GpuEntity {
        let EntityBuilder { scene, mut entity } = self;
        entity.id = Id::new(scene.entities.len() as u32);
        scene.entities.push(entity.clone());
        entity
    }
}
