//! Gltf support for the [`Stage`](crate::Stage).
use super::*;
use crate::{
    scene::TextureParams,
    shader::{array::Array, stage::TextureAddressMode},
    slab::Slabbed,
    SceneImage,
};
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum StageGltfError {}

#[repr(transparent)]
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfBuffer(Array<u32>);

/// A legend of Gltf data.
///
/// This tells where certain parts of the Gltf document are stored in the [`Stage`]'s slab.
#[derive(Default, Clone, Copy, Slabbed)]
pub struct GltfLegend {
    pub buffers: Array<GltfBuffer>,
}

impl Stage {
    pub fn load_gltf_document(
        &self,
        document: gltf::Document,
        buffers: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> GltfLegend {
        // Load the buffers into the GPU
        let gltf_buffers: Vec<GltfBuffer> = buffers
            .into_iter()
            .map(|buffer| {
                let slice: &[u32] = bytemuck::cast_slice(&buffer);
                GltfBuffer(self.append_slice(slice))
            })
            .collect::<Vec<_>>();
        let buffers = self.append_slice(&gltf_buffers);

        // Create the images in the atlas
        let images = images.into_iter().map(SceneImage::from).collect::<Vec<_>>();
        //let texture_params = document
        //    .textures()
        //    .map(|texture| {
        //        let index = texture.index();
        //        let name = texture.name().map(String::from);
        //        let image_index = texture.source().index();

        //        fn mode(mode: gltf::texture::WrappingMode) -> TextureAddressMode {
        //            match mode {
        //                gltf::texture::WrappingMode::ClampToEdge => {
        //                    TextureAddressMode::CLAMP_TO_EDGE
        //                }
        //                gltf::texture::WrappingMode::MirroredRepeat => {
        //                    TextureAddressMode::MIRRORED_REPEAT
        //                }
        //                gltf::texture::WrappingMode::Repeat => TextureAddressMode::REPEAT,
        //            }
        //        }
        //        let mode_s = mode(texture.sampler().wrap_s());
        //        let mode_t = mode(texture.sampler().wrap_t());
        //        let params = TextureParams {
        //            image_index,
        //            mode_s,
        //            mode_t,
        //        };

        //        let texture_id = builder.add_texture(params);
        //        log::trace!(
        //            "adding texture index:{index} name:{name:?} id:{texture_id:?} with wrapping \
        //     s:{mode_s} t:{mode_t}"
        //        );
        //        let _ = self.textures.insert(index, name, texture_id);
        //        Ok(texture_id)
        //    })
        //    .collect::<Vec<_>>();

        GltfLegend { buffers }
    }
}

#[cfg(test)]
mod test {
    //use glam::Vec3;

    //use crate::Renderling;

    //#[test]
    //fn normal_mapping_brick_sphere() {
    //    let size = 600;
    //    let mut r = Renderling::headless(size, size)
    //        .unwrap()
    //        .with_background_color(Vec3::splat(1.0).extend(1.0));
    //    let (device, queue) = r.get_device_and_queue_owned();

    //    let stage = Stage::new(device, queue, StageLegend::default());
    //    let (document, buffers, images) =
    //        gltf::import(path).map_err(|source| gltf_support::GltfLoaderError::Gltf {
    //            source,
    //            cwd: std::env::current_dir()
    //                .map(|p| format!("{}", p.display()))
    //                .unwrap_or("?".to_string()),
    //        })?;
    //    let loader = builder.gltf_load("../../gltf/red_brick_03_1k.glb").unwrap();
    //    let (projection, view) = loader.cameras.get(0).copied().unwrap();
    //    builder.set_camera(projection, view);

    //    let scene = builder.build().unwrap();
    //    r.setup_render_graph(RenderGraphConfig {
    //        scene: Some(scene),
    //        with_screen_capture: true,
    //        ..Default::default()
    //    });

    //    let img = r.render_image().unwrap();
    //    println!("saving frame");
    //    img_diff::assert_img_eq("gltf_normal_mapping_brick_sphere.png", img);
    //}
}
