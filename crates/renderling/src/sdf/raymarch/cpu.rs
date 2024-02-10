//! CPU-only raymarching / SDF stuff.

use core::{marker::PhantomData, ops::Deref};

use crabslab::{GrowableSlab, Id};
use glam::Vec3;

use crate::sdf::StackParam;

pub struct Stack<Ty> {
    params: Vec<StackParam>,
    _phantom: PhantomData<Ty>,
}

impl<T> Stack<T> {
    pub fn one(param: StackParam) -> Self {
        Stack {
            params: vec![param],
            _phantom: PhantomData,
        }
    }

    pub fn many(params: impl IntoIterator<Item = StackParam>) -> Self {
        Stack {
            params: params.into_iter().collect(),
            _phantom: PhantomData,
        }
    }
}

impl From<f32> for Stack<f32> {
    fn from(value: f32) -> Self {
        Stack::one(StackParam::Float(value.to_bits()))
    }
}

impl From<Vec3> for Stack<Vec3> {
    fn from(value: Vec3) -> Self {
        Stack::many([
            StackParam::Float(value.x.to_bits()),
            StackParam::Float(value.y.to_bits()),
            StackParam::Float(value.z.to_bits()),
        ])
    }
}

impl<T> From<Id<T>> for Stack<T> {
    fn from(value: Id<T>) -> Self {
        Stack::one(StackParam::Var(value.inner()))
    }
}

fn combine<L, R, C>(left: Stack<L>, right: Stack<R>) -> Stack<C> {
    let mut params = vec![];
    params.extend(right.params);
    params.extend(left.params);
    Stack {
        params,
        _phantom: PhantomData,
    }
}

pub trait Apply<Input> {
    type Output;

    fn apply(self, a: Stack<Input>) -> Stack<Self::Output>;
}

impl<A, B> Apply<A> for Stack<fn(A) -> B> {
    type Output = B;

    fn apply(self, a: Stack<A>) -> Stack<B> {
        combine(self, a)
    }
}

impl<A, B, C> Apply<A> for Stack<fn(A, B) -> C> {
    type Output = fn(B) -> C;

    fn apply(self, a: Stack<A>) -> Stack<Self::Output> {
        combine(self, a)
    }
}

impl<A, B, C, D> Apply<A> for Stack<fn(A, B, C) -> D> {
    type Output = fn(B, C) -> D;

    fn apply(self, a: Stack<A>) -> Stack<Self::Output> {
        combine(self, a)
    }
}

pub fn var<T>(id: Id<T>) -> Stack<T> {
    id.into()
}

pub fn constant<T>(t: T) -> Stack<T>
where
    T: Into<Stack<T>>,
{
    t.into()
}

pub fn input_position() -> Stack<Vec3> {
    Stack {
        params: vec![StackParam::InputPosition],
        _phantom: PhantomData,
    }
}

/// Takes position, radius
pub fn fn_sdf_sphere() -> Stack<fn(Vec3, f32) -> f32> {
    Stack {
        params: vec![StackParam::OpDistanceSphere],
        _phantom: PhantomData,
    }
}

pub fn sdf_sphere(position: impl Into<Stack<Vec3>>, radius: impl Into<Stack<f32>>) -> Stack<f32> {
    fn_sdf_sphere().apply(position.into()).apply(radius.into())
}

/// Takes position, normal and height.
pub fn fn_sdf_plane() -> Stack<fn(Vec3, Vec3, f32) -> f32> {
    Stack {
        params: vec![StackParam::OpDistancePlane],
        _phantom: PhantomData,
    }
}

pub fn sdf_plane(
    position: impl Into<Stack<Vec3>>,
    normal: impl Into<Stack<Vec3>>,
    height: impl Into<Stack<f32>>,
) -> Stack<f32> {
    fn_sdf_plane()
        .apply(position.into())
        .apply(normal.into())
        .apply(height.into())
}

/// Union the result of two sdf functions.
pub fn fn_union() -> Stack<fn(f32, f32) -> f32> {
    Stack {
        params: vec![StackParam::OpUnion],
        _phantom: PhantomData,
    }
}

pub fn fn_difference() -> Stack<fn(f32, f32) -> f32> {
    Stack {
        params: vec![StackParam::OpDifference],
        _phantom: PhantomData,
    }
}

impl<T> Deref for Stack<T> {
    type Target = [StackParam];

    fn deref(&self) -> &[StackParam] {
        &self.params
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cpu_stack_sanity() {
        let radius = 1.0;
        let distance_fn = sdf_sphere(input_position(), radius);
        assert_eq!(
            vec![
                StackParam::InputPosition,
                StackParam::Float(radius.to_bits()),
                StackParam::OpDistanceSphere
            ],
            distance_fn.params
        );
    }
}
