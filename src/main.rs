#![allow(unused)]
use drowsed_math::{linear::{self, DVec3, FMat2, FVec2, FMat3, FVec4, FMat4, smoothing::{smoothstep, lerp}}};
pub mod debug;
use crate::{complex::quaternion::Quaternion, linear::{FVec3}, debug::create_cube};

mod complex;
fn main() {
    let mat1 = FMat4::identity(2.0).scale(FVec3::new(1.0, 3.0, 2.0)).translate(FVec3::new(4.0, 1.0, 3.0));
    let mat2 = FMat4::identity(1.0).scale(FVec3::new(4.0, 1.0, 1.0)).translate(FVec3::new(2.0, 5.0, 1.0));

    for i in 1..26 {
        let t = (i as f32) / 25.0;
        println!("{}: {:?}",i,  smoothstep(FVec3::new(25.0, 12.0, 32.0), FVec3::new(0.0, 1.0, 64.0), t));
    }
}