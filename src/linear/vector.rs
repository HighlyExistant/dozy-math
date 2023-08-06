use num_traits::{Zero, Signed, Num, NumOps, Float};

use std::{ops::{Add, AddAssign, SubAssign, DivAssign, MulAssign, RemAssign}, iter::Sum};

use super::{matrix::{Matrix4, Matrix3, Matrix2}, traits::{FloatingPoint, Number, SignedNumber}, geometry::EuclideanGeometry};
pub trait Vector: 
    Copy + Clone + 
    AddAssign + SubAssign + DivAssign + MulAssign + 
    PartialEq + 
    std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> +
    std::ops::Add<Self::Scalar, Output = Self> + std::ops::Sub<Self::Scalar, Output = Self> + std::ops::Mul<Self::Scalar, Output = Self> + std::ops::Div<Self::Scalar, Output = Self> {
    type Scalar: Number;
    fn size() -> usize;
    fn dot(&self, other: &Self) -> Self::Scalar;
    fn magnitude(&self) -> Self::Scalar
        where <Self as Vector>::Scalar: FloatingPoint;
    fn normalize(&self) -> Self
        where <Self as Vector>::Scalar: FloatingPoint;
    fn project(&self, other: &Self) -> Self
        where <Self as Vector>::Scalar: FloatingPoint;
    /// the distance between two vectors is calculated the same way as the length
    /// in euclidean geometry so this function can be used the same way.
    fn length(&self) -> <Self as Vector>::Scalar
    where <Self as Vector>::Scalar: FloatingPoint;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
#[cfg(feature = "glsl")]
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
#[repr(C)]
#[cfg(not(feature = "glsl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}
impl<T: Clone + Copy> Vector2<T> {
}
impl<T: Clone + Copy> Vector3<T> {
    #[inline]
    pub fn xy(&self) -> Vector2<T> {
        Vector2 { x: self.x, y: self.y }
    }
}
impl<T: Clone + Copy> Vector4<T> {
    #[inline]
    pub fn xyz(&self) -> Vector3<T> {
        Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

impl<T: Clone + Copy> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value }
    }
}
impl<T: Clone + Copy> From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value, z: value }
    }
}
impl<T: Clone + Copy> From<T> for Vector4<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value, z: value, w: value }
    }
}
impl<T: FloatingPoint> Vector2<T> {
    pub fn get_orthonomal(&self, polarity: bool, allow_zero: bool) -> Self {
        let length = self.length();
        if length.is_zero() {
            if polarity {
                Self { x: T::zero(), y: if allow_zero { T::zero() } else { T::one()} }
            } else {
                Self { x: T::zero(), y: if allow_zero { T::zero() } else { -T::one() } }
            }
        } else {
            if polarity {
                Self { x: -self.y/length, y: self.x/length }
            } else {
                Self { x: self.y/length, y: -self.x/length }
            }
        }
    }
    pub fn pow(&self, n: Self) -> Self {
        Self::new(self.x.powf(n.x), self.y.powf(n.y))
    }
}
impl<T: FloatingPoint> Vector3<T>  {
    pub fn pow(&self, n: Self) -> Self {
        Self::new(self.x.powf(n.x), self.y.powf(n.y), self.z.powf(n.z))
    }
}
impl<T: FloatingPoint> Vector4<T>  {
    pub fn pow(&self, n: Self) -> Self {
        Self::new(self.x.powf(n.x), self.y.powf(n.y), self.z.powf(n.z), self.w.powf(n.w))
    }
}
impl<T: Number> Vector for Vector2<T> {
    type Scalar = T;
    fn size() -> usize {
        2
    }
    fn dot(&self, other: &Self) -> Self::Scalar {
        (self.x * other.x) + (self.y * other.y)
    }
    fn magnitude(&self) -> Self::Scalar
            where <Self as Vector>::Scalar: FloatingPoint {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
    fn project(&self, other: &Self) -> Self
            where <Self as Vector>::Scalar: FloatingPoint {
        let vector = self.normalize();
        let t = vector.dot(other);
        Self { x: (vector.x * t), y: (vector.y * t) }
    }
    fn normalize(&self) -> Self
            where <Self as Vector>::Scalar: FloatingPoint {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude }
    }
    fn length(&self) -> Self::Scalar
            where Self::Scalar: FloatingPoint {
        self.dot(self).sqrt()
    }
}
impl<T: Number> Vector for Vector3<T> {
    type Scalar = T;
    fn size() -> usize {
        3
    }
    fn dot(&self, other: &Self) -> Self::Scalar {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    fn magnitude(&self) -> Self::Scalar
            where <Self as Vector>::Scalar: FloatingPoint {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
    fn normalize(&self) -> Self
            where <Self as Vector>::Scalar: FloatingPoint {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude, z: self.z / magnitude }
    }
    fn project(&self, other: &Self) -> Self
            where <Self as Vector>::Scalar: FloatingPoint {
        let vector = self.normalize();
        let t = vector.dot(other);
        Self { x: (vector.x * t), y: (vector.y * t), z: (vector.z * t) }
    }
    fn length(&self) -> Self::Scalar
            where Self::Scalar: FloatingPoint {
        self.dot(self).sqrt()
    }
    
}
impl<T: Number> From<Vector2<T>> for Vector3<T> {
    fn from(value: Vector2<T>) -> Self {
        Self { x: value.x, y: value.y, z: T::one() }
    }
}
impl<T: Number> Vector for Vector4<T> {
    type Scalar = T;
    fn size() -> usize {
        4
    }
    fn dot(&self, other: &Self) -> Self::Scalar {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }
    fn magnitude(&self) -> Self::Scalar
            where <Self as Vector>::Scalar: FloatingPoint {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }
    fn normalize(&self) -> Self
            where <Self as Vector>::Scalar: FloatingPoint {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude, z: self.z / magnitude, w: self.w / magnitude }
    }
    fn project(&self, other: &Self) -> Self
            where <Self as Vector>::Scalar: FloatingPoint {
        let vector = self.normalize();
        let t = vector.dot(other);
        Self { x: (vector.x * t), y: (vector.y * t), z: (vector.z * t), w: (vector.w * t) }
    }
    fn length(&self) -> Self::Scalar
            where Self::Scalar: FloatingPoint {
        self.dot(self).sqrt()
    }
}

impl<T: Number> EuclideanGeometry for Vector2<T> {
    type CrossProduct = T;

    fn cross(&self, other: Self) -> Self::CrossProduct {
        (self.x * other.y) - (self.y * other.x)
    }
}
impl<T: Number> EuclideanGeometry for Vector3<T> {
    type CrossProduct = Self;
    
    fn cross(&self, other: Self) -> Self::CrossProduct {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
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

macro_rules! impl_ops {
    ($vector:ident , $($element:tt),+) => {
        impl<T: Number> Add for $vector <T>  {
            fn add(self, rhs: Self) -> Self::Output {
                
                Self { $($element : self.$element + rhs.$element),+ }
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Rem for $vector<T>  {
            fn rem(self, rhs: Self) -> Self::Output {
                Self { $($element : self.$element % rhs.$element),+ }
            }
            type Output = Self;
        }
        
        impl<T: SignedNumber> std::ops::Neg for $vector<T> {
            fn neg(self) -> Self::Output {
                Self { $($element : -self.$element),+ }
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

macro_rules! impl_all_from_vec {
    ($mac:ident) => {
        impl_all_from!($mac, f32, f64, i8, i16, i32, i64, u8, u16, u32, u64, usize);
        impl_all_from!($mac, f64, f32, i8, i16, i32, i64, u8, u16, u32, u64, usize);

        impl_all_from!($mac, i8, i16, i32, i64, u8, u16, u32, u64, usize, f32, f64);
        impl_all_from!($mac, i16, i32, i64, u8, u16, u32, u64, usize, f64, f32, i8);

        impl_all_from!($mac, i32, i64, u8, u16, u32, u64, usize, f32, f64, i8, i16);
        impl_all_from!($mac, i64, u8, u16, u32, u64, usize, f64, f32, i8, i16, i32);

        impl_all_from!($mac, u8, u16, u32, u64, usize, f32, f64, i8, i16, i32, i64);
        impl_all_from!($mac, u16, u32, u64, usize, f64, f32, i8, i16, i32, i64, u8);

        impl_all_from!($mac, u32, u64, usize, f32, f64, i8, i16, i32, i64, u8, u16);
        impl_all_from!($mac, u64, usize, f64, f32, i8, i16, i32, i64, u8, u16, u32);
        
        impl_all_from!($mac, usize, u64, f64, f32, i8, i16, i32, i64, u8, u16, u32);
    };
}
impl_all_from_vec!(impl_fromvec2);
impl_all_from_vec!(impl_fromvec3);
impl_all_from_vec!(impl_fromvec4);

impl<T: Number> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: T::zero() }
    }
}
impl<T: Number> From<Vector4<T>> for Vector3<T> {
    fn from(value: Vector4<T>) -> Self {
        Self { x: value.x, y: value.y, z: value.z }
    }
}
impl<T: Number> Sum for Vector2<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector2::zero(), |a: Self, b: Self| { a + b })
    }
}

macro_rules! impl_num_traits {
    ($structure:ident, $($element:tt),+) => {
        impl<T: Number> num_traits::One for $structure<T> {
            fn is_one(&self) -> bool
                where
                    Self: PartialEq, {
                true $(
                    && self.$element.is_one()
                )+
            }
            fn one() -> Self {
                Self { 
                    $(
                        $element: T::one()
                    ),+
                }
            }
            fn set_one(&mut self) {
                $(
                    self.$element = T::one();
                )+
            }
        }
        
        impl<T: Number> num_traits::Zero for $structure<T> {
            fn is_zero(&self) -> bool {
                true $(
                    && self.$element.is_zero()
                )+
            }
            fn zero() -> Self {
                Self { 
                    $(
                        $element: T::zero()
                    ),+
                }
            }
            fn set_zero(&mut self) {
                $(
                    self.$element = T::zero();
                )+
            }
        }
    };
}
impl_num_traits!(Vector2, x, y);
impl_num_traits!(Vector3, x, y, z);
impl_num_traits!(Vector4, x, y, z, w);

macro_rules! impl_signed {
    ($structure:ident, $($element:tt),+) => {
        impl<T: SignedNumber> Signed for $structure<T> {
            fn abs(&self) -> Self {
                Self { 
                    $(
                        $element: self.$element.abs()
                    ),+
                }
            }
            fn abs_sub(&self, other: &Self) -> Self {
                Self { 
                    $(
                        $element: self.$element.abs_sub(&other.$element)
                    ),+
                }
            }
            fn is_negative(&self) -> bool {
                true $(
                    && self.$element.is_negative()
                )+
            }
            fn is_positive(&self) -> bool {
                true $(
                    && self.$element.is_positive()
                )+
            }
            fn signum(&self) -> Self {
                Self {
                    $(
                        $element: self.$element.signum()
                    ),+
                }
            }
        }
    };
}
impl<T: Number> Num for Vector2<T> {
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let split_str: Vec<_> = str.split(',').collect();
        let mut values = Vec::new();

        for i in 0..Self::size() {
            if i > split_str.len() {
                let result = T::from_str_radix(split_str[i], radix);
                match result {
                    Err(e) => return Err(e),
                    Ok(o) => { values.push(o) },
                }
            }
        }

        match split_str.len() {
            0 => Ok(Self::from(T::zero())),
            1 => Ok(Self::new(values[0],values[0])),
            _ => Ok(Self::new(values[0],values[1])),
        }
    }
    type FromStrRadixErr = <T as Num>::FromStrRadixErr;
}

impl<T: Number> Num for Vector3<T> {
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let split_str: Vec<_> = str.split(',').collect();
        let mut values = Vec::new();

        for i in 0..Self::size() {
            if i > split_str.len() {
                let result = T::from_str_radix(split_str[i], radix);
                match result {
                    Err(e) => return Err(e),
                    Ok(o) => { values.push(o) },
                }
            }
        }

        match split_str.len() {
            0 => Ok(Self::from(T::zero())),
            1 => Ok(Self::new(values[0],values[0], values[0])),
            2 => Ok(Self::new(values[0],values[1], values[1])),
            _ => Ok(Self::new(values[0],values[1], values[2])),
        }
    }
    type FromStrRadixErr = <T as Num>::FromStrRadixErr;
}

impl<T: Number> Num for Vector4<T> {
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let split_str: Vec<_> = str.split(',').collect();
        let mut values = Vec::new();

        for i in 0..Self::size() {
            if i > split_str.len() {
                let result = T::from_str_radix(split_str[i], radix);
                match result {
                    Err(e) => return Err(e),
                    Ok(o) => { values.push(o) },
                }
            }
        }

        match split_str.len() {
            0 => Ok(Self::from(T::zero())),
            1 => Ok(Self::new(values[0],values[0], values[0], values[0])),
            2 => Ok(Self::new(values[0],values[1], values[1], values[1])),
            3 => Ok(Self::new(values[0],values[1], values[2], values[2])),
            _ => Ok(Self::new(values[0],values[1], values[2], values[3])),
        }
    }
    type FromStrRadixErr = <T as Num>::FromStrRadixErr;
}

impl_signed!(Vector2, x, y);
impl_signed!(Vector3, x, y, z);
impl_signed!(Vector4, x, y, z, w);
macro_rules! impl_clamp {
    () => {
        pub fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized,
        {
            assert!(min <= max);
            if self < min {
                min
            } else if self > max {
                max
            } else {
                self
            }
        }
    };
}
impl<T: Number> Vector2<T>  {
    impl_clamp!();
}
impl<T: Number> Vector3<T>  {
    impl_clamp!();
}
impl<T: Number> Vector4<T>  {
    impl_clamp!();
}