use num_traits::Num;

use crate::linear::{vector::Vector3, traits::{Number, FloatingPoint}, self, matrix::Matrix3};


#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Quaternion<T> {
    pub vector: Vector3<T>,
    pub scalar: T,
}

/// ===========================================================
/// 
/// Implementation for Quaternion
/// 
/// ===========================================================

///
/// These functions are available for all vectors.
/// 
impl<T> Quaternion<T>  {
    pub fn new(v: Vector3<T>, s: T) -> Self {
        Self { vector: v, scalar: s }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl Quaternion<f32>  {
    pub fn from_euler(v: Vector3<f32>) -> Self {
        let x = f32::sin(v.x/2.0) * f32::cos(v.z/2.0) * f32::cos(v.y/2.0) - f32::cos(v.x/2.0) * f32::sin(v.z/2.0) * f32::sin(v.y/2.0);
        let y = f32::cos(v.x/2.0) * f32::sin(v.z/2.0) * f32::cos(v.y/2.0) + f32::sin(v.x/2.0) * f32::cos(v.z/2.0) * f32::sin(v.y/2.0);
        let z = f32::cos(v.x/2.0) * f32::cos(v.z/2.0) * f32::sin(v.y/2.0) - f32::sin(v.x/2.0) * f32::sin(v.z/2.0) * f32::cos(v.y/2.0);
        let w = f32::cos(v.x/2.0) * f32::cos(v.z/2.0) * f32::cos(v.y/2.0) + f32::sin(v.x/2.0) * f32::sin(v.z/2.0) * f32::sin(v.y/2.0);
        Self { vector: Vector3 { x, y, z }, scalar: w }
    }
}
impl Quaternion<f64>  {
    pub fn from_euler(v: Vector3<f64>) -> Self {
        let x = f64::sin(v.x/2.0) * f64::cos(v.z/2.0) * f64::cos(v.y/2.0) - f64::cos(v.x/2.0) * f64::sin(v.z/2.0) * f64::sin(v.y/2.0);
        let y = f64::cos(v.x/2.0) * f64::sin(v.z/2.0) * f64::cos(v.y/2.0) + f64::sin(v.x/2.0) * f64::cos(v.z/2.0) * f64::sin(v.y/2.0);
        let z = f64::cos(v.x/2.0) * f64::cos(v.z/2.0) * f64::sin(v.y/2.0) - f64::sin(v.x/2.0) * f64::sin(v.z/2.0) * f64::cos(v.y/2.0);
        let w = f64::cos(v.x/2.0) * f64::cos(v.z/2.0) * f64::cos(v.y/2.0) + f64::sin(v.x/2.0) * f64::sin(v.z/2.0) * f64::sin(v.y/2.0);
        Self { vector: Vector3 { x, y, z }, scalar: w }
    }
}

impl<T: Number> From<Quaternion<T>> for Matrix3<T> {
    fn from(value: Quaternion<T>) -> Self {
        let x2 = value.vector.x + value.vector.x;
        let y2 = value.vector.y + value.vector.y;
        let z2 = value.vector.z + value.vector.z;

        let xx2 = x2 * value.vector.x;
        let xy2 = x2 * value.vector.y;
        let xz2 = x2 * value.vector.z;

        let yy2 = y2 * value.vector.y;
        let yz2 = y2 * value.vector.z;
        let zz2 = z2 * value.vector.z;

        let sy2 = y2 * value.scalar;
        let sz2 = z2 * value.scalar;
        let sx2 = x2 * value.scalar;

        Self { 
            x: Vector3 { 
                x: T::one() - yy2 - zz2, 
                y: xy2 + sz2, 
                z: xz2 - sy2, 
            }, 
            y: Vector3 { 
                x: xy2 - sz2, 
                y: T::one() - xx2 - zz2, 
                z: yz2 + sx2, 
            }, 
            z: Vector3 { 
                x: xz2 + sy2, 
                y: yz2 - sx2, 
                z: T::one() - xx2 - yy2, 
            }, 
        }
    }
}
// traits for bitwise operations

impl<T: Number> std::ops::Mul<Vector3<T>> for Quaternion<T> {
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        let mat3 = linear::matrix::Matrix3::<T>::from(self);

        rhs * mat3
    }
    type Output = Vector3<T>;
}
