use std::vec::Vec;
use num_complex::Complex64;

use rust_convex_hull::convex_hull;

#[test]
fn no_point() {
    let polygon: Vec<Complex64> = Vec::new();
    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err(), Some("only 0 Point(s)".to_string()));
}

#[test]
fn one_point() {
    let point: Complex64 = Complex64::new(0.0, 0.0);
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(point);

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err(), Some("only 1 Point(s)".to_string()));
}

#[test]
fn two_points() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err(), Some("only 2 Point(s)".to_string()));
}

#[test]
fn three_points_convex() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));
    polygon.push(Complex64::new(0.0, 0.1));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn three_points_colinear_horizontal() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));
    polygon.push(Complex64::new(0.2, 0.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn three_points_colinear_vertical() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.0, 0.1));
    polygon.push(Complex64::new(0.0, 0.2));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn four_points_colinear_horizontal() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));
    polygon.push(Complex64::new(0.2, 0.0));
    polygon.push(Complex64::new(0.3, 0.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn four_points_colinear_vertical() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.0, 0.1));
    polygon.push(Complex64::new(0.0, 0.2));
    polygon.push(Complex64::new(0.0, 0.3));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}
