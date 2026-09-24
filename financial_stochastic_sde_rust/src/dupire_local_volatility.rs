//! Dupire Local Volatility

pub fn dupire_denominator(s: f64, d2c_ds2: f64) -> f64 { 0.5 * s * s * d2c_ds2 }
