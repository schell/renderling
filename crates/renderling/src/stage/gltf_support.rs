//! Gltf support for the [`Stage`](crate::Stage).
use super::*;
use crate::{
    shader::{
        gltf::*,
        pbr::PbrMaterial,
        stage::{Camera, GltfVertexData, LightingModel, VertexData},
        texture::{GpuTexture, TextureAddressMode, TextureModes},
    },
    SceneImage,
};
use glam::{Quat, Vec2, Vec3, Vec4};
use renderling_shader::stage::{Transform, Vertex};
use snafu::{OptionExt, ResultExt, Snafu};

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

    #[snafu(display("Missing gltf camera at index {index}"))]
    MissingCamera { index: usize },

    #[snafu(display("{source}"))]
    Slab { source: crate::slab::SlabError },
}

impl From<crate::slab::SlabError> for StageGltfError {
    fn from(source: crate::slab::SlabError) -> Self {
        Self::Slab { source }
    }
}

impl From<gltf::Error> for StageGltfError {
    fn from(source: gltf::Error) -> Self {
        Self::Gltf { source }
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

pub fn make_accessor(accessor: gltf::Accessor<'_>, buffers: &Array<GltfBuffer>) -> GltfAccessor {
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
    GltfAccessor {
        size,
        count,
        buffer,
        view_offset,
        view_stride,
        data_type: component_type,
        dimensions,
        normalized,
    }
}

impl Stage {
    pub fn load_gltf_document_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(gltf::Document, GltfDocument), StageGltfError> {
        let (document, buffers, images) = gltf::import(path)?;
        let gpu_doc = self.load_gltf_document(&document, buffers, images)?;
        Ok((document, gpu_doc))
    }

    pub fn load_gltf_document(
        &self,
        document: &gltf::Document,
        buffer_data: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<GltfDocument, StageGltfError> {
        log::trace!("Loading buffers into the GPU");
        let buffers = self.allocate_array::<GltfBuffer>(buffer_data.len());
        for (i, buffer) in buffer_data.iter().enumerate() {
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
            let id = views.at(view.index());
            let gltf_view = GltfBufferView {
                buffer,
                offset,
                length,
                stride,
            };
            self.write(id, &gltf_view)?;
        }

        log::trace!("Loading accessors into the GPU");
        let accessors = document
            .accessors()
            .map(|accessor| make_accessor(accessor, &buffers))
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
        // if the materials require us to apply a linear transfer. So we'll get the preview
        // repacking first, then update the frames in the textures.
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
        let textures = self.allocate_array::<GpuTexture>(document.textures().len());
        for (i, texture) in document.textures().enumerate() {
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
            let texture = GpuTexture {
                offset_px,
                size_px,
                modes: TextureModes::default()
                    .with_wrap_s(mode_s)
                    .with_wrap_t(mode_t),
                atlas_index: (image_index + atlas_offset) as u32,
            };
            let texture_id = textures.at(i);
            log::trace!("  texture {i} {texture_id:?}: {texture:#?}");
            self.write(texture_id, &texture)?;
        }

        log::trace!("Creating materials");
        let mut default_material = Id::<PbrMaterial>::NONE;
        let materials = self.allocate_array::<PbrMaterial>(document.materials().len());
        for material in document.materials() {
            let material_id = if let Some(index) = material.index() {
                materials.at(index)
            } else {
                // Allocate some extra space for this default material
                default_material = self.allocate::<PbrMaterial>();
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
            log::trace!("  material {material_id:?}: {material:#?}",);
            self.write(material_id, &material)?;
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
            let size_id = Id::new(0) + StageLegend::offset_of_atlas_size();
            self.write(size_id, &size)?;
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
                        let buffer_id = self.append(&buffer);
                        let accessor = GltfAccessor {
                            size: 12,
                            buffer: buffer_id,
                            view_offset: 0,
                            view_stride: 12,
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
                        let buffer_id = self.append(&buffer);
                        let accessor = GltfAccessor {
                            size: 16,
                            buffer: buffer_id,
                            view_offset: 0,
                            view_stride: 16,
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
                self.write(id, &prim)?;
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

    // For now we have to keep the original document around to figure out
    // what to draw.
    fn draw_gltf_node_with<'a>(
        &self,
        gpu_doc: &GltfDocument,
        camera_id: Id<Camera>,
        node: gltf::Node<'a>,
        parents: Vec<Id<GltfNode>>,
    ) -> Vec<Id<RenderUnit>> {
        let mut units = if let Some(mesh) = node.mesh() {
            let primitives = mesh.primitives();
            let mesh = gpu_doc.meshes.at(mesh.index());
            primitives
                .map(|primitive| {
                    let parent_node_path = self.append_array(&parents);
                    let vertex_data_id = self.append(&GltfVertexData {
                        parent_node_path,
                        mesh,
                        primitive_index: primitive.index() as u32,
                    });
                    let (t, r, s) = node.transform().decomposed();
                    let transform = self.append(&Transform {
                        translation: Vec3::from(t),
                        rotation: Quat::from_array(r),
                        scale: Vec3::from(s),
                    });
                    let render_unit = RenderUnit {
                        vertex_data: VertexData::new_gltf(vertex_data_id),
                        vertex_count: super::get_vertex_count(&primitive),
                        transform,
                        camera: camera_id,
                        ..Default::default()
                    };
                    self.draw_unit(&render_unit)
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

    /// Draw the given [`gltf::Node`] using the given [`Camera`] and return the ids of the
    /// render units that were created.
    pub fn draw_gltf_node(
        &self,
        gpu_doc: &GltfDocument,
        camera_id: Id<Camera>,
        node: gltf::Node<'_>,
    ) -> Vec<Id<RenderUnit>> {
        self.draw_gltf_node_with(gpu_doc, camera_id, node, vec![])
    }

    /// Draw the given [`gltf::Scene`] using the given [`Camera`] and return the ids of the
    /// render units that were created.
    pub fn draw_gltf_scene(
        &self,
        gpu_doc: &GltfDocument,
        camera_id: Id<Camera>,
        scene: gltf::Scene<'_>,
    ) -> Vec<Id<RenderUnit>> {
        scene
            .nodes()
            .flat_map(|node| self.draw_gltf_node(gpu_doc, camera_id, node))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use glam::{Vec3, Vec4};

    use crate::{
        shader::{
            array::Array,
            gltf::*,
            pbr::PbrMaterial,
            slab::Slab,
            stage::{
                Camera, GltfVertexData, LightingModel, NativeVertexData, RenderUnit, Transform,
                Vertex, VertexData,
            },
        },
        Id, Renderling, Stage,
    };

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
        println!("{:08b}", 1u8);
        println!("{:08b}", 1i8);
        println!("{:08b}", -1i8);
        println!("{} {}", u8::MAX, i8::MAX);
        let u16buffer = [1u16, 1u16, 1u16, 1u16];
        for chunk in u16buffer.chunks(2) {
            match chunk {
                [a, b] => {
                    println!("{a:016b} {b:016b}");
                }
                _ => panic!("bad chunk"),
            }
        }
        let u32buffer = bytemuck::cast_slice::<u16, u32>(&u16buffer).to_vec();
        for u in u32buffer.iter() {
            println!("{u:032b}");
        }
        println!("u32buffer: {u32buffer:?}");
        assert_eq!(2, u32buffer.len());
        let mut data = [0u32; 256];
        let buffer_index = data.write_slice(&u32buffer, 0);
        assert_eq!(2, buffer_index);
        let buffer = GltfBuffer(Array::new(0, buffer_index as u32));
        let _ = data.write(&buffer, buffer_index);
        let accessor = GltfAccessor {
            size: 2,
            count: 3,
            buffer: Id::from(buffer_index),
            view_offset: 0,
            view_stride: 0,
            data_type: DataType::U16,
            dimensions: Dimensions::Scalar,
            normalized: false,
        };
        let i0 = accessor.get_u32(0, &data);
        assert_eq!(1, i0);
        let i1 = accessor.get_u32(1, &data);
        assert_eq!(1, i1);
        let i2 = accessor.get_u32(2, &data);
        assert_eq!(1, i2);
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
            gltf::import("../../gltf/gltfTutorial_008_SimpleMeshes.gltf").unwrap();
        let projection = crate::camera::perspective(100.0, 50.0);
        let position = Vec3::new(1.0, 0.5, 1.5);
        let view = crate::camera::look_at(position, Vec3::new(1.0, 0.5, 0.0), Vec3::Y);
        let stage = Stage::new(device.clone(), queue.clone()).with_lighting(false);
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

        let data = futures_lite::future::block_on(stage.slab.read_raw(
            &device,
            &queue,
            0,
            stage.slab.len(),
        ))
        .unwrap();

        let draws = stage.get_draws();
        let slab = &data;

        for i in 0..gpu_doc.accessors.len() {
            let accessor = slab.read(gpu_doc.accessors.at(i));
            println!("accessor {i}: {accessor:#?}", i = i, accessor = accessor);
            let buffer = slab.read(accessor.buffer);
            println!("buffer: {buffer:#?}");
            let buffer_data = slab.read_vec(buffer.0);
            println!("buffer_data: {buffer_data:#?}");
        }

        let indices = draws
            .iter()
            .map(|draw| {
                let unit_id = draw.id;
                let unit = slab.read(unit_id);
                assert_eq!(unit.vertex_data.is_gltf(), true);
                let vertex_data_id = Id::<GltfVertexData>::from(unit.vertex_data.index);
                let vertex_data = slab.read(vertex_data_id);
                let mesh = slab.read(vertex_data.mesh);
                let primitive_id = mesh.primitives.at(vertex_data.primitive_index as usize);
                let primitive = slab.read(primitive_id);
                if primitive.indices.is_some() {
                    let indices_accessor = slab.read(primitive.indices);
                    (0..draw.vertex_count)
                        .map(|i| {
                            let index = indices_accessor.get_u32(i as usize, slab);
                            index
                        })
                        .collect::<Vec<_>>()
                } else {
                    (0..draw.vertex_count).collect::<Vec<_>>()
                }
            })
            .collect::<Vec<_>>();
        assert_eq!([0, 1, 2], indices[0].as_slice());
        assert_eq!([0, 1, 2], indices[1].as_slice());

        let invocations = draws
            .into_iter()
            .flat_map(|draw| {
                let render_unit_id = draw.id;
                let instance_index = render_unit_id.inner();
                let render_unit = data.read(render_unit_id);
                let data = &data;
                (0..draw.vertex_count).map(move |vertex_index| {
                    let mut invocation = crate::test::VertexInvocation {
                        render_unit_id,
                        render_unit,
                        instance_index,
                        vertex_index,
                        ..Default::default()
                    };
                    renderling_shader::stage::new_stage_vertex(
                        instance_index,
                        vertex_index,
                        data,
                        &mut invocation.out_camera,
                        &mut invocation.out_material,
                        &mut invocation.out_color,
                        &mut invocation.out_uv0,
                        &mut invocation.out_uv1,
                        &mut invocation.out_norm,
                        &mut invocation.out_tangent,
                        &mut invocation.out_bitangent,
                        &mut invocation.out_pos,
                        &mut invocation.clip_pos,
                    );
                    invocation
                })
            })
            .collect::<Vec<_>>();
        let seen_positions = invocations
            .iter()
            .map(|inv| inv.out_pos)
            .take(3)
            .collect::<Vec<_>>();
        let mesh = document.meshes().next().unwrap();
        let prim = mesh.primitives().next().unwrap();
        let expected_positions_reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));
        let expected_positions = expected_positions_reader
            .read_positions()
            .unwrap()
            .map(|pos| Vec3::from(pos))
            .collect::<Vec<_>>();
        assert_eq!(expected_positions, seen_positions);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_simple_meshes.png", img);
    }

    #[test]
    // Ensures we can read a minimal gltf file with a simple triangle mesh.
    fn minimal_mesh() {
        let mut r =
            Renderling::headless(20, 20).with_background_color(Vec3::splat(0.0).extend(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let stage = Stage::new(device, queue).with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (document, buffers, images) =
            gltf::import("../../gltf/gltfTutorial_003_MinimalGltfFile.gltf").unwrap();
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
    // Tests importing a gltf file and rendering the first image as a 2d object.
    //
    // This ensures we are decoding images correctly.
    fn stage_gltf_images() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let stage = Stage::new(device.clone(), queue.clone()).with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (document, buffers, images) = gltf::import("../../gltf/cheetah_cone.glb").unwrap();
        let gpu_doc = stage
            .load_gltf_document(&document, buffers, images)
            .unwrap();
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let camera_id = stage.append(&Camera {
            projection,
            view,
            position: Vec3::ZERO,
        });
        assert!(!gpu_doc.textures.is_empty());
        let albedo_texture_id = gpu_doc.textures.at(0);
        assert!(albedo_texture_id.is_some());
        let material_id = stage.append(&PbrMaterial {
            albedo_texture: albedo_texture_id,
            lighting_model: LightingModel::NO_LIGHTING,
            ..Default::default()
        });
        println!("material_id: {:#?}", material_id);
        let vertices = stage.append_array(&vec![
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
        ]);
        let indices = stage.append_array(&[0, 3, 2, 0, 2, 1]);
        let native_data_id = stage.append(&NativeVertexData {
            vertices,
            indices,
            material: material_id,
        });
        let transform = stage.append(&Transform {
            scale: Vec3::new(100.0, 100.0, 1.0),
            ..Default::default()
        });
        let _unit_id = stage.draw_unit(&RenderUnit {
            vertex_data: VertexData::new_native(native_data_id),
            camera: camera_id,
            transform,
            vertex_count: indices.len() as u32,
        });
        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_images.png", img);
    }

    #[test]
    fn simple_texture() {
        let size = 100;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(0.0).extend(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let stage = Stage::new(device.clone(), queue.clone())
            // There are no lights in the scene and the material isn't marked as "unlit", so
            // let's force it to be unlit.
            .with_lighting(false);
        stage.configure_graph(&mut r, true);
        let (cpu_doc, gpu_doc) = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_013_SimpleTexture.gltf")
            .unwrap();

        let position = Vec3::new(0.5, 0.5, 1.25);
        let projection = crate::camera::perspective(size as f32, size as f32);
        let view = crate::camera::look_at(position, Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let camera = stage.append(&Camera {
            projection,
            view,
            position,
        });
        let _unit_ids = stage.draw_gltf_scene(&gpu_doc, camera, cpu_doc.default_scene().unwrap());

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_simple_texture.png", img);
    }

    #[test]
    fn normal_mapping_brick_sphere() {
        let size = 600;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(1.0).extend(1.0));
        let stage = r.new_stage().with_lighting(true).with_bloom(true);
        stage.configure_graph(&mut r, true);
        let (cpu_doc, gpu_doc) = stage
            .load_gltf_document_from_path("../../gltf/red_brick_03_1k.glb")
            .unwrap();
        let camera = stage.create_camera_from_gltf(&cpu_doc, 0).unwrap();
        let camera_id = stage.append(&camera);
        let _unit_ids =
            stage.draw_gltf_scene(&gpu_doc, camera_id, cpu_doc.default_scene().unwrap());

        let img = r.render_image().unwrap();
        println!("saving frame");
        img_diff::assert_img_eq("gltf_normal_mapping_brick_sphere.png", img);
    }
}
