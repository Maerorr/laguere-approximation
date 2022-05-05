use std::mem::swap;

use crate::{Function, functions::{function_value, horner, weight}};


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
    sum += horner(&poly, a).powf(2.) * weight(a);
    sum += 4. * horner(&poly, a + h).powf(2.) * weight(a + h);
    sum += horner(&poly, b) * horner(&poly, b) * weight(b);

    sum * h / 3.
}

/// Returns the value of the Newton-Cotes integration formula for the given function with given precision.
/// * f - chosen function from the Function enum
/// * (a, b) - integration left and right bound
/// * eps - precision calculated as difference between next iterations.
pub fn newton_cotes(f: Function, poly: Vec<f64>, mut a: f64, mut b: f64, eps: f64, which: bool) -> f64 {
    if a > b {
        swap(&mut a, &mut b);
    }

    match which {
        true => {
            // first iteration
            let mut value1 = newton_cotes_function_poly(f, poly.to_vec(), a, b);

            // second iteration
            let mut step = (b - a) / 2.;
            let mut value2 = newton_cotes_function_poly(f, poly.to_vec(), a, a + step);
            value2 += newton_cotes_function_poly(f, poly.to_vec(), a + step, b);

            // check for precision
            if (value2 - value1).abs() < eps {
                return value2;
            } else {
                // all next iterations
                let mut iteration = 3;
                // loop based on precision
                while (value2 - value1).abs() > eps {
                    value1 = value2;
                    value2 = 0.;
                    step = (b - a) / iteration as f64;
                    for i in 0..iteration {
                        value2 +=
                        newton_cotes_function_poly(f, poly.to_vec(), a + (i as f64) * step, a + (i as f64 + 1.) * step);
                    }
                    iteration += 1;
                }

                return value2
            }
        },
        false => {
            // first iteration
            let mut value1 = newton_cotes_poly_weight(f, poly.to_vec(), a, b);

            // second iteration
            let mut step = (b - a) / 2.;
            let mut value2 = newton_cotes_poly_weight(f, poly.to_vec(), a, a + step);
            value2 += newton_cotes_poly_weight(f, poly.to_vec(), a + step, b);

            // check for precision
            if (value2 - value1).abs() < eps {
                return value2;
            } else {
                // all next iterations
                let mut iteration = 3;
                // loop based on precision
                while (value2 - value1).abs() > eps {
                    value1 = value2;
                    value2 = 0.;
                    step = (b - a) / iteration as f64;
                    for i in 0..iteration {
                        value2 +=
                        newton_cotes_poly_weight(f, poly.to_vec(), a + (i as f64) * step, a + (i as f64 + 1.) * step);
                    }
                    iteration += 1;
                }

                return value2
            }
        },
    };  
}