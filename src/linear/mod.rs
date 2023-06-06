#![allow(unused)]

use self::{traits::{FloatingPoint, Number}, matrix::Matrix4, vector::Vector3};
pub mod traits;
pub mod vector;
pub mod matrix;

// Vector types

pub type FVec2 = vector::Vector2<f32>;
pub type DVec2 = vector::Vector2<f64>;

pub type CVec2 = vector::Vector2<i8>;
pub type SVec2 = vector::Vector2<i16>;
pub type IVec2 = vector::Vector2<i32>;
pub type LVec2 = vector::Vector2<i64>;

pub type UCVec2 = vector::Vector2<u8>;
pub type USVec2 = vector::Vector2<u16>;
pub type UIVec2 = vector::Vector2<u32>;
pub type ULVec2 = vector::Vector2<u64>;

pub type FVec3 = vector::Vector3<f32>;
pub type DVec3 = vector::Vector3<f64>;

pub type CVec3 = vector::Vector3<i8>;
pub type SVec3 = vector::Vector3<i16>;
pub type IVec3 = vector::Vector3<i32>;
pub type LVec3 = vector::Vector3<i64>;

pub type UCVec3 = vector::Vector3<u8>;
pub type USVec3 = vector::Vector3<u16>;
pub type UIVec3 = vector::Vector3<u32>;
pub type ULVec3 = vector::Vector3<u64>;

pub type FVec4 = vector::Vector4<f32>;
pub type DVec4 = vector::Vector4<f64>;

pub type CVec4 = vector::Vector4<i8>;
pub type SVec4 = vector::Vector4<i16>;
pub type IVec4 = vector::Vector4<i32>;
pub type LVec4 = vector::Vector4<i64>;

pub type UCVec4 = vector::Vector4<u8>;
pub type USVec4 = vector::Vector4<u16>;
pub type UIVec4 = vector::Vector4<u32>;
pub type ULVec4 = vector::Vector4<u64>;

// Matrix Types

pub type FMat2 = matrix::Matrix2<f32>;
pub type DMat2 = matrix::Matrix2<f64>;

pub type CMat2 = matrix::Matrix2<i8>;
pub type SMat2 = matrix::Matrix2<i16>;
pub type IMat2 = matrix::Matrix2<i32>;
pub type LMat2 = matrix::Matrix2<i64>;

pub type UCMat2 = matrix::Matrix2<u8>;
pub type USMat2 = matrix::Matrix2<u16>;
pub type UIMat2 = matrix::Matrix2<u32>;
pub type ULMat2 = matrix::Matrix2<u64>;

pub type FMat3 = matrix::Matrix3<f32>;
pub type DMat3 = matrix::Matrix3<f64>;

pub type CMat3 = matrix::Matrix3<i8>;
pub type SMat3 = matrix::Matrix3<i16>;
pub type IMat3 = matrix::Matrix3<i32>;
pub type LMat3 = matrix::Matrix3<i64>;

pub type UCMat3 = matrix::Matrix3<u8>;
pub type USMat3 = matrix::Matrix3<u16>;
pub type UIMat3 = matrix::Matrix3<u32>;
pub type ULMat3 = matrix::Matrix3<u64>;

pub type FMat4 = matrix::Matrix4<f32>;
pub type DMat4 = matrix::Matrix4<f64>;

pub type CMat4 = matrix::Matrix4<i8>;
pub type SMat4 = matrix::Matrix4<i16>;
pub type IMat4 = matrix::Matrix4<i32>;
pub type LMat4 = matrix::Matrix4<i64>;

pub type UCMat4 = matrix::Matrix4<u8>;
pub type USMat4 = matrix::Matrix4<u16>;
pub type UIMat4 = matrix::Matrix4<u32>;
pub type ULMat4 = matrix::Matrix4<u64>;

/// # rotate
/// 
/// perform a rotation on a Floating Point Matrix4 on an axis specified by v.
/// 
/// # Example
/// ```
/// 
/// let mut transform = FMat4::identity(1.0);
/// // Rotating along the y axis
/// transform = rotate(&transform, self.rotation.y, FVec3::new(0.0, 1.0, 0.0));
/// // Rotating along the x axis
/// transform = rotate(&transform, self.rotation.y, FVec3::new(1.0, 0.0, 0.0));
/// // Rotating along the z axis
/// transform = rotate(&transform, self.rotation.y, FVec3::new(0.0, 0.0, 1.0));
/// ```
/// This function is heavily influenced from the implementation [glm](https://github.com/g-truc/glm) uses.
/// other resources from [wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix).
pub fn rotate<T: FloatingPoint + Default>(m: &Matrix4<T>, angle: T, v: Vector3<T>) -> Matrix4<T> {
    let a = angle;
    let c = a.cos();
    let s = a.sin();

    let axis = v.normalize();
    let temp: Vector3<T> = axis * (T::one() - c) ;

    let mut rotate = Matrix4::<T>::default();
    rotate.x.x = c + temp.x * axis.x;
    rotate.x.y = temp.x * axis.y + s * axis.z;
    rotate.x.z = temp.x * axis.z - s * axis.y;

    rotate.y.x = temp.y * axis.x - s * axis.z;
    rotate.y.y = c + temp.y * axis.y;
    rotate.y.z = temp.y * axis.z + s * axis.x;

    rotate.z.x = temp.z * axis.x + s * axis.y;
    rotate.z.y = temp.z * axis.y - s * axis.x;
    rotate.z.z = c + temp.z * axis.z;

    let mut result = Matrix4::<T>::default();
    result.x = m.x * rotate.x.x + m.y * rotate.x.y + m.z * rotate.x.z;
    result.y = m.x * rotate.y.x + m.y * rotate.y.y + m.z * rotate.y.z;
    result.z = m.x * rotate.z.x + m.y * rotate.z.y + m.z * rotate.z.z;
    result.w = m.w;
    result
}

pub fn translate<T: Number>(m: &Matrix4<T>, v: Vector3<T>) -> Matrix4<T> {
    let mut result = *m;
    result.w = m.x * v.x + m.y * v.y + m.z * v.z + m.w;
    result
}