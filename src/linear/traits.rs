use core::fmt;
use std::ops::{SubAssign, MulAssign, DivAssign, RemAssign, AddAssign};

use num_traits::{Num, NumCast, Float, Signed, Unsigned, Bounded, AsPrimitive};

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
    Number
    + Float
    + AsPrimitive<f32>
    + AsPrimitive<f64>
{
}

impl<T> FloatingPoint for T where
    T: Number
        + Float
        + AsPrimitive<f32>
        + AsPrimitive<f64>
{
}
pub trait FurthestVector: Sized {
    fn furthest_vector(&self, this: &[Self], transform: &Self::Transform);
    type Transform;
}