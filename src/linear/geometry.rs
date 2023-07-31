use super::{traits::FloatingPoint, Vector};

pub trait EuclideanGeometry: Vector {
    /// In 2 Dimensions the Cross Product is a single scalar value while in 3 dimensions it is a
    /// 3 dimensional vector.
    type CrossProduct;
    fn cross(&self, other: Self) -> Self::CrossProduct;
}
pub trait NonEuclideanGeometry {
    
}