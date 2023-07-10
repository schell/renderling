//! Animation helpers for gltf.

use glam::{Quat, Vec3};
use snafu::prelude::*;

use crate::{GpuEntity, Id};

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

#[derive(Debug)]
pub enum GltfInterpolation {
    Linear,
    Step,
    CubicSpline,
}

impl std::fmt::Display for GltfInterpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GltfInterpolation::Linear => "linear",
            GltfInterpolation::Step => "step",
            GltfInterpolation::CubicSpline => "cubic spline",
        })
    }
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

impl GltfInterpolation {
    fn is_cubic_spline(&self) -> bool {
        matches!(self, GltfInterpolation::CubicSpline)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Keyframe(pub f32);

#[derive(Debug)]
pub enum TweenProperty {
    Translation(Vec3),
    Rotation(Quat),
    Scale(Vec3),
    MorphTargetWeights(Vec<f32>),
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

    fn as_morph_target_weights(&self) -> Option<&Vec<f32>> {
        match self {
            TweenProperty::MorphTargetWeights(ws) => Some(ws),
            _ => None,
        }
    }
}

/// Holds many keyframes worth of tweening properties.
#[derive(Debug)]
pub enum TweenProperties {
    Translations(Vec<Vec3>),
    Rotations(Vec<Quat>),
    Scales(Vec<Vec3>),
    MorphTargetWeights(Vec<Vec<f32>>),
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
            TweenProperties::MorphTargetWeights(weights) => weights
                .get(index)
                .map(|weights| TweenProperty::MorphTargetWeights(weights.clone())),
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
            TweenProperties::MorphTargetWeights(weights) => {
                if let Some([p0, p1, p2]) = weights.get(start..end) {
                    Some([
                        TweenProperty::MorphTargetWeights(p0.clone()),
                        TweenProperty::MorphTargetWeights(p1.clone()),
                        TweenProperty::MorphTargetWeights(p2.clone()),
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
    // Times (inputs)
    pub keyframes: Vec<Keyframe>,
    // Properties (outputs)
    pub properties: TweenProperties,
    // The type of interpolation
    pub interpolation: GltfInterpolation,
    // The gltf "nodes" index of the target node this tween applies to
    pub target_node_index: usize,
    // The entity id of the target node this tween applies to
    pub target_entity_id: Id<GpuEntity>,
}

impl Tween {
    /// Compute the interpolated tween property at the given time.
    ///
    /// If the given time is before the first keyframe or after the the last
    /// keyframe, `Ok(None)` is returned.
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
    /// If the time is greater than the last keyframe, the time will be wrapped
    /// to loop the tween.
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
        // UNWRAP: safe because we know this is either the first index or was found
        // above
        let [to_in, to, _] =
            self.properties
                .get_cubic(next_keyframe_ndx)
                .context(MissingPropertyIndexSnafu {
                    index: next_keyframe_ndx,
                })?;

        let delta_time = next_time - prev_time;
        let amount = (time - prev_time) / (next_time - prev_time);

        fn cubic_spline<T>(
            previous_point: T,
            previous_tangent: T,
            next_point: T,
            next_tangent: T,
            t: f32,
        ) -> T
        where
            T: std::ops::Mul<f32, Output = T> + std::ops::Add<Output = T>,
        {
            let t2 = t * t;
            let t3 = t2 * t;
            previous_point * (2.0 * t3 - 3.0 * t2 + 1.0)
                + previous_tangent * (t3 - 2.0 * t2 + t)
                + next_point * (-2.0 * t3 + 3.0 * t2)
                + next_tangent * (t3 - t2)
        }

        Ok(Some(match from {
            TweenProperty::Translation(from) => {
                let from_out = *from_out
                    .as_translation()
                    .context(MismatchedPropertiesSnafu)?;
                let to_in = *to_in.as_translation().context(MismatchedPropertiesSnafu)?;
                let to = *to.as_translation().context(MismatchedPropertiesSnafu)?;
                let previous_tangent = delta_time * from_out;
                let next_tangent = delta_time * to_in;
                TweenProperty::Translation(cubic_spline(
                    from,
                    previous_tangent,
                    to,
                    next_tangent,
                    amount,
                ))
            }
            TweenProperty::Rotation(from) => {
                let from_out = *from_out.as_rotation().context(MismatchedPropertiesSnafu)?;
                let to_in = *to_in.as_rotation().context(MismatchedPropertiesSnafu)?;
                let to = *to.as_rotation().context(MismatchedPropertiesSnafu)?;
                let previous_tangent = from_out * delta_time;
                let next_tangent = to_in * delta_time;
                TweenProperty::Rotation(cubic_spline(
                    from,
                    previous_tangent,
                    to,
                    next_tangent,
                    amount,
                ))
            }
            TweenProperty::Scale(from) => {
                let from_out = *from_out.as_scale().context(MismatchedPropertiesSnafu)?;
                let to_in = *to_in.as_scale().context(MismatchedPropertiesSnafu)?;
                let to = *to.as_scale().context(MismatchedPropertiesSnafu)?;
                let previous_tangent = from_out * delta_time;
                let next_tangent = to_in * delta_time;
                TweenProperty::Scale(cubic_spline(
                    from,
                    previous_tangent,
                    to,
                    next_tangent,
                    amount,
                ))
            }
            TweenProperty::MorphTargetWeights(from) => {
                let from_out = from_out
                    .as_morph_target_weights()
                    .context(MismatchedPropertiesSnafu)?;
                let to_in = to_in
                    .as_morph_target_weights()
                    .context(MismatchedPropertiesSnafu)?;
                let to = to
                    .as_morph_target_weights()
                    .context(MismatchedPropertiesSnafu)?;

                let weights = from
                    .into_iter()
                    .zip(
                        from_out
                            .into_iter()
                            .zip(to_in.into_iter().zip(to.into_iter())),
                    )
                    .map(|(from, (from_out, (to_in, to)))| -> f32 {
                        let previous_tangent = from_out * delta_time;
                        let next_tangent = to_in * delta_time;
                        cubic_spline(from, previous_tangent, *to, next_tangent, amount)
                    });
                TweenProperty::MorphTargetWeights(weights.collect())
            }
        }))
    }

    fn interpolate_linear(&self, time: f32) -> Result<Option<TweenProperty>, InterpolationError> {
        let last_keyframe = self.keyframes.len() - 1;
        let last_time = self.keyframes[last_keyframe].0;
        let time = time.min(last_time);
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
        let from = self.properties.get(prev_keyframe_ndx).unwrap();

        // UNWRAP: safe because we know this is either the first index or was found
        // above
        let to = self.properties.get(next_keyframe_ndx).unwrap();

        let amount = (time - prev_time) / (next_time - prev_time);
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
            TweenProperty::MorphTargetWeights(a) => {
                let b = to
                    .as_morph_target_weights()
                    .context(MismatchedPropertiesSnafu)?;
                TweenProperty::MorphTargetWeights(
                    a.into_iter()
                        .zip(b)
                        .map(|(a, b)| a + (b - a) * amount)
                        .collect(),
                )
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
                if self.interpolation.is_cubic_spline() {
                    ts.iter().nth(1).copied().map(TweenProperty::Translation)
                } else {
                    ts.first().copied().map(TweenProperty::Translation)
                }
            }
            TweenProperties::Rotations(rs) => {
                if self.interpolation.is_cubic_spline() {
                    rs.iter().nth(1).copied().map(TweenProperty::Rotation)
                } else {
                    rs.first().copied().map(TweenProperty::Rotation)
                }
            }
            TweenProperties::Scales(ss) => {
                if self.interpolation.is_cubic_spline() {
                    ss.iter().nth(1).copied().map(TweenProperty::Scale)
                } else {
                    ss.first().copied().map(TweenProperty::Scale)
                }
            }
            TweenProperties::MorphTargetWeights(ws) => {
                if self.interpolation.is_cubic_spline() {
                    ws.iter()
                        .nth(1)
                        .cloned()
                        .map(TweenProperty::MorphTargetWeights)
                } else {
                    ws.first().cloned().map(TweenProperty::MorphTargetWeights)
                }
            }
        }
    }

    pub fn get_last_keyframe_property(&self) -> Option<TweenProperty> {
        match &self.properties {
            TweenProperties::Translations(ts) => {
                if self.interpolation.is_cubic_spline() {
                    let second_last = ts.len() - 2;
                    ts.get(second_last).copied().map(TweenProperty::Translation)
                } else {
                    ts.last().copied().map(TweenProperty::Translation)
                }
            }
            TweenProperties::Rotations(rs) => {
                if self.interpolation.is_cubic_spline() {
                    let second_last = rs.len() - 2;
                    rs.get(second_last).copied().map(TweenProperty::Rotation)
                } else {
                    rs.last().copied().map(TweenProperty::Rotation)
                }
            }
            TweenProperties::Scales(ss) => {
                if self.interpolation.is_cubic_spline() {
                    let second_last = ss.len() - 2;
                    ss.get(second_last).copied().map(TweenProperty::Scale)
                } else {
                    ss.last().copied().map(TweenProperty::Scale)
                }
            }
            TweenProperties::MorphTargetWeights(ws) => {
                if self.interpolation.is_cubic_spline() {
                    let second_last = ws.len() - 2;
                    ws.get(second_last)
                        .cloned()
                        .map(TweenProperty::MorphTargetWeights)
                } else {
                    ws.last().cloned().map(TweenProperty::MorphTargetWeights)
                }
            }
        }
    }
}

#[derive(Debug, Snafu)]
pub enum AnimationError {
    #[snafu(display("{}", source))]
    Interpolation { source: InterpolationError },
    #[snafu(display("missing node {}", index))]
    MissingNode { index: usize },
    #[snafu(display("node is not an entity (might be a light or camera)"))]
    ExpectedEntity,
}

#[derive(Default, Debug)]
pub struct GltfAnimation {
    pub tweens: Vec<Tween>,
    // A time to use as the current amount of seconds elapsed in the running
    // of this animation.
    pub stored_timestamp: f32,
    // The name of this animation, if any.
    pub name: Option<String>,
}

impl GltfAnimation {
    pub fn length_in_seconds(&self) -> f32 {
        self.tweens
            .iter()
            .flat_map(|tween| tween.keyframes.iter().map(|k| k.0))
            .max_by(f32::total_cmp)
            .unwrap_or_default()
    }

    pub fn get_properties_at_time(
        &self,
        t: f32,
    ) -> Result<Vec<(Id<GpuEntity>, TweenProperty)>, AnimationError> {
        let mut tweens = vec![];
        for tween in self.tweens.iter() {
            let prop = if let Some(prop) = tween.interpolate(t).context(InterpolationSnafu)? {
                prop
            } else if t >= tween.length_in_seconds() {
                tween.get_last_keyframe_property().unwrap()
            } else {
                tween.get_first_keyframe_property().unwrap()
            };
            tweens.push((tween.target_entity_id, prop));
        }

        Ok(tweens.into_iter().collect())
    }
}
