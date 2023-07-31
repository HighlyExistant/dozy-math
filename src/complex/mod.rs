use num_traits::MulAddAssign;

use crate::{FloatingPoint, Vector2, Matrix2, Number};

pub mod quaternion;
#[derive(Clone, Copy, Debug, Default)]
pub struct Complex<T: Number> {
    pub re: T,
    pub im: T
}

impl<T: Number> Complex<T>  {
    pub fn new(real: T, imaginary: T) -> Self {
        Self { re: real, im: imaginary }
    }
    // matrix representation of a complex number
    pub fn matrix(&self) -> Matrix2<T> {
        Matrix2 { x: Vector2::<T>::new(self.re, self.im) , y: Vector2::<T>::new(self.im, self.re)  }
    }
    // matrix representation of a complex number with only the imaginary part
    pub fn matrix_im(&self) -> Matrix2<T> {
        Matrix2 { x: Vector2::<T>::new(T::zero(), self.im) , y: Vector2::<T>::new(self.im, T::zero())  }
    }
    // matrix representation of a complex number with only the real part
    pub fn matrix_re(&self) -> Matrix2<T> {
        Matrix2 { x: Vector2::<T>::new(self.re, T::zero()) , y: Vector2::<T>::new(T::zero(), self.re)  }
    }
}
impl<T: Number> std::ops::Mul for Complex<T>  {
    fn mul(self, rhs: Self) -> Self {
        Self { re: self.re * rhs.re - self.im * rhs.im, im: (self.re * rhs.im + self.im * rhs.re) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Add for Complex<T>  {
    fn add(self, rhs: Self) -> Self {
        Self { re: self.re + rhs.re, im: self.im + rhs.im }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Complex<T>  {
    fn sub(self, rhs: Self) -> Self {
        Self { re: self.re - rhs.re, im: self.im - rhs.im }
    }
    type Output = Self;
}