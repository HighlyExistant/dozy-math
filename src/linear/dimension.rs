use std::marker::PhantomData;

use crate::{Vector, Vector3, Number, Vector2};

pub trait Dimension {
    type RotationRepresentation: Sized;
    type VectorRepresentation: Vector + Sized;
}

pub struct SolidDimension<T: Number> {
    phantom: PhantomData<T>
}
pub struct FlatDimension<T: Number> {
    phantom: PhantomData<T>
}
impl<T: Number> Dimension for SolidDimension<T> {
    type RotationRepresentation = Vector3<T>;
    type VectorRepresentation = Vector3<T>;
}
impl<T: Number> Dimension for FlatDimension<T> {
    type RotationRepresentation = f32;
    type VectorRepresentation = Vector2<T>;
}