use num_traits::{AsPrimitive, clamp, Signed};

use crate::{complex::Complex, FloatingPoint, Number, Vector, Vector2, EuclideanGeometry, Vector3};
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

pub fn sd_bezier<T: FloatingPoint >(pos: Vector2<T>, v0: Vector2<T>, v1: Vector2<T>, v2: Vector2<T>) -> T
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T>,
{    
    let a: Vector2<T> = v1 - v0;
    let b: Vector2<T> = v0 - v1 * 2.0.as_() + v2;
    let c: Vector2<T> = a * 2.0.as_();
    let d: Vector2<T> = v0 - pos;

    let kk = T::one()/b.dot(&b);
    let kx = kk * a.dot(&b);
    let ky = kk * (2.0.as_()*a.dot(&a)+d.dot(&b))/3.0.as_();
    let kz = kk * d.dot(&a);      

    let mut res = T::zero();
    let mut sgn = T::zero();

    let p: T = ky - kx*kx;
    let q: T = kx*(2.0.as_()*kx*kx - 3.0.as_()*ky) + kz;
    let p3: T = p*p*p;
    let q2: T = q*q;
    let mut h: T = q2 + 4.0.as_()*p3;

    if( h>=T::zero() ) 
    {   // 1 root
        h = h.sqrt();
        let mut x = (Vector2::new(h,-h)-q)/2.0.as_();

        // #if 0
        // When p≈0 and p<0, h-q has catastrophic cancelation. So, we do
        // h=√(q²+4p³)=q·√(1+4p³/q²)=q·√(1+w) instead. Now we approximate
        // √ by a linear Taylor expansion into h≈q(1+½w) so that the q's
        // cancel each other in h-q. Expanding and simplifying further we
        // get x=vec2(p³/q,-p³/q-q). And using a second degree Taylor
        // expansion instead: x=vec2(k,-k-q) with k=(1-p³/q²)·p³/q
        if( p.abs()<0.001.as_() )
        {
            let k = p3/q;              // linear approx
          //float k = (1.0-p3/q2)*p3/q;  // quadratic approx 
            x = Vector2::new(k,-k-q);  
        }
        // #endif

        // let uv = x.signum()*pow(abs(x), vec2(1.0/3.0));
        let uv = x.signum() * x.abs().pow(Vector2::from(T::one()/3.0.as_()));
        let t = clamp( uv.x+uv.y-kx, T::zero(), T::one() );
        let q = d+(c+b*t)*t;
        res = q.dot(&q); // ! Double check this
        //     	sgn = cro(c+2.0*b*t,q);
        let dum1: Vector2<T> = c+b*t*2.0.as_();
    	sgn = dum1.cross(q); // ! Might be cross product idk
    }
    else 
    {   // 3 roots
        let z = (-p).sqrt();
        let v = (q/(p*z*2.0.as_())).acos()/3.0.as_();
        let m = v.cos();
        let n = v.sin()*1.732050808.as_();
        let mut t = Vector3::new(m+m,-n-m,n-m)*z-kx;
        t = t.clamp(Vector3::from(T::zero()), Vector3::from(T::one()) );
        
        let  qx=d+(c+b*t.x)*t.x; 
        let dx= qx.dot(&qx);
        let dum2: Vector2<T> = c+b*t.x*2.0.as_();
        let sx: T = dum2.cross(qx);
        
        let  qy=d+(c+b*t.y)*t.y; 
        let dy= qy.dot(&qy);
        let dum3: Vector2<T> = c+b*t.y*2.0.as_();
        let sy = dum3.cross(qy);

        if dx<dy { res=dx; sgn=sx; } else {res=dy; sgn=sy; }
    }
    
    return res.sqrt()*sgn.signum();
}