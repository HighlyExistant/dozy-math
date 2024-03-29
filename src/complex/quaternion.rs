use std::fmt::Debug;

use num_traits::AsPrimitive;

use crate::{Vector3, Matrix3, Number, FloatingPoint, Vector};

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
    /// # from_euler
    /// 
    /// converts euler angles into quaternion form.
    /// 
    /// this function is heavily inspired by [this stackexchange post](https://math.stackexchange.com/questions/2975109/how-to-convert-euler-angles-to-quaternions-and-get-the-same-euler-angles-back-fr)
    pub fn from_euler(v: Vector3<T>) -> Self 
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        let (yaw, pitch, roll) = (v.x, v.y, v.z);
        let x = T::sin(roll/2.0.as_()) * T::cos(pitch/2.0.as_()) * T::cos(yaw/2.0.as_()) - T::cos(roll/2.0.as_()) * T::sin(pitch/2.0.as_()) * T::sin(yaw/2.0.as_());
        let y = T::cos(roll/2.0.as_()) * T::sin(pitch/2.0.as_()) * T::cos(yaw/2.0.as_()) + T::sin(roll/2.0.as_()) * T::cos(pitch/2.0.as_()) * T::sin(yaw/2.0.as_());
        let z = T::cos(roll/2.0.as_()) * T::cos(pitch/2.0.as_()) * T::sin(yaw/2.0.as_()) - T::sin(roll/2.0.as_()) * T::sin(pitch/2.0.as_()) * T::cos(yaw/2.0.as_());
        let w = T::cos(roll/2.0.as_()) * T::cos(pitch/2.0.as_()) * T::cos(yaw/2.0.as_()) + T::sin(roll/2.0.as_()) * T::sin(pitch/2.0.as_()) * T::sin(yaw/2.0.as_());
        Self { vector: Vector3 { x, y, z }, scalar: w }
    }
    pub fn angle_axis(angle: T, vector: Vector3<T>) -> Self 
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        let half_angle = angle * 0.5.as_();
        let s = half_angle.sin();
        Self { vector: vector * s, scalar: half_angle.cos() }
    }
    pub fn to_euler(&self) -> Vector3<T>
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        // let roll: T = T::atan2(2.0.as_() * (self.scalar * self.vector.x + self.vector.y * self.vector.z), self.scalar*self.scalar - self.vector.x*self.vector.x - self.vector.y*self.vector.y + self.vector.z*self.vector.z);
        // let pitch: T = T::asin((-2.0).as_() * (self.scalar * self.vector.y - self.vector.z * self.vector.x));
        // let yaw = T::atan2(2.0.as_() * (self.scalar * self.vector.z + self.vector.x * self.vector.y), self.scalar*self.scalar + self.vector.x*self.vector.x - self.vector.y*self.vector.y - self.vector.z*self.vector.z);
        // Vector3 { x: roll, y: pitch, z: yaw }

        let (x, y, z, w) = (self.vector.x, self.vector.y, self.vector.z, self.scalar);
        let t0 = 2.0.as_() * (w * x + y * z);
        let t1 = 1.0.as_() - 2.0.as_() * (x * x + y * y);
        let roll = T::atan2(t0, t1);
        let t2 = 2.0.as_() * (w * y - z * x);
        let t2 = if t2 > 1.0.as_() {1.0.as_()} else {t2};
        let t2 = if t2 < (-1.0).as_() {(-1.0).as_()} else {t2};
        let pitch = T::asin(t2);
        let t3 = 2.0.as_() * (w * z + x * y);
        let t4 = 1.0.as_() - 2.0.as_() * (y * y + z * z);
        let yaw = T::atan2(t3, t4);
        return Vector3::new(yaw, pitch, roll);

        // let sinr_cosp = 2.0.as_() * (self.scalar * self.vector.x + self.vector.y * self.vector.z);
        // let cosr_cosp = 1.0.as_() - 2.0.as_() * (self.vector.x * self.vector.x + self.vector.y * self.vector.y);
        // let roll = T::atan2(sinr_cosp, cosr_cosp);
// 
        // // pitch (y-axis rotation)
        // let sinp = T::sqrt(1.0.as_() + 2.0.as_() * (self.scalar * self.vector.y - self.vector.x * self.vector.z));
        // let cosp = T::sqrt(1.0.as_() - 2.0.as_() * (self.scalar * self.vector.y - self.vector.x * self.vector.z));
        // let pitch = 2.0.as_() * T::atan2(sinp, cosp) - std::f64::consts::PI.as_() / 2.0.as_();
// 
        // // yaw (z-axis rotation)
        // let siny_cosp = 2.0.as_() * (self.scalar * self.vector.z + self.vector.x * self.vector.y);
        // let cosy_cosp = 1.0.as_() - 2.0.as_() * (self.vector.y * self.vector.y + self.vector.z * self.vector.z);
        // let yaw = T::atan2(siny_cosp, cosy_cosp);
// 
        // return Vector3::new(roll, pitch, yaw);
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
        let x2 = self.vector.x + self.vector.x;
        let y2 = self.vector.y + self.vector.y;
        let z2 = self.vector.z + self.vector.z;
        
        let xx2 = x2 * self.vector.x;
        let xy2 = x2 * self.vector.y;
        let xz2 = x2 * self.vector.z;

        let yy2 = y2 * self.vector.y;
        let yz2 = y2 * self.vector.z;
        let zz2 = z2 * self.vector.z;

        let sy2 = y2 * self.scalar;
        let sz2 = z2 * self.scalar;
        let sx2 = x2 * self.scalar;

        Vector3 {
            x: (T::one() - (yy2 + zz2)) * rhs.x + (xy2 - sz2) * rhs.y + (xz2 + sy2) * rhs.z,
            y: (xy2 + sz2) * rhs.x + (T::one() - (xx2 + zz2)) * rhs.y + (yz2 - sx2) * rhs.z,
            z: (xz2 - sy2) * rhs.x + (yz2 + sx2) * rhs.y + (T::one() - (xx2 + yy2)) * rhs.z,
        }
    }
    type Output = Vector3<T>;
}
impl<T: Number> std::ops::Mul for Quaternion<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            vector: Vector3 { 
                x: self.scalar * rhs.vector.x + self.vector.x * rhs.scalar + self.vector.y * rhs.vector.z - self.vector.z * rhs.vector.y, 
                y: self.scalar * rhs.vector.y + self.vector.y * rhs.scalar + self.vector.z * rhs.vector.x - self.vector.x * rhs.vector.z, 
                z: self.scalar * rhs.vector.z + self.vector.z * rhs.scalar + self.vector.x * rhs.vector.y - self.vector.y * rhs.vector.x
            }, 
            scalar: self.scalar * rhs.scalar - self.vector.x * rhs.vector.x - self.vector.y * rhs.vector.y - self.vector.z * rhs.vector.z
        }
    }
    type Output = Self;
}