use std::ops::{Index, IndexMut};

use num_traits::Zero;

use crate::linear::vector::Vector;

use super::FVec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Triangle<T: Vector> {
    points: [T; 3]
}
impl<T: Vector> Triangle<T> {
    fn new(p0: T, p1: T, p2: T) -> Self {
        Self { points: [p0, p1, p2] }
    }
}
impl<T: Vector> Index<usize> for Triangle<T> {
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index > 2, "index for a triangle can not exceed index 2 since it only has 3 points");
        &self.points[index]
    }
    type Output = T;
}

impl<T: Vector> IndexMut<usize> for Triangle<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index > 2, "index for a triangle can not exceed index 2 since it only has 3 points");
        &mut self.points[index]
    }
}

// Simplex Code
pub struct Simplex<T: Vector + Zero, const N: usize> {
    pub points: [T; N],
    pub size: usize,
}

impl<T: Vector + Zero, const N: usize> Simplex<T, N> {
    pub fn new() -> Self {
        Self { points: [T::zero(); N], size: 0 }
    }
    pub fn push(&mut self, point: T) {
        for i in (1..N).rev() {
            self.points[i] = self.points[i - 1];
        }
        self.points[0] = point;
        self.size = std::cmp::min(self.size + 1, N);
    }
    pub fn initialize(&mut self, list: Vec<T>) {
        for (i, v) in list.iter().enumerate() { self.points[i] = *v; }
        self.size = list.len();
    }
}