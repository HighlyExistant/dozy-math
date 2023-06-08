#![allow(unused)]
use drowsed_math::linear::{self, DVec3, FMat2, FVec2, FMat3, FVec4, FMat4};

use crate::{complex::quaternion::Quaternion, linear::{vector::Vector3, FVec3, IndependentEulerRotation3D}};

mod complex;
fn main() {
    // let vec3 = DVec3::new(1.0, 1.0, 1.0);
    // println!("X 90 degrees: {:#?}", vec3.rotatex(std::f64::consts::PI/2.0));
    // println!("X 180 degrees: {:#?}", vec3.rotatex(std::f64::consts::PI));
    // println!("X 360 degrees: {:#?}", vec3.rotatex(std::f64::consts::PI*2.0));
    // println!("Y 90 degrees: {:#?}", vec3.rotatey(std::f64::consts::PI/2.0));
    // println!("Y 180 degrees: {:#?}", vec3.rotatey(std::f64::consts::PI));
    // println!("Y 360 degrees: {:#?}", vec3.rotatey(std::f64::consts::PI*2.0));
    // println!("Z 90 degrees: {:#?}", vec3.rotatez(std::f64::consts::PI/2.0));
    // println!("Z 180 degrees: {:#?}", vec3.rotatez(std::f64::consts::PI));
    // println!("Z 360 degrees: {:#?}", vec3.rotatez(std::f64::consts::PI*2.0));
    let quat1 = Quaternion::<f32>::from_euler(FVec3::new(0.0, 0.0, std::f32::consts::PI/2.0));
    let quat2 = Quaternion::<f32>::from_euler(FVec3::new(0.0, std::f32::consts::PI/2.0, std::f32::consts::PI/2.0));
    println!("quaternion 1: {:#?}", quat1);
    println!("quaternion 2: {:#?}", quat2);
    
    let vec3 =  FVec3::new(0.0, 1.0, 0.0);
    let x = quat1 * vec3;
    println!("x {}\ny {}\nz {}", x.x, x.y, x.z);
    println!("quaternion {:?}", (quat2 * quat1));
    // println!("vector {}", );
}