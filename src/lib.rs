extern crate num_complex;

use num_complex::Complex64;
use std::vec::Vec;

fn cross(a: Complex64, b: Complex64) -> f64 {
    return (a.conj() * b).im;
}

fn orientation(a: Complex64, b: Complex64, c: Complex64) -> f64 {
    return cross(b - a, c - b);
}

fn invalid_orientation(a: Complex64, b: Complex64, c: Complex64) -> bool {
    return orientation(a, b, c) <= 0.0;
}

fn is_horizontal_colinear(polygon: &Vec<Complex64>) -> bool {
    let origin_y: f64 = polygon[0].im;
    let mut colinear = true;
    for i in 1..(polygon.len()) {
        let point = polygon[i];
        if point.im != origin_y {
            colinear = false;
            break;
        }
    }
    return colinear;
}

fn is_vertical_colinear(polygon: &Vec<Complex64>) -> bool {
    let origin_x: f64 = polygon[0].re;
    let mut colinear = true;
    for i in 1..(polygon.len()) {
        let point = polygon[i];
        if point.re != origin_x {
            colinear = false;
            break;
        }
    }
    return colinear;
}

fn lower_bound(polygon: &Vec<Complex64>) -> Vec<Complex64> {
    let mut result: Vec<Complex64> = Vec::new();
    let size = polygon.len();
    for i in 0..size {
        if i > 0 && polygon[i] == polygon[i - 1] {
            continue;
        }

        let mut result_size = result.len();
        while result_size >= 2
            && invalid_orientation(result[result_size - 2], result[result_size - 1], polygon[i])
        {
            result.pop().unwrap();
            result_size = result.len();
        }
        result.push(polygon[i]);
    }

    return result;
}

fn upper_bound(mut result: Vec<Complex64>, polygon: Vec<Complex64>) -> Vec<Complex64> {
    let size = polygon.len();
    let length = result.len() + 1;
    for i in (0..=size - 2).rev() {
        if polygon[i] == polygon[i + 1] {
            continue;
        }

        let mut result_size = result.len();
        while result_size >= length
            && invalid_orientation(result[result_size - 2], result[result_size - 1], polygon[i])
        {
            result.pop().unwrap();
            result_size = result.len();
        }
        result.push(polygon[i]);
    }

    result.pop();
    return result;
}

fn sort_polygon_ccw(polygon: &mut Vec<Complex64>) {
    let eps: f64 = 1e-9;
    polygon.sort_unstable_by(|a, b| {
        if (a.re - b.re).abs() > eps {
            return a.re.partial_cmp(&b.re).unwrap();
        }
        return a.im.partial_cmp(&b.im).unwrap();
    });
}

pub fn convex_hull(mut polygon: Vec<Complex64>) -> Result<Vec<Complex64>, String> {
    if polygon.len() <= 2 {
        return Err(format!(
            "only {} Point(s), should be at least 3",
            polygon.len()
        ));
    }

    sort_polygon_ccw(&mut polygon);

    let result: Vec<Complex64> = lower_bound(&polygon);
    let result: Vec<Complex64> = upper_bound(result, polygon);

    if is_horizontal_colinear(&result) || is_vertical_colinear(&result) {
        return Result::Ok(Vec::new());
    }
    return Result::Ok(result);
}
