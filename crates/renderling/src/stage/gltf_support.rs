//! Gltf support for the [`Stage`](crate::Stage).
use super::*;
use crate::{
    shader::{
        gltf::*,
        pbr::PbrMaterial,
        stage::LightingModel,
        texture::{GpuTexture, TextureAddressMode, TextureModes},
    },
    SceneImage,
};
use glam::{Quat, Vec3, Vec4};
use snafu::{OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum StageGltfError {
    #[snafu(display("{source}"))]
    Atlas { source: crate::atlas::AtlasError },

    #[snafu(display("Missing image at index {index} atlas offset {offset}"))]
    MissingImage { offset: usize, index: usize },

    #[snafu(display("Wrong image at index {index} atlas offset {offset}"))]
    WrongImage { offset: usize, index: usize },

    #[snafu(display("Missing texture at gltf index {index} slab index {tex_id:?}"))]
    MissingTexture {
        index: usize,
        tex_id: Id<GpuTexture>,
    },

    #[snafu(display("Unsupported primitive mode: {:?}", mode))]
    PrimitiveMode { mode: gltf::mesh::Mode },

    #[snafu(display("No {} attribute for mesh", attribute.to_string()))]
    MissingAttribute { attribute: gltf::Semantic },

    #[snafu(display("No weights array"))]
    MissingWeights,

    #[snafu(display("Missing sampler"))]
    MissingSampler,

    #[snafu(display("{source}"))]
    Slab { source: crate::slab::SlabError },
}

impl From<crate::slab::SlabError> for StageGltfError {
    fn from(source: crate::slab::SlabError) -> Self {
        Self::Slab { source }
    }
}

impl Stage {
    pub fn load_gltf_document(
        &self,
        document: gltf::Document,
        buffer_data: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<GltfDocument, StageGltfError> {
        log::trace!("Loading buffers into the GPU");
        let buffers = self.allocate_array::<GltfBuffer>(buffer_data.len());
        for (i, buffer) in buffer_data.iter().enumerate() {
            log::trace!("  Loading buffer {i} size: {} bytes", buffer.len());
            let slice: &[u32] = bytemuck::cast_slice(&buffer);
            let buffer = self.append_array(slice);
            self.write(buffers.at(i), &GltfBuffer(buffer))?;
        }

        log::trace!("Loading views into the GPU");
        let views = self.allocate_array(document.views().len());
        for view in document.views() {
            let buffer = buffers.at(view.buffer().index());
            let offset = view.offset() as u32;
            let length = view.length() as u32;
            let stride = view.stride().unwrap_or_default() as u32;
            self.write(
                views.at(view.index()),
                &GltfBufferView {
                    buffer,
                    offset,
                    length,
                    stride,
                },
            )?;
        }

        log::trace!("Loading accessors into the GPU");
        let accessors = document
            .accessors()
            .map(|accessor| {
                let size = accessor.size() as u32;
                let buffer_view = accessor.view().unwrap();
                let view_buffer = buffer_view.buffer();
                let buffer_index = view_buffer.index();
                let buffer = buffers.at(buffer_index);
                let count = accessor.count() as u32;
                let view_offset = buffer_view.offset() as u32;
                let view_stride = buffer_view.stride().unwrap_or(0) as u32;
                let component_type = match accessor.data_type() {
                    gltf::accessor::DataType::I8 => DataType::I8,
                    gltf::accessor::DataType::U8 => DataType::U8,
                    gltf::accessor::DataType::I16 => DataType::I16,
                    gltf::accessor::DataType::U16 => DataType::U16,
                    gltf::accessor::DataType::U32 => DataType::U32,
                    gltf::accessor::DataType::F32 => DataType::F32,
                };
                let dimensions = match accessor.dimensions() {
                    gltf::accessor::Dimensions::Scalar => Dimensions::Scalar,
                    gltf::accessor::Dimensions::Vec2 => Dimensions::Vec2,
                    gltf::accessor::Dimensions::Vec3 => Dimensions::Vec3,
                    gltf::accessor::Dimensions::Vec4 => Dimensions::Vec4,
                    gltf::accessor::Dimensions::Mat2 => Dimensions::Mat2,
                    gltf::accessor::Dimensions::Mat3 => Dimensions::Mat3,
                    gltf::accessor::Dimensions::Mat4 => Dimensions::Mat4,
                };
                let normalized = accessor.normalized();
                let accessor = GltfAccessor {
                    size,
                    count,
                    buffer,
                    view_offset,
                    view_stride,
                    component_type,
                    dimensions,
                    normalized,
                };
                accessor
            })
            .collect::<Vec<_>>();
        let accessors = self.append_array(&accessors);

        log::trace!("Loading cameras into the GPU");
        let cameras = document
            .cameras()
            .map(|camera| match camera.projection() {
                gltf::camera::Projection::Perspective(perspective) => {
                    let aspect_ratio = perspective.aspect_ratio().unwrap_or(1.0);
                    let yfov = perspective.yfov();
                    let zfar = perspective.zfar().unwrap_or(f32::MAX);
                    let znear = perspective.znear();
                    let camera = GltfCamera::Perspective {
                        aspect_ratio,
                        yfov,
                        zfar,
                        znear,
                    };
                    camera
                }
                gltf::camera::Projection::Orthographic(orthographic) => {
                    let xmag = orthographic.xmag();
                    let ymag = orthographic.ymag();
                    let zfar = orthographic.zfar();
                    let znear = orthographic.znear();
                    let camera = GltfCamera::Orthographic {
                        xmag,
                        ymag,
                        zfar,
                        znear,
                    };
                    camera
                }
            })
            .collect::<Vec<_>>();
        let cameras = self.append_array(&cameras);

        // We need the (re)packing of the atlas before we marshal the images into the GPU
        // because we need their frames for textures and materials, but we need to know
        // if the materials are require us to apply a linear transfer. So we'll get the
        // preview repacking first, then update the frames in the textures.
        let (mut repacking, atlas_offset) = {
            // UNWRAP: if we can't lock the atlas, we want to panic.
            let atlas = self.atlas.read().unwrap();
            let atlas_offset = atlas.rects.len();
            (
                atlas
                    .repack_preview(&self.device, images.into_iter().map(SceneImage::from))
                    .context(AtlasSnafu)?,
                atlas_offset,
            )
        };

        log::trace!("Creating GPU textures");
        let mut gpu_textures = vec![];
        for texture in document.textures() {
            let image_index = texture.source().index();

            fn mode(mode: gltf::texture::WrappingMode) -> TextureAddressMode {
                match mode {
                    gltf::texture::WrappingMode::ClampToEdge => TextureAddressMode::CLAMP_TO_EDGE,
                    gltf::texture::WrappingMode::MirroredRepeat => {
                        TextureAddressMode::MIRRORED_REPEAT
                    }
                    gltf::texture::WrappingMode::Repeat => TextureAddressMode::REPEAT,
                }
            }

            let mode_s = mode(texture.sampler().wrap_s());
            let mode_t = mode(texture.sampler().wrap_t());
            let (offset_px, size_px) =
                repacking
                    .get_frame(image_index + atlas_offset)
                    .context(MissingImageSnafu {
                        index: image_index,
                        offset: atlas_offset,
                    })?;
            gpu_textures.push(GpuTexture {
                offset_px,
                size_px,
                modes: TextureModes::default()
                    .with_wrap_s(mode_s)
                    .with_wrap_t(mode_t),
                atlas_index: (image_index + atlas_offset) as u32,
            });
        }
        let gpu_textures = gpu_textures;
        let textures = self.append_array(&gpu_textures);

        log::trace!("Creating materials");
        let mut gpu_materials = vec![];
        for material in document.materials() {
            let index = material.index();
            let name = material.name().map(String::from);
            log::trace!("loading material {index:?} {name:?}");
            let pbr = material.pbr_metallic_roughness();
            let material = if material.unlit() {
                log::trace!("  is unlit");
                let (albedo_texture, albedo_tex_coord) =
                    if let Some(info) = pbr.base_color_texture() {
                        let texture = info.texture();
                        let index = texture.index();
                        let tex_id = textures.at(index);
                        // The index of the image in the original gltf document
                        let image_index = texture.source().index();
                        let image = repacking
                            .get_mut(atlas_offset + image_index)
                            .context(MissingImageSnafu {
                                index: image_index,
                                offset: atlas_offset,
                            })?
                            .as_scene_img_mut()
                            .context(WrongImageSnafu {
                                index: image_index,
                                offset: atlas_offset,
                            })?;
                        image.apply_linear_transfer = true;
                        (tex_id, info.tex_coord())
                    } else {
                        (Id::NONE, 0)
                    };

                PbrMaterial {
                    albedo_texture,
                    albedo_tex_coord,
                    albedo_factor: pbr.base_color_factor().into(),
                    ..Default::default()
                }
            } else {
                log::trace!("  is pbr");
                let albedo_factor: Vec4 = pbr.base_color_factor().into();
                let (albedo_texture, albedo_tex_coord) =
                    if let Some(info) = pbr.base_color_texture() {
                        let texture = info.texture();
                        let index = texture.index();
                        let tex_id = textures.at(index);
                        let image_index = texture.source().index();
                        let image = repacking
                            .get_mut(image_index + atlas_offset)
                            .context(MissingImageSnafu {
                                index: image_index,
                                offset: atlas_offset,
                            })?
                            .as_scene_img_mut()
                            .context(WrongImageSnafu {
                                index: image_index,
                                offset: atlas_offset,
                            })?;
                        image.apply_linear_transfer = true;
                        (tex_id, info.tex_coord())
                    } else {
                        (Id::NONE, 0)
                    };

                let (
                    metallic_factor,
                    roughness_factor,
                    metallic_roughness_texture,
                    metallic_roughness_tex_coord,
                ) = if let Some(info) = pbr.metallic_roughness_texture() {
                    let index = info.texture().index();
                    let tex_id = textures.at(index);
                    (1.0, 1.0, tex_id, info.tex_coord())
                } else {
                    (pbr.metallic_factor(), pbr.roughness_factor(), Id::NONE, 0)
                };

                let (normal_texture, normal_tex_coord) =
                    if let Some(norm_tex) = material.normal_texture() {
                        let tex_id = textures.at(norm_tex.texture().index());
                        (tex_id, norm_tex.tex_coord())
                    } else {
                        (Id::NONE, 0)
                    };

                let (ao_strength, ao_texture, ao_tex_coord) =
                    if let Some(occlusion_tex) = material.occlusion_texture() {
                        let tex_id = textures.at(occlusion_tex.texture().index());
                        (occlusion_tex.strength(), tex_id, occlusion_tex.tex_coord())
                    } else {
                        (0.0, Id::NONE, 0)
                    };

                let (emissive_texture, emissive_tex_coord) =
                    if let Some(emissive_tex) = material.emissive_texture() {
                        let texture = emissive_tex.texture();
                        let index = texture.index();
                        let tex_id = textures.at(index);
                        let image_index = texture.source().index();
                        let image = repacking
                            .get_mut(image_index + atlas_offset)
                            .context(MissingImageSnafu {
                                index: image_index,
                                offset: atlas_offset,
                            })?
                            .as_scene_img_mut()
                            .context(WrongImageSnafu {
                                index: image_index,
                                offset: atlas_offset,
                            })?;
                        image.apply_linear_transfer = true;
                        (tex_id, emissive_tex.tex_coord())
                    } else {
                        (Id::NONE, 0)
                    };
                let emissive_factor = Vec3::from(material.emissive_factor())
                    .extend(material.emissive_strength().unwrap_or(1.0));

                PbrMaterial {
                    albedo_factor,
                    metallic_factor,
                    roughness_factor,
                    albedo_texture,
                    metallic_roughness_texture,
                    normal_texture,
                    ao_texture,
                    albedo_tex_coord,
                    metallic_roughness_tex_coord,
                    normal_tex_coord,
                    ao_tex_coord,
                    ao_strength,
                    emissive_factor,
                    emissive_texture,
                    emissive_tex_coord,
                    lighting_model: LightingModel::PBR_LIGHTING,
                    ..Default::default()
                }
            };
            gpu_materials.push(material);
        }
        let gpu_materials = gpu_materials;
        let materials = self.append_array(&gpu_materials);

        log::trace!("Packing the atlas");
        {
            // UNWRAP: if we can't lock the atlas, we want to panic.
            let mut atlas = self.atlas.write().unwrap();
            let new_atlas = atlas
                .commit_repack_preview(&self.device, &self.queue, repacking)
                .context(AtlasSnafu)?;
            *atlas = new_atlas;
        }

        log::trace!("Loading meshes");
        let meshes = self.allocate_array::<GltfMesh>(document.meshes().len());
        for mesh in document.meshes() {
            let primitives = self.allocate_array::<GltfPrimitive>(mesh.primitives().len());
            for (j, primitive) in mesh.primitives().enumerate() {
                debug_assert_eq!(j, primitive.index());
                let material = primitive
                    .material()
                    .index()
                    .map(|i| materials.at(i))
                    .unwrap_or(Id::NONE);
                let indices = primitive
                    .indices()
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let positions = primitive
                    .get(&gltf::Semantic::Positions)
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let normals = primitive
                    .get(&gltf::Semantic::Normals)
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let tangents = primitive
                    .get(&gltf::Semantic::Tangents)
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let colors = primitive
                    .get(&gltf::Semantic::Colors(0))
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let tex_coords0 = primitive
                    .get(&gltf::Semantic::TexCoords(0))
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let tex_coords1 = primitive
                    .get(&gltf::Semantic::TexCoords(1))
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let joints = primitive
                    .get(&gltf::Semantic::Joints(0))
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();
                let weights = primitive
                    .get(&gltf::Semantic::Weights(0))
                    .map(|acc| accessors.at(acc.index()))
                    .unwrap_or_default();

                self.write(
                    primitives.at(primitive.index()),
                    &GltfPrimitive {
                        material,
                        indices,
                        positions,
                        normals,
                        tangents,
                        colors,
                        tex_coords0,
                        tex_coords1,
                        joints,
                        weights,
                    },
                )?;
            }
            let weights = mesh.weights().unwrap_or(&[]);
            let weights = self.append_array(weights);
            self.write(
                meshes.at(mesh.index()),
                &GltfMesh {
                    primitives,
                    weights,
                },
            )?;
        }
        log::trace!("Loading lights");
        let lights_array = self.allocate_array::<GltfLight>(
            document
                .lights()
                .map(|lights| lights.len())
                .unwrap_or_default(),
        );
        if let Some(lights) = document.lights() {
            for light in lights {
                let light_index = light.index();
                let color = Vec3::from(light.color());
                let range = light.range().unwrap_or(f32::MAX);
                let intensity = light.intensity();
                let kind = match light.kind() {
                    gltf::khr_lights_punctual::Kind::Directional => GltfLightKind::Directional,
                    gltf::khr_lights_punctual::Kind::Point => GltfLightKind::Point,
                    gltf::khr_lights_punctual::Kind::Spot {
                        inner_cone_angle,
                        outer_cone_angle,
                    } => GltfLightKind::Spot {
                        inner_cone_angle,
                        outer_cone_angle,
                    },
                };
                self.write(
                    lights_array.at(light_index),
                    &GltfLight {
                        color,
                        range,
                        intensity,
                        kind,
                    },
                )?;
            }
        }
        let lights = lights_array;

        // Preallocate nodes and skins so we can reference their ids before
        // we write them to the slab.
        let nodes = self.allocate_array::<GltfNode>(document.nodes().len());
        let skins = self.allocate_array::<GltfSkin>(document.skins().len());

        log::trace!("Loading nodes");
        for (i, node) in document.nodes().enumerate() {
            let node_index = node.index();
            debug_assert_eq!(i, node_index);
            let children = node
                .children()
                .map(|node| nodes.at(node.index()))
                .collect::<Vec<_>>();
            let children = self.append_array(&children);
            let (translation, rotation, scale) = node.transform().decomposed();
            let translation = Vec3::from(translation);
            let rotation = Quat::from_array(rotation);
            let scale = Vec3::from(scale);
            let mesh = node
                .mesh()
                .map(|mesh| meshes.at(mesh.index()))
                .unwrap_or_default();
            let skin = node
                .skin()
                .map(|skin| skins.at(skin.index()))
                .unwrap_or_default();
            let camera = node
                .camera()
                .map(|camera| cameras.at(camera.index()))
                .unwrap_or_default();
            let light = node
                .light()
                .map(|light| lights.at(light.index()))
                .unwrap_or_default();
            let weights = if let Some(weights) = node.weights() {
                self.append_array(weights)
            } else {
                Array::default()
            };
            self.write(
                nodes.at(node_index),
                &GltfNode {
                    children,
                    translation,
                    rotation,
                    scale,
                    mesh,
                    camera,
                    weights,
                    light,
                    skin,
                },
            )?;
        }

        log::trace!("Loading skins");
        for skin in document.skins() {
            let skin_index = skin.index();
            let joints = skin
                .joints()
                .map(|node| nodes.at(node.index()))
                .collect::<Vec<_>>();
            let joints = self.append_array(&joints);
            let inverse_bind_matrices = skin
                .inverse_bind_matrices()
                .map(|acc| accessors.at(acc.index()))
                .unwrap_or_default();
            let skeleton = skin
                .skeleton()
                .map(|node| nodes.at(node.index()))
                .unwrap_or_default();
            let _ = self.write(
                skins.at(skin_index),
                &GltfSkin {
                    joints,
                    inverse_bind_matrices,
                    skeleton,
                },
            );
        }

        log::trace!("Loading animations");
        let animations = self.allocate_array::<GltfAnimation>(document.animations().count());
        for animation in document.animations() {
            let samplers =
                self.allocate_array::<GltfAnimationSampler>(animation.samplers().count());
            fn create_sampler(
                accessors: Array<GltfAccessor>,
                sampler: gltf::animation::Sampler<'_>,
            ) -> GltfAnimationSampler {
                let interpolation = match sampler.interpolation() {
                    gltf::animation::Interpolation::Linear => GltfInterpolation::Linear,
                    gltf::animation::Interpolation::Step => GltfInterpolation::Step,
                    gltf::animation::Interpolation::CubicSpline => GltfInterpolation::CubicSpline,
                };
                let input = accessors.at(sampler.input().index());
                let output = accessors.at(sampler.output().index());
                GltfAnimationSampler {
                    interpolation,
                    input,
                    output,
                }
            }
            let mut stored_samplers = vec![];
            for (i, sampler) in animation.samplers().enumerate() {
                let sampler = create_sampler(accessors, sampler);
                self.write(samplers.at(i), &sampler)?;
                // Store it later so we can figure out the index of the sampler
                // used by the channel.
                stored_samplers.push(sampler);
            }
            let channels = self.allocate_array::<GltfChannel>(animation.channels().count());
            for (i, channel) in animation.channels().enumerate() {
                let target = channel.target();
                let node = nodes.at(target.node().index());
                let property = match target.property() {
                    gltf::animation::Property::Translation => GltfTargetProperty::Translation,
                    gltf::animation::Property::Rotation => GltfTargetProperty::Rotation,
                    gltf::animation::Property::Scale => GltfTargetProperty::Scale,
                    gltf::animation::Property::MorphTargetWeights => {
                        GltfTargetProperty::MorphTargetWeights
                    }
                };
                let target = GltfTarget { node, property };
                let sampler = create_sampler(accessors, channel.sampler());
                let index = stored_samplers
                    .iter()
                    .position(|s| s == &sampler)
                    .context(MissingSamplerSnafu)?;
                let sampler = samplers.at(index);
                self.write(channels.at(i), &GltfChannel { target, sampler })?;
            }
            self.write(
                animations.at(animation.index()),
                &GltfAnimation { channels, samplers },
            )?;
        }

        log::trace!("Loading scenes");
        let scenes = self.allocate_array::<GltfScene>(document.scenes().len());
        for scene in document.scenes() {
            let nodes = scene
                .nodes()
                .map(|node| nodes.at(node.index()))
                .collect::<Vec<_>>();
            let nodes = self.append_array(&nodes);
            self.write(scenes.at(scene.index()), &GltfScene { nodes })?;
        }

        log::trace!("Done loading gltf");

        Ok(GltfDocument {
            accessors,
            animations,
            buffers,
            cameras,
            materials,
            meshes,
            nodes,
            scenes,
            skins,
            textures,
            views,
        })
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;

    use crate::{Renderling, Stage};

    #[test]
    fn stage_normal_mapping_brick_sphere() {
        crate::init_logging();
        let size = 600;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(1.0).extend(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let stage = Stage::new(device, queue);
        stage.configure_graph(&mut r, true);

        log::trace!("Reading gltf");
        let (document, buffers, images) = gltf::import("../../gltf/red_brick_03_1k.glb").unwrap();
        log::trace!("Loading gltf");
        let _doc = stage.load_gltf_document(document, buffers, images).unwrap();
    }
}
