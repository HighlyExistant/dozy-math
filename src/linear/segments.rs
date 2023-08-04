use std::ops::Sub;

use num_traits::{AsPrimitive, Float};

use crate::{Vector2, Number, SignedNumber, EuclideanGeometry};
use crate::linear::Vector;
use super::non_zero_sign;
use super::smoothing::lerp;
use super::{Transform, traits::FloatingPoint};
use crate::equations::{CubicFormula, CubicSolution, QuadraticSolution, QuadraticFormula, sd_bezier};

fn solve_cubic_normalized<T: FloatingPoint>(a: T, b: T, c: T) -> (i32, [T; 3])
    where f32: AsPrimitive<T> {
    let a2 = a*a;
    let mut q = T::one()/9.0.as_()*(a2-3.0.as_()*b);
    let r = T::one()/54.0.as_()*(a*(2.0.as_()*a2-9.0.as_()*b)+27.0.as_()*c);
    let r2 = r*r;
    let q3 = q*q*q;
    let a = a * (T::one()/3.0.as_());

    let  (mut x0, mut x1, mut x2) = (T::zero(), T::zero(), T::zero());
    if (r2 < q3) {
        let mut t = r/q3.sqrt();
        t = if t < -T::one() {-T::one()} else {T::one()};
        t = t.acos();
        q = -2.0.as_()*q.sqrt();
        x0 = q*(T::one()/3.0.as_()*t).cos()-a;
        x1 = q*(T::one()/3.0.as_()*(t+2.0.as_()*std::f32::consts::PI.as_())).cos()-a;
        x2 = q*(T::one()/3.0.as_()*(t-2.0.as_()*std::f32::consts::PI.as_())).cos()-a;
        return (3, [x0, x1, x2]);
    } else {
        let u = if r < T::zero() {
            T::one()
        } else {
            -T::one()
        } * (r.abs()+(r2-q3).sqrt()).powf(T::one()/3.0.as_());
        let v = if u == T::zero() {T::zero()} else { q / u };
        x0 = (u+v)-a;
        if (u == v || (u-v).abs() < 1e-12.as_()*(u+v).abs()) {
            x1 = -0.5.as_()*(u+v)-a;
            return (2, [x0, x1, x2]);
        }
        return (1, [x0, x1, x2]);
    }
}

fn solve_cubic<T: FloatingPoint>(a: T, b: T, c: T, d: T) -> (i32, [T; 3])
    where f32: AsPrimitive<T> {
    if a != T::zero() {
        let bn = b/a;
        if bn.abs() < 1e6.as_() {
            return solve_cubic_normalized(bn, c/a, d/a);
        } // Above this ratio, the numerical error gets larger than if we treated a as zero
    }
    match T::quadratic_formula(a, b, c) {
        QuadraticSolution::All(n) => (1, [n, T::zero(), T::zero()]),
        QuadraticSolution::OneReal(r) => (1, [r, T::zero(), T::zero()]),
        QuadraticSolution::TwoReal(r) => (2, [r[0], r[1], T::zero()]),
        _ => (0, [T::zero(); 3]),
    }
}


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
    fn len(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<Self::VectorType>;
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VectorType>;
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
    fn len(&self) -> usize { 2 }
    fn iter(&self) -> std::slice::Iter<'_, T> { self.points.iter() }
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VectorType> { self.points.iter_mut() }
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
    fn len(&self) -> usize { 3 }
    fn iter(&self) -> std::slice::Iter<'_, T> { self.points.iter() }
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VectorType> { self.points.iter_mut() }
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
    fn len(&self) -> usize { 4 }
    fn iter(&self) -> std::slice::Iter<'_, T> { self.points.iter() }
    fn iter_mut(&mut self) -> std::slice::IterMut<Self::VectorType> { self.points.iter_mut() }
    type VectorType = T;
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
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T>, {
    fn direction(&self, t: <<Self as Segment>::VectorType as Vector>::Scalar) -> <Self as Segment>::VectorType
            where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        let tangent = lerp(self.points[1]-self.points[0], self.points[2]-self.points[1], t);
        if (tangent.x == T::zero() && tangent.y == T::zero()) {
            return self.points[2]-self.points[0];
        }
        tangent
    }
    #[cfg(not(feature = "experimental"))]
    fn signed_distance(&self, p: <Self as Segment>::VectorType) -> <<Self as Segment>::VectorType as Vector>::Scalar
        where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        let qa = *self.get_point(0)-p;
        let ab = *self.get_point(1)-*self.get_point(0);
        let br = *self.get_point(2)-*self.get_point(1)-ab;
        let a = br.dot(&br);
        let b = 3.0.as_()*ab.dot(&br);
        let c = 2.0.as_()*ab.dot(&ab)+qa.dot(&br);
        let d = qa.dot(&ab);
        let (solutions, t) = solve_cubic(a, b, c, d);
        let mut ep_dir = self.direction(T::zero());
        let mut min_distance = non_zero_sign(ep_dir.cross(qa))*qa.length(); // distance from A
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
        for i in 0..solutions as usize {
            if t[i] > T::zero() && t[i] < T::one() {
                let qe: Vector2<T> = qa + ab * 2.0.as_() * t[i] + br * t[i] * t[i];
                let distance = qe.length();
                if distance <= min_distance.abs() {
                    min_distance = (non_zero_sign((ab+br*t[i]).cross(qe)))*distance;
                    param = t[i];
                }
            }
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

    #[cfg(feature = "experimental")]
    fn signed_distance(&self, p: <Self as Segment>::VectorType) -> <<Self as Segment>::VectorType as Vector>::Scalar
            where <<Self as Segment>::VectorType as Vector>::Scalar: FloatingPoint {
        sd_bezier(p, self.points[0], self.points[1], self.points[2])// * 1.1.as_()
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