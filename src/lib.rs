extern crate num_complex;

use std::vec::Vec;
use num_complex::Complex64;

pub fn convex_hull(_polygon: Vec<Complex64>) -> Result<Vec<Complex64>, &'static str> {
    if _polygon.len() == 0 {
        return Err("only 0 Point(s)");
    }

    if _polygon.len() == 1 {
        return Err("only 1 Point(s)");
    }

    return Err("only 2 Point(s)");
}
