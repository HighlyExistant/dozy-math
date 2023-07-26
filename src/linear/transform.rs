#![allow(unused)]

use core::num;
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

use crate::{complex::quaternion::Quaternion};
use crate::linear::vector::Vector;

use super::{translate, FVec2, FMat2, FVec3, FMat4, Rotation, FVec4, FMat3};

pub trait Transform: Default + Clone + Copy {
    type Rotation;
    type Translation: num_traits::One + num_traits::Zero + Vector;
    type Scaling: num_traits::One  + num_traits::Zero + Vector;
    fn matrix4(&self) -> FMat4 { FMat4::identity(0.0) }
    fn matrix3(&self) -> FMat3 { FMat3::identity(0.0) }
    fn matrix2(&self) -> FMat2 { FMat2::identity(0.0) }
    fn normal_matrix(&self) -> FMat3;

    fn translation(&self) -> Self::Translation;
    fn rotation(&self) -> Self::Rotation;
    fn scaling(&self) -> Self::Scaling;

    // use trait?
    fn rotate(&self, rot: Self::Rotation) -> Self;
    // make trait?
    fn translate(&self, pos: Self::Translation) -> Self;
    // make trait?
    fn scale(&self, scale: Self::Scaling) -> Self;

    // use trait?
    fn set_rotation(&self, rot: Self::Rotation) -> Self;
    // make trait?
    fn set_translation(&self, pos: Self::Translation) -> Self;
    // make trait?
    fn set_scaling(&self, scale: Self::Scaling) -> Self;

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
    fn matrix2(&self) -> FMat2 {
        let sin_rot = self.rotation.sin();
        let cos_rot = self.rotation.cos();
        let mat_rot = FMat2::from_vec(FVec2::new(cos_rot, sin_rot), FVec2::new(-sin_rot, cos_rot));
        
        let mat_scale = FMat2::from_vec(FVec2::new(self.scale.x, 0.0), FVec2::new(0.0, self.scale.y));
        return mat_rot * mat_scale;
    }
    fn matrix3(&self) -> FMat3 {
        let sin_rot = self.rotation.sin();
        let cos_rot = self.rotation.cos();
        let mat_rot = FMat3::from_vec(FVec3::new(cos_rot, sin_rot, 0.0), FVec3::new(-sin_rot, cos_rot, 0.0), FVec3::from(0.0));
        
        let mat_scale = FMat3::from_vec(FVec3::new(self.scale.x, 0.0, 0.0), FVec3::new(0.0, self.scale.y, 0.0), FVec3::from(0.0));
        return mat_rot * mat_scale;
    }
    fn normal_matrix(&self) -> FMat3 {
        self.set_scaling(Self::Scaling::from(1.0) / self.scaling()).matrix3()
    }
    fn rotate(&self, rot: Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation += rot;
        rotation
    }
    fn translate(&self, pos: Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation += pos;
        translation
    }
    fn scale(&self, scale: Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale += scale;
        scaling
    }
    fn rotation(&self) -> Self::Rotation { self.rotation }
    fn scaling(&self) -> Self::Scaling { self.scale }
    fn translation(&self) -> Self::Translation { self.translation }
    fn set_rotation(&self, rot: Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation = rot;
        rotation
    }
    fn set_scaling(&self, scale: Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale = scale;
        scaling
    }
    fn set_translation(&self, pos: Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation = pos;
        translation
    }
}

impl Default for Transform2D {
    fn default() -> Self { Self { translation: FVec2::new(0.0, 0.0), scale: FVec2::new(1.0, 1.0), rotation: 0.0 } }
}
#[derive(Clone, Copy)]
pub struct Transform3D {
    pub translation: FVec3,
    pub scale: FVec3,
    pub rotation: FVec3,
}

impl Transform for Transform3D {
    type Rotation = FVec3;
    type Scaling = FVec3;
    type Translation = FVec3;
    fn matrix4(&self) -> FMat4 {
        let mut transform: super::matrix::Matrix4<f32> = FMat4::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;
        transform.w.w = 1.0;
        transform = translate(&transform, self.translation);
        
        transform = transform.rotate(FVec4::new(0.0, 1.0, 0.0, self.rotation.y));
        transform = transform.rotate(FVec4::new(1.0, 0.0, 0.0, self.rotation.x));
        transform = transform.rotate(FVec4::new(0.0, 0.0, 1.0, self.rotation.z));
        let scale = FMat4::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
    fn matrix3(&self) -> FMat3 {
        let mut transform = FMat3::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;

        transform = transform.rotate(FVec4::new(0.0, 1.0, 0.0, self.rotation.y));
        transform = transform.rotate(FVec4::new(1.0, 0.0, 0.0, self.rotation.x));
        transform = transform.rotate(FVec4::new(0.0, 0.0, 1.0, self.rotation.z));

        let scale = FMat3::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
    fn normal_matrix(&self) -> FMat3 {
        self.set_scaling(Self::Scaling::from(1.0) / self.scaling()).matrix3()
    }
    fn rotate(&self, rot: Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation += rot;
        rotation
    }
    fn translate(&self, pos: Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation += pos;
        translation
    }
    fn scale(&self, scale: Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale += scale;
        scaling
    }
    fn rotation(&self) -> Self::Rotation { self.rotation }
    fn scaling(&self) -> Self::Scaling { self.scale }
    fn translation(&self) -> Self::Translation { self.translation }
    fn set_rotation(&self, rot: Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation = rot;
        rotation
    }
    fn set_scaling(&self, scale: Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale = scale;
        scaling
    }
    fn set_translation(&self, pos: Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation = pos;
        translation
    }
}

impl Default for Transform3D {
    fn default() -> Self { Self { translation: FVec3::from(0.0), scale: FVec3::from(1.0), rotation: FVec3::from(0.0) } }
}
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
    fn matrix4(&self) -> FMat4 {
        // let mut transform = FMat4::from_translation(self.translation);
        let mut transform = FMat4::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;
        transform.w.w = 1.0;
        transform = translate(&transform, self.translation);

        let rotation = FMat4::from(self.rotation);

        transform = transform * rotation;

        let scale = FMat4::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
    fn matrix3(&self) -> FMat3 {
        let mut transform = FMat3::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;

        let rotation = FMat3::from(self.rotation);

        transform = transform * rotation;

        let scale = FMat3::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
    fn normal_matrix(&self) -> FMat3 {
        self.set_scaling(Self::Scaling::from(1.0) / self.scaling()).matrix3()
    }
    fn rotate(&self, rot: Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation = rotation.rotation.rotate(rot);
        rotation
    }
    fn translate(&self, pos: Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation += pos;
        translation
    }
    fn scale(&self, scale: Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale += scale;
        scaling
    }
    fn rotation(&self) -> Self::Rotation { self.rotation }
    fn scaling(&self) -> Self::Scaling { self.scale }
    fn translation(&self) -> Self::Translation { self.translation }
    fn set_rotation(&self, rot: Self::Rotation) -> Self {
        let mut rotation = *self;
        rotation.rotation = rot;
        rotation
    }
    fn set_scaling(&self, scale: Self::Scaling) -> Self {
        let mut scaling = *self;
        scaling.scale = scale;
        scaling
    }
    fn set_translation(&self, pos: Self::Translation) -> Self {
        let mut translation = *self;
        translation.translation = pos;
        translation
    }
}

impl Default for TransformQuaternion3D {
    fn default() -> Self { Self { translation: FVec3::from(0.0), scale: FVec3::from(1.0), rotation: Quaternion::<f32>::from_euler(FVec3::from(0.0)) } }
}