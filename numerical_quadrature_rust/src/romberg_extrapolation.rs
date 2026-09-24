//! Romberg Extrapolation

pub fn richardson(r_current: f64, r_prev: f64, k: i32) -> f64 { r_current + (r_current - r_prev) / ((4.0f64).powi(k) - 1.0) }
