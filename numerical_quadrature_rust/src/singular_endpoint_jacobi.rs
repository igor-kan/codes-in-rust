//! Singular Endpoint Jacobi

pub fn jacobi_alpha_beta_weight(x: f64, a: f64, b: f64) -> f64 { (1.0 - x).powf(a) * (1.0 + x).powf(b) }
