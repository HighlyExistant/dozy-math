use super::{traits::FloatingPoint, Vector};

pub trait EuclideanGeometry: Vector {
    /// In 2 Dimensions the Cross Product is a single scalar value while in 3 dimensions it is a
    /// 3 dimensional vector.
    type CrossProduct;
    /// the distance between two vectors is calculated the same way as the length
    /// in euclidean geometry so this function can be used the same way.
    fn length(&self, other: &Self) -> <Self as Vector>::Scalar
        where <Self as Vector>::Scalar: FloatingPoint;
    fn cross(&self, other: Self) -> Self::CrossProduct;
}
pub trait NonEuclideanGeometry {
    
}