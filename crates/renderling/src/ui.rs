//! Ui pipeline, material definitions and sub-graph.
use std::ops::Deref;

use crate::{AnyMaterialUniform, Id, Material, MaterialUniform, Pipeline, Pipelines, AnyMaterial, Queue, Device, RenderTarget};

#[cfg(feature = "text")]
mod text;
use moongraph::{Write, Read, Edges};
pub use renderling_shader::ui::UiColorBlend;
#[cfg(feature = "text")]
pub use text::*;

pub type UiVertex = crate::linkage::ui::Vertex;

pub struct UiMaterialUniform {
    bindgroup: wgpu::BindGroup,
}

impl MaterialUniform for UiMaterialUniform {
    fn get_bindgroup(&self) -> &wgpu::BindGroup {
        &self.bindgroup
    }
}

/// A material for ui meshes.
#[derive(Debug)]
pub struct UiMaterial {
    pub diffuse_texture: crate::texture::Texture,
    pub color_blend: UiColorBlend,
}

impl Material for UiMaterial {
    fn create_material_uniform(&self, device: &wgpu::Device) -> AnyMaterialUniform {
        AnyMaterialUniform::new(UiMaterialUniform {
            bindgroup: crate::linkage::ui::create_ui_material_bindgroup(device, &self),
        })
    }
}

/// A pipeline for UI.
pub struct UiPipeline {
    id: Id<Pipeline>,
    inner: Pipeline,
}

impl Deref for UiPipeline {
    type Target = Pipeline;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl UiPipeline {
    /// Creates a new UI shader pipeline.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        pipelines: &mut Pipelines,
    ) -> Self {
        let diffuse_texture = crate::Texture::new(
            device,
            queue,
            Some("ui-default-diffuse"),
            None,
            4,
            1,
            1,
            &[0, 0, 0, 0],
        );

        let material = crate::UiMaterial {
            diffuse_texture,
            color_blend: crate::UiColorBlend::ColorOnly,
        };

        UiPipeline {
            id: pipelines.ids.dequeue(),
            inner: Pipeline {
                pipeline: crate::linkage::ui::create_pipeline(device, format),
                default_material_uniform: material.create_material_uniform(device),
                default_material: AnyMaterial::new(material),
                bindgroup_index_config: crate::PipelineBindGroupIndexConfig {
                    camera_bindgroup_index: 0,
                    light_bindgroup_index: 2,
                    material_bindgroup_index: 1,
                },
            },
        }
    }

    pub fn id(&self) -> Id<Pipeline> {
        self.id
    }
}


/// Helper type to create a UiPipeline from a [`Renderer`].
#[derive(Edges)]
pub struct UiPipelineCreator {
    device: Read<Device>,
    queue: Read<Queue>,
    target: Read<RenderTarget>,
    pipelines: Write<Pipelines>,
}

impl UiPipelineCreator {
    /// Helper to create a UiPipeline from a [`Renderer`].
    pub fn create(Self { device, queue, target, mut pipelines }: Self) -> UiPipeline {
        UiPipeline::new(&device, &queue, target.format(), &mut pipelines)
    }
}
