use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

use crate::complex::quaternion::Quaternion;
use crate::linear::vector::Vector;

use crate::linear::*;
mod rotationold;
mod rotater;
mod translation;
pub use translation::*;
pub use rotater::*;
use num_traits::{One, Zero};
pub trait Transform: Default + Clone + Copy {
    type Rotation;
    // Translation also serves as our spatial vector
    type Translation: One + Zero + Vector;
    type Scaling: One + Zero;
    fn apply_matrix4(&self, mat: &mut FMat4) {
        *mat = FMat4::identity();
    }
    fn apply_matrix3(&self, mat: &mut FMat3) {
        *mat = FMat3::identity();
    }
    fn apply_matrix2(&self, mat: &mut FMat2) {
        *mat = FMat2::identity();
    }
    fn translation(&self) -> Self::Translation;
    fn rotation(&self) -> Self::Rotation;
    fn scaling(&self) -> Self::Scaling;
    
    fn rotate(&self, rot: &Self::Rotation) -> Self;
    fn translate(&self, pos: &Self::Translation) -> Self;
    fn scale(&self, scale: &Self::Scaling) -> Self;

    fn set_rotation(&self, rot: &Self::Rotation) -> Self;
    fn set_translation(&self, pos: &Self::Translation) -> Self;
    fn set_scaling(&self, scale: &Self::Scaling) -> Self;

    fn get_rotation_mut(&mut self) -> &mut Self::Rotation;
    fn get_translation_mut(&mut self) -> &mut Self::Translation;
    fn get_scaling_mut(&mut self) -> &mut Self::Scaling;
    fn normal_matrix(&self) -> FMat3;
}


#[derive(Clone, Copy)]
pub struct Transform2D {
    pub translation: FVec2,
    pub scale: FVec2,
    pub rotation: f32,
}
impl Transform for Transform2D {
    type Rotation = f32;
    type Scaling = FVec2;
    type Translation = FVec2;
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
    fn rotate(&self, rot: &Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation += *rot;
        rotation
    }
    fn translate(&self, pos: &Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation += *pos;
        translation
    }
    fn scale(&self, scale: &Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale += *scale;
        scaling
    }
    fn rotation(&self) -> Self::Rotation { self.rotation }
    fn scaling(&self) -> Self::Scaling { self.scale }
    fn translation(&self) -> Self::Translation { self.translation }
    fn set_rotation(&self, rot: &Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation = *rot;
        rotation
    }
    fn set_scaling(&self, scale: &Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale = *scale;
        scaling
    }
    fn set_translation(&self, pos: &Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation = *pos;
        translation
    }
    fn get_rotation_mut(&mut self) -> &mut Self::Rotation {
        &mut self.rotation
    }
    fn get_translation_mut(&mut self) -> &mut Self::Translation {
        &mut self.translation
    }
    fn get_scaling_mut(&mut self) -> &mut Self::Scaling {
        &mut self.scale
    }
    fn normal_matrix(&self) -> FMat3 {
        let x = self.set_scaling(&(Self::Scaling::from(1.0) / self.scaling()));
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

impl Transform for TransformQuaternion3D {
    type Rotation = Quaternion<f32>;
    type Scaling = FVec3;
    type Translation = FVec3;
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
    fn normal_matrix(&self) -> FMat3 {
        let x = self.set_scaling(&(Self::Scaling::from(1.0) / self.scaling()));
        let mut mat = FMat3::identity();
        x.apply_matrix3(&mut mat);
        mat
    }
    fn get_rotation_mut(&mut self) -> &mut Self::Rotation { &mut self.rotation }
    fn get_translation_mut(&mut self) -> &mut Self::Translation { &mut self.translation }
    fn get_scaling_mut(&mut self) -> &mut Self::Scaling { &mut self.scale }
    fn rotate(&self, rot: &Self::Rotation) -> Self {
        let mut this = *self;
        this.rotation = this.rotation * *rot;
        this
    }
    fn translate(&self, pos: &Self::Translation) -> Self {
        let mut this = *self;
        this.translation += *pos;
        this
    }
    fn scale(&self, scale: &Self::Scaling) -> Self {
        let mut this = *self;
        this.scale += *scale;
        this
    }
    fn rotation(&self) -> Self::Rotation {
        self.rotation
    }
    fn scaling(&self) -> Self::Scaling {
        self.scale
    }
    fn translation(&self) -> Self::Translation {
        self.translation
    }
    fn set_rotation(&self, rot: &Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation = *rot;
        rotation
    }
    fn set_scaling(&self, scale: &Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale = *scale;
        scaling
    }
    fn set_translation(&self, pos: &Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation = *pos;
        translation
    }
}

impl Default for TransformQuaternion3D {
    fn default() -> Self { Self { translation: FVec3::from(0.0), scale: FVec3::from(1.0), rotation: Quaternion::<f32>::from_euler(FVec3::from(0.0)) } }
}