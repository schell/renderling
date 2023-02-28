//! Collections of textures, materials, meshes, lights cameras and objects
//! arranged in 3d space.
//!
//! A scene is the structure that is built by importing a gltf file.
use std::sync::Arc;

use glam::{Mat4, Quat, Vec3, Vec4};
use gltf::khr_lights_punctual::Kind;
use rustc_hash::FxHashMap;
use snafu::prelude::*;
use splines::Interpolate;

use crate::{
    linkage::pbr, BlinnPhongMaterial, Camera, DirectionalLight, ForwardPipeline, LocalTransform,
    Mesh, MeshBuilder, Object, ObjectBuilderError, PointLight, Renderling, SpotLight, Texture,
};

#[derive(Debug, Snafu)]
pub enum GltfError {
    #[snafu(display("Unsupported gltf image format: {:?}", format))]
    UnsupportedImageFormat { format: gltf::image::Format },

    #[snafu(display("No {} attribute for mesh", attribute.to_string()))]
    MissingAttribute { attribute: gltf::Semantic },

    #[snafu(display("Missing buffer"))]
    MissingBuffer,

    #[snafu(display("Missing view"))]
    MissingView,

    #[snafu(display("Missing texture {}", index))]
    MissingTexture { index: usize },

    #[snafu(display("Missing mesh {} {:?}", index, name))]
    MissingMesh { index: usize, name: Option<String> },

    #[snafu(display("Missing material {:?} {:?}", index, name))]
    MissingMaterial {
        index: Option<usize>,
        name: Option<String>,
    },

    #[snafu(display("Missing image {}", index))]
    MissingImage { index: usize },

    #[snafu(display("Unsupported primitive mode: {:?}", mode))]
    PrimitiveMode { mode: gltf::mesh::Mode },

    #[snafu(display("{}", source))]
    Object { source: ObjectBuilderError },

    #[snafu(display("Parent is not an object"))]
    ParentNotAnObject,
}

#[derive(Debug, Snafu)]
pub enum InterpolationError {
    #[snafu(display("No keyframes"))]
    NoKeyframes,

    #[snafu(display("Not enough keyframes"))]
    NotEnoughKeyframes,

    #[snafu(display("Property with index {} is missing", index))]
    MissingPropertyIndex { index: usize },

    #[snafu(display("No previous keyframe, first is {first:?}"))]
    NoPreviousKeyframe { first: Keyframe },

    #[snafu(display("No next keyframe, last is {last:?}"))]
    NoNextKeyframe { last: Keyframe },

    #[snafu(display("Mismatched properties"))]
    MismatchedProperties,
}

fn image_data_format_to_wgpu(
    gltf_format: gltf::image::Format,
) -> Result<wgpu::TextureFormat, GltfError> {
    let format = match gltf_format {
        gltf::image::Format::R8 => wgpu::TextureFormat::R8Unorm,
        gltf::image::Format::R8G8 => wgpu::TextureFormat::Rg8Unorm,
        // wgpu doesn't have an rgb8unorm texture format 🤷
        gltf::image::Format::R8G8B8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        gltf::image::Format::R8G8B8A8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        format => return Err(GltfError::UnsupportedImageFormat { format }),
    };
    Ok(format)
}

fn image_data_format_num_channels(gltf_format: gltf::image::Format) -> u32 {
    match gltf_format {
        gltf::image::Format::R8 => 1,
        gltf::image::Format::R8G8 => 2,
        // wgpu doesn't have an rgb8unorm texture format 🤷, so we map to rgba8unormsrgb
        gltf::image::Format::R8G8B8 => 4,
        gltf::image::Format::R8G8B8A8 => 4,
        gltf::image::Format::R16 => 1,
        gltf::image::Format::R16G16 => 2,
        gltf::image::Format::R16G16B16 => 3,
        gltf::image::Format::R16G16B16A16 => 4,
        gltf::image::Format::R32G32B32FLOAT => 3,
        gltf::image::Format::R32G32B32A32FLOAT => 4,
    }
}

pub struct GltfStore<T> {
    dense: Vec<Option<T>>,
    names: FxHashMap<String, Vec<usize>>,
}

impl<T> Default for GltfStore<T> {
    fn default() -> Self {
        Self {
            dense: Default::default(),
            names: Default::default(),
        }
    }
}

impl<T> GltfStore<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.dense.iter().flatten()
    }

    pub fn remove(&mut self, index: usize, name: Option<String>) -> Option<T> {
        if let Some(name) = name {
            if let Some(indices) = self.names.get_mut(&name) {
                indices.retain(|i| *i != index);
            }
        }
        self.dense.get_mut(index)?.take()
    }

    pub fn insert(&mut self, index: usize, name: Option<String>, item: T) -> Option<T> {
        if self.dense.len() <= index {
            self.dense.resize_with(index + 1, || None);
        }
        while self.dense.len() <= index {
            self.dense.push(None);
        }
        let existing = self.remove(index, name.clone());
        self.dense[index] = Some(item);
        if let Some(name) = name {
            let indices = self.names.entry(name).or_default();
            indices.push(index);
        }
        existing
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.dense.get(index)?.as_ref()
    }

    pub fn get_by_name(&self, name: &str) -> Option<Result<&T, impl Iterator<Item = &T> + '_>> {
        let indices = self.names.get(name)?;
        match indices.as_slice() {
            [index] => self.get(*index).map(|t| Ok(t)),
            indices => Some(Err(indices.iter().flat_map(|index| self.get(*index)))),
        }
    }
}

#[derive(Clone)]
pub struct GltfMeshPrimitive {
    pub mesh: Arc<Mesh>,
    pub material: Arc<BlinnPhongMaterial>,
}

pub enum GltfLightVariant {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

impl GltfLightVariant {
    pub fn as_directional(&self) -> Option<&DirectionalLight> {
        match self {
            GltfLightVariant::Directional(d) => Some(&d),
            _ => None,
        }
    }
}

pub enum GltfNodeVariant {
    Camera(Camera),
    Object(Object),
    Light {
        light_index: usize,
        light_name: Option<String>,
        variant: GltfLightVariant,
    },
}

impl GltfNodeVariant {
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            GltfNodeVariant::Object(o) => Some(o),
            _ => None,
        }
    }
}

pub struct GltfNode {
    /// The name of this node, if available.
    pub name: Option<String>,
    /// The index of this node in the list of all nodes in the document.
    pub index: usize,
    /// The primitive mesh objects "nested" within this node, if any.
    ///
    /// This will be empty unless the node is an object.
    pub prim_objects: Vec<Object>,
    /// The variant of this node.
    ///
    /// This is the meat of the data of this node.
    pub variant: GltfNodeVariant,
}

impl GltfNode {
    pub fn set_tween_property(&self, property: TweenProperty) {
        match &self.variant {
            GltfNodeVariant::Camera(_) => todo!(),
            GltfNodeVariant::Object(o) => match property {
                TweenProperty::Translation(t) => o.set_position(t),
                TweenProperty::Rotation(r) => o.set_rotation(r),
                TweenProperty::Scale(s) => o.set_scale(s),
            },
            GltfNodeVariant::Light { .. } => todo!(),
        }
    }
}

fn decomposed_transform(transform: gltf::scene::Transform) -> LocalTransform {
    let (t, r, s) = transform.decomposed();
    LocalTransform::default()
        .with_position(Vec3::from(t))
        .with_rotation(Quat::from_array(r))
        .with_scale(Vec3::from(s))
}

#[derive(Debug)]
pub struct GltfVertex {
    position: [f32; 3],
    uv: Option<[f32; 2]>,
    normal: Option<[f32; 3]>,
}

#[derive(Debug)]
pub enum GltfInterpolation {
    Linear,
    Step,
    CubicSpline,
}

impl From<gltf::animation::Interpolation> for GltfInterpolation {
    fn from(value: gltf::animation::Interpolation) -> Self {
        match value {
            gltf::animation::Interpolation::Linear => GltfInterpolation::Linear,
            gltf::animation::Interpolation::Step => GltfInterpolation::Step,
            gltf::animation::Interpolation::CubicSpline => GltfInterpolation::CubicSpline,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Keyframe(pub f32);

#[derive(Debug)]
pub enum TweenProperty {
    Translation(Vec3),
    Rotation(Quat),
    Scale(Vec3),
}

impl TweenProperty {
    fn as_translation(&self) -> Option<&Vec3> {
        match self {
            TweenProperty::Translation(a) => Some(a),
            _ => None,
        }
    }

    fn as_rotation(&self) -> Option<&Quat> {
        match self {
            TweenProperty::Rotation(a) => Some(a),
            _ => None,
        }
    }

    fn as_scale(&self) -> Option<&Vec3> {
        match self {
            TweenProperty::Scale(a) => Some(a),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum TweenProperties {
    Translations(Vec<Vec3>),
    Rotations(Vec<Quat>),
    Scales(Vec<Vec3>),
}

impl TweenProperties {
    pub fn get(&self, index: usize) -> Option<TweenProperty> {
        match self {
            TweenProperties::Translations(translations) => translations
                .get(index)
                .map(|translation| TweenProperty::Translation(*translation)),
            TweenProperties::Rotations(rotations) => rotations
                .get(index)
                .map(|rotation| TweenProperty::Rotation(*rotation)),
            TweenProperties::Scales(scales) => {
                scales.get(index).map(|scale| TweenProperty::Scale(*scale))
            }
        }
    }

    pub fn get_cubic(&self, index: usize) -> Option<[TweenProperty; 3]> {
        let start = 3 * index;
        let end = start + 3;
        match self {
            TweenProperties::Translations(translations) => {
                if let Some([p0, p1, p2]) = translations.get(start..end) {
                    Some([
                        TweenProperty::Translation(p0.clone()),
                        TweenProperty::Translation(p1.clone()),
                        TweenProperty::Translation(p2.clone()),
                    ])
                } else {
                    None
                }
            }
            TweenProperties::Rotations(rotations) => {
                if let Some([p0, p1, p2]) = rotations.get(start..end) {
                    Some([
                        TweenProperty::Rotation(p0.clone()),
                        TweenProperty::Rotation(p1.clone()),
                        TweenProperty::Rotation(p2.clone()),
                    ])
                } else {
                    None
                }
            }
            TweenProperties::Scales(scales) => {
                if let Some([p0, p1, p2]) = scales.get(start..end) {
                    Some([
                        TweenProperty::Scale(p0.clone()),
                        TweenProperty::Scale(p1.clone()),
                        TweenProperty::Scale(p2.clone()),
                    ])
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Tween {
    // times (inputs)
    pub keyframes: Vec<Keyframe>,
    // properties (outputs)
    pub properties: TweenProperties,
    // the type of interpolation
    pub interpolation: GltfInterpolation,
    // the index of target node this tween applies to
    pub target_node_index: usize,
}

impl Tween {
    /// Compute the interpolated tween property at the given time.
    ///
    /// If the given time is before the first keyframe or after the the last keyframe, `Ok(None)`
    /// is returned.
    ///
    /// See https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_007_Animations.md
    pub fn interpolate(&self, time: f32) -> Result<Option<TweenProperty>, InterpolationError> {
        snafu::ensure!(!self.keyframes.is_empty(), NoKeyframesSnafu);

        match self.interpolation {
            GltfInterpolation::Linear => self.interpolate_linear(time),
            GltfInterpolation::Step => self.interpolate_step(time),
            GltfInterpolation::CubicSpline => self.interpolate_cubic(time),
        }
    }

    /// Compute the interpolated tween property at the given time.
    ///
    /// If the time is greater than the last keyframe, the time will be wrapped to loop the tween.
    ///
    /// Returns `None` if the properties don't match.
    pub fn interpolate_wrap(&self, time: f32) -> Result<Option<TweenProperty>, InterpolationError> {
        let total = self.length_in_seconds();
        let time = time % total;
        self.interpolate(time)
    }

    fn get_previous_keyframe(
        &self,
        time: f32,
    ) -> Result<Option<(usize, &Keyframe)>, InterpolationError> {
        snafu::ensure!(!self.keyframes.is_empty(), NoKeyframesSnafu);
        Ok(self
            .keyframes
            .iter()
            .enumerate()
            .filter(|(_, keyframe)| keyframe.0 <= time)
            .last())
    }

    fn get_next_keyframe(
        &self,
        time: f32,
    ) -> Result<Option<(usize, &Keyframe)>, InterpolationError> {
        snafu::ensure!(!self.keyframes.is_empty(), NoKeyframesSnafu);
        Ok(self
            .keyframes
            .iter()
            .enumerate()
            .find(|(_, keyframe)| keyframe.0 > time))
    }

    fn interpolate_step(&self, time: f32) -> Result<Option<TweenProperty>, InterpolationError> {
        log::trace!("step");
        if let Some((prev_keyframe_ndx, _)) = self.get_previous_keyframe(time)? {
            self.properties
                .get(prev_keyframe_ndx)
                .context(MissingPropertyIndexSnafu {
                    index: prev_keyframe_ndx,
                })
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn interpolate_cubic(&self, time: f32) -> Result<Option<TweenProperty>, InterpolationError> {
        log::trace!("cubic");
        snafu::ensure!(self.keyframes.len() >= 2, NotEnoughKeyframesSnafu);

        let (prev_keyframe_ndx, prev_keyframe) =
            if let Some(prev) = self.get_previous_keyframe(time)? {
                prev
            } else {
                return Ok(None);
            };
        let prev_time = prev_keyframe.0;

        let (next_keyframe_ndx, next_keyframe) = if let Some(next) = self.get_next_keyframe(time)? {
            next
        } else {
            return Ok(None);
        };
        let next_time = next_keyframe.0;

        // UNWRAP: safe because we know this was found above
        let [_, from, from_out] =
            self.properties
                .get_cubic(prev_keyframe_ndx)
                .context(MissingPropertyIndexSnafu {
                    index: prev_keyframe_ndx,
                })?;
        // UNWRAP: safe because we know this is either the first index or was found above
        let [to_in, to, _] =
            self.properties
                .get_cubic(next_keyframe_ndx)
                .context(MissingPropertyIndexSnafu {
                    index: next_keyframe_ndx,
                })?;

        let amount = (time - prev_time) / (next_time - prev_time);

        Ok(Some(match from {
            TweenProperty::Translation(from) => {
                let from_out = *from_out
                    .as_translation()
                    .context(MismatchedPropertiesSnafu)?;
                let to_in = *to_in.as_translation().context(MismatchedPropertiesSnafu)?;
                let to = *to.as_translation().context(MismatchedPropertiesSnafu)?;
                TweenProperty::Translation(Vec3::cubic_bezier(amount, from, from_out, to_in, to))
            }
            TweenProperty::Rotation(from) => {
                let from_out = *from_out.as_rotation().context(MismatchedPropertiesSnafu)?;
                let to_in = *to_in.as_rotation().context(MismatchedPropertiesSnafu)?;
                let to = *to.as_rotation().context(MismatchedPropertiesSnafu)?;
                TweenProperty::Rotation(Quat::cubic_bezier(amount, from, from_out, to_in, to))
            }
            TweenProperty::Scale(from) => {
                let from_out = *from_out.as_scale().context(MismatchedPropertiesSnafu)?;
                let to_in = *to_in.as_scale().context(MismatchedPropertiesSnafu)?;
                let to = *to.as_scale().context(MismatchedPropertiesSnafu)?;
                TweenProperty::Scale(Vec3::cubic_bezier(amount, from, from_out, to_in, to))
            }
        }))
    }

    fn interpolate_linear(&self, time: f32) -> Result<Option<TweenProperty>, InterpolationError> {
        log::trace!("linear");
        let last_keyframe = self.keyframes.len() - 1;
        let last_time = self.keyframes[last_keyframe].0;
        let time = time.min(last_time);
        log::trace!("time: {}", time);
        let (prev_keyframe_ndx, prev_keyframe) =
            if let Some(prev) = self.get_previous_keyframe(time)? {
                prev
            } else {
                return Ok(None);
            };
        let prev_time = prev_keyframe.0;
        log::trace!("prev_time: {}", prev_time);

        let (next_keyframe_ndx, next_keyframe) = if let Some(next) = self.get_next_keyframe(time)? {
            next
        } else {
            return Ok(None);
        };
        let next_time = next_keyframe.0;
        log::trace!("next_time: {}", next_time);

        // UNWRAP: safe because we know this was found above
        let from = self.properties.get(prev_keyframe_ndx).unwrap();
        log::trace!("from: {} {:?}", prev_keyframe_ndx, from);

        // UNWRAP: safe because we know this is either the first index or was found above
        let to = self.properties.get(next_keyframe_ndx).unwrap();
        log::trace!("to: {} {:?}", next_keyframe_ndx, to);

        let amount = (time - prev_time) / (next_time - prev_time);
        log::trace!("amount: {:?}", amount);
        Ok(Some(match from {
            TweenProperty::Translation(a) => {
                let b = to.as_translation().context(MismatchedPropertiesSnafu)?;
                TweenProperty::Translation(a.lerp(*b, amount))
            }
            TweenProperty::Rotation(a) => {
                let a = a.normalize();
                let b = to
                    .as_rotation()
                    .context(MismatchedPropertiesSnafu)?
                    .normalize();
                TweenProperty::Rotation(a.slerp(b, amount))
            }
            TweenProperty::Scale(a) => {
                let b = to.as_scale().context(MismatchedPropertiesSnafu)?;
                TweenProperty::Scale(a.lerp(*b, amount))
            }
        }))
    }

    pub fn length_in_seconds(&self) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        let last_keyframe = self.keyframes.len() - 1;
        let last_time = self.keyframes[last_keyframe].0;
        last_time
    }

    pub fn get_first_keyframe_property(&self) -> Option<TweenProperty> {
        match &self.properties {
            TweenProperties::Translations(ts) => {
                ts.first().copied().map(TweenProperty::Translation)
            }
            TweenProperties::Rotations(rs) => rs.first().copied().map(TweenProperty::Rotation),
            TweenProperties::Scales(ss) => ss.first().copied().map(TweenProperty::Scale),
        }
    }

    pub fn get_last_keyframe_property(&self) -> Option<TweenProperty> {
        match &self.properties {
            TweenProperties::Translations(ts) => ts.last().copied().map(TweenProperty::Translation),
            TweenProperties::Rotations(rs) => rs.last().copied().map(TweenProperty::Rotation),
            TweenProperties::Scales(ss) => ss.last().copied().map(TweenProperty::Scale),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum AnimationError {
    #[snafu(display("{}", source))]
    Interpolation { source: InterpolationError },
    #[snafu(display("missing node {}", index))]
    MissingNode { index: usize },
}

#[derive(Default, Debug)]
pub struct GltfAnimation {
    pub tweens: Vec<Tween>,
}

impl GltfAnimation {
    pub fn length_in_seconds(&self) -> f32 {
        self.tweens
            .iter()
            .flat_map(|tween| tween.keyframes.iter().map(|k| k.0))
            .max_by(f32::total_cmp)
            .unwrap_or_default()
    }

    pub fn set_time(&self, loader: &GltfLoader, t: f32) -> Result<(), AnimationError> {
        for tween in self.tweens.iter() {
            let prop = if let Some(prop) = tween.interpolate(t).context(InterpolationSnafu)? {
                prop
            } else if t >= tween.length_in_seconds() {
                tween.get_last_keyframe_property().unwrap()
            } else {
                tween.get_first_keyframe_property().unwrap()
            };
            let node = loader
                .get_node(tween.target_node_index)
                .context(MissingNodeSnafu {
                    index: tween.target_node_index,
                })?;
            node.set_tween_property(prop);
        }

        Ok(())
    }
}

pub struct GltfLoader {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    default_texture: Option<Texture>,
    textures: GltfStore<Texture>,
    default_material: Option<Arc<BlinnPhongMaterial>>,
    materials: GltfStore<Arc<BlinnPhongMaterial>>,
    meshes: GltfStore<Vec<GltfMeshPrimitive>>,
    nodes: GltfStore<GltfNode>,
    animations: GltfStore<GltfAnimation>,
    pub generate_normals: bool,
}

impl GltfLoader {
    /// Create a new loader.
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        GltfLoader {
            device,
            queue,
            default_texture: None,
            textures: Default::default(),
            default_material: None,
            materials: Default::default(),
            meshes: Default::default(),
            nodes: Default::default(),
            animations: Default::default(),
            generate_normals: true,
        }
    }

    pub fn unload(&mut self) {
        *self = GltfLoader::new(self.device.clone(), self.queue.clone());
    }

    /// Get a reference to all the textures loaded thus far.
    pub fn textures(&self) -> impl Iterator<Item = &Texture> {
        self.textures.iter()
    }

    pub fn get_default_texture(&mut self) -> Texture {
        if self.default_texture.is_none() {
            self.default_texture = Some(Texture::new(
                &self.device,
                &self.queue,
                Some("GltfLoader::new-default"),
                None,
                4,
                1,
                1,
                &[0xff, 0xff, 0xff, 0xff],
            ));
        }

        // UNWRAP: we just ensured it's Some
        self.default_texture.clone().unwrap()
    }

    pub fn texture_for<'b>(
        &mut self,
        texture: gltf::Texture<'b>,
        images: &[gltf::image::Data],
    ) -> Result<Texture, GltfError> {
        if let Some(texture) = self.textures.get(texture.index()) {
            Ok(texture.clone())
        } else {
            let index = texture.index();
            let dat = images.get(index).context(MissingImageSnafu { index })?;
            let format = image_data_format_to_wgpu(dat.format)?;
            let num_channels = image_data_format_num_channels(dat.format);
            let pixels = if dat.format == gltf::image::Format::R8G8B8 {
                dat.pixels
                    .as_slice()
                    .chunks(3)
                    .flat_map(|rgb| match rgb {
                        [r, g, b] => vec![*r, *g, *b, 255],
                        // UNREACHABLE: we check the format above
                        _ => unreachable!("not rgb"),
                    })
                    .collect::<Vec<_>>()
            } else {
                dat.pixels.to_vec()
            };

            let sampler = texture.sampler();

            fn to_address_mode(mode: gltf::texture::WrappingMode) -> wgpu::AddressMode {
                match mode {
                    gltf::texture::WrappingMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                    gltf::texture::WrappingMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
                    gltf::texture::WrappingMode::Repeat => wgpu::AddressMode::Repeat,
                }
            }

            let r_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: to_address_mode(sampler.wrap_s()),
                address_mode_v: to_address_mode(sampler.wrap_t()),
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: sampler
                    .mag_filter()
                    .map(|f| match f {
                        gltf::texture::MagFilter::Nearest => wgpu::FilterMode::Nearest,
                        gltf::texture::MagFilter::Linear => wgpu::FilterMode::Linear,
                    })
                    .unwrap_or(wgpu::FilterMode::Linear),
                min_filter: sampler
                    .min_filter()
                    .map(|f| match f {
                        gltf::texture::MinFilter::Nearest
                        | gltf::texture::MinFilter::NearestMipmapNearest
                        | gltf::texture::MinFilter::NearestMipmapLinear => {
                            wgpu::FilterMode::Nearest
                        }
                        gltf::texture::MinFilter::Linear
                        | gltf::texture::MinFilter::LinearMipmapNearest
                        | gltf::texture::MinFilter::LinearMipmapLinear => wgpu::FilterMode::Linear,
                    })
                    .unwrap_or(wgpu::FilterMode::Nearest),
                mipmap_filter: sampler
                    .min_filter()
                    .map(|f| match f {
                        gltf::texture::MinFilter::NearestMipmapNearest
                        | gltf::texture::MinFilter::LinearMipmapNearest => {
                            wgpu::FilterMode::Nearest
                        }
                        gltf::texture::MinFilter::NearestMipmapLinear
                        | gltf::texture::MinFilter::LinearMipmapLinear => wgpu::FilterMode::Linear,
                        _ => wgpu::FilterMode::Nearest,
                    })
                    .unwrap_or(wgpu::FilterMode::Nearest),
                ..Default::default()
            });
            let texture = Texture::new_with(
                &self.device,
                &self.queue,
                None,
                None,
                Some(r_sampler),
                format,
                num_channels,
                dat.width,
                dat.height,
                &pixels,
            );
            self.textures.insert(index, None, texture.clone());
            Ok(texture)
        }
    }

    pub fn load_material<'b>(
        &mut self,
        material: gltf::Material<'b>,
        images: &[gltf::image::Data],
    ) -> Result<Arc<BlinnPhongMaterial>, GltfError> {
        let metallic = material.pbr_metallic_roughness();
        log::trace!("material: {:?} {:?}", material.index(), material.name(),);
        log::trace!(
            "-metallic: base_color_factor: {:?}",
            metallic.base_color_factor()
        );
        let diffuse_texture: Texture = if let Some(texture_info) = metallic.base_color_texture() {
            let tex = texture_info.texture();
            log::trace!("--base_color_texture: {} {:?}", tex.index(), tex.name());
            self.texture_for(tex, images)?
        } else {
            let [r, g, b, a] = metallic.base_color_factor();
            let data = [
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
                (a * 255.0) as u8,
            ];
            Texture::new_with(
                &self.device,
                &self.queue,
                material.name(),
                None,
                None,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                4,
                1,
                1,
                &data,
            )
        };

        let specular_texture = self.get_default_texture();

        let blinn_phong = Arc::new(BlinnPhongMaterial {
            diffuse_texture,
            specular_texture,
            shininess: 16.0,
        });

        if let Some(index) = material.index() {
            self.materials.insert(
                index,
                material.name().map(|s| s.to_string()),
                blinn_phong.clone(),
            );
        } else {
            self.default_material = Some(blinn_phong.clone());
        }
        Ok(blinn_phong)
    }

    /// Load all materials from the gltf data into the loader.
    ///
    /// This also loads all textures.
    pub fn load_materials<'b>(
        &mut self,
        document: &'b gltf::Document,
        images: &[gltf::image::Data],
    ) -> Result<(), GltfError> {
        log::trace!("loading materials and textures");
        for material in document.materials() {
            let _ = self.load_material(material, images)?;
        }
        Ok(())
    }

    pub fn material_for<'b>(
        &mut self,
        material: gltf::Material<'b>,
        images: &[gltf::image::Data],
    ) -> Result<Arc<BlinnPhongMaterial>, GltfError> {
        let index = material.index();
        if index.is_none() {
            // this material is the default material
            if let Some(bp_mat) = self.default_material.as_ref() {
                return Ok(bp_mat.clone());
            }
        } else {
            // UNWRAP: safe because we checked its Some
            if let Some(bp_mat) = self.materials.get(index.unwrap()) {
                return Ok(bp_mat.clone());
            }
        }

        // since we're still here we need to generate/load the material
        self.load_material(material, images)
    }

    pub fn load_mesh_with<'b, Vertex: bytemuck::Pod>(
        &mut self,
        mesh: gltf::Mesh<'b>,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
        // a function that converts a GltfVertex, which may or may not contain certain
        // attributes, into a mesh vertex
        build_vertex: fn(GltfVertex) -> Vertex,
    ) -> Result<(), GltfError> {
        let mut prims = vec![];
        log::trace!("mesh: {} {:?}", mesh.index(), mesh.name());
        log::trace!("-weights: {:?}", mesh.weights());
        for primitive in mesh.primitives() {
            snafu::ensure!(
                primitive.mode() == gltf::mesh::Mode::Triangles,
                PrimitiveModeSnafu {
                    mode: primitive.mode()
                }
            );
            log::trace!("-primitive: {}", primitive.index());
            log::trace!("--bounding_box: {:?}", primitive.bounding_box());
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions = reader.read_positions().context(MissingAttributeSnafu {
                attribute: gltf::Semantic::Positions,
            })?;
            log::trace!("--positions: {} vertices", positions.len());
            let (positions, normals): (
                Box<dyn Iterator<Item = [f32; 3]>>,
                Box<dyn Iterator<Item = Option<[f32; 3]>>>,
            ) = if let Some(normals) = reader.read_normals() {
                log::trace!("--normals: {} vertices", normals.len());
                (Box::new(positions), Box::new(normals.map(Some)))
            } else if self.generate_normals {
                let positions = positions.collect::<Vec<_>>();
                let normals = positions
                    .chunks(3)
                    .flat_map(|t| match t {
                        [a, b, c] => {
                            let a = Vec3::from(*a);
                            let b = Vec3::from(*b);
                            let c = Vec3::from(*c);
                            let ab = b - a;
                            let ac = c - a;
                            let n = ab.cross(ac);
                            let n = [n.x, n.y, n.z];
                            [n, n, n]
                        }
                        _ => unreachable!("safe because we know these are triangles"),
                    })
                    .collect::<Vec<_>>();
                (
                    Box::new(positions.into_iter()),
                    Box::new(normals.into_iter().map(Some)),
                )
            } else {
                log::trace!("--normals: none");
                (Box::new(positions), Box::new(std::iter::repeat(None)))
            };
            let normalized = primitive
                .get(&gltf::Semantic::Normals)
                .map(|n| n.normalized());
            log::trace!("---normalized: {normalized:?}");
            let uvs: Box<dyn Iterator<Item = Option<[f32; 2]>>> =
                if let Some(uvs) = reader.read_tex_coords(0) {
                    let uvs = uvs.into_f32().map(Some);
                    log::trace!("--uvs: {} vertices", uvs.len());
                    Box::new(uvs)
                } else {
                    log::trace!("--uvs: none");
                    Box::new(std::iter::repeat(None))
                };

            let builder = positions.zip(normals.zip(uvs)).fold(
                MeshBuilder::<Vertex>::default(),
                |builder, (position, (normal, uv))| -> MeshBuilder<_> {
                    let vertex = GltfVertex {
                        position,
                        normal,
                        uv,
                    };
                    builder.with_vertex(build_vertex(vertex))
                },
            );
            let builder = if let Some(indices) = reader.read_indices() {
                let indices = match indices {
                    gltf::mesh::util::ReadIndices::U8(_) => todo!(),
                    gltf::mesh::util::ReadIndices::U16(indices) => indices,
                    gltf::mesh::util::ReadIndices::U32(_) => todo!(),
                };
                log::trace!("--indices: length {}", indices.len());
                builder.with_indices(indices)
            } else {
                builder
            };
            let material = primitive.material();
            let material: Arc<BlinnPhongMaterial> = self.material_for(material, images)?;
            let prim = GltfMeshPrimitive {
                mesh: Arc::new(builder.build(Some("gltf_support"), &self.device)),
                material,
            };
            prims.push(prim);
        }
        self.meshes
            .insert(mesh.index(), mesh.name().map(|s| s.to_string()), prims);

        Ok(())
    }

    pub fn load_mesh<'b>(
        &mut self,
        mesh: gltf::Mesh<'b>,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
    ) -> Result<(), GltfError> {
        self.load_mesh_with(mesh, buffers, images, |v| pbr::Vertex {
            position: v.position,
            uv: v.uv.unwrap_or_default(),
            normal: v.normal.unwrap(),
        })
    }

    /// Load all mesh primitives from the gltf document into the loader.
    pub fn load_meshes<'b>(
        &mut self,
        document: &'b gltf::Document,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
    ) -> Result<(), GltfError> {
        log::trace!("loading meshes");
        for mesh in document.meshes() {
            let _ = self.load_mesh(mesh, buffers, images)?;
        }

        Ok(())
    }

    pub fn mesh_primitives_for<'b>(
        &mut self,
        mesh: gltf::Mesh<'b>,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
    ) -> Result<Vec<GltfMeshPrimitive>, GltfError> {
        if let Some(prims) = self.meshes.get(mesh.index()) {
            Ok(prims.clone())
        } else {
            let index = mesh.index();
            let name = mesh.name().map(|n| n.to_string());
            self.load_mesh(mesh, buffers, images)?;
            let prims = self
                .meshes
                .get(index)
                .context(MissingMeshSnafu { index, name })?;
            Ok(prims.clone())
        }
    }

    pub fn load_node<'b>(
        &mut self,
        r: &mut Renderling<ForwardPipeline>,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
        node: gltf::Node<'b>,
        depth: usize,
    ) -> Result<&GltfNode, GltfError> {
        let pad = std::iter::repeat("-")
            .take(depth)
            .collect::<Vec<_>>()
            .join("");
        log::trace!("{pad}node: {} {:?}", node.index(), node.name());

        let transform = decomposed_transform(node.transform());
        log::trace!("{pad}-transform:");
        log::trace!("{pad}--position: {:?}", transform.position);
        log::trace!("{pad}--rotation: {:?}", transform.rotation);
        log::trace!("{pad}--scale: {:?}", transform.scale);

        let r_node = {
            if let Some(camera) = node.camera() {
                log::trace!("{pad}-camera: {} {:?}", camera.index(), camera.name());
                let view: Mat4 = Mat4::from_cols_array_2d(&node.transform().matrix()).inverse();
                let r_camera = r
                    .new_camera()
                    .with_projection({
                        let projection = camera.projection();
                        match projection {
                            gltf::camera::Projection::Orthographic(o) => Mat4::orthographic_rh(
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
                                let znear = p.znear();
                                log::trace!("{pad}--fovy: {fovy}");
                                log::trace!("{pad}--aspect: {aspect}");
                                log::trace!("{pad}--znear: {znear}");
                                log::trace!("{pad}--zfar: {:?}", p.zfar());
                                if let Some(zfar) = p.zfar() {
                                    Mat4::perspective_rh(fovy, aspect, p.znear(), zfar)
                                } else {
                                    Mat4::perspective_infinite_rh(
                                        p.yfov(),
                                        p.aspect_ratio().unwrap_or(1.0),
                                        p.znear(),
                                    )
                                }
                            }
                        }
                    })
                    .with_view(view.into())
                    .build();
                let name = node.name().map(|s| s.to_string());
                GltfNode {
                    name,
                    index: node.index(),
                    prim_objects: vec![],
                    variant: GltfNodeVariant::Camera(r_camera),
                }
            } else if let Some(mesh) = node.mesh() {
                log::trace!("{pad}-mesh: {} {:?}", mesh.index(), mesh.name());
                let prims = self.mesh_primitives_for(mesh, buffers, images)?;
                if prims.len() > 1 {
                    // the mesh is made up of multiple mesh "primitives", so add those
                    // as children
                    let mut children = vec![];
                    for prim in prims.iter() {
                        let child = r
                            .new_object()
                            .with_mesh(prim.mesh.clone())
                            .with_material::<BlinnPhongMaterial>(prim.material.clone())
                            .build()
                            .context(ObjectSnafu)?;
                        children.push(child);
                    }
                    let object = r
                        .new_object()
                        .with_transform(transform.clone())
                        .with_children(children.iter())
                        .build()
                        .context(ObjectSnafu)?;
                    log::trace!("{pad}-mesh-primitive-children: {}", children.len());
                    // hold on to the child objects so they don't get dropped
                    GltfNode {
                        name: node.name().map(|s| s.to_string()),
                        index: node.index(),
                        prim_objects: children,
                        variant: GltfNodeVariant::Object(object),
                    }
                } else {
                    let mesh = prims[0].mesh.clone();
                    let material = prims[0].material.clone();
                    let object = r
                        .new_object()
                        .with_transform(transform.clone())
                        .with_mesh(mesh)
                        .with_material::<BlinnPhongMaterial>(material)
                        .build()
                        .context(ObjectSnafu)?;
                    GltfNode {
                        name: node.name().map(|s| s.to_string()),
                        index: node.index(),
                        prim_objects: vec![],
                        variant: GltfNodeVariant::Object(object),
                    }
                }
            } else if let Some(light) = node.light() {
                log::trace!("{pad}-light: {} {:?}", light.index(), light.name());
                let color = Vec3::from(light.color()).extend(1.0);
                let transparent = Vec4::splat(0.0);
                let direction = Mat4::from_quat(transform.rotation).transform_vector3(Vec3::NEG_Z);
                let variant = match light.kind() {
                    Kind::Directional => {
                        log::trace!("{pad}--kind: directional");
                        log::trace!("{pad}---color: {:?}", color);
                        log::trace!("{pad}---direction: {:?}", direction);
                        GltfLightVariant::Directional(
                            r.new_directional_light()
                                .with_direction(direction)
                                .with_diffuse_color(color)
                                .with_specular_color(color)
                                .with_ambient_color(transparent)
                                .build(),
                        )
                    }
                    Kind::Point => {
                        log::trace!("{pad}--kind: point");
                        log::trace!("{pad}---color: {:?}", color);
                        log::trace!("{pad}---position: {:?}", transform.position);
                        GltfLightVariant::Point(
                            r.new_point_light()
                                .with_position(transform.position)
                                .with_diffuse_color(color)
                                .build(),
                        )
                    }
                    Kind::Spot {
                        inner_cone_angle,
                        outer_cone_angle,
                    } => {
                        log::trace!("{pad}--kind: spot");
                        log::trace!("{pad}---color: {:?}", color);
                        log::trace!("{pad}---direction: {:?}", direction);
                        GltfLightVariant::Spot(
                            r.new_spot_light()
                                .with_position(transform.position)
                                .with_direction(direction)
                                .with_diffuse_color(color)
                                .with_cutoff(inner_cone_angle, outer_cone_angle)
                                .build(),
                        )
                    }
                };

                let name = node.name().map(|s| s.to_string());
                GltfNode {
                    name,
                    index: node.index(),
                    prim_objects: vec![],
                    variant: GltfNodeVariant::Light {
                        light_index: light.index(),
                        light_name: light.name().map(|s| s.to_string()),
                        variant,
                    },
                }
            } else {
                log::trace!("{pad}-node is just a container");
                GltfNode {
                    name: node.name().map(|s| s.to_string()),
                    index: node.index(),
                    prim_objects: vec![],
                    variant: GltfNodeVariant::Object(
                        r.new_object()
                            .with_transform(transform.clone())
                            .build()
                            .context(ObjectSnafu)?,
                    ),
                }
            }
        };

        let mut printed = false;
        for node in node.children() {
            if !printed {
                log::trace!("{pad}-children");
                printed = true;
            }

            let child_node = self.load_node(r, buffers, images, node, depth + 2)?;
            if let Some(child_object) = child_node.variant.as_object() {
                let parent = r_node.variant.as_object().context(ParentNotAnObjectSnafu)?;
                parent.append_child(child_object);
            }
        }

        self.nodes
            .insert(node.index(), node.name().map(|s| s.to_string()), r_node);
        Ok(self.nodes.get(node.index()).unwrap())
    }

    /// Load everything.
    pub fn load<'b>(
        &mut self,
        r: &mut Renderling<ForwardPipeline>,
        document: &'b gltf::Document,
        buffers: &[gltf::buffer::Data],
        images: &[gltf::image::Data],
    ) -> Result<(), GltfError> {
        for scene in document.scenes() {
            for node in scene.nodes() {
                let _ = self.load_node(r, buffers, images, node, 1)?;
            }
        }
        self.load_animations(&document, &buffers)?;

        Ok(())
    }

    pub fn get_light_by_index(&self, index: usize) -> Option<&GltfLightVariant> {
        for node in self.nodes.iter() {
            match &node.variant {
                GltfNodeVariant::Camera(_) => todo!(),
                GltfNodeVariant::Object(_) => todo!(),
                GltfNodeVariant::Light {
                    light_index,
                    light_name: _,
                    variant,
                } => {
                    if *light_index == index {
                        return Some(variant);
                    }
                }
            }
        }
        None
    }

    pub fn get_node(&self, index: usize) -> Option<&GltfNode> {
        self.nodes.get(index)
    }

    /// Iterate over the nodes that are cameras
    pub fn cameras(&self) -> impl Iterator<Item = &GltfNode> {
        self.nodes
            .iter()
            .filter(|node| matches!(node.variant, GltfNodeVariant::Camera(_)))
    }

    /// Iterate over the nodes that are lights
    pub fn lights(&self) -> impl Iterator<Item = &GltfNode> {
        self.nodes
            .iter()
            .filter(|node| matches!(node.variant, GltfNodeVariant::Light { .. }))
    }

    pub fn load_animations(
        &mut self,
        document: &gltf::Document,
        buffers: &[gltf::buffer::Data],
    ) -> Result<(), GltfError> {
        for animation in document.animations() {
            let mut r_animation = GltfAnimation::default();
            for channel in animation.channels() {
                let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
                let inputs = reader.read_inputs().context(MissingBufferSnafu)?;
                let outputs = reader.read_outputs().context(MissingBufferSnafu)?;
                let tween = Tween {
                    keyframes: inputs.map(|t| Keyframe(t)).collect::<Vec<_>>(),
                    properties: match outputs {
                        gltf::animation::util::ReadOutputs::Translations(ts) => {
                            TweenProperties::Translations(ts.map(Vec3::from).collect())
                        }
                        gltf::animation::util::ReadOutputs::Rotations(rs) => {
                            TweenProperties::Rotations(
                                rs.into_f32().map(Quat::from_array).collect(),
                            )
                        }
                        gltf::animation::util::ReadOutputs::Scales(ss) => {
                            TweenProperties::Scales(ss.map(Vec3::from).collect())
                        }
                        gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => {
                            todo!("morph targets unsupported")
                        }
                    },
                    interpolation: channel.sampler().interpolation().into(),
                    target_node_index: channel.target().node().index(),
                };
                r_animation.tweens.push(tween);
            }

            self.animations.insert(
                animation.index(),
                animation.name().map(|s| s.to_string()),
                r_animation,
            );
        }

        Ok(())
    }

    pub fn animations(&self) -> impl Iterator<Item = &GltfAnimation> {
        self.animations.iter()
    }

    pub fn get_animation(&self, index: usize) -> Option<&GltfAnimation> {
        self.animations.get(index)
    }
}
