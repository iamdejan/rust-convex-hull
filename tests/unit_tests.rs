use std::vec::Vec;
use num_complex::Complex64;

use rust_convex_hull::convex_hull;

#[test]
fn single_point() {
    let point: Complex64 = Complex64::new(0.0, 0.0);
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(point);

    let result: Result<Vec<Complex64>, &'static str> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
}
