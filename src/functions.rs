use std::{
    f64::consts::{E},
    vec,
};

use crate::Function;

pub fn polynomial1(x: f64) -> f64 {
    // 0.15x^2 - x - 1
    -1. + x * (-1. + x * 0.15)
}

pub fn polynomial2(x: f64) -> f64 {
    //0.07*x^4+0.3*x^3-0.2*x^2-x-1.
    -1. + x * (-1. + x * (-0.2 + x * (-0.3 + x * 0.07)))
}

pub fn linear(x: f64) -> f64 {
    0.5 * x + 2.
}

pub fn sinusoidal(x: f64) -> f64 {
    x.cos()
}

pub fn absolute(x: f64) -> f64 {
    //((x-2.).abs()-2.).abs()
    x.abs()
}

pub fn mixed(x: f64) -> f64 {
    ((x - 2.).abs() - 2.).abs() + x.sin() + 0.05 * x.powf(3.)
}

pub fn function_value(x: f64, func: Function, with_weight: bool) -> f64 {
    match with_weight {
        true => {
            match func {
                Function::Poly1 => polynomial1(x) * weight(x),
                Function::Poly2 => polynomial2(x) * weight(x),
                Function::Linear => linear(x) * weight(x),
                Function::Sinusoidal => sinusoidal(x) * weight(x),
                Function::Absolute => absolute(x) * weight(x),
                Function::Mixed => mixed(x) * weight(x),
            }
        }
        false => {
            match func {
                Function::Poly1 => polynomial1(x),
                Function::Poly2 => polynomial2(x),
                Function::Linear => linear(x),
                Function::Sinusoidal => sinusoidal(x),
                Function::Absolute => absolute(x),
                Function::Mixed => mixed(x),
            }
        }
    }
}

/// returns e^(-x)
/// weight taken from the excerscise
pub fn weight(x: f64) -> f64 {
    (-x).exp()
}

pub fn factorial(n: usize) -> i64 {
    if n == 0 {
        1
    } else {
        n as i64 * factorial(n - 1)
    }
}

pub fn binomial_coeff(top: usize, bot: usize) -> i64 {
    if top > bot {
        factorial(top) / (factorial(top - bot) * factorial(bot))
    }
    else if top == bot {
        1
    } else {
        0
    }
}

pub fn pow(x: i64, n: usize) -> i64 {
    if n == 0 {
        return 1
    }
    if n == 1 {
        return x
    }
    let mut out = x;
    for i in 1..n {
        out *= x;
    }
    out
}

/// Returns a values of Laguerre polynomial coefficients of a given degree.
pub fn laguere_poly(n: usize) -> Vec<f64> {
    let mut val: Vec<f64> = Vec::new();
    for k in 0..(n+1) {
        // WIKIPEDIA LAGUERRE POLYNOMIAL
        val.push( binomial_coeff(n, k) as f64 * pow(-1, k) as f64 / factorial(k) as f64 );

        // LECTURE LAGUERRE POLYNOMIAL (terribly wrong)
        // val.push( binomial_coeff(n, k) as f64 * pow(-1, k) as f64 * factorial(k) as f64);
    } 
    val
}

/// Returns the value of a function in point x.
/// Uses Horner's method.
/// * a - vector of coefficients of a function for example 3x^2 + 2x + 1 = {1, 2, 3}
pub fn horner(a: &Vec<f64>, x: f64) -> f64 {
    let mut i = a.len() - 1;
    let mut out = a[i];
    for i in (0..a.len() - 1).rev() {
        out = out * x + a[i];
    }
    out
}