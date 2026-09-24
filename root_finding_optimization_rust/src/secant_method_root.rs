//! Secant Method Root

pub fn secant_step(x1: f64, x0: f64, f1: f64, f0: f64) -> f64 { x1 - f1 * (x1 - x0) / (f1 - f0) }
