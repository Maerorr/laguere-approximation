use crate::{Function, functions::{laguere_poly, horner, factorial}, integral::{newton_cotes, error_integral}};

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
    out
}

// returns the value of approximated function at x.
pub fn laguere_approx_value(lambdas: &Vec<f64> , x: f64) -> f64 {
    let mut sum = 0.;
    for (i, lambda) in lambdas.iter().enumerate() {
        sum += lambda * horner(&laguere_poly(i), x);
    }
    sum
}

pub fn approx_error(f: Function, lambdas: &Vec<f64>, nodes: usize, a: f64, b: f64) -> f64 {
    let h = (b - a) / (nodes as f64);
    let mut sum = 0.;
    let mut x = a;
    for i in 0..nodes {
        sum += error_integral(f, lambdas.to_vec(), x, x + h);
        x += h;
    }
    return sum.sqrt()
}