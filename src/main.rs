#![allow(unused)]

use linear::FVec2;
use num_traits::One;
use data::Average;
mod debug;
mod data;
mod linear;
mod complex;
use linear::barycentric_coordinates;
fn main() {
    let start = FVec2::new(0.5, 1.0);
    let control = FVec2::new(0.0, 1.4);
    let end = FVec2::new(-0.5, 0.9);
    let p = FVec2::new(0.0, 1.3);
    let bar_coords1 = barycentric_coordinates(start, end, control, p);
    let bar_coords2 = barycentric_coordinates(control, start, end, p);
    let bar_coords3 = barycentric_coordinates(end, control, start, p);
    let bar_coords4 = barycentric_coordinates(control, end, start, p);
    let x = &[43, 134,34,3,4,34,3,134];

    println!("{}", x.mean());
    println!("{:?}", bar_coords1);
    println!("{:?}", bar_coords2);
    println!("{:?}", bar_coords3);
    println!("{:?}", bar_coords4);
}