// #![allow(unused)]
mod linear;
pub mod complex;
pub mod data;
pub mod equations;
use core::num;

pub use linear::*;

use crate::complex::Complex;
use crate::complex::quaternion::Quaternion;
use crate::smoothing::inverse_lerp;
use equations::QuadraticFormula;
use equations::CubicFormula;
fn main() {
    let transform = TransformQuaternion3D {
        ..Default::default()
    };
    
    println!("{:#?}", Quaternion::<f32>::from_euler(FVec3::new(1.0, 0.0, 0.0)));
    println!("{:#?}", Quaternion::<f32>::from_euler(FVec3::new(0.0, 1.0, 0.0)));
    println!("{:#?}", Quaternion::<f32>::from_euler(FVec3::new(0.0, 0.0, 1.0)));
    // println!("{:#?}", Quaternion::from_euler(FVec3::new(0.2,1.12,2.31)).to_euler());
}