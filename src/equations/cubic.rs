use std::fmt::Debug;

use crate::{Number, complex::Complex, FloatingPoint};

use super::QuadraticFormula;
/// Contains all values in QuadraticSolution but extended to handle cubics.
#[derive(Debug)]
pub enum CubicSolution<T: Number> {
    None(T),
    All(T),
    OneReal(T),
    // Used either when the solution is quadratic or when a cubic has a repeating solution
    TwoReal([T; 2]),
    TwoComplex([Complex<T>; 2]),
    ThreeReal([T; 3]),
    OneRealTwoComplex(T, [Complex<T>; 2]),
}
pub trait CubicFormula: QuadraticFormula {
    fn cubic_formula(a: Self, b: Self, c: Self, d: Self) -> CubicSolution<Self>;
}

impl CubicFormula for f32 {
    fn cubic_formula(a: Self, b: Self, c: Self, d: Self) -> CubicSolution<f32> {
        const INV_9: f32 = 1.0/9.0;
        const INV_3: f32 = 1.0/3.0;
        const INV_54: f32 = 1.0/54.0;
        const SQRT_3: f32 = 1.7320508;
        // if a equals 0 the cubic becomes a quadratic
        if a == 0.0 {
            return match f32::quadratic_formula(b, c, d) {
                crate::equations::QuadraticSolution::None(n) => { CubicSolution::None(n) }
                crate::equations::QuadraticSolution::All(a) => { CubicSolution::All(a) }
                crate::equations::QuadraticSolution::OneReal(r) => { CubicSolution::OneReal(r) }
                crate::equations::QuadraticSolution::TwoReal(r) => { CubicSolution::TwoReal(r) }
                crate::equations::QuadraticSolution::TwoComplex(c) => { CubicSolution::TwoComplex(c) }
            }
        }
        if d == 0.0 {
            return CubicSolution::OneReal(0.0);
        }
        let b = b / a;
        let c = c / a;
        let d = d / a;

        let b2 = b * b;
        let q = (3.0 * c - b2) * INV_9;
        let q3 = q * q * q;
        let r = (-(27.0*d) + b*(9.0*c - 2.0*(b2))) * INV_54;
        let discriminant = q3 * r * r;
        let mut term1 = b * INV_3;
        if discriminant > 0.0 { // contains 1 real and 2 complex roots
            let mut s = r + discriminant.sqrt();
            s = if s < 0.0 { (-s).powf(INV_3) } else { s.powf(INV_3) };
            let mut t = r - discriminant.sqrt();
            t = if t < 0.0 { (-t).powf(INV_3) } else { t.powf(INV_3) };
            
            let x1re = -term1 + s + t;
            term1 += (s + t)/2.0;
            let x3re = -term1;
            let x2re = x3re;
            term1 = SQRT_3*(-t + s)/2.0;
            let x2im = term1;
            let x3im = -term1;
            return CubicSolution::OneRealTwoComplex(x1re, [Complex::new(x2re, x2im), Complex::new(x3re, x3im)]);
        }

        // when discriminant is 0 it contains 3 real solutions 2 of which repeat.
        if discriminant == 0.0 {
            let r13 = if r < 0.0 { (-r).powf(INV_3) } else { r.powf(INV_3) };
            let real_solution = -term1 + 2.0 * r13;
            let real_solution_repeating = -(r13 + term1);
            return CubicSolution::TwoReal([real_solution, real_solution_repeating]);
        }

        // discriminant < 0

        let sqrt_acos_q3 = (r/(-q3).sqrt()).acos();
        let sqrt_q2 = (-q).sqrt() * 2.0;
        let real_1 = -term1 + sqrt_q2 * (sqrt_acos_q3 * INV_3);
        let real_2 = -term1 + sqrt_q2*((sqrt_acos_q3 + 2.0*std::f32::consts::PI) * INV_3).cos();
        let real_3 = -term1 + sqrt_q2*((sqrt_acos_q3 + 4.0*std::f32::consts::PI) * INV_3).cos();

        CubicSolution::ThreeReal([real_1, real_2, real_3])
    }
}

impl CubicFormula for f64 {
    fn cubic_formula(a: Self, b: Self, c: Self, d: Self) -> CubicSolution<f64> {
        const INV_9: f64 = 1.0/9.0;
        const INV_3: f64 = 1.0/3.0;
        const INV_54: f64 = 1.0/54.0;
        const SQRT_3: f64 = 1.7320508075688772;
        // if a equals 0 the cubic becomes a quadratic
        if a == 0.0 {
            return match f64::quadratic_formula(b, c, d) {
                crate::equations::QuadraticSolution::None(n) => { CubicSolution::None(n) }
                crate::equations::QuadraticSolution::All(a) => { CubicSolution::All(a) }
                crate::equations::QuadraticSolution::OneReal(r) => { CubicSolution::OneReal(r) }
                crate::equations::QuadraticSolution::TwoReal(r) => { CubicSolution::TwoReal(r) }
                crate::equations::QuadraticSolution::TwoComplex(c) => { CubicSolution::TwoComplex(c) }
            }
        }

        let b2 = b * b;
        let q = (3.0 * c - b2) * INV_9;
        let q3 = q * q * q;
        let r = (-(27.0*d) + b*(9.0*c - 2.0*b2)) * INV_54;
        let discriminant = q3 + r * r;
        let mut term1 = b * INV_3;
        if discriminant > 0.0 { // contains 1 real and 2 complex roots
            let mut s = r + discriminant.sqrt();
            s = if s < 0.0 { -(-s).powf(INV_3) } else { s.powf(INV_3) };
            let mut t = r - discriminant.sqrt();
            t = if t < 0.0 { -(-t).powf(INV_3) } else { t.powf(INV_3) };
            
            let x1re = -term1 + s + t;
            term1 += (s + t)/2.0;
            let x3re = -term1;
            let x2re = x3re;
            term1 = SQRT_3*(-t + s)/2.0;
            let x2im = term1;
            let x3im = -term1;
            return CubicSolution::OneRealTwoComplex(x1re, [Complex::new(x2re, x2im), Complex::new(x3re, x3im)]);
        }

        // when discriminant is 0 it contains 3 real solutions 2 of which repeat.
        if discriminant == 0.0 {
            let r13 = if r < 0.0 { (-r).powf(INV_3) } else { r.powf(INV_3) };
            let real_solution = -term1 + 2.0 * r13;
            let real_solution_repeating = -(r13 + term1);
            return CubicSolution::TwoReal([real_solution, real_solution_repeating]);
        }

        // discriminant < 0

        let sqrt_acos_q3 = (r/(-q3).sqrt()).acos();
        let sqrt_q2 = (-q).sqrt() * 2.0;
        let real_1 = -term1 + sqrt_q2 * (sqrt_acos_q3 * INV_3).cos();
        let real_2 = -term1 + sqrt_q2*((sqrt_acos_q3 + 2.0*std::f64::consts::PI) * INV_3).cos();
        let real_3 = -term1 + sqrt_q2*((sqrt_acos_q3 + 4.0*std::f64::consts::PI) * INV_3).cos();

        CubicSolution::ThreeReal([real_1, real_2, real_3])
    }
}