#![allow(unused)]
use drowsed_math::{linear::{self, DVec3, FMat2, FVec2, FMat3, FVec4, FMat4, lerp}};
pub mod debug;
use crate::{complex::quaternion::Quaternion, linear::{vector::Vector3, FVec3}, debug::create_cube};

mod complex;
fn main() {
    let vec1 = FVec2::new(0.0, 1.0);
    let vec2 = FVec2::new(1.0, 1.0);
    println!("{:?}", vec1 + vec2);
}