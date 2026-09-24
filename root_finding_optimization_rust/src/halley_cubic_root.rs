//! Halley Cubic Root

pub fn halley_step(x: f64, f: f64, df: f64, d2f: f64) -> f64 { x - (2.0 * f * df) / (2.0 * df * df - f * d2f) }
