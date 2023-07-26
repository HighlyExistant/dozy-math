use crate::linear::Vector;

use super::{FVec2, vector::Vector2, traits::Number, FVec3, geometry::EuclideanGeometry};

pub fn line_sdf<T: Number>(a: Vector2<T>, b: Vector2<T>, p: Vector2<T>) -> T {
    (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y)
}
/// barycentric_t should be equal to 1.0 - barycentric_coordinates.x - barycentric_coordinates.y
pub fn quadratic_bezier_curve_sdf(start: FVec2, end: FVec2, control: FVec2, barycentric_coordinates: FVec3) -> f32 {
    let control_to_start = start - control;
    let control_to_end = end - control;
    let cross_z = control_to_start.cross(control_to_end);
    let uv_p = FVec2::new(0.5 * barycentric_coordinates.x + barycentric_coordinates.z, barycentric_coordinates.z);
    if cross_z < 0.0 {
        uv_p.y - uv_p.x * uv_p.x
    } else {
        uv_p.x * uv_p.x - uv_p.y
    }
}