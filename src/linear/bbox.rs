use num_traits::{Zero, One};

use super::{Number, shapes::{Rectangle, Cube}, LinearSegment, Vector2};
pub trait BoundingBox2D<T: Number> {
    fn bbox(&self) -> Rectangle<T>;
}
pub trait BoundingBox3D<T: Number> {
    fn bbox(&self) -> Cube<T>;
}


impl<T: Number> BoundingBox2D<T> for LinearSegment<Vector2<T>> {
    fn bbox(&self) -> Rectangle<T> {
        Rectangle::new(Vector2::<T>::one(), Vector2::<T>::one(), Vector2::<T>::one(), Vector2::<T>::one())
    }
}