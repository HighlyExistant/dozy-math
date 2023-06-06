use core::fmt;
use std::ops::{SubAssign, MulAssign, DivAssign, RemAssign, AddAssign};

use num_traits::{Num, NumCast, Float, Signed, Unsigned};


pub trait Number:
    Copy
    + Clone
    + fmt::Debug
    + Num
    + Signed
    + NumCast
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
}

impl<T> Number for T where
    T: Copy
        + Clone
        + fmt::Debug
        + Num
        + Signed
        + NumCast
        + PartialOrd
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
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
    Number
    + Float
{
}

impl<T> FloatingPoint for T where
    T: Number
        + Float
{
}