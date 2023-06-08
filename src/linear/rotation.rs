use num_traits::{Num, One};

use crate::linear::{Vector3};

use super::{traits::{Number, FloatingPoint}, vector::{Vector2, Vector4}, matrix::{Matrix2, Matrix4}};
pub trait Rotation2D<T: Sized + Copy + One + FloatingPoint> {
    type Output;
    fn rotate(&self, rotate: T) -> Self::Output;
}
pub trait Rotation3D<T: Sized + Copy + One + FloatingPoint> {
    type Output;
    fn rotate(&self, angle: T, rotate: Vector3<T>) -> Self::Output;
}
pub trait IndependentEulerRotation3D<T: Sized + Copy + One + FloatingPoint> {
    type Output;
    fn rotatex(&self, angle: T) -> Self::Output;
    fn rotatey(&self, angle: T) -> Self::Output;
    fn rotatez(&self, angle: T) -> Self::Output;
}

///
/// Rotations for floating point values in 2 dimensions.
/// 
impl<T: FloatingPoint> Rotation2D<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn rotate(&self, rotate: T) -> Self::Output {
        Self { 
            x: self.x * T::cos(rotate) - self.y * T::sin(rotate), 
            y: self.x * T::sin(rotate) - self.y * T::cos(rotate) 
        }
    }
}

impl<T: FloatingPoint> Rotation2D<T> for Matrix2<T>  {
    type Output = Self;
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

impl<T: FloatingPoint + Default> Rotation3D<T> for Matrix4<T> {
    type Output = Self;
    /// # rotate
    /// 
    /// perform a rotation on a Floating Point Matrix4 on an axis specified by v and an angle in radians.
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
    fn rotate(&self, angle: T, v: Vector3<T>) -> Matrix4<T> {
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
        result.x = self.x * rotate.x.x + self.y * rotate.x.y + self.z * rotate.x.z;
        result.y = self.x * rotate.y.x + self.y * rotate.y.y + self.z * rotate.y.z;
        result.z = self.x * rotate.z.x + self.y * rotate.z.y + self.z * rotate.z.z;
        result.w = self.w;
        result
    }
}

impl<T: FloatingPoint> IndependentEulerRotation3D<T> for Vector3<T> {
    type Output = Self;
    
    /// # rotatex
    /// 
    /// rotates a vector around the x axis by a specified angle in radians.
    /// more information about the implemenation of this function
    /// can be found in [this stack overflow post](https://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space)
    /// and [this wikipedia article](https://en.wikipedia.org/wiki/Rotation_matrix)
    /// 
    fn rotatex(&self, angle: T) -> Self::Output {
        Self {
            x: self.x,
            y: self.y * T::cos(angle) - self.z * T::sin(angle),
            z: self.y * T::sin(angle) + self.z * T::cos(angle)
        }
    }
    /// # rotatey
    /// 
    /// rotates a vector around the y axis by a specified angle in radians.
    /// more information about the implemenation of this function
    /// can be found in [this stack overflow post](https://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space)
    /// and [this wikipedia article](https://en.wikipedia.org/wiki/Rotation_matrix)
    /// 
    fn rotatey(&self, angle: T) -> Self::Output {
        Self {
            x: self.x * T::cos(angle) + self.z * T::sin(angle),
            y: self.y,
            z: self.z * T::cos(angle) - self.x * T::sin(angle)
        }
    }
    /// # rotatez
    /// 
    /// rotates a vector around the z axis by a specified angle in radians.
    /// more information about the implemenation of this function
    /// can be found in [this stack overflow post](https://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space)
    /// and [this wikipedia article](https://en.wikipedia.org/wiki/Rotation_matrix)
    /// 
    fn rotatez(&self, angle: T) -> Self::Output {
        Self {
            x: self.x * T::cos(angle) - self.y * T::sin(angle),
            y: self.x * T::sin(angle) + self.y * T::cos(angle),
            z: self.z
        }
    }

}