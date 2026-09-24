//! Triangular Solve Sparse

pub fn back_subst(b: f64, s: f64, diag: f64) -> f64 { (b - s) / diag }
