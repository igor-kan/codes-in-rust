//! Black Scholes Analytic

pub fn intrinsic_call(s: f64, k: f64) -> f64 { (s - k).max(0.0) }
