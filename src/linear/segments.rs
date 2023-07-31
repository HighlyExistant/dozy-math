use crate::linear::Vector;
use super::smoothing::lerp;
use super::{Transform, traits::FloatingPoint};

pub trait Segment {
    fn interpolate(&self, t: <Self::VectorType as Vector>::Scalar) -> Self::VectorType
        where <Self::VectorType as Vector>::Scalar: FloatingPoint;
    fn start(&self) -> &Self::VectorType;
    fn end(&self) -> &Self::VectorType;
    // fn transform<T: Transform>(&mut self, transform: &T);
    fn get_point(&self, idx: usize) -> &Self::VectorType;
    fn set_start(&mut self, value: &Self::VectorType);
    fn set_end(&mut self, value: &Self::VectorType);
    // fn distance(&self, p: &Self::VectorType) -> <Self::VectorType as Vector>::Scalar
    //     where <Self::VectorType as Vector>::Scalar: FloatingPoint;
    type VectorType: Vector;
}

pub struct LinearSegment<T: Vector> {
    points: [T; 2]
}
pub struct QuadraticSegment<T: Vector> {
    points: [T; 3]
}
pub struct CubicSegment<T: Vector> {
    points: [T; 4]
}

impl<T: Vector> LinearSegment<T> {
    pub fn new(v0: T, v1: T) -> Self {
        Self { points: [v0, v1] }
    }
}
impl<T: Vector> QuadraticSegment<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Self {
        Self { points: [v0, v1, v2] }
    }
}
impl<T: Vector> CubicSegment<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Self { points: [v0, v1, v2, v3] }
    }
}

impl<T: Vector> Segment for LinearSegment<T> {
    fn start(&self) -> &Self::VectorType {
        &self.points[0]
    }
    fn end(&self) -> &Self::VectorType {
        &self.points[1]
    }
    fn get_point(&self, idx: usize) -> &Self::VectorType {
        &self.points[idx]
    }
    fn set_start(&mut self, value: &Self::VectorType) {
        self.points[0] = *value;
    }
    fn set_end(&mut self, value: &Self::VectorType) {
        self.points[1] = *value;
    }
    fn interpolate(&self, t: <Self::VectorType as Vector>::Scalar) -> Self::VectorType
        where <Self::VectorType as Vector>::Scalar: FloatingPoint {
        lerp(self.points[0], self.points[1], t)
    }
    type VectorType = T;
}
impl<T: Vector> Segment for QuadraticSegment<T> {
    fn start(&self) -> &Self::VectorType {
        &self.points[0]
    }
    fn end(&self) -> &Self::VectorType {
        &self.points[2]
    }
    fn get_point(&self, idx: usize) -> &Self::VectorType {
        &self.points[idx]
    }
    fn set_start(&mut self, value: &Self::VectorType) {
        self.points[0] = *value;
    }
    fn set_end(&mut self, value: &Self::VectorType) {
        self.points[2] = *value;
    }
    fn interpolate(&self, t: <Self::VectorType as Vector>::Scalar) -> Self::VectorType
            where <Self::VectorType as Vector>::Scalar: FloatingPoint {
        lerp(lerp(self.points[0], self.points[1], t), lerp(self.points[1], self.points[2], t), t)
    }
    type VectorType = T;
}
impl<T: Vector> Segment for CubicSegment<T> {
    fn start(&self) -> &Self::VectorType {
        &self.points[0]
    }
    fn end(&self) -> &Self::VectorType {
        &self.points[3]
    }
    fn get_point(&self, idx: usize) -> &Self::VectorType {
        &self.points[idx]
    }
    fn set_start(&mut self, value: &Self::VectorType) {
        self.points[0] = *value;
    }
    fn set_end(&mut self, value: &Self::VectorType) {
        self.points[3] = *value;
    }
    fn interpolate(&self, t: <Self::VectorType as Vector>::Scalar) -> Self::VectorType
            where <Self::VectorType as Vector>::Scalar: FloatingPoint {
        let p12 = lerp(self.points[1], self.points[2], t);
        lerp(lerp(lerp(self.points[0], self.points[1], t), p12, t), lerp(p12, lerp(self.points[2], self.points[3], t), t), t)
    }
    type VectorType = T;
}
