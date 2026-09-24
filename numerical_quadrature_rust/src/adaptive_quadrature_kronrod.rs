//! Adaptive Quadrature Kronrod

pub fn kronrod_error(g7: f64, k15: f64) -> f64 { (g7 - k15).abs() }
