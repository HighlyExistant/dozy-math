use std::ops::Sub;

use num_traits::{AsPrimitive, Float};

use crate::{Vector2, Number, SignedNumber, EuclideanGeometry};
use crate::linear::Vector;
use super::smoothing::lerp;
use super::{Transform, traits::FloatingPoint};
use crate::equations::{CubicFormula, CubicSolution};
// functions for n dimensional segments
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
// functions for 2 dimensional segments
pub trait FlatSegment: Segment {
    fn direction(&self, t: <<Self as Segment>::VectorType as Vector>::Scalar) -> <Self as Segment>::VectorType
        where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint;
    fn signed_distance(&self, p: <Self as Segment>::VectorType) -> <<Self as Segment>::VectorType as Vector>::Scalar
        where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint;
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
fn non_zero_sign<T: FloatingPoint>(n: T) -> T
    where f32: AsPrimitive<T> {
    2.0.as_()*((n > T::zero()) as i32 as f32).as_()-T::one()
}
impl<T: Number + 'static> FlatSegment for LinearSegment<Vector2<T>>
    where f32: AsPrimitive<T>, f64: AsPrimitive<T> {
    fn signed_distance(&self, p: <Self as Segment>::VectorType) -> <<Self as Segment>::VectorType as Vector>::Scalar
        where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {

        let aq = p-*self.get_point(0);
        let ab = *self.get_point(1)-*self.get_point(0);
        let param = aq.dot(&ab)/ab.dot(&ab);
        let eq = *self.get_point((param > 0.5.as_()) as usize)-p;
        let endpoint_distance = eq.length();
        if param > T::zero() && param < T::one() {
            let ortho_distance = ab.get_orthonomal(false, false).dot(&aq);
            if ortho_distance.abs() < endpoint_distance {
                return ortho_distance;
            }
        }
        return (non_zero_sign(aq.cross(ab))) * endpoint_distance;
    }
    fn direction(&self, t: <<Self as Segment>::VectorType as Vector>::Scalar) -> <Self as Segment>::VectorType
            where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        *self.end() - *self.start()
    }
}

impl<T: Number + 'static> FlatSegment for QuadraticSegment<Vector2<T>>
    where f32: AsPrimitive<T> {
    fn direction(&self, t: <<Self as Segment>::VectorType as Vector>::Scalar) -> <Self as Segment>::VectorType
            where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        let tangent = lerp(self.points[1]-self.points[0], self.points[2]-self.points[1], t);
        if (tangent.x == T::zero() && tangent.y == T::zero()) {
            return self.points[2]-self.points[0];
        }
        tangent
    }
    fn signed_distance(&self, p: <Self as Segment>::VectorType) -> <<Self as Segment>::VectorType as Vector>::Scalar
        where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        let qa = *self.get_point(0)-p;
        let ab = *self.get_point(1)-*self.get_point(0);
        let br = *self.get_point(2)-*self.get_point(1)-ab;
        let a = br.dot(&br);
        let b = 3.0.as_()*ab.dot(&br);
        let c = 2.0.as_()*ab.dot(&ab)+qa.dot(&br);
        let d = qa.dot(&ab);
        let solution = T::cubic_formula(a, b, c, d);
        let mut ep_dir = self.direction(T::zero());
        let mut min_distance = (non_zero_sign(ep_dir.cross(qa)))*qa.length(); // distance from A
        let mut param = -qa.dot(&ep_dir)/ep_dir.dot(&ep_dir);
        {
            ep_dir = self.direction(T::one());
            let distance = (*self.get_point(2)-p).length(); // distance from B
            if distance < min_distance.abs() {
                let crossepdir = ep_dir.cross(*self.get_point(2)-p);
                min_distance = (non_zero_sign(crossepdir))*distance;
                param = (p-*self.get_point(1)).dot(&ep_dir)/ep_dir.dot(&ep_dir);
            }
        }
        let mut calculate_distance = |sol: T, min_distance: T, param: T| {
            if sol > T::zero() && sol < T::one() {
                let qe = qa + ab * 2.0.as_() * sol + br * sol * sol;
                let distance = qe.length();
                if distance <= min_distance.abs() {
                    let new_min_distance = (non_zero_sign((ab+br*sol).cross(qe)))*distance;
                    let new_param = sol;
                    return (new_min_distance, new_param);
                }
            }
            return (min_distance, param);
        };
        match solution {
            CubicSolution::ThreeReal(sol) => for i in sol { (min_distance, param) = calculate_distance(i, min_distance, param); }
            CubicSolution::TwoReal(sol) => for i in sol { (min_distance, param) = calculate_distance(i, min_distance, param); }
            CubicSolution::OneReal(sol) | CubicSolution::All(sol) | CubicSolution::OneRealTwoComplex(sol, _) => (min_distance, param) = calculate_distance(sol, min_distance, param),
            _ => {}
        }
        if param >= T::zero() && param <= T::one() {
            return min_distance;
        }
        if param < 0.5.as_() {
            return min_distance;
        }
        else {
            return min_distance;
        }
    }
}

// impl<T: Number + 'static> FlatSegment for CubicSegment<Vector2<T>>
//     where f32: AsPrimitive<T>, f64: AsPrimitive<T> {
//     fn direction(&self, t: <<Self as Segment>::VectorType as Vector>::Scalar) -> <Self as Segment>::VectorType
//             where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
//         let tangent = lerp(lerp(self.points[1]-self.points[0], self.points[2]-self.points[1], t), lerp(self.points[2]-self.points[1], self.points[3]-self.points[2], t), t);
//         if tangent.x == T::zero() && tangent.y == T::zero() {
//             if t == T::zero() { return self.points[2]-self.points[0] }
//             if t == T::one() { return self.points[3]-self.points[1] }
//         }
//         tangent
//     }
//     fn signed_distance(&self, p: <Self as Segment>::VectorType) -> <<Self as Segment>::VectorType as Vector>::Scalar
//             where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        
//     }
// }