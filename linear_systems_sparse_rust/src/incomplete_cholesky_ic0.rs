//! Incomplete Cholesky Ic0

pub fn ic0_pivot(a_ii: f64, s: f64) -> f64 { (a_ii - s).sqrt() }
