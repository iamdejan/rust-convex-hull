extern crate num_complex;

use std::vec::Vec;
use num_complex::Complex64;

pub fn convex_hull(_polygon: Vec<Complex64>) -> Result<Vec<Complex64>, &'static str> {
    return Err("there is only 1 Point");
}
