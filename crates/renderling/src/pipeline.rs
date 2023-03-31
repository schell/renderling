//! Pipeline types and utilities.

use crate::BankOfIds;

pub struct PipelineBindGroupIndexConfig {
    // The index of the camera's bindgroup
    pub camera_bindgroup_index: u32,
    // The index of the lights's bindgroup
    pub light_bindgroup_index: u32,
    // The index of the material's bindgroup
    pub material_bindgroup_index: u32,
}

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    // default material to use when there is no other
    pub default_material: crate::AnyMaterial,
    // default material bindgroup
    pub default_material_uniform: crate::AnyMaterialUniform,
    // bindgroup index config
    pub bindgroup_index_config: PipelineBindGroupIndexConfig
}

#[derive(Default)]
pub struct Pipelines {
    pub(crate) ids: BankOfIds<Pipeline>
}
