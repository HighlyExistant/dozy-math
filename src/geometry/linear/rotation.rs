use num_traits::{Num, One};

use crate::{linear::{Vector3}, complex::quaternion::Quaternion};

use super::{traits::{Number, FloatingPoint}, vector::{Vector2, Vector4}, matrix::{Matrix2, Matrix4, Matrix3}};

pub trait Rotation {
    type Output;
    type Rotate;
    fn rotate(&self, rotate: Self::Rotate) -> Self::Output;
}

///
/// Rotations for floating point values in 2 dimensions.
/// 
impl<T: FloatingPoint> Rotation for Vector2<T> {
    type Output = Vector2<T>;
    type Rotate = T;

    fn rotate(&self, rotate: T) -> Self::Output {
        Self { 
            x: self.x * T::cos(rotate) - self.y * T::sin(rotate), 
            y: self.x * T::sin(rotate) - self.y * T::cos(rotate) 
        }
    }
}

impl<T: FloatingPoint> Rotation for Matrix2<T>  {
    type Output = Self;
    type Rotate = T;
    /// # rotate
    /// 
    /// performs a rotation in 2 dimensions on a matrix using a linear transformation
    /// ```
    /// |  x: xcos(0) y: -ysin(0)  |
    /// |  x: xsin(0) y: -ycos(0)  |
    /// ```
    /// for more information see [this stack overflow post](https://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space) and
    /// [this wikipedia article](https://en.wikipedia.org/wiki/Rotation_matrix)
    fn rotate(&self, rotate: T) -> Self::Output {
        Self {
            x: Vector2 { 
                x: self.x.x * T::cos(rotate), 
                y: self.x.y * T::sin(rotate) 
            },      // Column 1
            y: Vector2 { 
                x: -(self.y.x * T::sin(rotate)), 
                y: self.y.y * T::cos(rotate) 
            },   // Column 2
        }
    }
}


///
/// Rotations for floating point values in 3 dimensions.
/// 

impl<T: FloatingPoint> Rotation for Matrix3<T> {
    /// # rotate
    /// 
    /// perform a rotation on a Floating Point Matrix4 on an axis specified by v and an angle in radians which is
    /// the fourth scalar in v: *v.w*.
    /// 
    /// # Example
    /// ```
    /// 
    /// let mut transform = FMat3::identity(1.0);
    /// // Rotating along the y axis
    /// transform = rotate(&transform, self.rotation.y, FVec4::new(0.0, 1.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the x axis
    /// transform = rotate(&transform, self.rotation.y, FVec4::new(1.0, 0.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the z axis
    /// transform = rotate(&transform, self.rotation.y, FVec4::new(0.0, 0.0, 1.0, std::f32::consts::PI));
    /// ```
    /// This function is heavily influenced from the implementation [glm](https://github.com/g-truc/glm) uses.
    /// other resources from [wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix).
    fn rotate(&self, v: Vector4<T>) -> Matrix3<T> {
        let a = v.w;
        let c = a.cos();
        let s = a.sin();
        
        let axis = v.normalize().xyz();
        let temp: Vector3<T> = axis * (T::one() - c) ;

        let mut rotate = Matrix3::<T>::from(T::zero());
        rotate.x.x = c + temp.x * axis.x;
        rotate.x.y = temp.x * axis.y + s * axis.z;
        rotate.x.z = temp.x * axis.z - s * axis.y;

        rotate.y.x = temp.y * axis.x - s * axis.z;
        rotate.y.y = c + temp.y * axis.y;
        rotate.y.z = temp.y * axis.z + s * axis.x;

        rotate.z.x = temp.z * axis.x + s * axis.y;
        rotate.z.y = temp.z * axis.y - s * axis.x;
        rotate.z.z = c + temp.z * axis.z;

        let mut result = Matrix3::<T>::from(T::zero());
        result.x = self.x * rotate.x.x + self.y * rotate.x.y + self.z * rotate.x.z;
        result.y = self.x * rotate.y.x + self.y * rotate.y.y + self.z * rotate.y.z;
        result.z = self.x * rotate.z.x + self.y * rotate.z.y + self.z * rotate.z.z;
        result
    }
    type Output = Self;
    type Rotate = Vector4<T>;
}

impl<T: FloatingPoint> Rotation for Matrix4<T> {
    /// # rotate
    /// 
    /// perform a rotation on a Floating Point Matrix4 on an axis specified by v and an angle in radians which is
    /// the fourth scalar in v: *v.w*.
    /// 
    /// # Example
    /// ```
    /// 
    /// let mut transform = FMat4::identity(1.0);
    /// // Rotating along the y axis
    /// transform = rotate(&transform, self.rotation.y, FVec4::new(0.0, 1.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the x axis
    /// transform = rotate(&transform, self.rotation.y, FVec4::new(1.0, 0.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the z axis
    /// transform = rotate(&transform, self.rotation.y, FVec4::new(0.0, 0.0, 1.0, std::f32::consts::PI));
    /// ```
    /// This function is heavily influenced from the implementation [glm](https://github.com/g-truc/glm) uses.
    /// other resources from [wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix).
    fn rotate(&self, v: Vector4<T>) -> Matrix4<T> {
        let a = v.w;
        let c = a.cos();
        let s = a.sin();
        
        let axis = v.normalize().xyz();
        let temp: Vector3<T> = axis * (T::one() - c) ;

        let mut rotate = Matrix4::<T>::from(T::zero());
        rotate.x.x = c + temp.x * axis.x;
        rotate.x.y = temp.x * axis.y + s * axis.z;
        rotate.x.z = temp.x * axis.z - s * axis.y;

        rotate.y.x = temp.y * axis.x - s * axis.z;
        rotate.y.y = c + temp.y * axis.y;
        rotate.y.z = temp.y * axis.z + s * axis.x;

        rotate.z.x = temp.z * axis.x + s * axis.y;
        rotate.z.y = temp.z * axis.y - s * axis.x;
        rotate.z.z = c + temp.z * axis.z;

        let mut result = Matrix4::<T>::from(T::zero());
        result.x = self.x * rotate.x.x + self.y * rotate.x.y + self.z * rotate.x.z;
        result.y = self.x * rotate.y.x + self.y * rotate.y.y + self.z * rotate.y.z;
        result.z = self.x * rotate.z.x + self.y * rotate.z.y + self.z * rotate.z.z;
        result.w = self.w;
        result
    }
    type Output = Self;
    type Rotate = Vector4<T>;
}

impl<T: FloatingPoint> Rotation for Quaternion<T> {
    /// wrapper around quaternion multiplication
    #[inline]
    fn rotate(&self, rotate: Self::Rotate) -> Self::Output {
        *self * rotate
    }
    type Output = Quaternion<T>;
    type Rotate = Quaternion<T>;
}