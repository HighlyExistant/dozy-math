use crate::{complex::Complex, FloatingPoint, Number};
#[derive(Debug)]
pub enum QuadraticSolution<T: Number> {
    // used when all variables apart from the final offset variable are 0 this causes the graph to 
    // be at a constant change equal to the value.
    None(T),
    // used when all variables are equal to 0 in which all values in the graph are roots
    All(T),
    // used either when the quadratic term is 0 making it a linear equation or when the discriminant is equal to 0
    // making a repeated root.
    OneReal(T),
    TwoReal([T; 2]),
    TwoComplex([Complex<T>; 2]),
}
pub trait QuadraticFormula: Sized + Number {
    /// used to solve the equation ax^2 + bx + c
    /// returns the amount of solutions found and an array filled with
    /// the solutions. Unlike a Linear equation or a Cubic equation 
    /// the quadratic is not garunteed to have a real solution.
    /// for more information on the quadratic formula check [this wikipedia article](https://en.wikipedia.org/wiki/Quadratic_formula).
    /// the definition of this function is heavily inspired from the *solveQuadratic* function from [msdfgen](https://github.com/Chlumsky/msdfgen/blob/master/core/equation-solver.cpp).
    fn quadratic_formula(a: Self, b: Self, c: Self) -> QuadraticSolution<Self>;
}

impl QuadraticFormula for f32 {
    fn quadratic_formula(a: Self, b: Self, c: Self) -> QuadraticSolution<Self> {
        // when a == 0 the quadratic part of the equation is 0 making it a linear equation
        // limiting it to only 1 solution
        if a == 0.0 || b.abs() > 1e12 * a.abs() {
            // if both a and b are equal to 0 there are no solutions
            if b == 0.0 {
                if c == 0.0 {
                    return QuadraticSolution::All(0.0);
                }
                return QuadraticSolution::None(0.0);
            }
            // since this is now a linear equation we calculate the root by dividing
            // the displacement by the slope.
            
            return QuadraticSolution::OneReal(-c/b);
        }
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let sqrt_dscr = discriminant.sqrt();
            return QuadraticSolution::TwoReal([(-b+sqrt_dscr)/(2.0*a), (-b-sqrt_dscr)/(2.0*a)]);
        } else if discriminant == 0.0 {
            return QuadraticSolution::OneReal(-b/(2.0*a));
        } else {
            let isqrt_dscr_1 = Complex::new(-b, discriminant.abs().sqrt()) * Complex::new(0.0, 1.0/(2.0*a));
            let isqrt_dscr_2 = Complex::new(-b, -discriminant.abs().sqrt()) * Complex::new(0.0, 1.0/(2.0*a));
            return QuadraticSolution::TwoComplex([isqrt_dscr_1, isqrt_dscr_2]);
        }
    }
}
impl QuadraticFormula for f64 {
    fn quadratic_formula(a: Self, b: Self, c: Self) -> QuadraticSolution<Self> {
        // when a == 0 the quadratic part of the equation is 0 making it a linear equation
        // limiting it to only 1 solution
        if a == 0.0 || b.abs() > 1e12 * a.abs() {
            // if both a and b are equal to 0 there are no solutions
            if b == 0.0 {
                if c == 0.0 {
                    return QuadraticSolution::All(0.0);
                }
                return QuadraticSolution::None(0.0);
            }
            // since this is now a linear equation we calculate the root by dividing
            // the displacement by the slope.
            
            return QuadraticSolution::OneReal(-c/b);
        }
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let sqrt_dscr = discriminant.sqrt();
            return QuadraticSolution::TwoReal([(-b+sqrt_dscr)/(2.0*a), (-b-sqrt_dscr)/(2.0*a)]);
        } else if discriminant == 0.0 {
            return QuadraticSolution::OneReal(-b/(2.0*a));
        } else { // when the discriminant is less than zero there two complex solutions
            let a2 = 2.0 * a;
            
            let solution1 = Complex::new(-b / a2, discriminant.abs().sqrt() / a2);
            let solution2 = Complex::new(solution1.re, -solution1.im);
            return QuadraticSolution::TwoComplex([solution1, solution2]);
        }
    }
}