//! Newton Raphson 1D

pub fn newton_step(x: f64, f: f64, df: f64) -> f64 { x - f / df }
