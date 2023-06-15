#![allow(unused)]
use std::ops::Add;

use num_traits::{Num, AsPrimitive, Zero};

use super::{traits::{Number, FloatingPoint, UnsignedNumber}, matrix::{Matrix2, Matrix3, Matrix4}};

pub trait Vector: Copy + Clone {
    fn size() -> usize;
}

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
impl<T: Clone + Copy + Number> Vector for Vector2<T> {
    fn size() -> usize {
        2
    }
}

impl<T: Number> Zero for Vector2<T>  {
    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
    fn set_zero(&mut self) {
        self.x = T::zero();
        self.y = T::zero();
    }
    fn zero() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Vector2<T>  {
    pub fn cross(&self, other: Self) -> T {
        (self.x * other.y) - (self.y * other.x)
    }
    pub fn dot(&self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<T: Number> std::ops::Mul<Matrix2<T>> for Vector2<T>  {
    /// # Multiplying Vector2 with Matrix2
    /// 
    /// when you multiply a Vector2 with a Matrix2 we treat the vector
    /// as a 1x2 matrix * 2x2 matrix since it is impossible to multiply
    /// a 2x2 matrix * 1x2 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Matrix2<T>) -> Self::Output {
        Vector2::<T> {
            x: self.x * rhs.x.x + self.y * rhs.y.x,
            y: self.x * rhs.x.y + self.y * rhs.y.y
        }
    }
    type Output = Vector2<T>;
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

impl<T: Clone + Copy + Number> Vector for Vector3<T> {
    fn size() -> usize {
        3
    }
}

impl<T: Number> Zero for Vector3<T>  {
    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero()
    }
    fn set_zero(&mut self) {
        self.x = T::zero();
        self.y = T::zero();
        self.z = T::zero();
    }
    fn zero() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::zero() }
    }
}
///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Vector3<T>  {
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
    pub fn dot(&self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn xy(&self) -> Vector2<T> {
        Vector2 { x: self.x, y: self.y }
    }
}
impl<T: Number> std::ops::Mul<Matrix3<T>> for Vector3<T>  {
    /// # Multiplying Vector3 with Matrix3
    /// 
    /// when you multiply a Vector3 with a Matrix3 we treat the vector
    /// as a 1x3 matrix * 3x3 matrix since it is impossible to multiply
    /// a 3x3 matrix * 1x3 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Matrix3<T>) -> Self::Output {
        Vector3::<T> {
            x: self.x * rhs.x.x + self.y * rhs.y.x + self.z * rhs.z.x,
            y: self.x * rhs.x.y + self.y * rhs.y.y + self.z * rhs.z.y,
            z: self.x * rhs.x.z + self.y * rhs.y.z + self.z * rhs.z.z
        }
    }
    type Output = Vector3<T>;
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

impl<T: Clone + Copy + Number> Vector for Vector4<T> {
    fn size() -> usize {
        4
    }
}
impl<T: Number> Zero for Vector4<T>  {
    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero() && self.w == T::zero()
    }
    fn set_zero(&mut self) {
        self.x = T::zero();
        self.y = T::zero();
        self.z = T::zero();
        self.w = T::zero();
    }
    fn zero() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() }
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
    
    pub fn xyz(&self) -> Vector3<T> {
        Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

impl<T: Number> std::ops::Mul<Matrix4<T>> for Vector4<T>  {
    /// # Multiplying Vector4 with Matrix4
    /// 
    /// when you multiply a Vector4 with a Matrix4 we treat the vector
    /// as a 1x4 matrix * 4x4 matrix since it is impossible to multiply
    /// a 4x4 matrix * 1x4 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Matrix4<T>) -> Self::Output {
        Vector4::<T> {
            x: self.x * rhs.x.x + self.y * rhs.y.x + self.z * rhs.z.x + self.w * rhs.w.x,
            y: self.x * rhs.x.y + self.y * rhs.y.y + self.z * rhs.z.y + self.w * rhs.w.y,
            z: self.x * rhs.x.z + self.y * rhs.y.z + self.z * rhs.z.z + self.w * rhs.w.z,
            w: self.x * rhs.x.w + self.y * rhs.y.w + self.z * rhs.z.w + self.w * rhs.w.w
        }
    }
    type Output = Vector4<T>;
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
macro_rules! impl_ops {
    ($vector:ident , $($element:tt),+) => {
        impl<T: Number> Add for $vector <T>  {
            fn add(self, rhs: Self) -> Self::Output {
                
                Self { $($element : self.$element + rhs.$element),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Sub for $vector<T>  {
            fn sub(self, rhs: Self) -> Self::Output {
                Self { $($element: self.$element - rhs.$element),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Mul for $vector<T>  {
            fn mul(self, rhs: Self) -> Self::Output {
                Self { $($element: self.$element * rhs.$element),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Div for $vector<T>  {
            fn div(self, rhs: Self) -> Self::Output {
                Self { $($element: self.$element / rhs.$element),+ }
            }
            type Output = Self;
        }

        // Operations on scalar values
        impl<T: Number> std::ops::Add<T> for $vector<T>  {
            fn add(self, rhs: T) -> Self::Output {
                Self { $($element: self.$element + rhs),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Sub<T> for $vector<T>  {
            fn sub(self, rhs: T) -> Self::Output {
                Self { $($element: self.$element - rhs),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Mul<T> for $vector<T>  {
            fn mul(self, rhs: T) -> Self::Output {
                Self { $($element: self.$element * rhs),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Div<T> for $vector<T>  {
            fn div(self, rhs: T) -> Self::Output {
                Self { $($element: self.$element / rhs),+ }
            }
            type Output = Self;
        }

        impl<T: Number> std::ops::AddAssign for $vector<T>  {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$element += rhs.$element);+
            }
        }
        impl<T: Number> std::ops::SubAssign for $vector<T>  {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$element -= rhs.$element);+
            }
        }
        impl<T: Number> std::ops::MulAssign for $vector<T>  {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$element *= rhs.$element);+
            }
        }
        impl<T: Number> std::ops::DivAssign for $vector<T>  {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$element /= rhs.$element);+
            }
        }
        impl<T: Number> std::cmp::PartialEq<T>  for $vector<T>  {
            fn eq(&self, other: &T) -> bool {
                true $(&& self.$element == *other)+
            }
            fn ne(&self, other: &T) -> bool {
                true $(&& self.$element == *other)+
            }
        }
    };
}

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
macro_rules! impl_all_op {
    ($mac:ident, $op:ident, $typea:ident, $($typeb:ident),+) => {
        $(
            $mac!($typea, $typeb);
        )+
    };
}
impl_ops!(Vector2, x, y);
impl_ops!(Vector3, x, y, z);
impl_ops!(Vector4, x, y, z, w);

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