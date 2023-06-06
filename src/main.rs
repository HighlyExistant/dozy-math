use crate::{complex::quaternion::Quaternion, linear::vector::Vector3};

mod linear;
mod complex;
fn main() {
    // let mat3f32_1 = linear::matrix::Matrix4::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0);
    // let mat3f32_2 = linear::matrix::Matrix4::<f32>::new(16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0);
    let vec2f32 = linear::USVec2::new(1, 2);
    let vec2f64 = linear::UIVec2::from(vec2f32);
    let vec2f32_2 = linear::USVec2::from(vec2f64);
    let vec2f32_3 = linear::FVec2::from(vec2f32_2);
    println!("{:#?}\n", Quaternion::<f64>::from_euler(Vector3::new(1.45, 1.45, 1.45)));
}