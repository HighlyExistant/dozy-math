#![allow(unused)]

use crate::complex::quaternion::Quaternion;

use super::{translate, FVec2, FMat2, FVec3, vector::Vector3, FMat4, Rotation3D};
#[derive(Clone, Copy)]
pub struct Transform2D {
    pub translation: FVec2,
    pub scale: FVec2,
    pub rotation: f32,
}

impl Transform2D {
    #[inline]
    fn new(translation: FVec2, scale: FVec2, rotation: f32) -> Self {
        Self { translation, scale, rotation }
    }
    pub fn mat2(&self) -> FMat2 {
        let sin_rot = self.rotation.sin();
        let cos_rot = self.rotation.cos();
        let mat_rot = FMat2::from_vec(FVec2::new(cos_rot, sin_rot), FVec2::new(-sin_rot, cos_rot));
        
        let mat_scale = FMat2::from_vec(FVec2::new(self.scale.x, 0.0), FVec2::new(0.0, self.scale.y));
        return mat_rot * mat_scale;
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        let translation = FVec2::new(0.0, 0.0);
        let scale = FVec2::new(1.0, 1.0);
        let rotation = 0.0;
        Self { translation, scale, rotation }
    }
}
#[derive(Clone, Copy)]
pub struct Transform3D {
    pub translation: FVec3,
    pub scale: FVec3,
    pub rotation: FVec3,
}
impl Default for Transform3D {
    fn default() -> Self {
        Self { translation: FVec3::from(0.0), scale: FVec3::from(1.0), rotation: FVec3::from(0.0) }
    }
}
impl Transform3D {
    #[inline]
    fn new(translation: FVec3, scale: FVec3, rotation: FVec3) -> Self {
        Self { translation, scale, rotation }
    }
    pub fn mat4(&self) -> FMat4 {
        // let mut transform = FMat4::from_translation(self.translation);
        let mut transform = FMat4::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;
        transform.w.w = 1.0;
        transform = translate(&transform, self.translation);
        // let rotation = FMat4::from(self.rotation);

        transform = transform.rotate(self.rotation.y, FVec3::new(0.0, 1.0, 0.0));
        transform = transform.rotate(self.rotation.x, FVec3::new(1.0, 0.0, 0.0));
        transform = transform.rotate(self.rotation.z, FVec3::new(0.0, 0.0, 1.0));
        let scale = FMat4::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
}

#[derive(Clone, Copy)]
pub struct TransformQuaternion3D {
    pub translation: FVec3,
    pub scale: FVec3,
    pub rotation: Quaternion<f32>,
}
impl TransformQuaternion3D {
    #[inline]
    fn new(translation: FVec3, scale: FVec3, rotation: Quaternion<f32>) -> Self {
        Self { translation, scale, rotation }
    }
    pub fn mat4(&self) -> FMat4 {
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
}