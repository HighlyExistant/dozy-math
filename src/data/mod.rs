use std::ops::DivAssign;

use crate::linear::traits::Number;

pub trait Average { fn mean(&self) -> Self::Output; type Output; }
// macro_rules! impl_average { ($($primitive:ty),+) => { $( impl Average for $primitive { fn mean(data: &[Self]) -> Self { data.iter().sum::<Self>() / data.len() as Self } } )+ }; }
// impl_average!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, f32, f64);

impl<T: Number + std::convert::From<usize>, const N: usize> Average for [T; N] {
    fn mean(&self) -> T {
        let iter = self.iter();
        let sum: T = iter.fold(T::zero(), |a, b|{ a + *b }) / self.len().into();
        sum
    }
    type Output = T;
}