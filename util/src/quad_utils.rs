use std::ops::{Add, Div, Mul, Sub};

pub fn find_intersection<T>(a1: T, b1: T, c1: T, a2: T, b2: T, c2: T) -> Option<(f64, f64)>
where T: Into<f64> + Add<Output = T> + Sub<Output = T> + Mul + Div + Copy
{
    let (a1, b1, c1, a2, b2, c2) = (a1.into(), b1.into(), c1.into(), a2.into(), b2.into(), c2.into());
    let (a, b, c) = (a1 - a2, b1 - b2, c1 - c2);
    if a == 0f64 {
        return Some(((-1f64 * c) / b, (-1f64 * c) / b));
    }
    let d_square = (b.powi(2)) - (4f64 * a * c);
    if d_square < 0f64 {
        return None;
    }
    let d = d_square.powf(0.5);
    Some(((-b -d) / (2.0 * a), (-b + d) / (2.0 * a)))
}
