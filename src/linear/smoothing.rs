use std::ops::{Add, Sub, Mul, DivAssign};

use num_traits::AsPrimitive;

use super::traits::{Number, FloatingPoint};
/// classical lerp used to linearly interpolate
/// between two values depending on what variable t
/// says. note that t should be a number between 0.0 - 1.0
pub fn lerp<T: FloatingPoint, V: Add<T, Output = V> + Add<Output = V> + Sub<V, Output = V> +Mul<T, Output = V> + Copy>(a: V, b: V, t: T) -> V {
    a + (b - a) * t
}
pub fn smoothstep<T: FloatingPoint, V: Add<T, Output = V> + Add<Output = V> + Sub<V, Output = V> +Mul<T, Output = V> + Copy>(a: V, b: V, t: T) -> V {
    (a - b * t) * (t * t)
}
pub fn inverse_lerp<T: AsPrimitive<V>, V: FloatingPoint + Sub + DivAssign + AsPrimitive<V>>(from: V, to: V, t: T) -> V {
    (t.as_() - from.as_()) / (to - from)
}
pub fn remap<T: AsPrimitive<V>, V: FloatingPoint + Sub + DivAssign + AsPrimitive<V>>(imin: V, imax: V, omin: V, omax: V, t: T) -> V {
    let t2 = inverse_lerp(imin, imax, t);
    lerp(omin, omax, t2)
}