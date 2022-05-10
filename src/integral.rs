use std::mem::swap;

use crate::{Function, functions::{function_value, horner, weight}, laguere::laguere_approx_value};


// IMPLEMENTATION OF NEWTON COTES INTEGRATION FROM EXCERCISE 4.1

/// Calculates the numerator of the lambda coefficients
/// ### Integral of w(x) * f(x) * L_k(x) from a to b
pub fn newton_cotes_function_poly(f: Function, poly: Vec<f64>, a: f64, b: f64) -> f64 {
    let h = (b - a) / 2. as f64;
    let mut sum = 0.;
    sum += function_value(a, f, false) * horner(&poly, a) * weight(a);
    sum += 4. * function_value(a + h, f, false) * horner(&poly, a + h) * weight(a + h);
    sum += function_value(b, f, false) * horner(&poly, b) * weight(b);
    sum * h / 3.
}

/// Calculates the denominator of the lambda coefficients
/// ### Integral of w(x) * L_k(x) * L_k(x) from a to b
pub fn newton_cotes_poly_weight(_f: Function, poly: Vec<f64>, a: f64, b: f64) -> f64 {
    let h = (b - a) / 2. as f64;
    let mut sum = 0.;
    sum += horner(&poly, a) * horner(&poly, a) * weight(a);
    sum += 4. * horner(&poly, a+h) * horner(&poly, a+h) * weight(a + h);
    sum += horner(&poly, b) * horner(&poly, b) * weight(b);

    sum * h / 3.
}

/// Returns the value of the Newton-Cotes integration formula for the given function with given precision.
/// * f - chosen function from the Function enum
/// * (a, b) - integration left and right bound
/// * eps - precision calculated as difference between next iterations.
pub fn newton_cotes(f: Function, poly: Vec<f64>, mut a: f64, mut b: f64, nodes: usize, which: bool) -> f64 {
    if a > b {
        swap(&mut a, &mut b);
    }

    match which {
        true => {
            let h = (b - a) / (nodes as f64);
            let mut sum = 0.;
            let mut x = a;
            for i in 0..nodes {
                sum += newton_cotes_function_poly(f, poly.to_vec(), x, x + h);
                x += h;
            }
            return sum
        },
        false => {
            let h = (b - a) / (nodes as f64);
            let mut sum = 0.;
            let mut x = a;
            for i in 0..nodes {
                sum += newton_cotes_poly_weight(f, poly.to_vec(), x, x + h);
                x += h;
            }
            return sum
        },
    };  
}

pub fn error_integral(f: Function, lambdas: Vec<f64>, a: f64, b: f64) -> f64 {
    let h = (b - a) / 2.;
    let mut sum = 0.;

    sum += weight(a) * ((function_value(a, f, false) - laguere_approx_value(&lambdas, a)).powf(2.));
    sum += 4. * weight(a+h) * ((function_value(a+h, f, false) - laguere_approx_value(&lambdas, a+h)).powf(2.));
    sum += weight(b) * ((function_value(b, f, false) - laguere_approx_value(&lambdas, b)).powf(2.));

    sum * h / 3.
}