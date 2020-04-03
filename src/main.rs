use std::io::{self, Write};
use std::string::String;
use std::vec::Vec;

use num_complex::Complex64;

use rust_convex_hull::convex_hull;

fn get_line() -> String {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");
    return buffer;
}

fn main() {
    let size = get_line().trim().parse().unwrap();

    let mut polygon: Vec<Complex64> = Vec::new();
    for _ in 0..size {
        let line: String = get_line().trim().to_string();
        let numbers: Vec<f64> = line
            .split_ascii_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();
        let point: Complex64 = Complex64::new(numbers[0], numbers[1]);
        polygon.push(point);
    }
    let result = convex_hull(polygon);
    match result {
        Ok(v) => {
            println!("{}", v.len());
            for i in 0..v.len() {
                println!("{} {}", v[i].re, v[i].im);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    };

    io::stdout().flush().unwrap();
}
