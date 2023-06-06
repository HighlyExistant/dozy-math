use crate::linear::{vector::Vector3, traits::{Number, FloatingPoint}};


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
        let x = f32::sin(v.x/2.0) * f32::cos(v.y/2.0) * f32::cos(v.z/2.0) - f32::cos(v.x/2.0) * f32::sin(v.y/2.0) * f32::sin(v.z/2.0);
        let y = f32::cos(v.x/2.0) * f32::sin(v.y/2.0) * f32::cos(v.z/2.0) + f32::sin(v.x/2.0) * f32::cos(v.y/2.0) * f32::sin(v.z/2.0);
        let z = f32::cos(v.x/2.0) * f32::cos(v.y/2.0) * f32::sin(v.z/2.0) - f32::sin(v.x/2.0) * f32::sin(v.y/2.0) * f32::cos(v.z/2.0);
        let w = f32::cos(v.x/2.0) * f32::cos(v.y/2.0) * f32::cos(v.z/2.0) + f32::sin(v.x/2.0) * f32::sin(v.y/2.0) * f32::sin(v.z/2.0);
        Self { vector: Vector3 { x, y, z }, scalar: w }
    }
}
impl Quaternion<f64>  {
    pub fn from_euler(v: Vector3<f64>) -> Self {
        let x = f64::sin(v.x/2.0) * f64::cos(v.y/2.0) * f64::cos(v.z/2.0) - f64::cos(v.x/2.0) * f64::sin(v.y/2.0) * f64::sin(v.z/2.0);
        let y = f64::cos(v.x/2.0) * f64::sin(v.y/2.0) * f64::cos(v.z/2.0) + f64::sin(v.x/2.0) * f64::cos(v.y/2.0) * f64::sin(v.z/2.0);
        let z = f64::cos(v.x/2.0) * f64::cos(v.y/2.0) * f64::sin(v.z/2.0) - f64::sin(v.x/2.0) * f64::sin(v.y/2.0) * f64::cos(v.z/2.0);
        let w = f64::cos(v.x/2.0) * f64::cos(v.y/2.0) * f64::cos(v.z/2.0) + f64::sin(v.x/2.0) * f64::sin(v.y/2.0) * f64::sin(v.z/2.0);
        Self { vector: Vector3 { x, y, z }, scalar: w }
    }
}

// traits for bitwise operations
// impl<T: Number> std::ops::Mul for Quaternion<T>  {
//     fn mul(self, rhs: Self) -> Self::Output {
        
//     }
//     type Output = Self;
// }
// impl<T: Number> std::ops::MulAssign for Quaternion<T>  {
//     fn mul_assign(&mut self, rhs: Self) {
        
//     }
// }

///
/// These functions are available for all floating point 
/// numbers.
/// 
impl<T: FloatingPoint> Quaternion<T>  {
}

