use crate::{complex::quaternion::Quaternion, FloatingPoint, Matrix3, Matrix4, Matrix2, Vector2};

pub trait Rotation<M> {
    fn rotate(&self, value: &mut M);
}

impl<T: FloatingPoint> Rotation<Vector2<T>> for T {
    fn rotate(&self, value: &mut Vector2<T>) {
        *value = Vector2 {
            x: value.x * self.sin() - value.y * self.sin(),
            y: value.x * self.sin() - value.y * self.cos() 
        }
    }
}
impl<T: FloatingPoint> Rotation<Matrix2<T>> for T  {
    fn rotate(&self, value: &mut Matrix2<T>) {
        *value = Matrix2 {
            x: Vector2 { 
                x: self.cos(), 
                y: self.sin() 
            },
            y: Vector2 { 
                x: -self.sin(), 
                y: self.cos() 
            }
        }
    }
}
impl<T: FloatingPoint> Rotation<Matrix3<T>> for Quaternion<T> {
    fn rotate(&self, value: &mut Matrix3<T>) {
        *value = *value * Matrix3::from(*self);
    }
}
impl<T: FloatingPoint> Rotation<Matrix4<T>> for Quaternion<T> {
    fn rotate(&self, value: &mut Matrix4<T>) {
        *value = *value * Matrix4::from(*self);
    }
}