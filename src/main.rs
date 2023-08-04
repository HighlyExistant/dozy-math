// #![allow(unused)]
mod linear;
pub mod complex;
pub mod data;
pub mod equations;
use core::num;

pub use linear::*;

use crate::complex::Complex;
use crate::smoothing::inverse_lerp;
use equations::QuadraticFormula;
use equations::CubicFormula;
fn main() {
    let mat2x3 = FMat2x3::new( 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let mat3x2 = FMat3x2::new( 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    println!("{:#?}", mat2x3 * mat3x2);
}