//! Gltf support for the [`Stage`](crate::Stage).
use std::collections::HashMap;

use crabslab::{Array, GrowableSlab, Id, Slab};
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::{
    pbr::{
        light::{DirectionalLight, Light, LightStyle, PointLight, SpotLight},
        Material, PbrConfig,
    },
    slab::*,
    stage::Vertex,
    AtlasImage, AtlasTexture, Camera, NestedTransform, Renderlet, RepackPreview, Stage,
    TextureAddressMode, TextureModes, Transform,
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

    #[snafu(display("Missing material with index {index}"))]
    MissingMaterial { index: usize },

    #[snafu(display("Missing primitive with index {index}"))]
    MissingPrimitive { index: usize },

    #[snafu(display("Missing mesh with index {index}"))]
    MissingMesh { index: usize },

    #[snafu(display("Missing node with index {index}"))]
    MissingNode { index: usize },

    #[snafu(display("Missing light with index {index}"))]
    MissingLight { index: usize },

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

impl TextureAddressMode {
    fn from_gltf(mode: gltf::texture::WrappingMode) -> TextureAddressMode {
        match mode {
            gltf::texture::WrappingMode::ClampToEdge => TextureAddressMode::ClampToEdge,
            gltf::texture::WrappingMode::MirroredRepeat => TextureAddressMode::MirroredRepeat,
            gltf::texture::WrappingMode::Repeat => TextureAddressMode::Repeat,
        }
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

impl AtlasTexture {
    pub fn from_gltf(
        texture: gltf::Texture,
        repacking: &mut RepackPreview,
        atlas_offset: usize,
    ) -> Result<AtlasTexture, StageGltfError> {
        let image_index = texture.source().index();
        let mode_s = TextureAddressMode::from_gltf(texture.sampler().wrap_s());
        let mode_t = TextureAddressMode::from_gltf(texture.sampler().wrap_t());
        let (offset_px, size_px) =
            repacking
                .get_frame(image_index + atlas_offset)
                .context(MissingImageSnafu {
                    index: image_index,
                    offset: atlas_offset,
                })?;
        Ok(AtlasTexture {
            offset_px,
            size_px,
            modes: TextureModes {
                s: mode_s,
                t: mode_t,
            },
            atlas_index: (image_index + atlas_offset) as u32,
        })
    }
}

impl Material {
    /// Convert a [`gltf::Material`] to a [`Material`].
    pub fn from_gltf(
        material: gltf::Material,
        repacking: &mut RepackPreview,
        textures: Array<AtlasTexture>,
        atlas_offset: usize,
    ) -> Result<Material, StageGltfError> {
        let name = material.name().map(String::from);
        log::trace!("loading material {:?} {name:?}", material.index());
        let pbr = material.pbr_metallic_roughness();
        let material = if material.unlit() {
            log::trace!("  is unlit");
            let (albedo_texture, albedo_tex_coord) = if let Some(info) = pbr.base_color_texture() {
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
            let (albedo_texture, albedo_tex_coord) = if let Some(info) = pbr.base_color_texture() {
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
        Ok(material)
    }
}

#[derive(Debug)]
pub struct GltfPrimitive {
    pub indices: HybridArray<u32>,
    pub vertices: HybridArray<Vertex>,
    pub bounding_box: (Vec3, Vec3),
    pub material: Id<Material>,
}

impl GltfPrimitive {
    pub fn from_gltf(
        stage: &mut Stage,
        primitive: gltf::Primitive,
        buffer_data: &[gltf::buffer::Data],
        materials: &HybridArray<Material>,
    ) -> Self {
        let material = primitive
            .material()
            .index()
            .map(|index| materials.array().at(index))
            .unwrap_or_default();

        let reader = primitive.reader(|buffer| {
            let data = buffer_data.get(buffer.index())?;
            Some(data.0.as_slice())
        });

        let indices = reader
            .read_indices()
            .map(|is| {
                let indices = is.into_u32().collect::<Vec<_>>();
                assert_eq!(indices.len() % 3, 0, "indices do not form triangles");
                indices
            })
            .unwrap_or_default();

        let positions = reader
            .read_positions()
            .into_iter()
            .flat_map(|ps| ps.map(Vec3::from))
            .collect::<Vec<_>>();

        let uv0s = reader
            .read_tex_coords(0)
            .into_iter()
            .flat_map(|uvs| uvs.into_f32().map(Vec2::from))
            .chain(std::iter::repeat(Vec2::ZERO))
            .take(positions.len())
            .collect::<Vec<_>>();

        let uv1s = reader
            .read_tex_coords(0)
            .into_iter()
            .flat_map(|uvs| uvs.into_f32().map(Vec2::from))
            .chain(std::iter::repeat(Vec2::ZERO))
            .take(positions.len());

        let mut normals = vec![Vec3::Z; positions.len()];
        if let Some(ns) = reader.read_normals() {
            let ns = ns.map(Vec3::from).collect::<Vec<_>>();
            debug_assert_eq!(positions.len(), ns.len());
            normals = ns;
        } else {
            log::trace!("    generating normals");

            let indices = if indices.is_empty() {
                (0..positions.len() as u32).collect::<Vec<_>>()
            } else {
                indices.iter().copied().collect::<Vec<_>>()
            };

            indices.chunks(3).for_each(|chunk| match chunk {
                [i, j, k] => {
                    let a = positions[*i as usize];
                    let b = positions[*j as usize];
                    let c = positions[*k as usize];
                    let n = Vertex::generate_normal(a, b, c);
                    normals[*i as usize] = n;
                    normals[*j as usize] = n;
                    normals[*k as usize] = n;
                }
                _ => panic!("not triangles!"),
            });
        }

        let mut tangents = vec![Vec4::ZERO; positions.len()];
        if let Some(ts) = reader.read_tangents() {
            let ts = ts.map(Vec4::from).collect::<Vec<_>>();
            debug_assert_eq!(positions.len(), ts.len());
            tangents = ts;
        } else {
            log::trace!("    generating tangents");
            let indices = if indices.is_empty() {
                (0..positions.len() as u32).collect::<Vec<_>>()
            } else {
                indices.iter().copied().collect::<Vec<_>>()
            };

            indices.chunks(3).for_each(|chunk| match chunk {
                [i, j, k] => {
                    let a = positions[*i as usize];
                    let b = positions[*j as usize];
                    let c = positions[*k as usize];
                    let a_uv = uv0s[*i as usize];
                    let b_uv = uv0s[*j as usize];
                    let c_uv = uv0s[*k as usize];

                    let t = Vertex::generate_tangent(a, a_uv, b, b_uv, c, c_uv);
                    tangents[*i as usize] = t;
                    tangents[*j as usize] = t;
                    tangents[*k as usize] = t;
                }
                _ => panic!("not triangles!"),
            });
        }
        let colors = reader
            .read_colors(0)
            .into_iter()
            .flat_map(|cs| cs.into_rgba_f32().map(Vec4::from))
            .chain(std::iter::repeat(Vec4::ONE))
            .take(positions.len());

        let joints = reader
            .read_joints(0)
            .into_iter()
            .flat_map(|js| {
                js.into_u16()
                    .map(|[a, b, c, d]| [a as u32, b as u32, c as u32, d as u32])
            })
            .chain(std::iter::repeat([u32::MAX; 4]))
            .take(positions.len());

        let weights = reader
            .read_weights(0)
            .into_iter()
            .flat_map(|ws| ws.into_f32())
            .chain(std::iter::repeat([f32::MAX; 4]))
            .take(positions.len());
        let vs = joints.zip(weights);
        let vs = colors.zip(vs);
        let vs = tangents.into_iter().zip(vs);
        let vs = normals.into_iter().zip(vs);
        let vs = uv1s.zip(vs);
        let vs = uv0s.into_iter().zip(vs);
        let vs = positions.into_iter().zip(vs);
        let vertices = vs
            .map(
                |(position, (uv0, (uv1, (normal, (tangent, (color, (joints, weights)))))))| {
                    Vertex {
                        position,
                        color,
                        uv0,
                        uv1,
                        normal,
                        tangent,
                        joints,
                        weights,
                    }
                },
            )
            .collect::<Vec<_>>();
        let vertices = stage.new_hybrid_array(vertices);
        let indices = stage.new_hybrid_array(indices);
        let gltf::mesh::Bounds { min, max } = primitive.bounding_box();
        let min = Vec3::from_array(min);
        let max = Vec3::from_array(max);
        Self {
            vertices,
            indices,
            material,
            bounding_box: (min, max),
        }
    }
}

#[derive(Debug)]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
    pub weights: HybridArray<f32>,
}

impl GltfMesh {
    fn from_gltf(
        stage: &mut Stage,
        buffer_data: &[gltf::buffer::Data],
        materials: &HybridArray<Material>,
        mesh: gltf::Mesh,
    ) -> Self {
        log::debug!("Loading primitives for mesh {}", mesh.index());
        let primitives = mesh
            .primitives()
            .map(|prim| GltfPrimitive::from_gltf(stage, prim, buffer_data, &materials))
            .collect::<Vec<_>>();
        log::trace!("  loaded {} primitives", primitives.len());
        let weights = mesh.weights().unwrap_or(&[]).iter().copied();
        GltfMesh {
            primitives,
            weights: stage.new_hybrid_array(weights),
        }
    }
}

#[derive(Debug)]
pub struct GltfCamera {
    pub index: usize,
    pub name: Option<String>,
    pub node_transform: NestedTransform,
    projection: Mat4,
    pub camera: Hybrid<Camera>,
}

impl<'a> GltfCamera {
    fn new(stage: &mut Stage, gltf_camera: gltf::Camera<'a>, transform: &NestedTransform) -> Self {
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
        let view = Mat4::from(transform.get_global_transform()).inverse();
        let camera = stage.new_hybrid(Camera::new(projection, view));
        GltfCamera {
            index: gltf_camera.index(),
            name: gltf_camera.name().map(String::from),
            projection,
            node_transform: transform.clone(),
            camera,
        }
    }

    pub fn get_camera(&self) -> Camera {
        let view = Mat4::from(self.node_transform.get_global_transform()).inverse();
        Camera::new(self.projection, view)
    }
}

#[derive(Clone, Debug)]
pub enum LightDetails {
    Directional(Hybrid<DirectionalLight>),
    Point(Hybrid<PointLight>),
    Spot(Hybrid<SpotLight>),
}

#[derive(Debug)]
pub struct GltfLight {
    pub details: LightDetails,
    pub node_transform: NestedTransform,
    pub light: Hybrid<Light>,
}

/// A node in a GLTF document, ready to be 'drawn'.
#[derive(Clone, Debug)]
pub struct GltfNode {
    /// Index of this node in the `StagedGltfDocument`'s `nodes` field.
    pub index: usize,
    /// Name of the node, if any,
    pub name: Option<String>,
    /// Id of the light this node refers to.
    pub light: Option<usize>,
    /// Index of the mesh in the document's meshes, if any.
    pub mesh: Option<usize>,
    /// Index into the cameras array, if any.
    pub camera: Option<usize>,
    /// Index of the skin in the document's skins, if any.
    pub skin: Option<usize>,
    /// Indices of the children of this node.
    ///
    /// Each element indexes into the `StagedGltfDocument`'s `nodes` field.
    pub children: Vec<usize>,
    /// Array of weights
    pub weights: HybridArray<f32>,
    /// This node's transform.
    pub transform: NestedTransform,
}

impl GltfNode {
    pub fn global_transform(&self) -> Transform {
        todo!()
    }
}

#[derive(Debug)]
pub struct GltfDocument {
    pub nodes: Vec<GltfNode>,
    pub cameras: Vec<GltfCamera>,
    pub meshes: Vec<GltfMesh>,
    pub default_material: Hybrid<Material>,
    pub materials: HybridArray<Material>,
    /// Vector of scenes - each being a list of nodes.
    pub scenes: Vec<Vec<usize>>,
    pub default_scene: Option<usize>,
    pub textures: Array<AtlasTexture>,
    pub lights: Vec<GltfLight>,
}

impl GltfDocument {
    pub fn from_gltf(
        stage: &mut Stage,
        document: &gltf::Document,
        buffer_data: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<GltfDocument, StageGltfError> {
        log::debug!("Loading nodes");
        let mut nodes = vec![];
        let mut node_transforms = HashMap::<usize, NestedTransform>::new();
        fn transform_for_node(
            stage: &mut Stage,
            cache: &mut HashMap<usize, NestedTransform>,
            node: &gltf::Node,
        ) -> NestedTransform {
            if let Some(nt) = cache.get(&node.index()) {
                nt.clone()
            } else {
                let mut transform = stage.new_nested_transform();
                let mat4 = Mat4::from_cols_array_2d(&node.transform().matrix());
                transform.set_local_transform(mat4.into());
                for node in node.children() {
                    let child_transform = transform_for_node(stage, cache, &node);
                    transform.add_child(&child_transform);
                }
                cache.insert(node.index(), transform.clone());
                transform
            }
        }
        let mut camera_index_to_node_index = HashMap::<usize, usize>::new();
        let mut light_index_to_node_index = HashMap::<usize, usize>::new();
        for (i, node) in document.nodes().enumerate() {
            let node_index = node.index();
            if let Some(camera) = node.camera() {
                camera_index_to_node_index.insert(camera.index(), node_index);
            }
            if let Some(light) = node.light() {
                light_index_to_node_index.insert(light.index(), node_index);
            }

            debug_assert_eq!(i, node_index);
            let children = node.children().map(|node| node.index()).collect::<Vec<_>>();
            let mesh = node.mesh().map(|mesh| mesh.index());
            let skin = node.skin().map(|skin| skin.index());
            let camera = node.camera().map(|camera| camera.index());
            let light = node.light().map(|light| light.index());
            let weights = node.weights().map(|w| w.to_vec()).unwrap_or_default();
            let weights = stage.new_hybrid_array(weights);
            let transform = transform_for_node(stage, &mut node_transforms, &node);
            nodes.push(GltfNode {
                index: node.index(),
                name: node.name().map(String::from),
                light,
                mesh,
                camera,
                skin,
                children,
                weights,
                transform,
            });
        }
        log::trace!("  loaded {} nodes", nodes.len());

        log::trace!("Loading cameras");
        let mut cameras = vec![];
        for camera in document.cameras() {
            let camera_index = camera.index();
            let node_index =
                *camera_index_to_node_index
                    .get(&camera_index)
                    .context(MissingCameraSnafu {
                        index: camera_index,
                    })?;
            let transform = node_transforms
                .get(&camera_index)
                .context(MissingNodeSnafu { index: node_index })?;
            cameras.push(GltfCamera::new(stage, camera, transform));
        }

        // We need the (re)packing of the atlas before we marshal the images into the
        // GPU because we need their frames for textures and materials, but we
        // need to know if the materials require us to apply a linear transfer.
        // So we'll get the preview repacking first, then update the frames in
        // the textures.
        let (mut repacking, atlas_offset) = {
            // UNWRAP: if we can't lock the atlas, we want to panic.
            let atlas = stage.atlas.read().unwrap();
            let atlas_offset = atlas.rects.len();
            (
                atlas
                    .repack_preview(&stage.device, images.into_iter().map(AtlasImage::from))
                    .context(AtlasSnafu)?,
                atlas_offset,
            )
        };

        log::debug!("Creating atlas textures");
        let textures = stage.allocate_array::<AtlasTexture>(document.textures().len());
        for (i, texture) in document.textures().enumerate() {
            let texture = AtlasTexture::from_gltf(texture, &mut repacking, atlas_offset)?;
            let texture_id = textures.at(i);
            log::trace!("  texture {i} {texture_id:?}: {texture:#?}");
            stage.write(texture_id, &texture);
        }

        log::debug!("Creating materials");
        let default_material = stage.new_hybrid(Material::default());
        let mut materials = vec![];
        for gltf_material in document.materials() {
            let material_index = gltf_material.index();
            let material =
                Material::from_gltf(gltf_material, &mut repacking, textures, atlas_offset)?;
            if let Some(index) = material_index {
                log::trace!("  created material {index}");
                debug_assert_eq!(index, materials.len(), "unexpected material index");
                materials.push(material);
            } else {
                log::trace!("  created default material");
                default_material.set(material);
            }
        }
        let materials = stage.new_hybrid_array(materials);
        log::trace!("  created {} materials", materials.len());

        let number_of_new_images = repacking.new_images_len();
        if number_of_new_images > 0 {
            log::trace!("Packing the atlas");
            log::trace!("  adding {number_of_new_images} new images",);
            // UNWRAP: if we can't lock the atlas, we want to panic.
            let mut atlas = stage.atlas.write().unwrap();
            let new_atlas = atlas
                .commit_repack_preview(&stage.device, &stage.queue, repacking)
                .context(AtlasSnafu)?;
            let size = new_atlas.size;
            *atlas = new_atlas;
            // The bindgroup will have to be remade
            let _ = stage.textures_bindgroup.lock().unwrap().take();
            // The atlas size must be reset
            stage.pbr_config.modify(|cfg| cfg.atlas_size = size);
        }

        log::debug!("Loading meshes");
        let mut meshes = vec![];
        for mesh in document.meshes() {
            let mesh = GltfMesh::from_gltf(stage, &buffer_data, &materials, mesh);
            meshes.push(mesh);
        }
        log::trace!("  loaded {} meshes", meshes.len());

        log::trace!("Loading lights");
        let mut lights = vec![];
        if let Some(gltf_lights) = document.lights() {
            for gltf_light in gltf_lights {
                let color = Vec3::from(gltf_light.color()).extend(1.0);
                let intensity = gltf_light.intensity();
                let (mut light, details): (Light, _) = match gltf_light.kind() {
                    gltf::khr_lights_punctual::Kind::Directional => {
                        let light = stage.new_hybrid(DirectionalLight {
                            direction: Vec3::NEG_Z,
                            color,
                            intensity,
                        });

                        (light.id().into(), LightDetails::Directional(light))
                    }
                    gltf::khr_lights_punctual::Kind::Point => {
                        let light = stage.new_hybrid(PointLight {
                            position: Vec3::ZERO,
                            color,
                            intensity,
                        });
                        (light.id().into(), LightDetails::Point(light))
                    }
                    gltf::khr_lights_punctual::Kind::Spot {
                        inner_cone_angle,
                        outer_cone_angle,
                    } => {
                        let light = stage.new_hybrid(SpotLight {
                            position: Vec3::ZERO,
                            direction: Vec3::NEG_Z,
                            inner_cutoff: inner_cone_angle,
                            outer_cutoff: outer_cone_angle,
                            color,
                            intensity,
                        });
                        (light.id().into(), LightDetails::Spot(light))
                    }
                };
                let node_index = *light_index_to_node_index.get(&gltf_light.index()).context(
                    MissingCameraSnafu {
                        index: gltf_light.index(),
                    },
                )?;
                let node_transform = node_transforms
                    .get(&node_index)
                    .context(MissingNodeSnafu { index: node_index })?
                    .clone();
                light.transform = node_transform.global_transform_id();

                let light = stage.new_hybrid(light);
                lights.push(GltfLight {
                    details,
                    node_transform,
                    light,
                });
            }
        }

        //let skins = self.allocate_array::<GltfSkin>(document.skins().len());

        // TODO: GLTF skinning
        // log::trace!("Loading skins");
        // for skin in document.skins() {
        //     let skin_index = skin.index();
        //     let joints = skin
        //         .joints()
        //         .map(|node| nodes.at(node.index()))
        //         .collect::<Vec<_>>();
        //     let joints = self.append_array(&joints);
        //     let inverse_bind_matrices = skin
        //         .inverse_bind_matrices()
        //         .map(|acc| accessors.at(acc.index()))
        //         .unwrap_or_default();
        //     let skeleton = skin
        //         .skeleton()
        //         .map(|node| nodes.at(node.index()))
        //         .unwrap_or_default();
        //     let _ = self.write(
        //         skins.at(skin_index),
        //         &GltfSkin {
        //             joints,
        //             inverse_bind_matrices,
        //             skeleton,
        //         },
        //     );
        // }

        // log::trace!("Loading animations");
        // let animations = self.allocate_array::<GltfAnimation>(document.animations().count());
        // for animation in document.animations() {
        //     let samplers =
        //         self.allocate_array::<GltfAnimationSampler>(animation.samplers().count());
        //     fn create_sampler(
        //         accessors: Array<GltfAccessor>,
        //         sampler: gltf::animation::Sampler<'_>,
        //     ) -> GltfAnimationSampler {
        //         let interpolation = match sampler.interpolation() {
        //             gltf::animation::Interpolation::Linear => GltfInterpolation::Linear,
        //             gltf::animation::Interpolation::Step => GltfInterpolation::Step,
        //             gltf::animation::Interpolation::CubicSpline => GltfInterpolation::CubicSpline,
        //         };
        //         let input = accessors.at(sampler.input().index());
        //         let output = accessors.at(sampler.output().index());
        //         GltfAnimationSampler {
        //             interpolation,
        //             input,
        //             output,
        //         }
        //     }
        //     let mut stored_samplers = vec![];
        //     for (i, sampler) in animation.samplers().enumerate() {
        //         let sampler = create_sampler(accessors, sampler);
        //         self.write(samplers.at(i), &sampler);
        //         // Store it later so we can figure out the index of the sampler
        //         // used by the channel.
        //         //
        //         // TODO: Remove `stored_samplers` once `gltf` provides `.index()`
        //         // @see https://github.com/gltf-rs/gltf/issues/398
        //         stored_samplers.push(sampler);
        //     }
        //     let channels = self.allocate_array::<GltfChannel>(animation.channels().count());
        //     for (i, channel) in animation.channels().enumerate() {
        //         let target = channel.target();
        //         let node = nodes.at(target.node().index());
        //         let property = match target.property() {
        //             gltf::animation::Property::Translation => GltfTargetProperty::Translation,
        //             gltf::animation::Property::Rotation => GltfTargetProperty::Rotation,
        //             gltf::animation::Property::Scale => GltfTargetProperty::Scale,
        //             gltf::animation::Property::MorphTargetWeights => {
        //                 GltfTargetProperty::MorphTargetWeights
        //             }
        //         };
        //         let target = GltfTarget { node, property };
        //         let sampler = create_sampler(accessors, channel.sampler());
        //         let index = stored_samplers
        //             .iter()
        //             .position(|s| s == &sampler)
        //             .context(MissingSamplerSnafu)?;
        //         let sampler = samplers.at(index);
        //         self.write(channels.at(i), &GltfChannel { target, sampler });
        //     }
        //     self.write(
        //         animations.at(animation.index()),
        //         &GltfAnimation { channels, samplers },
        //     );
        // }

        log::debug!("Loading scenes");
        let scenes = document
            .scenes()
            .map(|scene| scene.nodes().map(|node| node.index()).collect())
            .collect();

        log::trace!("Done loading gltf");

        Ok(GltfDocument {
            //animations,
            lights,
            cameras,
            materials,
            default_material,
            meshes,
            nodes,
            scenes,
            default_scene: document.default_scene().map(|scene| scene.index()),
            textures,
        })
    }
}

#[derive(Debug)]
pub struct Node {
    pub gltf_node: GltfNode,
    pub renderlets: Vec<Hybrid<Renderlet>>,
}

impl Stage {
    pub fn load_gltf_document_from_path(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<GltfDocument, StageGltfError> {
        let (document, buffers, images) = gltf::import(path)?;
        GltfDocument::from_gltf(self, &document, buffers, images)
    }

    /// Draws the `StagedGltfNode` with the given `Camera`.
    pub fn draw_gltf_node(
        &mut self,
        doc: &GltfDocument,
        node_index: usize,
        camera: Id<Camera>,
    ) -> Result<Node, StageGltfError> {
        let gltf_node = doc
            .nodes
            .get(node_index)
            .context(MissingNodeSnafu { index: node_index })?
            .clone();

        log::trace!("drawing GLTF node {node_index} {:?}", gltf_node.name);

        let mut renderlets = vec![];
        if let Some(mesh_index) = gltf_node.mesh {
            let mesh = doc
                .meshes
                .get(mesh_index)
                .context(MissingMeshSnafu { index: mesh_index })?;
            for prim in mesh.primitives.iter() {
                let hybrid = self.draw(Renderlet {
                    vertices: prim.vertices.array(),
                    indices: prim.indices.array(),
                    camera,
                    transform: gltf_node.transform.global_transform_id(),
                    material: prim.material,
                    ..Default::default()
                });
                renderlets.push(hybrid);
            }
        }

        Ok(Node {
            gltf_node,
            renderlets,
        })
    }

    pub fn draw_gltf_scene(
        &mut self,
        doc: &GltfDocument,
        // Any list of nodes makes a scene.
        nodes: impl IntoIterator<Item = usize>,
        camera: Id<Camera>,
    ) -> Result<Vec<Node>, StageGltfError> {
        let mut scene_nodes = vec![];
        for node_index in nodes.into_iter() {
            let node = self.draw_gltf_node(doc, node_index, camera)?;
            scene_nodes.push(node);
        }
        Ok(scene_nodes)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        pbr::{Material, PbrConfig},
        stage::Vertex,
        Camera, Renderlet, Renderling, Stage, Transform,
    };
    use crabslab::{Array, GrowableSlab, Id, Slab, SlabItem};
    use glam::{UVec2, Vec2, Vec3, Vec4, Vec4Swizzles};

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
    // ensures we can
    // * read simple meshes
    // * support multiple nodes that reference the same mesh
    // * support primitives w/ positions and normal attributes
    // * support transforming nodes (T * R * S)
    fn stage_gltf_simple_meshes() {
        let mut r =
            Renderling::headless(100, 50).with_background_color(Vec3::splat(0.0).extend(1.0));
        let projection = crate::camera::perspective(100.0, 50.0);
        let position = Vec3::new(1.0, 0.5, 1.5);
        let view = crate::camera::look_at(position, Vec3::new(1.0, 0.5, 0.0), Vec3::Y);
        let mut stage = r.new_stage().with_lighting(false);
        stage.configure_graph(&mut r, true);
        let mut doc = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_008_SimpleMeshes.gltf")
            .unwrap();
        println!("doc: {doc:#?}");
        let camera = Camera {
            projection,
            view,
            position,
        };
        let camera_id = stage.append(&camera);
        let scene_index = doc.default_scene.unwrap();
        let scene_nodes = doc.scenes.get(scene_index).unwrap().clone();
        let scene = stage
            .draw_gltf_scene(&mut doc, scene_nodes, camera_id)
            .unwrap();
        println!("scene: {scene:#?}");

        // let default_scene = document.default_scene().unwrap();
        // let unit_ids = stage.draw_gltf_scene(&gpu_doc, camera_id, default_scene);
        // assert_eq!(2, unit_ids.len());

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_simple_meshes.png", img);
    }

    #[test]
    // Ensures we can read a minimal gltf file with a simple triangle mesh.
    fn minimal_mesh() {
        let mut r =
            Renderling::headless(20, 20).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r.new_stage().with_lighting(false);
        stage.configure_graph(&mut r, true);
        let mut doc = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_003_MinimalGltfFile.gltf")
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
        let default_scene = doc.default_scene.unwrap();
        let nodes = doc.scenes.get(default_scene).unwrap().clone();
        let scene = stage.draw_gltf_scene(&mut doc, nodes, camera_id).unwrap();
        for (i, node) in scene.into_iter().enumerate() {
            println!("node_{i}: {node:#?}");
        }

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_minimal_mesh.png", img);
    }

    #[test]
    // Tests importing a gltf file and rendering the first image as a 2d object.
    //
    // This ensures we are decoding images correctly.
    fn gltf_images() {
        let mut r = Renderling::headless(100, 100).with_background_color(Vec4::splat(1.0));
        let mut stage = r.new_stage().with_lighting(false);
        stage.configure_graph(&mut r, true);
        let doc = stage
            .load_gltf_document_from_path("../../gltf/cheetah_cone.glb")
            .unwrap();
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let camera_id = stage.append(&Camera::new(projection, view));
        assert!(!doc.textures.is_empty());
        let albedo_texture_id = doc.textures.at(0);
        assert!(albedo_texture_id.is_some());
        let material_id = stage.append(&Material {
            albedo_texture: albedo_texture_id,
            has_lighting: false,
            ..Default::default()
        });
        println!("material_id: {:#?}", material_id);
        let vertices = stage.append_array(&[
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
        let indices = stage.append_array(&[0u32, 3, 2, 0, 2, 1]);
        let transform = stage.append(&Transform {
            scale: Vec3::new(100.0, 100.0, 1.0),
            ..Default::default()
        });
        let _renderlet = stage.draw(Renderlet {
            vertices,
            indices,
            material: material_id,
            transform,
            camera: camera_id,
            ..Default::default()
        });

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("gltf_images.png", img);
    }

    #[test]
    fn simple_texture() {
        let size = 100;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r
            .new_stage()
            // There are no lights in the scene and the material isn't marked as "unlit", so
            // let's force it to be unlit.
            .with_lighting(false);
        stage.configure_graph(&mut r, true);
        let mut doc = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_013_SimpleTexture.gltf")
            .unwrap();

        let projection = crate::camera::perspective(size as f32, size as f32);
        let view =
            crate::camera::look_at(Vec3::new(0.5, 0.5, 1.25), Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let camera = stage.append(&Camera::new(projection, view));
        let default_scene_index = doc.default_scene.unwrap();
        let nodes = doc.scenes.get(default_scene_index).unwrap().clone();
        let _scene = stage.draw_gltf_scene(&mut doc, nodes, camera).unwrap();

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_simple_texture.png", img);
    }

    #[test]
    // Demonstrates how to load and render a gltf file containing lighting and a
    // normal map.
    fn normal_mapping_brick_sphere() {
        let size = 600;
        let mut r =
            Renderling::headless(size, size).with_background_color(Vec3::splat(1.0).extend(1.0));
        let mut stage = r.new_stage().with_lighting(true).with_bloom(true);
        stage.configure_graph(&mut r, true);
        let doc = stage
            .load_gltf_document_from_path("../../gltf/red_brick_03_1k.glb")
            .unwrap();
        let gltf_camera = doc.cameras.get(0).unwrap();
        let scene_index = doc.default_scene.unwrap();
        let nodes = doc.scenes.get(scene_index).unwrap().iter().copied();
        let _scene = stage.draw_gltf_scene(&doc, nodes, gltf_camera.camera.id());

        stage.set_lights(doc.lights.iter().map(|gltf_light| gltf_light.light.id()));

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("gltf_normal_mapping_brick_sphere.png", img);
    }

    /// A helper struct that contains all outputs of the vertex shader.
    #[allow(unused)]
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GltfVertexInvocation {
        pub instance_index: u32,
        pub vertex_index: u32,
        pub renderlet_id: Id<Renderlet>,
        pub renderlet: Renderlet,
        pub out_camera: Id<Camera>,
        pub out_material: Id<Material>,
        pub out_pbr_config: Id<PbrConfig>,
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
        pub fn invoke(
            instance_index: u32,
            vertex_index: u32,
            slab: &[u32],
            debug: &mut [u32],
        ) -> Self {
            let mut v = Self {
                instance_index,
                vertex_index,
                ..Default::default()
            };
            v.renderlet_id = Id::from(v.instance_index);
            v.renderlet = slab.read(v.renderlet_id);
            crate::stage::renderlet_vertex(
                v.renderlet_id,
                v.vertex_index,
                slab,
                debug,
                &mut v.out_camera,
                &mut v.out_material,
                &mut v.out_pbr_config,
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
