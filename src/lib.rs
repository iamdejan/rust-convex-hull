extern crate num_complex;

use std::vec::Vec;
use num_complex::Complex64;

pub fn convex_hull(polygon: Vec<Complex64>) -> Result<Vec<Complex64>, String> {
    if polygon.len() <= 2 {
        let message = format!("only {} Point(s)", polygon.len());
        return Err(message);
    }

    let origin_y: f64 = 0.0;
    let mut same_y = true;
    for i in 1..(polygon.len()) {
        let point = polygon[i];
        if point.im != origin_y {
            same_y = false;
            break;
        }
    }
    if same_y {
        return Result::Ok(Vec::new());
    }

    return Result::Ok(polygon);
}
