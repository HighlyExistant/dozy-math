use crate::linear::Vector;

use super::Transform;

pub trait Segment {
    fn interpolate(&self, t: <Self::VectorType as Vector>::Scalar) -> Self::VectorType;
    fn start(&self) -> Self::VectorType;
    fn end(&self) -> Self::VectorType;
    fn transform<T: Transform>(&mut self, transform: &T);
    fn get_point(&self, idx: usize) -> &Self::VectorType;
    type VectorType: Vector;
}