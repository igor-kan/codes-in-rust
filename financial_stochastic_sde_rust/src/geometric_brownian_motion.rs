//! Geometric Brownian Motion

pub fn gbm_path(s: f64, drift: f64, vol: f64) -> f64 { s * (drift + vol).exp() }
