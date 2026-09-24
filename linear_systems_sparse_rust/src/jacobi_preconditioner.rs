//! Jacobi Preconditioner

pub fn inv_diagonal(diag: &[f64]) -> Vec<f64> { diag.iter().map(|&d| 1.0 / d).collect() }
