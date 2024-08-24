//! Gltf support for the [`Stage`](crate::Stage).
use std::collections::HashMap;

use crabslab::{Array, Id};
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use rustc_hash::{FxHashMap, FxHashSet};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::{
    atlas::{AtlasError, AtlasFrame, AtlasImage, AtlasTexture, TextureAddressMode, TextureModes},
    camera::Camera,
    pbr::{
        light::{DirectionalLight, Light, LightStyle, PointLight, SpotLight},
        Material,
    },
    slab::*,
    stage::{NestedTransform, Renderlet, Skin, Stage, Vertex},
    transform::Transform,
};

mod anime;
pub use anime::*;

#[derive(Debug, Snafu)]
pub enum StageGltfError {
    #[snafu(display("{source}"))]
    Stage { source: crate::stage::StageError },

    #[snafu(display("{source}"))]
    Gltf { source: gltf::Error },

    #[snafu(display("{source}"))]
    Atlas { source: crate::atlas::AtlasError },

    #[snafu(display("Missing image at index {index}"))]
    MissingImage { index: usize },

    #[snafu(display("Wrong image at index {index} atlas offset {offset}"))]
    WrongImage { offset: usize, index: usize },

    #[snafu(display("Missing atlas image frame {index}"))]
    MissingFrame { index: usize },

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

    #[snafu(display("Node has no skin"))]
    NoSkin,

    #[snafu(display("Missing gltf skin at index {index}"))]
    MissingSkin { index: usize },

    #[snafu(display("{source}"))]
    Animation { source: anime::AnimationError },
}

impl From<gltf::Error> for StageGltfError {
    fn from(source: gltf::Error) -> Self {
        Self::Gltf { source }
    }
}

impl From<AtlasError> for StageGltfError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
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
    } else if let Some(positions) = primitive.get(&gltf::Semantic::Positions) {
        let count = positions.count() as u32;
        log::trace!("    has {count} positions");
        count
    } else {
        log::trace!("    has no indices nor positions");
        0
    }
}

impl Material {
    pub fn from_gltf(
        material: gltf::Material,
        textures: Array<AtlasTexture>,
        images: &mut [AtlasImage],
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
                let image = images
                    .get_mut(image_index)
                    .context(MissingImageSnafu { index: image_index })?;
                image.apply_linear_transfer = true;
                (tex_id, info.tex_coord())
            } else {
                (Id::NONE, 0)
            };

            Material {
                albedo_texture_id: albedo_texture,
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
                let image = images
                    .get_mut(image_index)
                    .context(MissingImageSnafu { index: image_index })?;
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
                    let image = images
                        .get_mut(image_index)
                        .context(MissingImageSnafu { index: image_index })?;
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
                albedo_texture_id: albedo_texture,
                metallic_roughness_texture_id: metallic_roughness_texture,
                normal_texture_id: normal_texture,
                ao_texture_id: ao_texture,
                albedo_tex_coord,
                metallic_roughness_tex_coord,
                normal_tex_coord,
                ao_tex_coord,
                ao_strength,
                emissive_factor,
                emissive_strength_multiplier,
                emissive_texture_id: emissive_texture,
                emissive_tex_coord,
                has_lighting: true,
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
                indices.to_vec()
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
                indices.to_vec()
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
        let joints = joints.collect::<Vec<_>>();
        let mut all_joints = FxHashSet::default();
        for js in joints.iter() {
            all_joints.extend(*js);
        }
        log::debug!("  joints: {all_joints:?}");

        const UNWEIGHTED_WEIGHTS: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let mut logged_weights_not_f32 = false;
        let weights = reader
            .read_weights(0)
            .into_iter()
            .flat_map(|ws| {
                if !logged_weights_not_f32 {
                    match ws {
                        gltf::mesh::util::ReadWeights::U8(_) => log::warn!("weights are u8"),
                        gltf::mesh::util::ReadWeights::U16(_) => log::warn!("weights are u16"),
                        gltf::mesh::util::ReadWeights::F32(_) => {}
                    }
                    logged_weights_not_f32 = true;
                }
                ws.into_f32().map(|weights| {
                    // normalize the weights
                    let sum = weights[0] + weights[1] + weights[2] + weights[3];
                    weights.map(|w| w / sum)
                })
            })
            .chain(std::iter::repeat(UNWEIGHTED_WEIGHTS))
            .take(positions.len());
        let vs = joints.into_iter().zip(weights);
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
        let vertices = stage.new_array(vertices);
        log::debug!("{} vertices, {:?}", vertices.len(), vertices.array());
        let indices = stage.new_array(indices);
        log::debug!("{} indices, {:?}", indices.len(), indices.array());
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
            .map(|prim| GltfPrimitive::from_gltf(stage, prim, buffer_data, materials))
            .collect::<Vec<_>>();
        log::trace!("  loaded {} primitives", primitives.len());
        let weights = mesh.weights().unwrap_or(&[]).iter().copied();
        GltfMesh {
            primitives,
            weights: stage.new_array(weights),
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
        let camera = stage.new_value(Camera::new(projection, view));
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
    /// Each element indexes into the `GltfDocument`'s `nodes` field.
    pub children: Vec<usize>,
    /// Array of weights
    pub weights: HybridArray<f32>,
    /// This node's transform.
    pub transform: NestedTransform,
}

impl From<&GltfNode> for (usize, NestedTransform) {
    fn from(node: &GltfNode) -> Self {
        (node.index, node.transform.clone())
    }
}

impl GltfNode {
    pub fn global_transform(&self) -> Transform {
        self.transform.get_global_transform()
    }
}

#[derive(Clone, Debug)]
pub struct GltfSkin {
    pub index: usize,
    // Indices of the skeleton nodes used as joints in this skin, unused internally
    // but possibly useful.
    pub joint_nodes: Vec<usize>,
    pub joint_transforms: HybridArray<Id<Transform>>,
    // Containins the 4x4 inverse-bind matrices.
    //
    // When None, each matrix is assumed to be the 4x4 identity matrix which implies that the
    // inverse-bind matrices were pre-applied.
    pub inverse_bind_matrices: Option<HybridArray<Mat4>>,
    // Index of the node used as the skeleton root.
    // When None, joints transforms resolve to scene root.
    pub skeleton: Option<usize>,
    // Skin as seen by shaders, on the GPU
    pub skin: Hybrid<Skin>,
}

impl GltfSkin {
    pub fn from_gltf(
        stage: &mut Stage,
        buffer_data: &[gltf::buffer::Data],
        nodes: &[GltfNode],
        skin: gltf::Skin,
    ) -> Result<Self, StageGltfError> {
        log::debug!("reading skin {} {:?}", skin.index(), skin.name());
        let joint_nodes = skin.joints().map(|n| n.index()).collect::<Vec<_>>();
        log::debug!("  has {} joints", joint_nodes.len());
        let mut joint_transforms = vec![];
        for node_index in joint_nodes.iter() {
            let gltf_node: &GltfNode = nodes
                .get(*node_index)
                .context(MissingNodeSnafu { index: *node_index })?;
            let transform_id = gltf_node.transform.global_transform_id();
            log::debug!("    joint node {node_index} is {transform_id:?}");
            joint_transforms.push(transform_id);
        }
        let joint_transforms = stage.new_array(joint_transforms);
        let reader = skin.reader(|b| buffer_data.get(b.index()).map(|d| d.0.as_slice()));
        let inverse_bind_matrices = if let Some(mats) = reader.read_inverse_bind_matrices() {
            let invs = mats
                .into_iter()
                .map(|m| Mat4::from_cols_array_2d(&m))
                .collect::<Vec<_>>();
            log::debug!("  has {} inverse bind matrices", invs.len());
            Some(stage.new_array(invs))
        } else {
            log::debug!("  no inverse bind matrices");
            None
        };
        let skeleton = if let Some(n) = skin.skeleton() {
            let index = n.index();
            log::debug!("  skeleton is node {index}, {:?}", n.name());
            Some(index)
        } else {
            log::debug!("  skeleton is assumed to be the scene root");
            None
        };
        Ok(GltfSkin {
            index: skin.index(),
            skin: stage.new_value(Skin {
                joints: joint_transforms.array(),
                inverse_bind_matrices: inverse_bind_matrices
                    .as_ref()
                    .map(|a| a.array())
                    .unwrap_or_default(),
            }),
            joint_nodes,
            joint_transforms,
            inverse_bind_matrices,
            skeleton,
        })
    }
}

#[derive(Debug)]
pub struct GltfDocument {
    pub animations: Vec<Animation>,
    pub cameras: Vec<GltfCamera>,
    pub default_material: Hybrid<Material>,
    pub default_scene: Option<usize>,
    pub extensions: Option<serde_json::Value>,
    pub frames: Vec<Hybrid<AtlasFrame>>,
    pub lights: Vec<GltfLight>,
    pub meshes: Vec<GltfMesh>,
    pub nodes: Vec<GltfNode>,
    pub materials: HybridArray<Material>,
    // map of node index to renderlets
    pub renderlets: FxHashMap<usize, Vec<Hybrid<Renderlet>>>,
    /// Vector of scenes - each being a list of nodes.
    pub scenes: Vec<Vec<usize>>,
    pub skins: Vec<GltfSkin>,
    pub textures: HybridArray<AtlasTexture>,
}

impl GltfDocument {
    pub fn from_gltf(
        stage: &mut Stage,
        document: &gltf::Document,
        buffer_data: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
        // Camera id to use for any created Renderlets
        camera_id: Id<Camera>,
    ) -> Result<GltfDocument, StageGltfError> {
        let mut images = images.into_iter().map(AtlasImage::from).collect::<Vec<_>>();

        let num_textures = document.textures().len();
        log::debug!("Preallocating an array of {num_textures} textures");
        let textures = stage.new_array(vec![AtlasTexture::default(); num_textures]);

        log::debug!("Creating materials");
        let default_material = stage.new_value(Material::default());
        let mut materials = vec![];
        for gltf_material in document.materials() {
            let material_index = gltf_material.index();
            let material = Material::from_gltf(gltf_material, textures.array(), &mut images)?;
            if let Some(index) = material_index {
                log::trace!("  created material {index}");
                debug_assert_eq!(index, materials.len(), "unexpected material index");
                materials.push(material);
            } else {
                log::trace!("  created default material");
                default_material.set(material);
            }
        }
        let materials = stage.new_array(materials);
        log::trace!("  created {} materials", materials.len());

        log::debug!("Loading {} images into the atlas", images.len());
        let frames = stage.add_images(images).context(StageSnafu)?;

        log::debug!("Writing textures");
        for (i, texture) in document.textures().enumerate() {
            let index = texture.index();
            let frame = frames.get(index).context(MissingFrameSnafu { index })?;
            let texture = AtlasTexture {
                frame_id: frame.id(),
                modes: TextureModes {
                    s: TextureAddressMode::from_gltf(texture.sampler().wrap_s()),
                    t: TextureAddressMode::from_gltf(texture.sampler().wrap_t()),
                },
            };
            let texture_id = textures.array().at(i);
            log::trace!("  texture {i} {texture_id:?}: {texture:#?}");
            textures.set_item(i, texture);
        }

        log::debug!("Loading {} nodes", document.nodes().count());
        let mut nodes = vec![];
        let mut node_transforms = HashMap::<usize, NestedTransform>::new();

        fn transform_for_node(
            nesting_level: usize,
            stage: &mut Stage,
            cache: &mut HashMap<usize, NestedTransform>,
            node: &gltf::Node,
        ) -> NestedTransform {
            let padding = std::iter::repeat(" ")
                .take(nesting_level * 2)
                .collect::<Vec<_>>()
                .join("");
            let nt = if let Some(nt) = cache.get(&node.index()) {
                nt.clone()
            } else {
                let transform = stage.new_nested_transform();
                let (translation, rotation, scale) = &node.transform().decomposed();
                let t = Transform {
                    translation: Vec3::from_array(*translation),
                    rotation: Quat::from_array(*rotation),
                    scale: Vec3::from_array(*scale),
                };
                transform.set(t);
                for node in node.children() {
                    let child_transform =
                        transform_for_node(nesting_level + 1, stage, cache, &node);
                    transform.add_child(&child_transform);
                }
                cache.insert(node.index(), transform.clone());
                transform
            };
            let t = nt.get();
            log::trace!(
                "{padding}{} {:?} {:?} {:?} {:?}",
                node.index(),
                node.name(),
                t.translation,
                t.rotation,
                t.scale
            );
            nt
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
            let weights = stage.new_array(weights);
            let transform = transform_for_node(0, stage, &mut node_transforms, &node);
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
                        let light = stage.new_value(DirectionalLight {
                            direction: Vec3::NEG_Z,
                            color,
                            intensity,
                        });

                        (light.id().into(), LightDetails::Directional(light))
                    }
                    gltf::khr_lights_punctual::Kind::Point => {
                        let light = stage.new_value(PointLight {
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
                        let light = stage.new_value(SpotLight {
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

                let light = stage.new_value(light);
                lights.push(GltfLight {
                    details,
                    node_transform,
                    light,
                });
            }
        }

        log::trace!("Loading skins");
        let mut skins = vec![];
        for skin in document.skins() {
            skins.push(GltfSkin::from_gltf(stage, &buffer_data, &nodes, skin)?);
        }

        log::trace!("Loading animations");
        let mut animations = vec![];
        for animation in document.animations() {
            animations.push(Animation::from_gltf(&buffer_data, animation).context(AnimationSnafu)?);
        }

        log::debug!("Loading scenes");
        let scenes = document
            .scenes()
            .map(|scene| scene.nodes().map(|node| node.index()).collect())
            .collect();

        log::debug!("Creating renderlets");
        let mut renderlets = FxHashMap::default();
        for gltf_node in nodes.iter() {
            let mut node_renderlets = vec![];
            let skin_id = if let Some(skin_index) = gltf_node.skin {
                log::debug!("  node {} {:?} has skin", gltf_node.index, gltf_node.name);
                let gltf_skin = skins
                    .get(skin_index)
                    .context(MissingSkinSnafu { index: skin_index })?;
                gltf_skin.skin.id()
            } else {
                Id::NONE
            };

            if let Some(mesh_index) = gltf_node.mesh {
                log::debug!(
                    "  node {} {:?} has mesh {mesh_index}",
                    gltf_node.index,
                    gltf_node.name
                );
                let mesh = meshes
                    .get(mesh_index)
                    .context(MissingMeshSnafu { index: mesh_index })?;
                let num_prims = mesh.primitives.len();
                log::debug!("    has {num_prims} primitives");
                for (prim, i) in mesh.primitives.iter().zip(1..) {
                    let hybrid = stage.new_value(Renderlet {
                        vertices_array: prim.vertices.array(),
                        indices_array: prim.indices.array(),
                        transform_id: gltf_node.transform.global_transform_id(),
                        material_id: prim.material,
                        camera_id,
                        skin_id,
                        ..Default::default()
                    });
                    log::debug!("    created renderlet {i}/{num_prims}: {:#?}", hybrid.get());
                    stage.add_renderlet(&hybrid);
                    node_renderlets.push(hybrid);
                }
            }
            if !node_renderlets.is_empty() {
                renderlets.insert(gltf_node.index, node_renderlets);
            }
        }

        log::debug!("Extensions used: {:?}", document.extensions_used());
        log::debug!("Extensions required: {:?}", document.extensions_required());
        log::debug!("Done loading gltf");

        Ok(GltfDocument {
            frames,
            animations,
            lights,
            cameras,
            materials,
            default_material,
            meshes,
            nodes,
            scenes,
            skins,
            default_scene: document.default_scene().map(|scene| scene.index()),
            textures,
            renderlets,
            extensions: document
                .extensions()
                .cloned()
                .map(serde_json::Value::Object),
        })
    }

    pub fn renderlets_iter(&self) -> impl Iterator<Item = &Hybrid<Renderlet>> {
        self.renderlets.iter().flat_map(|(_, rs)| rs.iter())
    }

    pub fn nodes_in_scene(&self, scene_index: usize) -> impl Iterator<Item = &GltfNode> {
        let scene = self.scenes.get(scene_index);
        let mut nodes = vec![];
        if let Some(indices) = scene {
            for node_index in indices {
                if let Some(node) = self.nodes.get(*node_index) {
                    nodes.push(node);
                }
            }
        }
        nodes.into_iter()
    }
}

impl Stage {
    pub fn load_gltf_document_from_path(
        &mut self,
        path: impl AsRef<std::path::Path>,
        camera_id: Id<Camera>,
    ) -> Result<GltfDocument, StageGltfError> {
        let (document, buffers, images) = gltf::import(path)?;
        GltfDocument::from_gltf(self, &document, buffers, images, camera_id)
    }

    pub fn load_gltf_document_from_bytes(
        &mut self,
        bytes: impl AsRef<[u8]>,
        camera_id: Id<Camera>,
    ) -> Result<GltfDocument, StageGltfError> {
        let (document, buffers, images) = gltf::import_slice(bytes)?;
        GltfDocument::from_gltf(self, &document, buffers, images, camera_id)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        camera::Camera,
        pbr::{Material, PbrConfig},
        stage::{Renderlet, Vertex},
        transform::Transform,
        Context,
    };
    use crabslab::{Id, Slab};
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
    // ensures we can
    // * read simple meshes
    // * support multiple nodes that reference the same mesh
    // * support primitives w/ positions and normal attributes
    // * support transforming nodes (T * R * S)
    fn stage_gltf_simple_meshes() {
        let ctx = Context::headless(100, 50);
        let projection = crate::camera::perspective(100.0, 50.0);
        let position = Vec3::new(1.0, 0.5, 1.5);
        let view = crate::camera::look_at(position, Vec3::new(1.0, 0.5, 0.0), Vec3::Y);
        let mut stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let camera = stage.new_value(Camera {
            projection,
            view,
            position,
        });
        let _doc = stage
            .load_gltf_document_from_path(
                "../../gltf/gltfTutorial_008_SimpleMeshes.gltf",
                camera.id(),
            )
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("gltf/simple_meshes.png", img);
    }

    #[test]
    // Ensures we can read a minimal gltf file with a simple triangle mesh.
    fn minimal_mesh() {
        let ctx = Context::headless(20, 20);
        let mut stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.0).extend(1.0));

        let projection = crate::camera::perspective(20.0, 20.0);
        let eye = Vec3::new(0.5, 0.5, 2.0);
        let view = crate::camera::look_at(eye, Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let camera = stage.new_value(Camera {
            projection,
            view,
            position: Vec3::new(0.5, 0.5, 2.0),
        });

        let _doc = stage
            .load_gltf_document_from_path(
                "../../gltf/gltfTutorial_003_MinimalGltfFile.gltf",
                camera.id(),
            )
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("gltf/minimal_mesh.png", img);
    }

    #[test]
    // Tests importing a gltf file and rendering the first image as a 2d object.
    //
    // This ensures we are decoding images correctly.
    fn gltf_images() {
        let ctx = Context::headless(100, 100);
        let mut stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_background_color(Vec4::splat(1.0));
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let doc = stage
            .load_gltf_document_from_path("../../gltf/cheetah_cone.glb", camera.id())
            .unwrap();
        assert!(!doc.textures.is_empty());
        let albedo_texture = doc.textures.get(0);
        assert!(albedo_texture.is_some());
        let material = stage.new_value(Material {
            albedo_texture_id: doc.textures.get_id(0),
            has_lighting: false,
            ..Default::default()
        });
        println!("material_id: {:#?}", material.id());
        let vertices = stage.new_array([
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
        let indices = stage.new_array([0u32, 3, 2, 0, 2, 1]);
        let transform = stage.new_value(Transform {
            scale: Vec3::new(100.0, 100.0, 1.0),
            ..Default::default()
        });
        let renderlet = stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            indices_array: indices.array(),
            material_id: material.id(),
            transform_id: transform.id(),
            camera_id: camera.id(),
            ..Default::default()
        });
        stage.add_renderlet(&renderlet);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_linear_image().unwrap();
        img_diff::assert_img_eq("gltf/images.png", img);
    }

    #[test]
    fn simple_texture() {
        let size = 100;
        let ctx = Context::headless(size, size);
        let mut stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0))
            // There are no lights in the scene and the material isn't marked as "unlit", so
            // let's force it to be unlit.
            .with_lighting(false)
            .with_bloom(false);
        let projection = crate::camera::perspective(size as f32, size as f32);
        let view =
            crate::camera::look_at(Vec3::new(0.5, 0.5, 1.25), Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let camera = stage.new_value(Camera::new(projection, view));

        let _doc = stage
            .load_gltf_document_from_path(
                "../../gltf/gltfTutorial_013_SimpleTexture.gltf",
                camera.id(),
            )
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("gltf/simple_texture.png", img);
    }

    #[test]
    // Demonstrates how to load and render a gltf file containing lighting and a
    // normal map.
    fn normal_mapping_brick_sphere() {
        let size = 600;
        let ctx = Context::headless(size, size);
        let mut stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_background_color(Vec4::ONE);

        let doc = stage
            .load_gltf_document_from_path("../../gltf/red_brick_03_1k.glb", Id::NONE)
            .unwrap();
        let gltf_camera = doc.cameras.first().unwrap();
        doc.renderlets_iter().for_each(|hybrid| {
            hybrid.modify(|r| {
                r.camera_id = gltf_camera.camera.id();
            });
        });

        stage.set_lights(doc.lights.iter().map(|gltf_light| gltf_light.light.id()));

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("gltf/normal_mapping_brick_sphere.png", img);
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
        pub fn invoke(instance_index: u32, vertex_index: u32, slab: &[u32]) -> Self {
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

    #[test]
    fn rigged_fox() {
        let ctx = Context::headless(256, 256);
        let mut stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_vertex_skinning(false)
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.5).extend(1.0));

        let aspect = 256.0 / 256.0;
        let fovy = core::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 1000.0;
        let projection = glam::Mat4::perspective_rh(fovy, aspect, znear, zfar);
        let y = 50.0;
        let eye = Vec3::new(120.0, y, 120.0);
        let target = Vec3::new(0.0, y, 0.0);
        let up = Vec3::Y;
        let view = glam::Mat4::look_at_rh(eye, target, up);

        let camera = stage.new_value(Camera::new(projection, view));
        let doc = stage
            .load_gltf_document_from_path("../../gltf/Fox.glb", camera.id())
            .unwrap();
        log::info!("renderlets: {:#?}", doc.renderlets);

        // render a frame without vertex skinning as a baseline
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("gltf/skinning/rigged_fox_no_skinning.png", img);

        // render a frame with vertex skinning to ensure our rigging is correct
        stage.set_has_vertex_skinning(true);
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "gltf/skinning/rigged_fox_no_skinning.png",
            img,
            img_diff::DiffCfg {
                test_name: Some("gltf/skinning/rigged_fox"),
                ..Default::default()
            },
        );

        // let mut animator = doc
        //     .animations
        //     .get(0)
        //     .unwrap()
        //     .clone()
        //     .into_animator(doc.nodes.iter().map(|n| (n.index, n.transform.clone())));
        // animator.progress(0.0).unwrap();
        // let frame = ctx.get_next_frame().unwrap();
        // stage.render(&frame.view());
        // let img = frame.read_image().unwrap();
        // img_diff::assert_img_eq_cfg(
        //     "gltf/skinning/rigged_fox_no_skinning.png",
        //     img,
        //     img_diff::DiffCfg {
        //         test_name: Some("gltf/skinning/rigged_fox_0"),
        //         ..Default::default()
        //     },
        // );

        // let slab = futures_lite::future::block_on(stage.read(
        //     ctx.get_device(),
        //     ctx.get_queue(),
        //     Some("stage slab"),
        //     ..,
        // ))
        // .unwrap();

        // assert_eq!(1, doc.skins.len());
        // let skin = doc.skins[0].skin.get();
        // for joint_index in 0..skin.joints.len() {
        //     // skin.get_joint_matrix(, , )
        // }
    }
}
