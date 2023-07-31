// #![allow(unused)]
mod linear;
pub mod complex;
pub mod data;
pub mod equations;
use core::num;

pub use linear::*;

use crate::complex::Complex;
use equations::QuadraticFormula;
use equations::CubicFormula;
fn main() {
    let segment = QuadraticSegment::new(FVec2::new(0.5,1.0), FVec2::new(0.0,1.4), FVec2::new(-0.5,0.9));
    let v0 = FVec2::new(0.0, 1.1);
    println!("{:#?}", segment.signed_distance(v0));
}