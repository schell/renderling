//! Pipeline types and utilities.

use crate::bank::Bank;

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    // default material to use when there is no other
    default_material: crate::AnyMaterial,
    // default material bindgroup
    default_material_uniform: Option<crate::AnyMaterialUniform>,
    // whether object meshes have a 3x3 normal matrix attribute (defaults to `true`)
    meshes_have_normal_matrix_attribute: bool,
    // The index of the camera's bindgroup
    camera_bindgroup_index: u32,
    // The index of the lights's bindgroup
    light_bindgroup_index: u32,
    // The index of the material's bindgroup
    material_bindgroup_index: u32,
}

impl Pipeline {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

#[derive(Default)]
pub struct Pipelines {
    bank: Bank<Pipeline>
}
