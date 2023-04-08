//! Forward pipeline and material definitions
use std::ops::Deref;

use glam::Vec4;
use moongraph::{Edges, Read, Write};

use crate::{
    AnyMaterial, AnyMaterialUniform, Device, Id, Material, MaterialUniform, Pipeline, Pipelines,
    Queue, RenderTarget,
};

pub type ForwardVertex = crate::linkage::pbr::Vertex;

impl MaterialUniform for crate::linkage::pbr::MaterialUniform {
    fn get_bindgroup(&self) -> &wgpu::BindGroup {
        &self.bindgroup
    }
}

/// A blinn-phong material for forward meshes.
#[derive(Debug)]
pub struct BlinnPhongMaterial {
    pub diffuse_texture: crate::texture::Texture,
    pub specular_texture: crate::texture::Texture,
    pub shininess: f32,
}

impl crate::Material for BlinnPhongMaterial {
    fn create_material_uniform(&self, device: &wgpu::Device) -> crate::AnyMaterialUniform {
        let material_uniform = crate::linkage::pbr::MaterialUniform::new(device, &self);
        AnyMaterialUniform::new(material_uniform)
    }
}

fn to_color_bytes(color: Vec4) -> [u8; 4] {
    let [r, g, b, a] = color.to_array();
    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ]
}

impl BlinnPhongMaterial {
    pub fn from_colors(
        r: &crate::Renderling,
        diffuse: glam::Vec4,
        specular: glam::Vec4,
        shininess: f32,
    ) -> Self {
        let diffuse_texture = crate::Texture::new(
            r.get_device(),
            r.get_queue(),
            None,
            None,
            4,
            1,
            1,
            &to_color_bytes(diffuse),
        );
        let specular_texture = crate::Texture::new(
            &r.get_device(),
            &r.get_queue(),
            None,
            None,
            4,
            1,
            1,
            &to_color_bytes(specular),
        );
        BlinnPhongMaterial {
            diffuse_texture,
            specular_texture,
            shininess,
        }
    }
}

/// A pipeline for phong shaded 3d objects.
pub struct ForwardPipeline {
    _id: Id<Pipeline>,
    inner: Pipeline,
}

impl Deref for ForwardPipeline {
    type Target = Pipeline;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ForwardPipeline {
    /// Creates a new forward shader pipeline.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        primitive: Option<wgpu::PrimitiveState>,
        pipelines: &mut Pipelines,
    ) -> Self {
        // this is the _default_ texture bind group which will be used when
        // there is no available texture to bind.
        let grey = [0x7f, 0x7f, 0x7f, 0xff];
        let diffuse_texture = crate::Texture::new(
            device,
            queue,
            Some("forward-default-diffuse"),
            None,
            4,
            1,
            1,
            &grey,
        );
        let specular_texture = crate::Texture::new(
            device,
            queue,
            Some("forward-default-specular"),
            None,
            4,
            1,
            1,
            &grey,
        );
        let material = crate::forward::BlinnPhongMaterial {
            diffuse_texture,
            specular_texture,
            shininess: 16.0,
        };

        ForwardPipeline {
            _id: pipelines.ids.dequeue(),
            inner: Pipeline {
                pipeline: crate::linkage::pbr::create_pipeline(device, format, primitive),
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
}

/// Helper type to create a ForwardPipeline from a [`Renderer`].
#[derive(Edges)]
pub struct ForwardPipelineCreator {
    device: Read<Device>,
    queue: Read<Queue>,
    target: Read<RenderTarget>,
    pipelines: Write<Pipelines>,
}

impl ForwardPipelineCreator {
    /// Helper to create a ForwardPipeline by visiting a render graph.
    pub fn create(
        Self {
            device,
            queue,
            target,
            mut pipelines,
        }: Self,
    ) -> ForwardPipeline {
        ForwardPipeline::new(&device, &queue, target.format(), None, &mut pipelines)
    }

    /// Helper to create a ForwardPipeline from a render graph.
    pub fn create_with_prim(
        prim: wgpu::PrimitiveState
    ) -> impl FnOnce(Self) -> ForwardPipeline {
        move | Self {
            device,
            queue,
            target,
            mut pipelines,
        }: Self| {
            ForwardPipeline::new(&device, &queue, target.format(), Some(prim), &mut pipelines)
        }
    }
}
