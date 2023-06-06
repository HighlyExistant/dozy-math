use crate::complex::quaternion::Quaternion;

use super::{vector::{Vector2, Vector3, Vector4}, traits::Number};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix2<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix3<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix4<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>,
}

/// ===========================================================
/// 
/// Implementation for Matrix2
/// 
/// ===========================================================

///
/// These functions are available for all matrices.
/// 
impl<T> Matrix2<T>  {
    pub fn new(xx: T, xy: T, yx: T, yy: T) -> Self {
        Self { x: Vector2 { x: xx, y: xy }, y: Vector2 { x: yx, y: yy } }
    }
    pub fn from_vec(x: Vector2<T>, y: Vector2<T>) -> Self {
        Self { x, y }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Matrix2<T>  {
    fn identity(val: T) -> Self {
        Self { x: Vector2 { x: val, y: T::zero() }, y: Vector2 { x: T::zero(), y: val } }
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix2<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix2<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Matrix2<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector2 { 
                x: self.x.x * rhs.x.x + self.y.x * rhs.x.y, 
                y: self.x.y * rhs.x.x + self.y.y * rhs.x.y 
            }, 
            y: Vector2 { 
                x: self.x.x * rhs.y.x + self.y.x * rhs.y.y, 
                y: self.x.y * rhs.y.x + self.y.y * rhs.y.y 
            } 
        }
    }
    type Output = Self;
}

/// ===========================================================
/// 
/// Implementation for Matrix3
/// 
/// ===========================================================

///
/// These functions are available for all matrices.
/// 
impl<T> Matrix3<T>  {
    pub fn new(xx: T, xy: T, xz: T, yx: T, yy: T, yz: T, zx: T, zy: T, zz: T) -> Self {
        Self { x: Vector3 { x: xx, y: xy, z: xz }, y: Vector3 { x: yx, y: yy, z: yz }, z: Vector3 { x: zx, y: zy, z: zz } }
    }
    pub fn from_vec(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self { x, y, z }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Matrix3<T>  {
    fn identity(val: T) -> Self {
        Self { x: Vector3 { x: val, y: T::zero(), z: T::zero() }, y: Vector3 { x: T::zero(), y: val, z: T::zero() }, z: Vector3 { x: T::zero(), y: T::zero(), z: val } }
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix3<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix3<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Matrix3<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector3 { 
                x: rhs.x.x * self.x.x + rhs.x.y * self.y.x + rhs.x.z * self.z.x,
                y: rhs.x.x * self.x.y + rhs.x.y * self.y.y + rhs.x.z * self.z.y,
                z: rhs.x.x * self.x.z + rhs.x.y * self.y.z + rhs.x.z * self.z.z,
            }, 
            y: Vector3 { 
                x: rhs.y.x * self.x.x + rhs.y.y * self.y.x + rhs.y.z * self.z.x,
                y: rhs.y.x * self.x.y + rhs.y.y * self.y.y + rhs.y.z * self.z.y,
                z: rhs.y.x * self.x.z + rhs.y.y * self.y.z + rhs.y.z * self.z.z,
            },
            z: Vector3 { 
                x: rhs.z.x * self.x.x + rhs.z.y * self.y.x + rhs.z.z * self.z.x,
                y: rhs.z.x * self.x.y + rhs.z.y * self.y.y + rhs.z.z * self.z.y,
                z: rhs.z.x * self.x.z + rhs.z.y * self.y.z + rhs.z.z * self.z.z,
            }
        }
    }
    type Output = Self;
}


/// ===========================================================
/// 
/// Implementation for Matrix4
/// 
/// ===========================================================

///
/// These functions are available for all matrices.
/// 
impl<T> Matrix4<T>  {
    pub fn new(xx: T, xy: T, xz: T, xw: T, yx: T, yy: T, yz: T, yw: T, zx: T, zy: T, zz: T, zw: T, wx: T, wy: T, wz: T, ww: T) -> Self {
        Self { x: Vector4 { x: xx, y: xy, z: xz, w: xw }, y: Vector4 { x: yx, y: yy, z: yz, w: yw }, z: Vector4 { x: zx, y: zy, z: zz, w: zw }, w: Vector4 { x: wx, y: wy, z: wz, w: ww } }
    }
    pub fn from_vec(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y, z, w }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Matrix4<T>  {
    fn identity(val: T) -> Self {
        Self { 
            x: Vector4 { x: val, y: T::zero(), z: T::zero(), w: T::zero() }, 
            y: Vector4 { x: T::zero(), y: val, z: T::zero(), w: T::zero() }, 
            z: Vector4 { x: T::zero(), y: T::zero(), z: val, w: T::zero() }, 
            w: Vector4 { x: T::zero(), y: T::zero(), z: T::zero(), w: val } 
        }
    }
    pub fn from_translation(v: Vector3<T>) -> Self {
        Matrix4::new(
            T::one(), T::zero(), T::zero(), T::zero(),
            T::zero(), T::one(), T::zero(), T::zero(),
            T::zero(), T::zero(), T::one(), T::zero(),
            v.x, v.y, v.z, T::one(),
        )
    }
    pub fn from_scale(v: Vector3<T>) -> Self {
        Matrix4::new(
            v.x, T::zero(), T::zero(), T::zero(),
            T::zero(), v.y, T::zero(), T::zero(),
            T::zero(), T::zero(), v.z, T::zero(),
            T::zero(), T::zero(), T::zero(), T::one(),
        )
    }
    pub fn scale(&self, v: Vector3<T>) -> Self {
        Self { x: self.x * v.x, y: self.y * v.y, z: self.z * v.z, w: self.w }
    }
}
impl<T: Number> From<Quaternion<T>> for Matrix4<T> {
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

        let sy2: T = y2 * value.scalar;
        let sz2 = z2 * value.scalar;
        let sx2 = x2 * value.scalar;

        Self { 
            x: Vector4 { 
                x: T::one() - yy2 - zz2, 
                y: xy2 + sz2, 
                z: xz2 - sy2, 
                w: T::zero()
            }, 
            y: Vector4 { 
                x: xy2 - sz2, 
                y: T::one() - xx2 - zz2, 
                z: yz2 + sx2, 
                w: T::zero()
            }, 
            z: Vector4 { 
                x: xz2 + sy2, 
                y: yz2 - sx2, 
                z: T::one() - xx2 - yy2, 
                w: T::zero()
            }, 
            w: Vector4 { 
                x: T::zero(), 
                y: T::zero(), 
                z: T::zero(), 
                w: T::one() 
            } 
        }
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix4<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z), w: (self.w + rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix4<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z), w: (self.w - rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Matrix4<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector4 { 
                x: rhs.x.x * self.x.x + rhs.x.y * self.y.x + rhs.x.z * self.z.x + rhs.x.w * self.w.x,
                y: rhs.x.x * self.x.y + rhs.x.y * self.y.y + rhs.x.z * self.z.y + rhs.x.w * self.w.y,
                z: rhs.x.x * self.x.z + rhs.x.y * self.y.z + rhs.x.z * self.z.z + rhs.x.w * self.w.z,
                w: rhs.x.x * self.x.w + rhs.x.y * self.y.w + rhs.x.z * self.z.w + rhs.x.w * self.w.w 
            }, 
            y: Vector4 { 
                x: rhs.y.x * self.x.x + rhs.y.y * self.y.x + rhs.y.z * self.z.x + rhs.y.w * self.w.x,
                y: rhs.y.x * self.x.y + rhs.y.y * self.y.y + rhs.y.z * self.z.y + rhs.y.w * self.w.y,
                z: rhs.y.x * self.x.z + rhs.y.y * self.y.z + rhs.y.z * self.z.z + rhs.y.w * self.w.z,
                w: rhs.y.x * self.x.w + rhs.y.y * self.y.w + rhs.y.z * self.z.w + rhs.y.w * self.w.w 
            },
            z: Vector4 { 
                x: rhs.z.x * self.x.x + rhs.z.y * self.y.x + rhs.z.z * self.z.x + rhs.z.w * self.w.x,
                y: rhs.z.x * self.x.y + rhs.z.y * self.y.y + rhs.z.z * self.z.y + rhs.z.w * self.w.y,
                z: rhs.z.x * self.x.z + rhs.z.y * self.y.z + rhs.z.z * self.z.z + rhs.z.w * self.w.z,
                w: rhs.z.x * self.x.w + rhs.z.y * self.y.w + rhs.z.z * self.z.w + rhs.z.w * self.w.w 
            },
            w: Vector4 { 
                x: rhs.w.x * self.x.x + rhs.w.y * self.y.x + rhs.w.z * self.z.x + rhs.w.w * self.w.x, 
                y: rhs.w.x * self.x.y + rhs.w.y * self.y.y + rhs.w.z * self.z.y + rhs.w.w * self.w.y, 
                z: rhs.w.x * self.x.z + rhs.w.y * self.y.z + rhs.w.z * self.z.z + rhs.w.w * self.w.z, 
                w: rhs.w.x * self.x.w + rhs.w.y * self.y.w + rhs.w.z * self.z.w + rhs.w.w * self.w.w 
            }
        }
    }
    type Output = Self;
}