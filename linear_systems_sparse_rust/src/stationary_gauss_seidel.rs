//! Stationary Gauss Seidel

pub fn gs_forward_step(b: f64, s: f64, a_ii: f64) -> f64 { (b - s) / a_ii }
