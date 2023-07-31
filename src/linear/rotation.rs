use num_traits::{Num, One};

use crate::{linear::Vector3, complex::quaternion::Quaternion};

use super::{traits::{Number, FloatingPoint}, matrix::{Matrix2, Matrix4, Matrix3}, Vector2, Vector4, Vector};

pub trait Rotation {
    type Output;
    type Rotate;
    fn rotate(&self, rotate: &Self::Rotate) -> Self::Output;
}

impl<T: FloatingPoint> Rotation for Vector2<T> {
    type Output = Vector2<T>;
    type Rotate = T;

    fn rotate(&self, rotate: &T) -> Self::Output {
        Self { 
            x: self.x * rotate.cos() - self.y * rotate.sin(), 
            y: self.x * rotate.sin() - self.y * rotate.cos() 
        }
    }
}

impl<T: FloatingPoint> Rotation for Matrix2<T>  {
    type Output = Self;
    type Rotate = T;
    /// 
    /// performs a rotation in 2 dimensions on a matrix using a linear transformation
    /// 
    /// |  x: xcos(0) y: -ysin(0)  |
    /// |  x: xsin(0) y: -ycos(0)  |
    /// 
    /// for more information see [this stack overflow post](https://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space) and
    /// [this wikipedia article](https://en.wikipedia.org/wiki/Rotation_matrix)
    fn rotate(&self, rotate: &T) -> Self::Output {
        Self {
            x: Vector2 { 
                x: self.x.x * rotate.cos(), 
                y: self.x.y * rotate.sin() 
            },      // Column 1
            y: Vector2 { 
                x: -(self.y.x * rotate.sin()), 
                y: self.y.y * rotate.cos()
            },   // Column 2
        }
    }
}


///
/// Rotations for floating point values in 3 dimensions.
/// 

impl<T: FloatingPoint> Rotation for Matrix3<T> {
    /// 
    /// perform a rotation on a Floating Point Matrix4 on an axis specified by v and an angle in radians which is
    /// the fourth scalar in v: *v.w*.
    /// 
    /// # Example
    /// ```
    /// use drowsed_math::{FMat3, FVec4, FVec3, Rotation};
    /// 
    /// let mut transform = FMat3::identity(1.0);
    /// // Rotating along the y axis
    /// transform = transform.rotate(&FVec4::new(0.0, 1.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the x axis
    /// transform = transform.rotate(&FVec4::new(1.0, 0.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the z axis
    /// transform = transform.rotate(&FVec4::new(0.0, 0.0, 1.0, std::f32::consts::PI));
    /// ```
    /// This function is heavily influenced from the implementation [glm](https://github.com/g-truc/glm) uses.
    /// other resources from [wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix).
    fn rotate(&self, v: &Vector4<T>) -> Matrix3<T> {
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
    /// 
    /// perform a rotation on a Floating Point Matrix4 on an axis specified by v and an angle in radians which is
    /// the fourth scalar in v: *v.w*.
    /// 
    /// # Example
    /// ```
    /// use drowsed_math::{FMat4, FVec4, FVec3, Rotation};
    /// 
    /// let mut transform = FMat4::identity(1.0);
    /// // Rotating along the y axis
    /// transform = transform.rotate(&FVec4::new(0.0, 1.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the x axis
    /// transform = transform.rotate(&FVec4::new(1.0, 0.0, 0.0, std::f32::consts::PI));
    /// // Rotating along the z axis
    /// transform = transform.rotate(&FVec4::new(0.0, 0.0, 1.0, std::f32::consts::PI));
    /// ```
    /// This function is heavily influenced from the implementation [glm](https://github.com/g-truc/glm) uses.
    /// other resources from [wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix).
    fn rotate(&self, v: &Vector4<T>) -> Matrix4<T> {
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
    fn rotate(&self, rotate: &Self::Rotate) -> Self::Output {
        *self * *rotate
    }
    type Output = Quaternion<T>;
    type Rotate = Quaternion<T>;
}