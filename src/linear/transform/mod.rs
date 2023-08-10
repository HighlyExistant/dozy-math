use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

use crate::complex::quaternion::Quaternion;
use crate::linear::vector::Vector;

use crate::linear::*;
mod rotater;
mod translation;
pub use translation::*;
pub use rotater::*;
use num_traits::{One, Zero};

use self::dimension::{Dimension, FlatDimension, SolidDimension};

pub trait Transform: Default + Clone + Copy + Sized {
    type Dimension: Dimension;
    // fn translation(&self) -> <Self::Dimension as Dimension>::VectorRepresentation;
    // fn rotation(&self) -> <Self::Dimension as Dimension>::RotationRepresentation;
    // fn scaling(&self) -> <Self::Dimension as Dimension>::VectorRepresentation;
    
    fn rotate(&mut self, rot: &<Self::Dimension as Dimension>::RotationRepresentation);
    fn translate(&mut self, pos: &<Self::Dimension as Dimension>::VectorRepresentation);
    fn scale(&mut self, scale: &<Self::Dimension as Dimension>::VectorRepresentation);

    fn set_rotation(&mut self, rot: &<Self::Dimension as Dimension>::RotationRepresentation);
    fn set_translation(&mut self, pos: &<Self::Dimension as Dimension>::VectorRepresentation);
    fn set_scaling(&mut self, scale: &<Self::Dimension as Dimension>::VectorRepresentation);

}

pub trait MatrixRepresentation<T: Number> {
    fn apply_matrix4(&self, mat: &mut Matrix4<T>) {
        *mat = Matrix4::<T>::identity();
    }
    fn apply_matrix3(&self, mat: &mut Matrix3<T>) {
        *mat = Matrix3::<T>::identity();
    }
    fn apply_matrix2(&self, mat: &mut Matrix2<T>) {
        *mat = Matrix2::<T>::identity();
    }
    fn normal_matrix(&self, mat: &mut Matrix3<T>) {
        *mat = Matrix3::<T>::identity();
    }
    fn apply_translation4d(&self, vec: &mut Vector4<T>) {}
    fn apply_translation3d(&self, vec: &mut Vector3<T>) {}
    fn apply_translation2d(&self, vec: &mut Vector2<T>) {}
    fn apply_rotation4d(&self, vec: &mut Quaternion<T>) {}
    fn apply_rotation3d(&self, vec: &mut Vector3<T>) {}
    fn apply_rotation2d(&self, vec: &mut T) {}
    fn apply_scaling4d(&self, vec: &mut  Vector4<T>) {}
    fn apply_scaling3d(&self, vec: &mut  Vector3<T>) {}
    fn apply_scaling2d(&self, vec: &mut  Vector2<T>) {}
}


#[derive(Clone, Copy)]
pub struct Transform2D {
    pub translation: FVec2,
    pub scale: FVec2,
    pub rotation: f32,
}
impl MatrixRepresentation<f32> for Transform2D {
    fn apply_matrix2(&self, mat: &mut FMat2) {
        let sin_rot = self.rotation.sin();
        let cos_rot = self.rotation.cos();
        let mat_rot = FMat2::from_vec(FVec2::new(cos_rot, sin_rot), FVec2::new(-sin_rot, cos_rot));
        
        let mat_scale = FMat2::from_vec(FVec2::new(self.scale.x, 0.0), FVec2::new(0.0, self.scale.y));
        
        *mat = mat_rot * mat_scale;
    }
    fn apply_matrix3(&self, mat: &mut FMat3) {
        let sin_rot = self.rotation.sin();
        let cos_rot = self.rotation.cos();
        let mat_rot = FMat3::from_vec(FVec3::new(cos_rot, sin_rot, 0.0), FVec3::new(-sin_rot, cos_rot, 0.0), FVec3::from(0.0));
        
        let mat_scale = FMat3::from_vec(FVec3::new(self.scale.x, 0.0, 0.0), FVec3::new(0.0, self.scale.y, 0.0), FVec3::from(0.0));
        *mat = mat_rot * mat_scale;
        mat.z.x = self.translation.x;
        mat.z.y = self.translation.y;
    }
    fn normal_matrix(&self, mat: &mut FMat3) {
        let mut x = *self;
        x.set_scaling(&(FVec2::from(1.0) / self.scale));
        *mat = FMat3::identity();
        x.apply_matrix3(mat);
    }
    fn apply_rotation2d(&self, vec: &mut f32) {
        *vec = self.rotation
    }
    fn apply_translation2d(&self, vec: &mut Vector2<f32>) {
        *vec = self.translation
    }
    fn apply_scaling2d(&self, vec: &mut  Vector2<f32>) {
        *vec = self.scale
    }
}
impl Transform for Transform2D {
    type Dimension = FlatDimension<f32>;
    
    fn rotate(&mut self, rot: &f32) {
        self.rotation += *rot;
    }
    fn translate(&mut self, pos: &FVec2) {
        self.translation += *pos;
    }
    fn scale(&mut self, scale: &FVec2) {
        self.scale += *scale;
    }
    // fn rotation(&self) -> f32 { self.rotation }
    // fn scaling(&self) -> FVec2 { self.scale }
    // fn translation(&self) -> FVec2 { self.translation }
    fn set_rotation(&mut self, rot: &f32) {
        self.rotation = *rot;
    }
    fn set_scaling(&mut self, scale: &FVec2) {
        self.scale = *scale;
    }
    fn set_translation(&mut self, pos: &FVec2) {
        self.translation = *pos;
    }
}
impl Default for Transform2D {
    fn default() -> Self { Self { translation: FVec2::new(0.0, 0.0), scale: FVec2::new(1.0, 1.0), rotation: 0.0 } }
}

/// 3D Transformation using a quaternion
#[derive(Clone, Copy)]
pub struct TransformQuaternion3D {
    pub translation: FVec3,
    pub scale: FVec3,
    pub rotation: Quaternion<f32>,
}
impl MatrixRepresentation<f32> for TransformQuaternion3D {
    fn apply_matrix4(&self, mat: &mut FMat4) {
        let mut mat_ = FMat4::identity();
        mat_ = translate(&mat_, self.translation);

        let rotation = FMat4::from(self.rotation);

        mat_ = mat_ * rotation;

        let scale = FMat4::from_scale(self.scale);

        mat_ = mat_ * scale;
        *mat = *mat * mat_;
    }
    fn apply_matrix3(&self, mat: &mut FMat3) {
        *mat = FMat3::identity();

        let rotation = FMat3::from(self.rotation);

        *mat = *mat * rotation;

        let scale = FMat3::from_scale(self.scale);

        *mat = *mat * scale;
    }
    fn normal_matrix(&self, mat: &mut FMat3) {
        let mut x = *self;
        x.set_scaling(&(FVec3::from(1.0) / self.scale));
        *mat = FMat3::identity();
        x.apply_matrix3(mat);
    }
    fn apply_rotation4d(&self, vec: &mut Quaternion<f32>) {
        *vec = self.rotation
    }
    fn apply_rotation3d(&self, vec: &mut Vector3<f32>) {
        *vec = self.rotation.to_euler()
    }
    fn apply_translation3d(&self, vec: &mut Vector3<f32>) {
        *vec = self.translation
    }
    fn apply_scaling3d(&self, vec: &mut  Vector3<f32>) {
        *vec = self.scale
    }
    fn apply_translation2d(&self, vec: &mut Vector2<f32>) {
        *vec = self.translation.xy()
    }
    fn apply_scaling2d(&self, vec: &mut  Vector2<f32>) {
        *vec = self.scale.xy()
    }
}
impl Transform for TransformQuaternion3D {
    type Dimension = SolidDimension<f32>;
    
    fn rotate(&mut self, rot: &FVec3) {
        self.rotation = self.rotation * Quaternion::from_euler(*rot);
    }
    fn translate(&mut self, pos: &FVec3) {
        self.translation += *pos;
    }
    fn scale(&mut self, scale: &FVec3) {
        self.scale += *scale;
    }
    // fn rotation(&self) -> FVec3 {
    //     self.rotation.to_euler()
    // }
    // fn scaling(&self) -> FVec3 {
    //     self.scale
    // }
    // fn translation(&self) -> FVec3 {
    //     self.translation
    // }
    fn set_rotation(&mut self, rot: &FVec3) {
        self.rotation = Quaternion::from_euler(*rot);
    }
    fn set_scaling(&mut self, scale: &FVec3) {
        self.scale = *scale;
    }
    fn set_translation(&mut self, pos: &FVec3) {
        self.translation = *pos;
    }
}

impl Default for TransformQuaternion3D {
    fn default() -> Self { Self { translation: FVec3::from(0.0), scale: FVec3::from(1.0), rotation: Quaternion::<f32>::from_euler(FVec3::from(0.0)) } }
}