//! Forward pipeline and blinn-phong material definitions
use crate::{AnyMaterialUniform, MaterialUniform};

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
        let material_uniform = crate::linkage::pbr::MaterialUniform::new(
            device,
            &self.diffuse_texture.view,
            &self.diffuse_texture.sampler,
            &self.specular_texture.view,
            &self.specular_texture.sampler,
            self.shininess,
        );
        AnyMaterialUniform::new(material_uniform)
    }
}

pub struct ForwardPipeline {
    inner: wgpu::RenderPipeline,
}

impl crate::Pipeline for ForwardPipeline {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.inner
    }

    //fn begin_render_pass<'a, 'b: 'a>(
    //    &'b self,
    //    encoder: &'a mut wgpu::CommandEncoder,
    //    frame_texture_view: &'a wgpu::TextureView,
    //    depth_texture_view: &'a wgpu::TextureView,
    //) -> wgpu::RenderPass<'a> {
    //    renderling_core::begin_render_pass(
    //        encoder,
    //        Some("forward-pipeline"),
    //        &self.inner,
    //        frame_texture_view,
    //        depth_texture_view,
    //    )
    //}

    //fn render_object<'a, 'b: 'a>(
    //    &'b self,
    //    render_pass: &'a mut wgpu::RenderPass<'b>,
    //    object: renderling_core::ShaderObject<'b>,
    //    default_material_bindgroup: &'b wgpu::BindGroup,
    //) {
    //    renderling_core::render_object(render_pass, object, default_material_bindgroup)
    //}
}

impl ForwardPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        ForwardPipeline { inner: crate::linkage::pbr::create_pipeline(device, format) }
    }
}
