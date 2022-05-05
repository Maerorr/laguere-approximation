use crate::{Function, functions::{weight, laguere_poly, function_value, horner, factorial}, integral::{newton_cotes, newton_cotes_function_poly, newton_cotes_poly_weight}};

pub fn calculate_lambdas(f: Function, deg: usize, a: f64, b: f64) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();
    for i in 0..(deg+1) {
        out.push(
           newton_cotes_function_poly(f, laguere_poly(i), a, b) / 
           newton_cotes_poly_weight(f, laguere_poly(i), a, b));
    }
    out
}

/// Returns approximated value of a function in x.
/// ### f - function to be approximated
/// ### deg - degree of the polynomial
/// ### a, b - bounds of approximation
/// ### x - point in which the function is approximated
pub fn laguere_approx(f: Function, deg: usize, a: f64, b: f64) -> Vec<f64> {
    let lambda = calculate_lambdas(f, deg, a, b);
    
    let laguerre_poly = laguere_poly(deg);

    let mut final_coefficients: Vec<f64> = Vec::new();
    for i in 0..lambda.len() {
        final_coefficients.push(lambda[i] * laguerre_poly[i]);
    }
    final_coefficients
}

// returns the value of approximated function at x.
pub fn laguere_approx_value(approx_coeffs: &Vec<f64> , x: f64) -> f64 {
    horner(&approx_coeffs, x)
}