//! Gltf support for the [`Stage`](crate::Stage).
use crabslab::{Array, GrowableSlab, Slab};
use glam::{Quat, Vec2, Vec3, Vec4};
use snafu::{OptionExt, ResultExt, Snafu};

use super::*;
use crate::{
    gltf::*,
    pbr::{light::LightStyle, Material, PbrConfig},
    stage::Vertex,
    AtlasImage, AtlasTexture, Camera, TextureAddressMode, TextureModes,
};

#[derive(Debug, Snafu)]
pub enum StageGltfError {
    #[snafu(display("{source}"))]
    Gltf { source: gltf::Error },

    #[snafu(display("{source}"))]
    Atlas { source: crate::atlas::AtlasError },

    #[snafu(display("Missing image at index {index} atlas offset {offset}"))]
    MissingImage { offset: usize, index: usize },

    #[snafu(display("Wrong image at index {index} atlas offset {offset}"))]
    WrongImage { offset: usize, index: usize },

    #[snafu(display("Missing texture at gltf index {index} slab index {tex_id:?}"))]
    MissingTexture {
        index: usize,
        tex_id: Id<AtlasTexture>,
    },

    #[snafu(display("Unsupported primitive mode: {:?}", mode))]
    PrimitiveMode { mode: gltf::mesh::Mode },

    #[snafu(display("No {} attribute for mesh", attribute.to_string()))]
    MissingAttribute { attribute: gltf::Semantic },

    #[snafu(display("No weights array"))]
    MissingWeights,

    #[snafu(display("Missing sampler"))]
    MissingSampler,

    #[snafu(display("Missing gltf camera at index {index}"))]
    MissingCamera { index: usize },

    #[snafu(display("{source}"))]
    Slab { source: crabslab::WgpuSlabError },
}

impl From<crabslab::WgpuSlabError> for StageGltfError {
    fn from(source: crabslab::WgpuSlabError) -> Self {
        Self::Slab { source }
    }
}

impl From<gltf::Error> for StageGltfError {
    fn from(source: gltf::Error) -> Self {
        Self::Gltf { source }
    }
}

pub fn from_gltf_light_kind(kind: gltf::khr_lights_punctual::Kind) -> LightStyle {
    match kind {
        gltf::khr_lights_punctual::Kind::Directional => LightStyle::Directional,
        gltf::khr_lights_punctual::Kind::Point => LightStyle::Point,
        gltf::khr_lights_punctual::Kind::Spot { .. } => LightStyle::Spot,
    }
}

pub fn gltf_light_intensity_units(kind: gltf::khr_lights_punctual::Kind) -> &'static str {
    match kind {
        gltf::khr_lights_punctual::Kind::Directional => "lux (lm/m^2)",
        // sr is "steradian"
        _ => "candelas (lm/sr)",
    }
}

fn texture_address_mode_from_gltf(mode: gltf::texture::WrappingMode) -> TextureAddressMode {
    match mode {
        gltf::texture::WrappingMode::ClampToEdge => TextureAddressMode::ClampToEdge,
        gltf::texture::WrappingMode::MirroredRepeat => TextureAddressMode::MirroredRepeat,
        gltf::texture::WrappingMode::Repeat => TextureAddressMode::Repeat,
    }
}

pub fn get_vertex_count(primitive: &gltf::Primitive<'_>) -> u32 {
    if let Some(indices) = primitive.indices() {
        let count = indices.count() as u32;
        log::trace!("    has {count} indices");
        count
    } else {
        if let Some(positions) = primitive.get(&gltf::Semantic::Positions) {
            let count = positions.count() as u32;
            log::trace!("    has {count} positions");
            count
        } else {
            log::trace!("    has no indices nor positions");
            0
        }
    }
}

pub fn make_accessor(accessor: gltf::Accessor<'_>, views: &Array<GltfBufferView>) -> GltfAccessor {
    let size = accessor.size() as u32;
    let buffer_view = accessor.view().unwrap();
    let offset = accessor.offset() as u32;
    let count = accessor.count() as u32;
    let view = views.at(buffer_view.index());
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
    GltfAccessor {
        size,
        view,
        count,
        offset,
        data_type: component_type,
        dimensions,
        normalized,
    }
}

impl Stage {
    pub fn load_gltf_document_from_path(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(gltf::Document, GltfDocument), StageGltfError> {
        let (document, buffers, images) = gltf::import(path)?;
        let gpu_doc = self.load_gltf_document(&document, buffers, images)?;
        Ok((document, gpu_doc))
    }

    pub fn load_gltf_document(
        &mut self,
        document: &gltf::Document,
        buffer_data: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<GltfDocument, StageGltfError> {
        log::trace!("Loading buffers into the GPU");
        let buffers = self.allocate_array::<GltfBuffer>(buffer_data.len());
        for (i, buffer) in buffer_data.iter().enumerate() {
            let slice: &[u32] = bytemuck::cast_slice(&buffer);
            let buffer = self.append_array(slice);
            self.write(buffers.at(i), &GltfBuffer(buffer));
        }

        log::trace!("Loading views into the GPU");
        let views = self.allocate_array(document.views().len());
        for view in document.views() {
            let buffer = buffers.at(view.buffer().index());
            let offset = view.offset() as u32;
            let length = view.length() as u32;
            let stride = view.stride().unwrap_or_default() as u32;
            let id = views.at(view.index());
            let gltf_view = GltfBufferView {
                buffer,
                offset,
                length,
                stride,
            };
            self.write(id, &gltf_view);
        }

        log::trace!("Loading accessors into the GPU");
        let accessors = document
            .accessors()
            .enumerate()
            .map(|(i, accessor)| {
                let a = make_accessor(accessor, &views);
                log::trace!("  accessor {i}: {a:#?}",);
                a
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

        // We need the (re)packing of the atlas before we marshal the images into the
        // GPU because we need their frames for textures and materials, but we
        // need to know if the materials require us to apply a linear transfer.
        // So we'll get the preview repacking first, then update the frames in
        // the textures.
        let (mut repacking, atlas_offset) = {
            // UNWRAP: if we can't lock the atlas, we want to panic.
            let atlas = self.atlas.read().unwrap();
            let atlas_offset = atlas.rects.len();
            (
                atlas
                    .repack_preview(&self.device, images.into_iter().map(AtlasImage::from))
                    .context(AtlasSnafu)?,
                atlas_offset,
            )
        };

        log::trace!("Creating GPU textures");
        let textures = self.allocate_array::<AtlasTexture>(document.textures().len());
        for (i, texture) in document.textures().enumerate() {
            let image_index = texture.source().index();
            let mode_s = texture_address_mode_from_gltf(texture.sampler().wrap_s());
            let mode_t = texture_address_mode_from_gltf(texture.sampler().wrap_t());
            let (offset_px, size_px) =
                repacking
                    .get_frame(image_index + atlas_offset)
                    .context(MissingImageSnafu {
                        index: image_index,
                        offset: atlas_offset,
                    })?;
            let texture = AtlasTexture {
                offset_px,
                size_px,
                modes: TextureModes {
                    s: mode_s,
                    t: mode_t,
                },
                atlas_index: (image_index + atlas_offset) as u32,
            };
            let texture_id = textures.at(i);
            log::trace!("  texture {i} {texture_id:?}: {texture:#?}");
            self.write(texture_id, &texture);
        }

        log::trace!("Creating materials");
        let mut default_material = Id::<Material>::NONE;
        let materials = self.allocate_array::<Material>(document.materials().len());
        for material in document.materials() {
            let material_id = if let Some(index) = material.index() {
                materials.at(index)
            } else {
                // Allocate some extra space for this default material
                default_material = self.allocate::<Material>();
                default_material
            };
            let name = material.name().map(String::from);
            log::trace!("loading material {:?} {name:?}", material.index());
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
                        // Update the image to ensure it gets transferred correctly
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

                Material {
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
                        // Update the image to ensure it gets transferred correctly
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
                        // Update the image to ensure it gets transferred correctly
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
                let emissive_factor = Vec3::from(material.emissive_factor());
                let emissive_strength_multiplier = material.emissive_strength().unwrap_or(1.0);

                Material {
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
                    emissive_strength_multiplier,
                    emissive_texture,
                    emissive_tex_coord,
                    has_lighting: true,
                    ..Default::default()
                }
            };
            log::trace!("  material {material_id:?}: {material:#?}",);
            self.write(material_id, &material);
        }

        let number_of_new_images = repacking.new_images_len();
        if number_of_new_images > 0 {
            log::trace!("Packing the atlas");
            log::trace!("  adding {number_of_new_images} new images",);
            // UNWRAP: if we can't lock the atlas, we want to panic.
            let mut atlas = self.atlas.write().unwrap();
            let new_atlas = atlas
                .commit_repack_preview(&self.device, &self.queue, repacking)
                .context(AtlasSnafu)?;
            let size = new_atlas.size;
            *atlas = new_atlas;
            // The bindgroup will have to be remade
            let _ = self.textures_bindgroup.lock().unwrap().take();
            // The atlas size must be reset
            let size_id = PbrConfig::offset_of_atlas_size().into();
            self.slab.write().unwrap().write(size_id, &size);
        }

        fn log_accessor(gltf_accessor: gltf::Accessor<'_>) {
            log::trace!("      count: {}", gltf_accessor.count());
            log::trace!("      size: {}", gltf_accessor.size());
            log::trace!("      data_type: {:?}", gltf_accessor.data_type());
            log::trace!("     dimensions: {:?}", gltf_accessor.dimensions());
        }
        log::trace!("Loading meshes");
        let meshes = self.allocate_array::<GltfMesh>(document.meshes().len());
        log::trace!("  reserved array: {meshes:#?}");
        for mesh in document.meshes() {
            let primitives = self.allocate_array::<GltfPrimitive>(mesh.primitives().len());
            log::trace!("    reserved array: {primitives:#?}");
            for (j, primitive) in mesh.primitives().enumerate() {
                log::trace!("  primitive {j}");
                debug_assert_eq!(j, primitive.index());
                let vertex_count = get_vertex_count(&primitive);
                let material = primitive
                    .material()
                    .index()
                    .map(|i| materials.at(i))
                    .unwrap_or(Id::NONE);
                let indices = primitive
                    .indices()
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    indices:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();
                let positions = primitive
                    .get(&gltf::Semantic::Positions)
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    positions:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();

                let mut indices_vec: Option<Vec<u32>> = None;
                fn get_indices<'a>(
                    buffer_data: &[gltf::buffer::Data],
                    primitive: &gltf::Primitive<'_>,
                    indicies_vec: &'a mut Option<Vec<u32>>,
                ) -> &'a Vec<u32> {
                    if indicies_vec.is_none() {
                        let reader = primitive.reader(|buffer| {
                            let data = buffer_data.get(buffer.index())?;
                            Some(data.0.as_slice())
                        });
                        let indices = reader
                            .read_indices()
                            .map(|is| is.into_u32().collect::<Vec<_>>())
                            .unwrap_or_else(|| {
                                let count = primitive
                                    .get(&gltf::Semantic::Positions)
                                    .map(|ps| ps.count())
                                    .unwrap_or_default()
                                    as u32;
                                (0u32..count).collect::<Vec<_>>()
                            });
                        assert_eq!(indices.len() % 3, 0, "indices do not form triangles");
                        *indicies_vec = Some(indices);
                    }
                    // UNWRAP: safe because we just set it to `Some` if previously `None`
                    indicies_vec.as_ref().unwrap()
                }

                // We may need the positions and uvs in-memory if we need
                // to generate normals or tangents, so we'll keep them in
                // a vec, if necessary, and access them through a function.
                let mut position_vec: Option<Vec<Vec3>> = None;
                fn get_positions<'a>(
                    buffer_data: &[gltf::buffer::Data],
                    primitive: &gltf::Primitive<'_>,
                    indices_vec: &'a mut Option<Vec<u32>>,
                    position_vec: &'a mut Option<Vec<Vec3>>,
                ) -> &'a Vec<Vec3> {
                    if position_vec.is_none() {
                        let reader = primitive.reader(|buffer| {
                            let data = buffer_data.get(buffer.index())?;
                            Some(data.0.as_slice())
                        });
                        let indices = get_indices(buffer_data, primitive, indices_vec);
                        let mut positions = reader
                            .read_positions()
                            .map(|ps| ps.map(Vec3::from).collect::<Vec<_>>())
                            .unwrap_or_else(|| vec![Vec3::ZERO; indices.len()]);
                        if positions.len() != indices.len() {
                            let mut new_positions = Vec::with_capacity(indices.len());
                            for index in indices {
                                new_positions.push(positions[*index as usize]);
                            }
                            positions = new_positions;
                        }
                        assert_eq!(
                            positions.len() % 3,
                            0,
                            "{} positions do not form triangles",
                            positions.len()
                        );
                        *position_vec = Some(positions);
                    }
                    // UNWRAP: safe because we just set it to `Some` if previously `None`
                    position_vec.as_ref().unwrap()
                }

                let mut uv_vec: Option<Vec<Vec2>> = None;
                fn get_uvs<'a>(
                    buffer_data: &[gltf::buffer::Data],
                    primitive: &gltf::Primitive<'_>,
                    indices: &'a mut Option<Vec<u32>>,
                    uv_vec: &'a mut Option<Vec<Vec2>>,
                ) -> &'a Vec<Vec2> {
                    // ensures we have position
                    if uv_vec.is_none() {
                        let reader = primitive.reader(|buffer| {
                            let data = buffer_data.get(buffer.index())?;
                            Some(data.0.as_slice())
                        });
                        let indices = get_indices(buffer_data, primitive, indices);
                        let mut uvs: Vec<Vec2> = reader
                            .read_tex_coords(0)
                            .map(|coords| coords.into_f32().map(Vec2::from).collect::<Vec<_>>())
                            .unwrap_or_else(|| vec![Vec2::ZERO; indices.len()]);
                        if uvs.len() != indices.len() {
                            let mut new_uvs = Vec::with_capacity(indices.len());
                            for index in indices {
                                new_uvs.push(uvs[*index as usize]);
                            }
                            uvs = new_uvs;
                        }
                        *uv_vec = Some(uvs);
                    }
                    // UNWRAP: safe because we just set it to `Some`
                    uv_vec.as_ref().unwrap()
                }

                let mut normals_were_generated = false;
                let normals = primitive
                    .get(&gltf::Semantic::Normals)
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    normals:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_else(|| {
                        log::trace!("    generating normals");
                        // Generate the normals
                        normals_were_generated = true;
                        let normals = get_positions(
                            &buffer_data,
                            &primitive,
                            &mut indices_vec,
                            &mut position_vec,
                        )
                        .chunks(3)
                        .flat_map(|chunk| match chunk {
                            [a, b, c] => {
                                let n = Vertex::generate_normal(*a, *b, *c);
                                [n, n, n]
                            }
                            _ => panic!("not triangles!"),
                        })
                        .collect::<Vec<_>>();
                        let normals_array = self.append_array(&normals);
                        let buffer = GltfBuffer(normals_array.into_u32_array());
                        let buffer = self.append(&buffer);
                        let view = self.append(&GltfBufferView {
                            buffer,
                            offset: 0,
                            length: normals.len() as u32 * 3 * 4, // 3 components * 4 bytes each
                            stride: 12,
                        });
                        let accessor = GltfAccessor {
                            size: 12,
                            view,
                            offset: 0,
                            count: normals.len() as u32,
                            data_type: DataType::F32,
                            dimensions: Dimensions::Vec3,
                            normalized: true,
                        };
                        self.append(&accessor)
                    });
                let mut tangents_were_generated = false;
                let tangents = primitive
                    .get(&gltf::Semantic::Tangents)
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    tangents:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_else(|| {
                        log::trace!("    generating tangents");
                        tangents_were_generated = true;
                        let positions = get_positions(
                            &buffer_data,
                            &primitive,
                            &mut indices_vec,
                            &mut position_vec,
                        )
                        .clone();
                        let uvs = get_uvs(&buffer_data, &primitive, &mut indices_vec, &mut uv_vec)
                            .clone();
                        let p_uvs = positions
                            .into_iter()
                            .zip(uvs.into_iter().chain(std::iter::repeat(Vec2::ZERO).cycle()))
                            .collect::<Vec<_>>();
                        let tangents = p_uvs
                            .chunks(3)
                            .flat_map(|chunk| match chunk {
                                [(a, a_uv), (b, b_uv), (c, c_uv)] => {
                                    let t =
                                        Vertex::generate_tangent(*a, *a_uv, *b, *b_uv, *c, *c_uv);
                                    [t, t, t]
                                }
                                _ => panic!("not triangles!"),
                            })
                            .collect::<Vec<_>>();
                        let tangents_array = self.append_array(&tangents);
                        let buffer = GltfBuffer(tangents_array.into_u32_array());
                        let buffer = self.append(&buffer);
                        let view = self.append(&GltfBufferView {
                            buffer,
                            offset: 0,
                            length: tangents.len() as u32 * 4 * 4, // 4 components * 4 bytes each
                            stride: 16,
                        });
                        let accessor = GltfAccessor {
                            size: 16,
                            view,
                            offset: 0,
                            count: tangents.len() as u32,
                            data_type: DataType::F32,
                            dimensions: Dimensions::Vec4,
                            normalized: true,
                        };
                        self.append(&accessor)
                    });
                let colors = primitive
                    .get(&gltf::Semantic::Colors(0))
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    colors:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();
                let tex_coords0 = primitive
                    .get(&gltf::Semantic::TexCoords(0))
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    tex_coords0:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();
                let tex_coords1 = primitive
                    .get(&gltf::Semantic::TexCoords(1))
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    tex_coords1:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();
                let joints = primitive
                    .get(&gltf::Semantic::Joints(0))
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    joints:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();
                let weights = primitive
                    .get(&gltf::Semantic::Weights(0))
                    .map(|acc| {
                        let gltf_accessor = document.accessors().nth(acc.index()).unwrap();
                        log::trace!("    weights:");
                        log_accessor(gltf_accessor);
                        accessors.at(acc.index())
                    })
                    .unwrap_or_default();

                let id = primitives.at(primitive.index());
                let prim = GltfPrimitive {
                    vertex_count: vertex_count as u32,
                    material,
                    indices,
                    positions,
                    normals,
                    normals_were_generated,
                    tangents,
                    tangents_were_generated,
                    colors,
                    tex_coords0,
                    tex_coords1,
                    joints,
                    weights,
                };
                log::trace!("    writing primitive {id:?}:\n{prim:#?}");
                self.write(id, &prim);
            }
            let weights = mesh.weights().unwrap_or(&[]);
            let weights = self.append_array(weights);
            self.write(
                meshes.at(mesh.index()),
                &GltfMesh {
                    primitives,
                    weights,
                },
            );
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
                );
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
            );
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
                self.write(samplers.at(i), &sampler);
                // Store it later so we can figure out the index of the sampler
                // used by the channel.
                //
                // TODO: Remove `stored_samplers` once `gltf` provides `.index()`
                // @see https://github.com/gltf-rs/gltf/issues/398
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
                self.write(channels.at(i), &GltfChannel { target, sampler });
            }
            self.write(
                animations.at(animation.index()),
                &GltfAnimation { channels, samplers },
            );
        }

        log::trace!("Loading scenes");
        let scenes = self.allocate_array::<GltfScene>(document.scenes().len());
        for scene in document.scenes() {
            let nodes = scene
                .nodes()
                .map(|node| nodes.at(node.index()))
                .collect::<Vec<_>>();
            let nodes = self.append_array(&nodes);
            self.write(scenes.at(scene.index()), &GltfScene { nodes });
        }

        log::trace!("Done loading gltf");

        Ok(GltfDocument {
            accessors,
            animations,
            buffers,
            cameras,
            materials,
            default_material,
            meshes,
            nodes,
            scenes,
            skins,
            textures,
            views,
        })
    }

    /// Create a native camera for the gltf camera with the given index.
    pub fn create_camera_from_gltf(
        &self,
        cpu_doc: &gltf::Document,
        index: usize,
    ) -> Result<Camera, StageGltfError> {
        let gltf_camera = cpu_doc
            .cameras()
            .nth(index)
            .context(MissingCameraSnafu { index })?;
        let projection = match gltf_camera.projection() {
            gltf::camera::Projection::Orthographic(o) => glam::Mat4::orthographic_rh(
                -o.xmag(),
                o.xmag(),
                -o.ymag(),
                o.ymag(),
                o.znear(),
                o.zfar(),
            ),
            gltf::camera::Projection::Perspective(p) => {
                let fovy = p.yfov();
                let aspect = p.aspect_ratio().unwrap_or(1.0);
                if let Some(zfar) = p.zfar() {
                    glam::Mat4::perspective_rh(fovy, aspect, p.znear(), zfar)
                } else {
                    glam::Mat4::perspective_infinite_rh(
                        p.yfov(),
                        p.aspect_ratio().unwrap_or(1.0),
                        p.znear(),
                    )
                }
            }
        };
        let view = cpu_doc
            .nodes()
            .find_map(|node| {
                if node.camera().map(|c| c.index()) == Some(index) {
                    Some(glam::Mat4::from_cols_array_2d(&node.transform().matrix()).inverse())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        Ok(Camera {
            projection,
            view,
            ..Default::default()
        })
    }

    /// Draw the given `gltf::Node` with the given `Camera`.
    /// `parents` is a list of the parent nodes of the given node.
    fn draw_gltf_node_with<'a>(
        &mut self,
        gpu_doc: &GltfDocument,
        camera_id: Id<Camera>,
        node: gltf::Node<'a>,
        parents: Vec<Id<GltfNode>>,
    ) -> Vec<Id<GltfRendering>> {
        if let Some(_light) = node.light() {
            // TODO: Support transforming lights based on node transforms
            ////let light = gpu_doc.lights.at(light.index());
            //let t = Mat4::from_cols_array_2d(&node.transform().matrix());
            //let position = t.transform_point3(Vec3::ZERO);

            //let light_index = light.index();
            //let color = Vec3::from(light.color());
            //let range = light.range().unwrap_or(f32::MAX);
            //let intensity = light.intensity();
            //match light.kind() {
            //    gltf::khr_lights_punctual::Kind::Directional =>
            // GltfLightKind::Directional,
            //    gltf::khr_lights_punctual::Kind::Point =>
            // GltfLightKind::Point,
            //    gltf::khr_lights_punctual::Kind::Spot {
            //        inner_cone_angle,
            //        outer_cone_angle,
            //    } => GltfLightKind::Spot {
            //        inner_cone_angle,
            //        outer_cone_angle,
            //    },
            //};

            //let transform = self.append(&transform);
            //let render_unit = RenderUnit {
            //    light,
            //    transform,
            //    ..Default::default()
            //};
            //return vec![self.draw_unit(&render_unit)];
        }
        let mut units = if let Some(mesh) = node.mesh() {
            log::trace!("drawing mesh {}", mesh.index());
            let primitives = mesh.primitives();
            let mesh = gpu_doc.meshes.at(mesh.index());
            let mut node_path = parents.clone();
            node_path.push(gpu_doc.nodes.at(node.index()));
            primitives
                .map(|primitive| {
                    let node_path = self.append_array(&node_path);
                    let render_unit = GltfRendering {
                        node_path,
                        mesh_index: mesh.index() as u32,
                        primitive_index: primitive.index() as u32,
                        vertex_count: super::get_vertex_count(&primitive),
                        camera: camera_id,
                        ..Default::default()
                    };
                    self.draw_gltf_rendering(&render_unit)
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };
        for child in node.children() {
            let mut parents = parents.clone();
            parents.push(gpu_doc.nodes.at(child.index()));
            units.extend(self.draw_gltf_node_with(gpu_doc, camera_id, child, parents));
        }
        units
    }

    /// Draw the given [`gltf::Node`] using the given [`Camera`] and return the
    /// ids of the render units that were created.
    pub fn draw_gltf_node(
        &mut self,
        gpu_doc: &GltfDocument,
        camera_id: Id<Camera>,
        node: gltf::Node<'_>,
    ) -> Vec<Id<GltfRendering>> {
        self.draw_gltf_node_with(gpu_doc, camera_id, node, vec![])
    }

    /// Draw the given [`gltf::Scene`] using the given [`Camera`] and return the
    /// ids of the render units that were created.
    pub fn draw_gltf_scene(
        &mut self,
        gpu_doc: &GltfDocument,
        camera_id: Id<Camera>,
        scene: gltf::Scene<'_>,
    ) -> Vec<Id<GltfRendering>> {
        scene
            .nodes()
            .flat_map(|node| self.draw_gltf_node(gpu_doc, camera_id, node))
            .collect()
    }

    /// Convenience method for creating a `GltfPrimitive` along with all its
    /// `GltfAccessor`s, `GltfBufferView`s and a `GltfBuffer`.
    ///
    /// ## Note
    /// This does **not** generate tangents or normals.
    pub fn new_primitive(
        &mut self,
        vertices: impl IntoIterator<Item = Vertex>,
        indices: impl IntoIterator<Item = u32>,
        material: Id<Material>,
    ) -> GltfPrimitive {
        let vertices: Vec<Vertex> = vertices.into_iter().collect();
        let indices: Vec<u32> = indices.into_iter().collect();
        let indices_len: usize = indices.len();
        let vertices_len: usize = vertices.len();
        let vertex_count = if vertices_len > indices_len {
            vertices_len
        } else {
            indices_len
        } as u32;
        let indices = if indices.is_empty() {
            Id::NONE
        } else {
            let buffer = GltfBuffer(self.append_array(&indices).into_u32_array());
            let buffer = self.append(&buffer);
            let view = self.append(&GltfBufferView {
                buffer,
                offset: 0,
                length: indices.len() as u32 * 4, // 4 bytes per u32
                stride: 4,                        // 4 bytes in a u32,
            });
            let accessor = self.append(&GltfAccessor {
                size: 4,
                view,
                offset: 0,
                count: indices.len() as u32,
                data_type: DataType::U32,
                dimensions: Dimensions::Scalar,
                normalized: false,
            });
            accessor
        };

        let vertex_buffer = GltfBuffer(self.append_array(&vertices).into_u32_array());
        let buffer = self.append(&vertex_buffer);
        let u32_stride = 4 // 4 position components,
            + 4 // 4 color components,
            + 4 // 4 uv components,
            + 4 // 4 normal components,
            + 4 // 4 tangent components,
            + 4 // 4 joint components,
            + 4; // 4 weight components
        let stride = u32_stride * 4; // 4 bytes in a u32

        let view = self.append(&GltfBufferView {
            buffer,
            offset: 0,
            length: vertices.len() as u32 * u32_stride * 4, // stride as u32s * 4 bytes each
            stride,
        });

        let positions = self.append(&GltfAccessor {
            size: 3 * 4, // 3 position components * 4 bytes each
            view,
            offset: 0,
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec3,
            normalized: false,
        });

        let colors = self.append(&GltfAccessor {
            size: 4 * 4, // 4 color components * 4 bytes each
            view,
            offset: 4 * 4, // 3 + 1 position components * 4 bytes each
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec4,
            normalized: false,
        });

        let tex_coords0 = self.append(&GltfAccessor {
            size: 2 * 4, // 2 uv components * 4 bytes each
            view,
            offset: 8 * 4, // (3 + 1) position + 4 color components * 4 bytes each
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec2,
            normalized: false,
        });

        let tex_coords1 = self.append(&GltfAccessor {
            size: 2 * 4, // 2 uv components * 4 bytes each
            view,
            offset: 10 * 4, // (3 + 1) position + 4 color + 2 uv components * 4 bytes each
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec2,
            normalized: false,
        });

        let normals = self.append(&GltfAccessor {
            size: 3 * 4, // 3 normal components * 4 bytes each
            view,
            offset: 12 * 4, // (3 + 1) position + 4 color + 4 uv components * 4 bytes each
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec3,
            normalized: false,
        });

        let tangents = self.append(&GltfAccessor {
            size: 4 * 4, // 4 tangent components * 4 bytes each
            view,
            offset: 16 * 4, /* (3 + 1) position + 4 color + 4 uv + (3 + 1) normal components * 4
                             * bytes each */
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec4,
            normalized: false,
        });

        let joints = self.append(&GltfAccessor {
            size: 4 * 4, // 4 joint components * 4 bytes each
            view,
            offset: 20 * 4, // (3 + 1) position + 4 color + 4 uv + (3 + 1) normal + 4 tangent components * 4 bytes each
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec4,
            normalized: false,
        });

        let weights = self.append(&GltfAccessor {
            size: 4 * 4, // 4 weight components * 4 bytes each
            view,
            offset: 24 * 4, // (3 + 1) position + 4 color + 4 uv + (3 + 1) normal + 4 tangent + 4 joint components * 4 bytes each
            count: vertex_count as u32,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec4,
            normalized: false,
        });

        GltfPrimitive {
            vertex_count,
            material,
            indices,
            positions,
            normals,
            normals_were_generated: false,
            tangents,
            tangents_were_generated: false,
            colors,
            tex_coords0,
            tex_coords1,
            joints,
            weights,
        }
    }

    /// Convenience method for creating a [`GltfMesh`] without having a
    /// [`gltf::Document`].
    ///
    /// This is useful if you have non-GLTF assets that you want to render.
    pub fn new_mesh(&mut self) -> GltfMeshBuilder {
        GltfMeshBuilder::new(self)
    }
}

/// Convenience builder for creating a [`GltfMesh`] without having a
/// [`gltf::Document`].
///
/// This is useful if you have non-GLTF assets that you want to render.
pub struct GltfMeshBuilder<'a> {
    stage: &'a mut Stage,
    primitives: Vec<GltfPrimitive>,
}

impl<'a> GltfMeshBuilder<'a> {
    pub fn new(stage: &'a mut Stage) -> Self {
        Self {
            stage,
            primitives: vec![],
        }
    }

    pub fn add_primitive(
        &mut self,
        vertices: impl IntoIterator<Item = Vertex>,
        indices: impl IntoIterator<Item = u32>,
        material_id: Id<Material>,
    ) {
        let primitive = self.stage.new_primitive(vertices, indices, material_id);
        self.primitives.push(primitive);
    }

    pub fn with_primitive(
        mut self,
        vertices: impl IntoIterator<Item = Vertex>,
        indices: impl IntoIterator<Item = u32>,
        material_id: Id<Material>,
    ) -> Self {
        self.add_primitive(vertices, indices, material_id);
        self
    }

    pub fn build(self) -> GltfMesh {
        let weights = Array::default();
        let primitives = self.stage.append_array(&self.primitives);
        GltfMesh {
            primitives,
            weights,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{gltf::*, pbr::Material, stage::Vertex, Camera, Renderling, Stage, Transform};
    use crabslab::{Array, GrowableSlab, Id, Slab, SlabItem};
    use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};

    #[test]
    fn get_vertex_count_primitive_sanity() {
        let (document, _, _) =
            gltf::import("../../gltf/gltfTutorial_008_SimpleMeshes.gltf").unwrap();
        let prim = document
            .meshes()
            .next()
            .unwrap()
            .primitives()
            .next()
            .unwrap();
        let vertex_count = super::get_vertex_count(&prim);
        assert_eq!(3, vertex_count);
    }

    #[test]
    fn accessor_sanity() {
        println!("1u8: {:08b}", 1u8);
        println!("1i8: {:08b}", 1i8);
        println!("-1i8: {:08b}", -1i8);
        println!("max: {} {}", u8::MAX, i8::MAX);
        let u16buffer = [1u16, 1u16, 1u16, 1u16];
        for (i, chunk) in u16buffer.chunks(2).enumerate() {
            match chunk {
                [a, b] => {
                    println!("u16buffer[{i}]: {a:016b} {b:016b}");
                }
                _ => panic!("bad chunk"),
            }
        }
        let u32buffer = bytemuck::cast_slice::<u16, u32>(&u16buffer).to_vec();
        for u in u32buffer.iter() {
            println!("u32buffer: {u:032b}");
        }
        println!("u32buffer: {u32buffer:?}");
        assert_eq!(2, u32buffer.len());
        let mut data = [0u32; 256];
        let buffer_index = data.write_indexed_slice(&u32buffer, 0);
        assert_eq!(2, buffer_index);
        assert_eq!(u32buffer, data[0..2], "unexpected buffer contents");
        let buffer = GltfBuffer(Array::new(0, buffer_index as u32));
        let view_index = data.write_indexed(&buffer, buffer_index);
        println!("view_index: {view_index}");
        let final_index = data.write_indexed(
            &GltfBufferView {
                buffer: Id::from(buffer_index),
                offset: 0,
                length: 4 * 2, // 4 elements * 2 bytes each
                stride: 2,
            },
            view_index,
        );
        assert_eq!(view_index + GltfBufferView::SLAB_SIZE, final_index);
        let accessor = GltfAccessor {
            size: 2,
            count: 3,
            view: Id::from(view_index),
            offset: 0,
            data_type: DataType::U16,
            dimensions: Dimensions::Scalar,
            normalized: false,
        };
        let i0 = accessor.get_u32(0, &data);
        let i1 = accessor.get_u32(1, &data);
        let i2 = accessor.get_u32(2, &data);
        println!("data: {:#?}...", &data[0..4]);
        assert_eq!([1, 1, 1], [i0, i1, i2]);
    }

    #[test]
    // ensures we can
    // * read simple meshes
    // * support multiple nodes that reference the same mesh
    // * support primitives w/ positions and normal attributes
    // * support transforming nodes (T * R * S)
    fn stage_gltf_simple_meshes() {
        let mut r =
            Renderling::headless(100, 50).with_background_color(Vec3::splat(0.0).extend(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let (document, buffers, images) =
            ::gltf::import("../../gltf/gltfTutorial_008_SimpleMeshes.gltf").unwrap();
        let projection = crate::camera::perspective(100.0, 50.0);
        let position = Vec3::new(1.0, 0.5, 1.5);
        let view = crate::camera::look_at(position, Vec3::new(1.0, 0.5, 0.0), Vec3::Y);
        let mut stage = Stage::new(device.clone(), queue.clone()).with_lighting(false);
        stage.configure_graph(&mut r, true);
        let gpu_doc = stage
            .load_gltf_document(&document, buffers.clone(), images)
            .unwrap();
        let camera = Camera {
            projection,
            view,
            position,
        };
        let camera_id = stage.append(&camera);

        let default_scene = document.default_scene().unwrap();
        let unit_ids = stage.draw_gltf_scene(&gpu_doc, camera_id, default_scene);
        assert_eq!(2, unit_ids.len());

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_simple_meshes.png", img);
    }

    #[test]
    // Ensures we can read a minimal gltf file with a simple triangle mesh.
    fn minimal_mesh() {
        let mut r =
            Renderling::headless(20, 20).with_background_color(Vec3::splat(0.0).extend(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let mut stage = Stage::new(device, queue).with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (document, buffers, images) =
            ::gltf::import("../../gltf/gltfTutorial_003_MinimalGltfFile.gltf").unwrap();
        let gpu_doc = stage
            .load_gltf_document(&document, buffers, images)
            .unwrap();
        let projection = crate::camera::perspective(20.0, 20.0);
        let eye = Vec3::new(0.5, 0.5, 2.0);
        let view = crate::camera::look_at(eye, Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let camera = Camera {
            projection,
            view,
            position: Vec3::new(0.5, 0.5, 2.0),
        };
        let camera_id = stage.append(&camera);
        let default_scene = document.default_scene().unwrap();
        let _unit_ids = stage.draw_gltf_scene(&gpu_doc, camera_id, default_scene);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_minimal_mesh.png", img);
    }

    #[test]
    // Test that the top-level transform on `RenderUnit` transforms their
    // child primitive's geometry correctly.
    fn render_unit_transforms_primitive_geometry() {
        let mut r = Renderling::headless(50, 50).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage().with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (projection, view) = crate::camera::default_ortho2d(50.0, 50.0);
        let camera = stage.append(&Camera::new(projection, view));
        let cyan = [0.0, 1.0, 1.0, 1.0];
        let magenta = [1.0, 0.0, 1.0, 1.0];
        let yellow = [1.0, 1.0, 0.0, 1.0];
        let white = [1.0, 1.0, 1.0, 1.0];
        let vertices = [
            Vertex::default()
                .with_position([0.0, 0.0, 0.0])
                .with_color(cyan),
            Vertex::default()
                .with_position([1.0, 0.0, 0.0])
                .with_color(magenta),
            Vertex::default()
                .with_position([1.0, 1.0, 0.0])
                .with_color(yellow),
            Vertex::default()
                .with_position([0.0, 1.0, 0.0])
                .with_color(white),
        ];
        let primitive = stage.new_primitive(vertices, [0, 3, 2, 0, 2, 1], Id::NONE);
        let primitives = stage.append_array(&[primitive]);
        let mesh = stage.append(&GltfMesh {
            primitives,
            ..Default::default()
        });
        let node = stage.append(&GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);
        let transform = stage.append(&Transform {
            scale: Vec3::new(50.0, 50.0, 1.0),
            ..Default::default()
        });
        let _unit_id = stage.draw_gltf_rendering(&GltfRendering {
            camera,
            transform,
            vertex_count: primitive.vertex_count,
            node_path,
            ..Default::default()
        });
        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("gltf/render_unit_transforms_primitive_geometry.png", img);
    }

    #[test]
    // Tests importing a gltf file and rendering the first image as a 2d object.
    //
    // This ensures we are decoding images correctly.
    fn gltf_images() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let mut stage = Stage::new(device.clone(), queue.clone()).with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (document, buffers, images) = gltf::import("../../gltf/cheetah_cone.glb").unwrap();
        let gpu_doc = stage
            .load_gltf_document(&document, buffers, images)
            .unwrap();
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let camera_id = stage.append(&Camera::new(projection, view));
        assert!(!gpu_doc.textures.is_empty());
        let albedo_texture_id = gpu_doc.textures.at(0);
        assert!(albedo_texture_id.is_some());
        let material_id = stage.append(&Material {
            albedo_texture: albedo_texture_id,
            has_lighting: false,
            ..Default::default()
        });
        println!("material_id: {:#?}", material_id);
        let mesh = stage
            .new_mesh()
            .with_primitive(
                [
                    Vertex::default()
                        .with_position([0.0, 0.0, 0.0])
                        .with_uv0([0.0, 0.0]),
                    Vertex::default()
                        .with_position([1.0, 0.0, 0.0])
                        .with_uv0([1.0, 0.0]),
                    Vertex::default()
                        .with_position([1.0, 1.0, 0.0])
                        .with_uv0([1.0, 1.0]),
                    Vertex::default()
                        .with_position([0.0, 1.0, 0.0])
                        .with_uv0([0.0, 1.0]),
                ],
                [0, 3, 2, 0, 2, 1],
                material_id,
            )
            .build();
        let mesh = stage.append(&mesh);
        let node = stage.append(&GltfNode {
            mesh,
            ..Default::default()
        });
        let node_path = stage.append_array(&[node]);

        let transform = stage.append(&Transform {
            scale: Vec3::new(100.0, 100.0, 1.0),
            ..Default::default()
        });

        let _unit_id = stage.draw_gltf_rendering(&GltfRendering {
            camera: camera_id,
            transform,
            vertex_count: 6,
            node_path,
            mesh_index: 0,
            primitive_index: 0,
        });

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("gltf_images.png", img);
    }

    #[test]
    fn simple_texture() {
        let size = 100;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(0.0).extend(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let mut stage = Stage::new(device.clone(), queue.clone())
            // There are no lights in the scene and the material isn't marked as "unlit", so
            // let's force it to be unlit.
            .with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (cpu_doc, gpu_doc) = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_013_SimpleTexture.gltf")
            .unwrap();

        let projection = crate::camera::perspective(size as f32, size as f32);
        let view =
            crate::camera::look_at(Vec3::new(0.5, 0.5, 1.25), Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let camera = stage.append(&Camera::new(projection, view));
        let _unit_ids = stage.draw_gltf_scene(&gpu_doc, camera, cpu_doc.default_scene().unwrap());

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_simple_texture.png", img);
    }

    // This can be uncommented when we support lighting from GLTF files
    //#[test]
    //// Demonstrates how to load and render a gltf file containing lighting and a
    //// normal map.
    //fn normal_mapping_brick_sphere() {
    //    let size = 600;
    //    let mut r =
    //        Renderling::headless(size,
    // size).with_background_color(Vec3::splat(1.0).extend(1.0));    let mut
    // stage = r.new_stage().with_lighting(true).with_bloom(true);
    //    stage.configure_graph(&mut r, true);
    //    let (cpu_doc, gpu_doc) = stage
    //        .load_gltf_document_from_path("../../gltf/red_brick_03_1k.glb")
    //        .unwrap();
    //    let camera = stage.create_camera_from_gltf(&cpu_doc, 0).unwrap();
    //    let camera_id = stage.append(&camera);
    //    let _unit_ids =
    //        stage.draw_gltf_scene(&gpu_doc, camera_id,
    // cpu_doc.default_scene().unwrap());

    //    let img = r.render_image().unwrap();
    //    img_diff::assert_img_eq("gltf_normal_mapping_brick_sphere.png", img);
    //}

    #[test]
    // Demonstrates how to generate a mesh primitive on the CPU, long hand.
    fn gltf_cmy_tri() {
        let size = 100;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r
            .new_stage()
            // There are no lights in the scene and the material isn't marked as "unlit", so
            // let's force it to be unlit.
            .with_lighting(false);
        stage.configure_graph(&mut r, true);

        // buffers
        let positions =
            stage.append_array(&[[0.0, 0.0, 0.5f32], [0.0, 100.0, 0.5], [100.0, 0.0, 0.5]]);
        let positions_buffer = GltfBuffer(positions.into_u32_array());
        let cyan = [0.0, 1.0, 1.0, 1.0f32];
        let magenta = [1.0, 0.0, 1.0, 1.0];
        let yellow = [1.0, 1.0, 0.0, 1.0];
        let colors = stage.append_array(&[cyan, magenta, yellow]);
        let colors_buffer = GltfBuffer(colors.into_u32_array());
        let buffers = stage.append_array(&[positions_buffer, colors_buffer]);

        // views
        let positions_view = GltfBufferView {
            buffer: buffers.at(0),
            offset: 0,
            length: 3 * 3 * 4, // 3 vertices * 3 components * 4 bytes each
            stride: 3 * 4,
        };
        let colors_view = GltfBufferView {
            buffer: buffers.at(1),
            offset: 0,
            length: 3 * 4 * 4, // 3 vertices * 4 components * 4 bytes each
            stride: 4 * 4,
        };
        let views = stage.append_array(&[positions_view, colors_view]);

        // accessors
        let positions_accessor = GltfAccessor {
            size: 3 * 4, // 3 components * 4 bytes each
            view: views.at(0),
            offset: 0,
            count: 3,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec3,
            normalized: false,
        };
        let colors_accessor = GltfAccessor {
            size: 4 * 4,
            view: views.at(1),
            offset: 0,
            count: 3,
            data_type: DataType::F32,
            dimensions: Dimensions::Vec4,
            normalized: false,
        };
        let accessors = stage.append_array(&[positions_accessor, colors_accessor]);

        // meshes
        let primitive = GltfPrimitive {
            vertex_count: 3,
            positions: accessors.at(0),
            colors: accessors.at(1),
            ..Default::default()
        };
        let primitives = stage.append_array(&[primitive]);
        let mesh = GltfMesh {
            primitives,
            ..Default::default()
        };
        let meshes = stage.append_array(&[mesh]);

        // nodes
        let node = GltfNode {
            mesh: meshes.at(0),
            ..Default::default()
        };
        let nodes = stage.append_array(&[node]);

        // doc
        let _doc = stage.append(&GltfDocument {
            accessors,
            buffers,
            meshes,
            nodes,
            ..Default::default()
        });

        // render unit
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera::new(projection, view));
        let node_path = stage.append_array(&[nodes.at(0)]);
        let rendering_id = stage.draw_gltf_rendering(&GltfRendering {
            camera,
            node_path,
            mesh_index: 0,
            primitive_index: 0,
            vertex_count: 3,
            ..Default::default()
        });

        let data = stage.read_slab().unwrap();
        let invocation = GltfVertexInvocation::invoke(rendering_id.inner(), 0, &data);
        println!("invoctaion: {invocation:#?}");

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("gltf/cmy_tri.png", img);
    }

    /// A helper struct that contains all outputs of the vertex shader.
    #[allow(unused)]
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GltfVertexInvocation {
        pub instance_index: u32,
        pub vertex_index: u32,
        pub render_unit_id: Id<GltfRendering>,
        pub render_unit: GltfRendering,
        pub out_camera: u32,
        pub out_material: u32,
        pub out_color: Vec4,
        pub out_uv0: Vec2,
        pub out_uv1: Vec2,
        pub out_norm: Vec3,
        pub out_tangent: Vec3,
        pub out_bitangent: Vec3,
        pub out_pos: Vec3,
        // output clip coordinates
        pub clip_pos: Vec4,
        // output normalized device coordinates
        pub ndc_pos: Vec3,
    }

    impl GltfVertexInvocation {
        #[allow(dead_code)]
        pub fn invoke(instance_index: u32, vertex_index: u32, slab: &[u32]) -> Self {
            let mut v = Self {
                instance_index,
                vertex_index,
                ..Default::default()
            };
            v.render_unit_id = Id::from(v.instance_index);
            v.render_unit = slab.read(v.render_unit_id);
            vertex(
                v.render_unit_id,
                v.vertex_index,
                slab,
                &mut v.out_camera,
                &mut v.out_material,
                &mut v.out_color,
                &mut v.out_uv0,
                &mut v.out_uv1,
                &mut v.out_norm,
                &mut v.out_tangent,
                &mut v.out_bitangent,
                &mut v.out_pos,
                &mut v.clip_pos,
            );
            v.ndc_pos = v.clip_pos.xyz() / v.clip_pos.w;
            v
        }
    }
}
