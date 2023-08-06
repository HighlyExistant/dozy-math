use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

use crate::complex::quaternion::Quaternion;
use crate::linear::vector::Vector;

use crate::linear::*;
mod dimension;
mod rotater;
mod translation;
pub use translation::*;
pub use rotater::*;
use num_traits::{One, Zero};

use self::dimension::{Dimension, FlatDimension, SolidDimension};

pub trait Transform: Default + Clone + Copy {
    type Dimension: Dimension;
    fn translation(&self) -> <Self::Dimension as Dimension>::VectorRepresentation;
    fn rotation(&self) -> <Self::Dimension as Dimension>::RotationRepresentation;
    fn scaling(&self) -> <Self::Dimension as Dimension>::VectorRepresentation;
    
    fn rotate(&self, rot: &<Self::Dimension as Dimension>::RotationRepresentation) -> Self;
    fn translate(&self, pos: &<Self::Dimension as Dimension>::VectorRepresentation) -> Self;
    fn scale(&self, scale: &<Self::Dimension as Dimension>::VectorRepresentation) -> Self;

    fn set_rotation(&self, rot: &<Self::Dimension as Dimension>::RotationRepresentation) -> Self;
    fn set_translation(&self, pos: &<Self::Dimension as Dimension>::VectorRepresentation) -> Self;
    fn set_scaling(&self, scale: &<Self::Dimension as Dimension>::VectorRepresentation) -> Self;

    fn normal_matrix(&self) -> FMat3;
}

pub trait TransformMatrix<T: Number>: Transform {
    fn apply_matrix4(&self, mat: &mut Matrix4<T>) {
        *mat = Matrix4::<T>::identity();
    }
    fn apply_matrix3(&self, mat: &mut Matrix3<T>) {
        *mat = Matrix3::<T>::identity();
    }
    fn apply_matrix2(&self, mat: &mut Matrix2<T>) {
        *mat = Matrix2::<T>::identity();
    }
}


#[derive(Clone, Copy)]
pub struct Transform2D {
    pub translation: FVec2,
    pub scale: FVec2,
    pub rotation: f32,
}
impl TransformMatrix<f32> for Transform2D {
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
}
impl Transform for Transform2D {
    type Dimension = FlatDimension<f32>;
    
    fn rotate(&self, rot: &f32) -> Self {
        let mut rotation = *self;
        rotation.rotation += *rot;
        rotation
    }
    fn translate(&self, pos: &FVec2) -> Self {
        let mut translation = *self;
        translation.translation += *pos;
        translation
    }
    fn scale(&self, scale: &FVec2) -> Self {
        let mut scaling = *self;
        scaling.scale += *scale;
        scaling
    }
    fn rotation(&self) -> f32 { self.rotation }
    fn scaling(&self) -> FVec2 { self.scale }
    fn translation(&self) -> FVec2 { self.translation }
    fn set_rotation(&self, rot: &f32) -> Self {
        let mut rotation = *self;
        rotation.rotation = *rot;
        rotation
    }
    fn set_scaling(&self, scale: &FVec2) -> Self {
        let mut scaling = *self;
        scaling.scale = *scale;
        scaling
    }
    fn set_translation(&self, pos: &FVec2) -> Self {
        let mut translation = *self;
        translation.translation = *pos;
        translation
    }
    fn normal_matrix(&self) -> FMat3 {
        let x = self.set_scaling(&(FVec2::from(1.0) / self.scaling()));
        let mut mat = FMat3::identity();
        x.apply_matrix3(&mut mat);
        mat
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
impl TransformMatrix<f32> for TransformQuaternion3D {
    fn apply_matrix4(&self, mat: &mut FMat4) {
        *mat = FMat4::identity();
        *mat = translate(mat, self.translation);

        let rotation = FMat4::from(self.rotation);

        *mat = *mat * rotation;

        let scale = FMat4::from_scale(self.scale);

        *mat = *mat * scale;
    }
    fn apply_matrix3(&self, mat: &mut FMat3) {
        *mat = FMat3::identity();

        let rotation = FMat3::from(self.rotation);

        *mat = *mat * rotation;

        let scale = FMat3::from_scale(self.scale);

        *mat = *mat * scale;
    }
}
impl Transform for TransformQuaternion3D {
    type Dimension = SolidDimension<f32>;
    
    fn normal_matrix(&self) -> FMat3 {
        let x = self.set_scaling(&(FVec3::from(1.0) / self.scaling()));
        let mut mat = FMat3::identity();
        x.apply_matrix3(&mut mat);
        mat
    }
    fn rotate(&self, rot: &FVec3) -> Self {
        let mut this = *self;
        this.rotation = this.rotation * Quaternion::from_euler(*rot);
        this
    }
    fn translate(&self, pos: &FVec3) -> Self {
        let mut this = *self;
        this.translation += *pos;
        this
    }
    fn scale(&self, scale: &FVec3) -> Self {
        let mut this = *self;
        this.scale += *scale;
        this
    }
    fn rotation(&self) -> FVec3 {
        self.rotation.to_euler()
    }
    fn scaling(&self) -> FVec3 {
        self.scale
    }
    fn translation(&self) -> FVec3 {
        self.translation
    }
    fn set_rotation(&self, rot: &FVec3) -> Self {
        let mut rotation = *self;
        rotation.rotation = Quaternion::from_euler(*rot);
        rotation
    }
    fn set_scaling(&self, scale: &FVec3) -> Self {
        let mut scaling = *self;
        scaling.scale = *scale;
        scaling
    }
    fn set_translation(&self, pos: &FVec3) -> Self {
        let mut translation = *self;
        translation.translation = *pos;
        translation
    }
}

impl Default for TransformQuaternion3D {
    fn default() -> Self { Self { translation: FVec3::from(0.0), scale: FVec3::from(1.0), rotation: Quaternion::<f32>::from_euler(FVec3::from(0.0)) } }
}