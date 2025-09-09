//! Gltf support for the [`Stage`](crate::Stage).
use std::{collections::HashMap, sync::Arc};

use craballoc::prelude::*;
use crabslab::Id;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use rustc_hash::{FxHashMap, FxHashSet};
use snafu::{OptionExt, ResultExt, Snafu};

use crate::{
    atlas::{
        AtlasError, AtlasImage, AtlasTexture, AtlasTextureDescriptor, TextureAddressMode,
        TextureModes,
    },
    bvol::Aabb,
    camera::{Camera, CameraDescriptor},
    geometry::{Indices, MorphTargetWeights, MorphTargets, Skin, Vertices},
    light::{AnalyticalLight, LightStyle},
    material::Material,
    stage::{MorphTarget, Renderlet, Stage, Vertex},
    transform::{NestedTransform, TransformDescriptor},
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

    #[snafu(display("Wrong image at index {index} atlas offset {offset}"))]
    WrongImage { offset: usize, index: usize },

    #[snafu(display("Missing image {index} '{name}'"))]
    MissingImage { index: usize, name: String },

    #[snafu(display("Missing texture at gltf index {index} slab index {tex_id:?}"))]
    MissingTexture {
        index: usize,
        tex_id: Id<AtlasTextureDescriptor>,
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

impl From<gltf::scene::Transform> for TransformDescriptor {
    fn from(transform: gltf::scene::Transform) -> Self {
        let (translation, rotation, scale) = transform.decomposed();
        TransformDescriptor {
            translation: Vec3::from_array(translation),
            rotation: Quat::from_array(rotation),
            scale: Vec3::from_array(scale),
        }
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
    pub fn preprocess_images(
        material: gltf::Material,
        images: &mut [AtlasImage],
    ) -> Result<(), StageGltfError> {
        let pbr = material.pbr_metallic_roughness();
        if material.unlit() {
            if let Some(info) = pbr.base_color_texture() {
                let texture = info.texture();
                // The index of the image in the original gltf document
                let image_index = texture.source().index();
                let name = texture.name().unwrap_or("unknown");
                // Update the image to ensure it gets transferred correctly
                let image = images.get_mut(image_index).context(MissingImageSnafu {
                    index: image_index,
                    name,
                })?;
                image.apply_linear_transfer = true;
            }
        } else {
            if let Some(info) = pbr.base_color_texture() {
                let texture = info.texture();
                let name = texture.name().unwrap_or("unknown");
                let image_index = texture.source().index();
                // Update the image to ensure it gets transferred correctly
                let image = images.get_mut(image_index).context(MissingImageSnafu {
                    index: image_index,
                    name,
                })?;
                image.apply_linear_transfer = true;
            }

            if let Some(emissive_tex) = material.emissive_texture() {
                let texture = emissive_tex.texture();
                let name = texture.name().unwrap_or("unknown");
                let image_index = texture.source().index();
                // Update the image to ensure it gets transferred correctly
                let image = images.get_mut(image_index).context(MissingImageSnafu {
                    index: image_index,
                    name,
                })?;
                image.apply_linear_transfer = true;
            }
        }
        Ok(())
    }

    pub fn from_gltf(
        stage: &Stage,
        material: gltf::Material,
        entries: &[AtlasTexture],
    ) -> Result<Material, StageGltfError> {
        let name = material.name().map(String::from);
        log::trace!("loading material {:?} {name:?}", material.index());
        let pbr = material.pbr_metallic_roughness();
        let builder = stage.new_material();
        if material.unlit() {
            log::trace!("  is unlit");
            builder.set_has_lighting(false);

            if let Some(info) = pbr.base_color_texture() {
                let texture = info.texture();
                let index = texture.index();
                if let Some(tex) = entries.get(index) {
                    builder.set_albedo_texture(tex);
                    builder.set_albedo_tex_coord(info.tex_coord());
                }
            }
            builder.set_albedo_factor(pbr.base_color_factor().into());
        } else {
            log::trace!("  is pbr");
            builder.set_has_lighting(true);

            if let Some(info) = pbr.base_color_texture() {
                let texture = info.texture();
                let index = texture.index();
                if let Some(tex) = entries.get(index) {
                    builder.set_albedo_texture(tex);
                    builder.set_albedo_tex_coord(info.tex_coord());
                }
            }
            builder.set_albedo_factor(pbr.base_color_factor().into());

            if let Some(info) = pbr.metallic_roughness_texture() {
                let index = info.texture().index();
                if let Some(tex) = entries.get(index) {
                    builder.set_metallic_roughness_texture(tex);
                    builder.set_metallic_roughness_tex_coord(info.tex_coord());
                }
            } else {
                builder.set_metallic_factor(pbr.metallic_factor());
                builder.set_roughness_factor(pbr.roughness_factor());
            }

            if let Some(norm_tex) = material.normal_texture() {
                if let Some(tex) = entries.get(norm_tex.texture().index()) {
                    builder.set_normal_texture(tex);
                    builder.set_normal_tex_coord(norm_tex.tex_coord());
                }
            }

            if let Some(occlusion_tex) = material.occlusion_texture() {
                if let Some(tex) = entries.get(occlusion_tex.texture().index()) {
                    builder.set_ambient_occlusion_texture(tex);
                    builder.set_ambient_occlusion_tex_coord(occlusion_tex.tex_coord());
                    builder.set_ambient_occlusion_strength(occlusion_tex.strength());
                }
            }

            if let Some(emissive_tex) = material.emissive_texture() {
                let texture = emissive_tex.texture();
                let index = texture.index();
                if let Some(tex) = entries.get(index) {
                    builder.set_emissive_texture(tex);
                    builder.set_emissive_tex_coord(emissive_tex.tex_coord());
                }
            }
            builder.set_emissive_strength_multiplier(material.emissive_strength().unwrap_or(1.0));
        };
        Ok(builder)
    }
}

#[derive(Debug)]
pub struct GltfPrimitive {
    pub indices: Indices,
    pub vertices: Vertices,
    pub bounding_box: (Vec3, Vec3),
    pub material_index: Option<usize>,
    pub morph_targets: MorphTargets,
}

impl GltfPrimitive {
    pub fn from_gltf(
        stage: &Stage,
        primitive: gltf::Primitive,
        buffer_data: &[gltf::buffer::Data],
    ) -> Self {
        let material_index = primitive.material().index();

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

        // See the GLTF spec on morph targets
        // https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#morph-targets
        //
        // TODO: Generate morph target normals and tangents if absent.
        // Although the spec says we have to generate normals or tangents if not specified,
        // we are explicitly *not* doing that here.
        let morph_targets: Vec<Vec<MorphTarget>> = reader
            .read_morph_targets()
            .map(|(may_ps, may_ns, may_ts)| {
                let ps = may_ps
                    .into_iter()
                    .flat_map(|iter| iter.map(Vec3::from_array))
                    .chain(std::iter::repeat(Vec3::ZERO))
                    .take(positions.len());

                let ns = may_ns
                    .into_iter()
                    .flat_map(|iter| iter.map(Vec3::from_array))
                    .chain(std::iter::repeat(Vec3::ZERO))
                    .take(positions.len());

                let ts = may_ts
                    .into_iter()
                    .flat_map(|iter| iter.map(Vec3::from_array))
                    .chain(std::iter::repeat(Vec3::ZERO))
                    .take(positions.len());

                ps.zip(ns)
                    .zip(ts)
                    .map(|((position, normal), tangent)| MorphTarget {
                        position,
                        normal,
                        tangent,
                    })
                    .collect()
            })
            .collect();
        log::debug!(
            "  {} morph_targets: {:?}",
            morph_targets.len(),
            morph_targets.iter().map(|mt| mt.len()).collect::<Vec<_>>()
        );
        let morph_targets = stage.new_morph_targets(morph_targets);
        let vs = joints.into_iter().zip(weights);
        let vs = colors.zip(vs);
        let vs = tangents.into_iter().zip(vs);
        let vs = normals.into_iter().zip(vs);
        let vs = uv1s.zip(vs);
        let vs = uv0s.into_iter().zip(vs);
        let vs = positions.into_iter().zip(vs);

        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        let vertices = vs
            .map(
                |(position, (uv0, (uv1, (normal, (tangent, (color, (joints, weights)))))))| {
                    min = min.min(position);
                    max = max.max(position);
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
        let vertices = stage.new_vertices(vertices);
        log::debug!(
            "{} vertices, {:?}",
            vertices.array().len(),
            vertices.array()
        );
        let indices = stage.new_indices(indices);
        log::debug!("{} indices, {:?}", indices.array().len(), indices.array());
        let (bbmin, bbmax) = {
            let gltf::mesh::Bounds { min, max } = primitive.bounding_box();
            (Vec3::from_array(min), Vec3::from_array(max))
        };
        if bbmin != min {
            log::warn!("gltf supplied bounding box min ({bbmin:?}) doesn't match seen ({min:?})");
        }
        if bbmax != max {
            log::warn!("gltf supplied bounding box max ({bbmax:?}) doesn't match seen ({max:?})");
        }
        let bounding_box = (min, max);

        log::info!("primitive '{}' bounds: {bounding_box:?}", primitive.index());

        Self {
            vertices,
            indices,
            material_index,
            morph_targets,
            bounding_box,
        }
    }
}

#[derive(Debug)]
pub struct GltfMesh {
    /// Mesh primitives, aka meshlets
    pub primitives: Vec<GltfPrimitive>,
    /// Morph target weights
    pub weights: MorphTargetWeights,
}

impl GltfMesh {
    fn from_gltf(stage: &Stage, buffer_data: &[gltf::buffer::Data], mesh: gltf::Mesh) -> Self {
        log::debug!("Loading primitives for mesh {}", mesh.index());
        let primitives = mesh
            .primitives()
            .map(|prim| GltfPrimitive::from_gltf(stage, prim, buffer_data))
            .collect::<Vec<_>>();
        log::debug!("  loaded {} primitives\n", primitives.len());
        let weights = mesh.weights().unwrap_or(&[]).iter().copied();
        GltfMesh {
            primitives,
            weights: stage.new_morph_target_weights(weights),
        }
    }
}

#[derive(Debug)]
pub struct GltfCamera {
    pub index: usize,
    pub name: Option<String>,
    pub node_transform: NestedTransform,
    projection: Mat4,
    pub camera: Camera,
}

impl AsRef<Camera> for GltfCamera {
    fn as_ref(&self) -> &Camera {
        &self.camera
    }
}

impl GltfCamera {
    fn new(stage: &Stage, gltf_camera: gltf::Camera<'_>, transform: &NestedTransform) -> Self {
        log::debug!("camera: {}", gltf_camera.name().unwrap_or("unknown"));
        log::debug!("  transform: {:#?}", transform.global_descriptor());
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
        let view = Mat4::from(transform.global_descriptor()).inverse();
        let camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        GltfCamera {
            index: gltf_camera.index(),
            name: gltf_camera.name().map(String::from),
            projection,
            node_transform: transform.clone(),
            camera,
        }
    }

    pub fn camera_descriptor(&self) -> CameraDescriptor {
        let view = Mat4::from(self.node_transform.global_descriptor()).inverse();
        CameraDescriptor::new(self.projection, view)
    }
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
    /// Morph target weights
    pub weights: MorphTargetWeights,
    /// This node's transform.
    pub transform: NestedTransform,
}

impl GltfNode {
    pub fn global_transform(&self) -> TransformDescriptor {
        self.transform.global_descriptor()
    }
}

#[derive(Clone, Debug)]
pub struct GltfSkin {
    pub index: usize,
    // Indices of the skeleton nodes used as joints in this skin, unused internally
    // but possibly useful.
    pub joint_nodes: Vec<usize>,
    // Index of the node used as the skeleton root.
    // When None, joints transforms resolve to scene root.
    pub skeleton: Option<usize>,
    // Skin as seen by renderling
    pub skin: Skin,
}

impl GltfSkin {
    pub fn from_gltf(
        stage: &Stage,
        buffer_data: &[gltf::buffer::Data],
        nodes: &[GltfNode],
        gltf_skin: gltf::Skin,
    ) -> Result<Self, StageGltfError> {
        log::debug!("reading skin {} {:?}", gltf_skin.index(), gltf_skin.name());
        let joint_nodes = gltf_skin.joints().map(|n| n.index()).collect::<Vec<_>>();
        log::debug!("  has {} joints", joint_nodes.len());
        let mut joint_transforms = vec![];
        for node_index in joint_nodes.iter() {
            let gltf_node: &GltfNode = nodes
                .get(*node_index)
                .context(MissingNodeSnafu { index: *node_index })?;
            let transform_id = gltf_node.transform.global_descriptor();
            log::debug!("    joint node {node_index} is {transform_id:?}");
            joint_transforms.push(gltf_node.transform.clone());
        }
        let reader = gltf_skin.reader(|b| buffer_data.get(b.index()).map(|d| d.0.as_slice()));
        let mut inverse_bind_matrices = vec![];
        if let Some(mats) = reader.read_inverse_bind_matrices() {
            let invs = mats
                .into_iter()
                .map(|m| Mat4::from_cols_array_2d(&m))
                .collect::<Vec<_>>();
            log::debug!("  has {} inverse bind matrices", invs.len());
            inverse_bind_matrices = invs;
        } else {
            log::debug!("  no inverse bind matrices");
        }
        let skeleton = if let Some(n) = gltf_skin.skeleton() {
            let index = n.index();
            log::debug!("  skeleton is node {index}, {:?}", n.name());
            Some(index)
        } else {
            log::debug!("  skeleton is assumed to be the scene root");
            None
        };
        Ok(GltfSkin {
            index: gltf_skin.index(),
            skin: stage.new_skin(joint_transforms, inverse_bind_matrices),
            joint_nodes,
            skeleton,
        })
    }
}

/// A loaded GLTF document.
pub struct GltfDocument {
    pub animations: Vec<Animation>,
    pub cameras: Vec<GltfCamera>,
    pub default_scene: Option<usize>,
    pub extensions: Option<serde_json::Value>,
    pub textures: Vec<AtlasTexture>,
    pub lights: Vec<AnalyticalLight>,
    pub meshes: Vec<GltfMesh>,
    pub nodes: Vec<GltfNode>,
    pub default_material: Material,
    pub materials: Vec<Material>,
    // map of node index to renderlets
    pub renderlets: FxHashMap<usize, Vec<Renderlet>>,
    /// Vector of scenes - each being a list of nodes.
    pub scenes: Vec<Vec<usize>>,
    pub skins: Vec<GltfSkin>,
}

impl GltfDocument {
    pub fn from_gltf(
        stage: &Stage,
        document: &gltf::Document,
        buffer_data: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<GltfDocument, StageGltfError> {
        let textures = {
            let mut images = images.into_iter().map(AtlasImage::from).collect::<Vec<_>>();
            for gltf_material in document.materials() {
                Material::preprocess_images(gltf_material, &mut images)?;
            }
            // Arc these images because they could be large and we don't want duplicates
            let images = images.into_iter().map(Arc::new).collect::<Vec<_>>();

            log::debug!("Loading {} images into the atlas", images.len());

            log::debug!("Writing textures");
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            struct Texture {
                source: usize,
                modes: TextureModes,
            }
            let mut textures = vec![];
            let mut deduped_textures = FxHashMap::<Texture, Vec<usize>>::default();
            for (i, texture) in document.textures().enumerate() {
                let index = texture.index();
                debug_assert_eq!(i, index);
                let name = texture.name().unwrap_or("unknown");
                log::trace!("  texture {i} '{name}'",);
                let source = texture.source().index();
                let modes = TextureModes {
                    s: TextureAddressMode::from_gltf(texture.sampler().wrap_s()),
                    t: TextureAddressMode::from_gltf(texture.sampler().wrap_t()),
                };
                let tex = Texture { modes, source };
                textures.push(tex);
                let entry = deduped_textures.entry(tex).or_default();
                entry.push(i);
            }

            // Prepare the textures for packing
            let mut deduped_textures = deduped_textures.into_iter().collect::<Vec<_>>();
            deduped_textures.sort();
            let mut prepared_images = vec![];
            for (tex, refs) in deduped_textures.iter() {
                let image = images
                    .get(tex.source)
                    .context(MissingImageSnafu {
                        index: refs[0],
                        name: "unknown".to_owned(),
                    })?
                    .clone();
                prepared_images.push(image);
            }
            let duplicated_image_count = prepared_images.len() - images.len();
            if duplicated_image_count > 0 {
                log::debug!("had to duplicate {duplicated_image_count} images...");
            }
            drop(images);

            let prepared_images: Vec<AtlasImage> = prepared_images
                .into_iter()
                .map(|aimg| match Arc::try_unwrap(aimg) {
                    Ok(img) => img,
                    Err(aimg) => aimg.as_ref().clone(),
                })
                .collect();
            let hybrid_textures = stage.add_images(prepared_images).context(StageSnafu)?;
            let mut texture_lookup = FxHashMap::<usize, AtlasTexture>::default();
            for (hybrid, (tex, refs)) in hybrid_textures.into_iter().zip(deduped_textures) {
                hybrid.set_modes(tex.modes);
                for tex_index in refs.into_iter() {
                    texture_lookup.insert(tex_index, hybrid.clone());
                }
            }
            let mut textures = texture_lookup.into_iter().collect::<Vec<_>>();
            textures.sort_by_key(|(index, _)| *index);
            textures
                .into_iter()
                .map(|(_, hybrid)| hybrid)
                .collect::<Vec<_>>()
        };

        log::debug!("Creating materials");
        let mut default_material = stage.default_material().clone();
        let mut materials = vec![];
        for gltf_material in document.materials() {
            let material_index = gltf_material.index();
            let material = Material::from_gltf(stage, gltf_material, &textures)?;
            if let Some(index) = material_index {
                log::trace!("  created material {index}");
                debug_assert_eq!(index, materials.len(), "unexpected material index");
                materials.push(material);
            } else {
                log::trace!("  created default material");
                default_material = material;
            }
        }
        log::trace!("  created {} materials", materials.len());

        log::debug!("Loading meshes");
        let mut meshes = vec![];
        for mesh in document.meshes() {
            let mesh = GltfMesh::from_gltf(stage, &buffer_data, mesh);
            meshes.push(mesh);
        }
        log::trace!("  loaded {} meshes", meshes.len());

        log::debug!("Loading {} nodes", document.nodes().count());
        let mut nodes = vec![];
        let mut node_transforms = HashMap::<usize, NestedTransform>::new();

        fn transform_for_node(
            nesting_level: usize,
            stage: &Stage,
            cache: &mut HashMap<usize, NestedTransform>,
            node: &gltf::Node,
        ) -> NestedTransform {
            let padding = std::iter::repeat_n(" ", nesting_level * 2)
                .collect::<Vec<_>>()
                .join("");
            let nt = if let Some(nt) = cache.get(&node.index()) {
                nt.clone()
            } else {
                let TransformDescriptor {
                    translation,
                    rotation,
                    scale,
                } = node.transform().into();
                let transform = stage
                    .new_nested_transform()
                    .with_local_translation(translation)
                    .with_local_rotation(rotation)
                    .with_local_scale(scale);
                for node in node.children() {
                    let child_transform =
                        transform_for_node(nesting_level + 1, stage, cache, &node);
                    transform.add_child(&child_transform);
                }
                cache.insert(node.index(), transform.clone());
                transform
            };
            let t = nt.local_descriptor();
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
            // From the glTF spec:
            //
            // A mesh with morph targets MAY also define an optional mesh.weights property
            // that stores the default targets' weights. These weights MUST be used when
            // node.weights is undefined. When mesh.weights is undefined, the default
            // targets' weights are zeros.
            let weights = if weights.is_empty() {
                if let Some(mesh) = node.mesh() {
                    meshes[mesh.index()].weights.clone()
                } else {
                    stage.new_morph_target_weights(weights)
                }
            } else {
                stage.new_morph_target_weights(weights)
            };
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
                .get(&node_index)
                .context(MissingNodeSnafu { index: node_index })?;
            cameras.push(GltfCamera::new(stage, camera, transform));
        }

        log::trace!("Loading lights");
        let mut lights = vec![];
        if let Some(gltf_lights) = document.lights() {
            for gltf_light in gltf_lights {
                let node_index = *light_index_to_node_index.get(&gltf_light.index()).context(
                    MissingCameraSnafu {
                        index: gltf_light.index(),
                    },
                )?;

                let node_transform = node_transforms
                    .get(&node_index)
                    .context(MissingNodeSnafu { index: node_index })?
                    .clone();

                let color = Vec3::from(gltf_light.color()).extend(1.0);
                let intensity = gltf_light.intensity();
                let light_bundle = match gltf_light.kind() {
                    gltf::khr_lights_punctual::Kind::Directional => {
                        stage
                            .new_directional_light()
                            .with_direction(Vec3::NEG_Z)
                            .with_color(color)
                            // TODO: Set a unit for lighting.
                            // We don't yet use a unit for our lighting, and we should.
                            // https://www.realtimerendering.com/blog/physical-units-for-lights/
                            //
                            // NOTE:
                            // glTF spec [1] says directional light is in lux, whereas spot and point are
                            // in candelas. I haven't really set a unit, it's implicit in the shader, but it seems we
                            // can roughly get candelas from lux by dividing by 683 [2].
                            // 1. https://github.com/KhronosGroup/glTF/blob/main/extensions/2.0/Khronos/KHR_lights_punctual/README.md
                            // 2. https://depts.washington.edu/mictech/optics/me557/Radiometry.pdf
                            // 3. https://projects.blender.org/blender/blender-addons/commit/9d903a93f03b
                            .with_intensity(intensity / 683.0)
                            .into_generic()
                    }

                    gltf::khr_lights_punctual::Kind::Point => stage
                        .new_point_light()
                        .with_position(Vec3::ZERO)
                        .with_color(color)
                        .with_intensity(intensity / 683.0)
                        .into_generic(),

                    gltf::khr_lights_punctual::Kind::Spot {
                        inner_cone_angle,
                        outer_cone_angle,
                    } => stage
                        .new_spot_light()
                        .with_position(Vec3::ZERO)
                        .with_direction(Vec3::NEG_Z)
                        .with_inner_cutoff(inner_cone_angle)
                        .with_outer_cutoff(outer_cone_angle)
                        .with_color(color)
                        .with_intensity(intensity / (683.0 * 4.0 * std::f32::consts::PI))
                        .into_generic(),
                };

                log::trace!(
                    "  linking light {:?} with node transform {:?}: {:#?}",
                    light_bundle.id(),
                    node_transform.global_id(),
                    node_transform.global_descriptor()
                );
                light_bundle.link_node_transform(&node_transform);
                lights.push(light_bundle);
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
            let maybe_skin = if let Some(skin_index) = gltf_node.skin {
                log::debug!("  node {} {:?} has skin", gltf_node.index, gltf_node.name);
                let gltf_skin = skins
                    .get(skin_index)
                    .context(MissingSkinSnafu { index: skin_index })?;
                Some(gltf_skin.skin.clone())
            } else {
                None
            };

            if let Some(mesh_index) = gltf_node.mesh {
                log::trace!(
                    "  node {} {:?} has mesh {mesh_index}",
                    gltf_node.index,
                    gltf_node.name
                );
                let mesh = meshes
                    .get(mesh_index)
                    .context(MissingMeshSnafu { index: mesh_index })?;
                let num_prims = mesh.primitives.len();
                log::trace!("    has {num_prims} primitives");
                for (prim, i) in mesh.primitives.iter().zip(1..) {
                    let material = prim
                        .material_index
                        .and_then(|index| materials.get(index))
                        .unwrap_or(&default_material);
                    let renderlet = stage
                        .new_renderlet()
                        .with_vertices(&prim.vertices)
                        .with_indices(&prim.indices)
                        .with_transform(&gltf_node.transform)
                        .with_material(material)
                        .with_bounds(prim.bounding_box.into())
                        .with_morph_targets(&prim.morph_targets, &gltf_node.weights);
                    if let Some(skin) = maybe_skin.as_ref() {
                        renderlet.set_skin(skin);
                    }
                    log::trace!(
                        "    created renderlet {i}/{num_prims}: {:#?}",
                        renderlet.id()
                    );
                    node_renderlets.push(renderlet);
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
            textures,
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
            renderlets,
            extensions: document
                .extensions()
                .cloned()
                .map(serde_json::Value::Object),
        })
    }

    pub fn renderlets_iter(&self) -> impl Iterator<Item = &Renderlet> {
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

    /// Returns the bounding volume of this document, if possible.
    ///
    /// This function will return `None` if this document does not contain meshes.
    pub fn bounding_volume(&self) -> Option<Aabb> {
        let mut aabbs = vec![];
        for node in self.nodes.iter() {
            if let Some(mesh_index) = node.mesh {
                let mesh = self.meshes.get(mesh_index)?;
                for prim in mesh.primitives.iter() {
                    let (prim_min, prim_max) = prim.bounding_box;
                    let prim_aabb = Aabb::new(prim_min, prim_max);
                    aabbs.push(prim_aabb);
                }
            }
        }
        let mut aabbs = aabbs.into_iter();
        let mut aabb = aabbs.next()?;
        for next_aabb in aabbs {
            aabb = Aabb::union(aabb, next_aabb);
        }
        Some(aabb)
    }
}

impl Stage {
    pub fn load_gltf_document_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<GltfDocument, StageGltfError> {
        let (document, buffers, images) = gltf::import(path)?;
        GltfDocument::from_gltf(self, &document, buffers, images)
    }

    pub fn load_gltf_document_from_bytes(
        &self,
        bytes: impl AsRef<[u8]>,
    ) -> Result<GltfDocument, StageGltfError> {
        let (document, buffers, images) = gltf::import_slice(bytes)?;
        GltfDocument::from_gltf(self, &document, buffers, images)
    }
}

#[cfg(test)]
mod test {
    use crate::{stage::Vertex, test::BlockOnFuture, Context};
    use glam::{Vec3, Vec4};

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
        let ctx = Context::headless(100, 50).block();
        let projection = crate::camera::perspective(100.0, 50.0);
        let position = Vec3::new(1.0, 0.5, 1.5);
        let view = crate::camera::look_at(position, Vec3::new(1.0, 0.5, 0.0), Vec3::Y);
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let _doc = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_008_SimpleMeshes.gltf")
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("gltf/simple_meshes.png", img);
    }

    #[test]
    // Ensures we can read a minimal gltf file with a simple triangle mesh.
    fn minimal_mesh() {
        let ctx = Context::headless(20, 20).block();
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_bloom(false)
            .with_background_color(Vec3::splat(0.0).extend(1.0));

        let projection = crate::camera::perspective(20.0, 20.0);
        let eye = Vec3::new(0.5, 0.5, 2.0);
        let view = crate::camera::look_at(eye, Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);

        let _doc = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_003_MinimalGltfFile.gltf")
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("gltf/minimal_mesh.png", img);
    }

    #[test]
    // Tests importing a gltf file and rendering the first image as a 2d object.
    //
    // This ensures we are decoding images correctly.
    fn gltf_images() {
        let ctx = Context::headless(100, 100).block();
        let stage = ctx
            .new_stage()
            .with_lighting(false)
            .with_background_color(Vec4::splat(1.0));
        let (projection, view) = crate::camera::default_ortho2d(100.0, 100.0);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let doc = stage
            .load_gltf_document_from_path("../../gltf/cheetah_cone.glb")
            .unwrap();
        assert!(!doc.textures.is_empty());
        let material = stage
            .new_material()
            .with_albedo_texture(&doc.textures[0])
            .with_has_lighting(false);
        let _rez = stage
            .new_renderlet()
            .with_material(&material)
            .with_vertices(
                stage.new_vertices([
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
                ]),
            )
            .with_indices(stage.new_indices([0u32, 3, 2, 0, 2, 1]))
            .with_transform(
                stage
                    .new_transform()
                    .with_scale(Vec3::new(100.0, 100.0, 1.0)),
            );
        println!("material_id: {:#?}", material.id());

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_linear_image().block().unwrap();
        img_diff::assert_img_eq("gltf/images.png", img);
    }

    #[test]
    fn simple_texture() {
        let size = 100;
        let ctx = Context::headless(size, size).block();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0))
            // There are no lights in the scene and the material isn't marked as "unlit", so
            // let's force it to be unlit.
            .with_lighting(false)
            .with_bloom(false);
        let projection = crate::camera::perspective(size as f32, size as f32);
        let view =
            crate::camera::look_at(Vec3::new(0.5, 0.5, 1.25), Vec3::new(0.5, 0.5, 0.0), Vec3::Y);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);

        let _doc = stage
            .load_gltf_document_from_path("../../gltf/gltfTutorial_013_SimpleTexture.gltf")
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("gltf/simple_texture.png", img);
    }

    #[test]
    // Demonstrates how to load and render a gltf file containing lighting and a
    // normal map.
    fn normal_mapping_brick_sphere() {
        let ctx = Context::headless(1920, 1080).block();
        let stage = ctx
            .new_stage()
            .with_lighting(true)
            .with_background_color(Vec4::new(0.01, 0.01, 0.01, 1.0));

        let _doc = stage
            .load_gltf_document_from_path("../../gltf/normal_mapping_brick_sphere.glb")
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("gltf/normal_mapping_brick_sphere.png", img);
    }

    #[test]
    fn rigged_fox() {
        let ctx = Context::headless(256, 256).block();
        let stage = ctx
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

        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let _doc = stage
            .load_gltf_document_from_path("../../gltf/Fox.glb")
            .unwrap();

        // render a frame without vertex skinning as a baseline
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("gltf/skinning/rigged_fox_no_skinning.png", img);

        // render a frame with vertex skinning to ensure our rigging is correct
        stage.set_has_vertex_skinning(true);
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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

    #[test]
    fn camera_position_sanity() {
        // Test that the camera has the expected translation,
        // taking into account that the gltf files may have been
        // saved with Y up, or with Z up
        let ctx = Context::headless(100, 100).block();
        let stage = ctx.new_stage();
        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_sanity_camera.gltf"),
            )
            .unwrap();
        let camera_a = doc.cameras.first().unwrap();

        let desc = camera_a.camera_descriptor();
        const THRESHOLD: f32 = 10e-6;
        let a = Vec3::new(14.699949, 4.958309, 12.676651);
        let b = Vec3::new(14.699949, -12.676651, 4.958309);
        let distance_a = a.distance(desc.position());
        let distance_b = b.distance(desc.position());
        if distance_a > THRESHOLD && distance_b > THRESHOLD {
            println!("desc: {desc:#?}");
            println!("distance_a: {distance_a}");
            println!("distance_b: {distance_b}");
            println!("threshold: {THRESHOLD}");
            panic!("distance greater than threshold");
        }

        let doc = stage
            .load_gltf_document_from_path(
                crate::test::workspace_dir()
                    .join("gltf")
                    .join("shadow_mapping_sanity.gltf"),
            )
            .unwrap();
        let camera_b = doc.cameras.first().unwrap();

        let eq = |a: Vec3, b: Vec3| {
            let c = Vec3::new(b.x, -b.z, b.y);
            println!("a: {a}");
            println!("b: {b}");
            println!("c: {c}");
            a.distance(b) <= 10e-6 || c.distance(c) <= 10e-6
        };
        assert!(eq(
            camera_a.camera_descriptor().position(),
            camera_b.camera_descriptor().position()
        ));
    }
}
