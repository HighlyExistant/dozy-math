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
    // let c0 = Complex::new(2.0, 3.0);
    // let c1 = Complex::new(4.0, 6.0);
    println!("{:#?}", f64::cubic_formula(1.0, 3.0, 1.0, -0.5));
}