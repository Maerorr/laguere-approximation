use std::{fs::File, io::Write};

use crate::{Function, functions::{weight, laguere_poly, function_value, horner, factorial}, integral::{newton_cotes, newton_cotes_function_poly, newton_cotes_poly_weight}};

/// Returns approximated value of a function in x.
/// ### f - function to be approximated
/// ### deg - degree of the polynomial
/// ### a, b - bounds of approximation
pub fn calculate_lambdas(f: Function, deg: usize, integral_nodes: usize, a: f64, b: f64) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();
    for i in 0..(deg+1) {
        let poly = laguere_poly(i);
        out.push(
           newton_cotes(f, poly.to_vec(), a, b, integral_nodes , true) /
           newton_cotes(f, poly.to_vec(), a, b, integral_nodes , false) )
    }
    let mut file = File::create("debug_lambdas.txt").unwrap();
    for x in &out {
        file.write(format!("{:.3}, ", x).as_bytes()).unwrap();
    }
    out
}

// pub fn laguere_approx(f: Function, deg: usize, integral_nodes: usize, a: f64, b: f64) -> Vec<f64> {
//     let lambda = calculate_lambdas(f, deg, integral_nodes, a, b);
//     lambda
// }

// returns the value of approximated function at x.
pub fn laguere_approx_value(lambdas: &Vec<f64> , x: f64) -> f64 {
    let mut sum = 0.;
    for (i, lambda) in lambdas.iter().enumerate() {
        sum += lambda * horner(&laguere_poly(i), x);
    }
    sum
}