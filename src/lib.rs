extern crate num_complex;

use std::vec::Vec;
use num_complex::Complex64;

pub fn convex_hull(polygon: Vec<Complex64>) -> Result<Vec<Complex64>, String> {
    if polygon.len() <= 2 {
        let message = format!("only {} Point(s)", polygon.len());
        return Err(message);
    }

    return Result::Ok(polygon);
}
