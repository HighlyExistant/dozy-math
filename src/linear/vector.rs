#![allow(unused)]
use num_traits::{Num, AsPrimitive};

use super::traits::{Number, FloatingPoint, UnsignedNumber};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}


/// ===========================================================
/// 
/// Implementation for Vector2
/// 
/// ===========================================================

///
/// These functions are available for all vectors.
/// 
impl<T> Vector2<T>  {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Clone + Copy> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Vector2<T>  {
    pub fn dot(&self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
    pub fn cross(&self, other: Self) -> T {
        (self.x * other.y) - (self.y * other.x)
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Vector2<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Vector2<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Vector2<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: (self.x * rhs.x), y: (self.y * rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Div for Vector2<T>  {
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: (self.x / rhs.x), y: (self.y / rhs.y) }
    }
    type Output = Self;
}

// Operations on scalar values
impl<T: Number> std::ops::Add<T> for Vector2<T>  {
    fn add(self, rhs: T) -> Self::Output {
        Self { x: (self.x + rhs), y: (self.y + rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub<T> for Vector2<T>  {
    fn sub(self, rhs: T) -> Self::Output {
        Self { x: (self.x - rhs), y: (self.y - rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<T> for Vector2<T>  {
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: (self.x * rhs), y: (self.y * rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Div<T> for Vector2<T>  {
    fn div(self, rhs: T) -> Self::Output {
        Self { x: (self.x / rhs), y: (self.y / rhs) }
    }
    type Output = Self;
}

impl<T: Number> std::ops::AddAssign for Vector2<T>  {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: Number> std::ops::SubAssign for Vector2<T>  {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T: Number> std::ops::MulAssign for Vector2<T>  {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl<T: Number> std::ops::DivAssign for Vector2<T>  {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

///
/// These functions are available for all floating point 
/// numbers.
/// 
impl<T: FloatingPoint> Vector2<T>  {
    pub fn magnitude(&self) -> T {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude }
    }
    pub fn project(&self, other: Self) -> Self {
        let vector = self.normalize();
        let t = vector.dot(other);
        Self { x: (vector.x * t), y: (vector.y * t) }
    }
    pub fn direction(&self) -> T {
        T::atan2(self.x, self.y)
    }
}


/// ===========================================================
/// 
/// Implementation for Vector3
/// 
/// ===========================================================

///
/// These functions are available for all vectors.
/// 
impl<T> Vector3<T>  {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Clone + Copy> From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value, z: value }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Vector3<T>  {
    pub fn dot(&self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Vector3<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Vector3<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Vector3<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: (self.x * rhs.x), y: (self.y * rhs.y), z: (self.z * rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Div for Vector3<T>  {
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: (self.x / rhs.x), y: (self.y / rhs.y), z: (self.z / rhs.z) }
    }
    type Output = Self;
}
// Operations on scalar values
impl<T: Number> std::ops::Add<T> for Vector3<T>  {
    fn add(self, rhs: T) -> Self::Output {
        Self { x: (self.x + rhs), y: (self.y + rhs), z: (self.z + rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub<T> for Vector3<T>  {
    fn sub(self, rhs: T) -> Self::Output {
        Self { x: (self.x - rhs), y: (self.y - rhs), z: (self.z - rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<T> for Vector3<T>  {
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: (self.x * rhs), y: (self.y * rhs), z: (self.z * rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Div<T> for Vector3<T>  {
    fn div(self, rhs: T) -> Self::Output {
        Self { x: (self.x / rhs), y: (self.y / rhs), z: (self.z / rhs) }
    }
    type Output = Self;
}

impl<T: Number> std::ops::AddAssign for Vector3<T>  {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<T: Number> std::ops::SubAssign for Vector3<T>  {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<T: Number> std::ops::MulAssign for Vector3<T>  {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl<T: Number> std::ops::DivAssign for Vector3<T>  {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

///
/// These functions are available for all floating point 
/// numbers.
/// 
impl<T: FloatingPoint> Vector3<T>  {
    pub fn magnitude(&self) -> T {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude, z: self.z / magnitude }
    }
    pub fn project(&self, other: Self) -> Self {
        let vector = self.normalize();
        let t = vector.dot(other);
        Self { x: (vector.x * t), y: (vector.y * t), z: (vector.z * t) }
    }
}


/// ===========================================================
/// 
/// Implementation for Vector4
/// 
/// ===========================================================

///
/// These functions are available for all vectors.
/// 
impl<T> Vector4<T>  {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Clone + Copy> From<T> for Vector4<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value, z: value, w: value }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Vector4<T>  {
    pub fn dot(&self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Vector4<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z), w: (self.w + rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Vector4<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z), w: (self.w - rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Vector4<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: (self.x * rhs.x), y: (self.y * rhs.y), z: (self.z * rhs.z), w: (self.w * rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Div for Vector4<T>  {
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: (self.x / rhs.x), y: (self.y / rhs.y), z: (self.z / rhs.z), w: (self.w / rhs.w) }
    }
    type Output = Self;
}
// Operations on scalar values

impl<T: Number> std::ops::Add<T> for Vector4<T>  {
    fn add(self, rhs: T) -> Self::Output {
        Self { x: (self.x + rhs), y: (self.y + rhs), z: (self.z + rhs), w: (self.w + rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub<T> for Vector4<T>  {
    fn sub(self, rhs: T) -> Self::Output {
        Self { x: (self.x - rhs), y: (self.y - rhs), z: (self.z - rhs), w: (self.w - rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<T> for Vector4<T>  {
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: (self.x * rhs), y: (self.y * rhs), z: (self.z * rhs), w: (self.w * rhs) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Div<T> for Vector4<T>  {
    fn div(self, rhs: T) -> Self::Output {
        Self { x: (self.x / rhs), y: (self.y / rhs), z: (self.z / rhs), w: (self.w / rhs) }
    }
    type Output = Self;
}

impl<T: Number> std::ops::AddAssign for Vector4<T>  {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
impl<T: Number> std::ops::SubAssign for Vector4<T>  {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
impl<T: Number> std::ops::MulAssign for Vector4<T>  {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}
impl<T: Number> std::ops::DivAssign for Vector4<T>  {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

///
/// These functions are available for all floating point 
/// numbers.
/// 
impl<T: FloatingPoint> Vector4<T>  {
    pub fn magnitude(&self) -> T {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude, z: self.z / magnitude, w: self.w / magnitude }
    }
    pub fn project(&self, other: Self) -> Self {
        let vector = self.normalize();
        let t = vector.dot(other);
        Self { x: (vector.x * t), y: (vector.y * t), z: (vector.z * t), w: (vector.w * t) }
    }
}

/// ===========================================================
/// 
/// Macro Implementations: The following implementations are extremely tedious and repetitive 
/// maybe even unnecessary but im bored so im leveraging macros.
/// 
/// ===========================================================

macro_rules! impl_fromvec2 {
    ($typea:ident, $typeb:ident) => {
        impl From<Vector2<$typea>> for Vector2<$typeb> {
            fn from(value: Vector2<$typea>) -> Self {
                Self { x: value.x as $typeb, y: value.y as $typeb }
            }
        }
    };
}
macro_rules! impl_fromvec3 {
    ($typea:ident, $typeb:ident) => {
        impl From<Vector3<$typea>> for Vector3<$typeb> {
            fn from(value: Vector3<$typea>) -> Self {
                Self { x: value.x as $typeb, y: value.y as $typeb, z: value.z as $typeb }
            }
        }
    };
}
macro_rules! impl_fromvec4 {
    ($typea:ident, $typeb:ident) => {
        impl From<Vector4<$typea>> for Vector4<$typeb> {
            fn from(value: Vector4<$typea>) -> Self {
                Self { x: value.x as $typeb, y: value.y as $typeb, z: value.z as $typeb, w: value.w as $typeb }
            }
        }
    };
}

macro_rules! impl_all_from {
    ($mac:ident, $typea:ident, $($typeb:ident),+) => {
        $(
            $mac!($typea, $typeb);
        )+
    };
}
impl_all_from!(impl_fromvec2, f32, f64, i8, i16, i32, i64, u8, u16, u32, u64);
impl_all_from!(impl_fromvec2, f64, f32, i8, i16, i32, i64, u8, u16, u32, u64);

impl_all_from!(impl_fromvec2, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_all_from!(impl_fromvec2, i16, i32, i64, u8, u16, u32, u64, f64, f32, i8);

impl_all_from!(impl_fromvec2, i32, i64, u8, u16, u32, u64, f32, f64, i8, i16);
impl_all_from!(impl_fromvec2, i64, u8, u16, u32, u64, f64, f32, i8, i16, i32);

impl_all_from!(impl_fromvec2, u8, u16, u32, u64, f32, f64, i8, i16, i32, i64);
impl_all_from!(impl_fromvec2, u16, u32, u64, f64, f32, i8, i16, i32, i64, u8);

impl_all_from!(impl_fromvec2, u32, u64, f32, f64, i8, i16, i32, i64, u8, u16);
impl_all_from!(impl_fromvec2, u64, f64, f32, i8, i16, i32, i64, u8, u16, u32);
// impl_fromvec2!(f32, f64);
// impl_fromvec2!(f64, f32);