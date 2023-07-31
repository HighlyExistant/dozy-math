#![allow(unused)]

use core::prelude::v1;

mod traits;
mod matrix;
mod shapes;
mod vector;
mod rotation;
mod transform;
mod segments;
// mod sdf;
mod bbox;
mod geometry;
pub mod smoothing;
pub use shapes::*;
pub use matrix::*;
pub use vector::*;
pub use traits::*;
pub use bbox::*;
pub use geometry::*;
// pub use sdf::*;
pub use segments::*;
pub use rotation::*;
pub use transform::*;

// Vector types
pub type FVec2 = Vector2<f32>;
pub type DVec2 = Vector2<f64>;

pub type CVec2 = Vector2<i8>;
pub type SVec2 = Vector2<i16>;
pub type IVec2 = Vector2<i32>;
pub type LVec2 = Vector2<i64>;

pub type UCVec2 = Vector2<u8>;
pub type USVec2 = Vector2<u16>;
pub type UIVec2 = Vector2<u32>;
pub type ULVec2 = Vector2<u64>;
pub type FVec3 = Vector3<f32>;
pub type DVec3 = Vector3<f64>;

pub type CVec3 = Vector3<i8>;
pub type SVec3 = Vector3<i16>;
pub type IVec3 = Vector3<i32>;
pub type LVec3 = Vector3<i64>;

pub type UCVec3 = Vector3<u8>;
pub type USVec3 = Vector3<u16>;
pub type UIVec3 = Vector3<u32>;
pub type ULVec3 = Vector3<u64>;

pub type FVec4 = Vector4<f32>;
pub type DVec4 = Vector4<f64>;

pub type CVec4 = Vector4<i8>;
pub type SVec4 = Vector4<i16>;
pub type IVec4 = Vector4<i32>;
pub type LVec4 = Vector4<i64>;

pub type UCVec4 = Vector4<u8>;
pub type USVec4 = Vector4<u16>;
pub type UIVec4 = Vector4<u32>;
pub type ULVec4 = Vector4<u64>;

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

pub fn translate<T: Number>(m: &Matrix4<T>, v: Vector3<T>) -> Matrix4<T> {
    let mut result = *m;
    result.w = m.x * v.x + m.y * v.y + m.z * v.z + m.w;
    result
}

pub fn slope<T: Number>(a: Vector2<T>, b: Vector2<T>) -> T {
    (b.y - a.y) / (b.x - a.x)
}

pub fn barycentric_coordinates<T: Number>(start: Vector2<T>, end: Vector2<T>, control: Vector2<T>, p: Vector2<T>) -> Vector3<T> {
    let div = ((start.y - end.y) * (control.x - end.x) + (end.x - start.x) * (control.y - end.y));
    let w1 = ((start.y - end.y) * (p.x - end.x) + (end.x - start.x) * (p.y - end.y)) / div;
    let w2 = ((end.y   - control.y) * (p.x - end.x) + (control.x - end.x) * (p.y - end.y)) / div;
    let w3 = T::one() - w1 - w2;
    Vector3 { x: w1, y: w2, z: w3 }
}

pub fn line_sdf<T: FloatingPoint>(a: Vector2<T>, b: Vector2<T>, p: Vector2<T>) -> T {
    let pa = p - a;
    let negba = -b + a;
    let ba = b - a;
    (pa.x * negba.y + pa.y * ba.x) / ((negba.y * negba.y) + (ba.x * ba.x)).sqrt()
}

pub fn line_pseudo_sdf<T: FloatingPoint>(a: Vector2<T>, b: Vector2<T>, p: Vector2<T>) -> T {
    let pa = p - a;
    let negba = -b + a;
    let ba = b - a;
    (pa.x * negba.y + pa.y * ba.x)
}
