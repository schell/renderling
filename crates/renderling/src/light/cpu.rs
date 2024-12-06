//! CPU-only lighting and shadows.

use std::sync::Arc;

use crate::slab::Hybrid;

use super::{DirectionalLight, Light, PointLight, SpotLight};

/// A wrapper around all types of analytical lighting.
#[derive(Clone, Debug)]
pub enum LightDetails {
    Directional(Hybrid<DirectionalLight>),
    Point(Hybrid<PointLight>),
    Spot(Hybrid<SpotLight>),
}

/// A depth map rendering of a scene from a light's point of view.
pub struct ShadowMap {
    /// A depth texture used to store the scene from the light's POV.
    texture: Arc<wgpu::Texture>,
    light: Hybrid<Light>,
    details: LightDetails,
}

impl ShadowMap {
    const LABEL: Option<&str> = Some("shadow-map");

    pub fn new(
        device: &wgpu::Device,
        size: wgpu::Extent3d,
        light: &Hybrid<Light>,
        details: &LightDetails,
    ) -> Self {
        ShadowMap {
            texture: device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Self::LABEL,
                    size,
                    mip_level_count: 0,
                    sample_count: 1,
                    // TODO: what about point lights? Does this need to be D3 then?
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Depth32Float,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .into(),
            light: light.clone(),
            details: details.clone(),
        }
    }

    pub fn size(&self) -> wgpu::Extent3d {
        self.texture.size()
    }
}

#[cfg(test)]
mod test {
    use img_diff::assert_img_eq;

    use crate::camera::Camera;

    use super::*;

    #[test]
    fn shadow_mapping_sanity() {
        let ctx = crate::Context::headless(1024, 800);
        let mut stage = ctx.new_stage().with_lighting(false);
        let camera = stage.new_value(Camera::default());
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_sanity.gltf"),
                camera.id(),
            )
            .unwrap();
        log::info!("cameras: {:#?}", doc.cameras);
        let gltf_camera = doc.cameras.first().unwrap();
        camera.set(gltf_camera.get_camera());
        let gltf_light = doc.lights.first().unwrap();
        stage.set_lights([gltf_light.light.id()]);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        frame.present();

        img_diff::save("shadows/shadow_mapping_sanity.png", img);
    }
}
