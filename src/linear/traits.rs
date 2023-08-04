use core::fmt;
use std::ops::{SubAssign, MulAssign, DivAssign, RemAssign, AddAssign};

use num_traits::{Num, NumCast, Float, Signed, Unsigned, Bounded, AsPrimitive};

use crate::equations::{QuadraticFormula, CubicFormula};

use super::{transform, matrix::Matrix4};


pub trait Number:
    Copy
    + Clone
    + fmt::Debug
    + Num
    + NumCast
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + PartialEq
    + PartialOrd
    + Bounded
{
}

impl<T> Number for T where
    T: Copy
        + Clone
        + fmt::Debug
        + Num
        + NumCast
        + PartialOrd
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + PartialEq
        + PartialOrd
        + Bounded
{
}
pub trait SignedNumber: 
    Signed 
    + Number {
    
}
impl<T> SignedNumber for T where
    T: Signed 
    + Number
{

}

pub trait UnsignedNumber: 
    Unsigned
    + Number {
    
}
impl<T> UnsignedNumber for T where
    T: Unsigned 
    + Number
{

}

/// Base floating point types
pub trait FloatingPoint:
    SignedNumber
    + Float
    + AsPrimitive<f32>
    + AsPrimitive<f64>
    + QuadraticFormula
    + CubicFormula
{
}

impl<T> FloatingPoint for T where
    T: SignedNumber
        + Float
        + AsPrimitive<T>
        + AsPrimitive<f32>
        + AsPrimitive<f64>
        + QuadraticFormula
        + CubicFormula
{
}