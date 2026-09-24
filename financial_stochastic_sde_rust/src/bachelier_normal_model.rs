//! Bachelier Normal Model

pub fn bachelier_call_intrinsic(s: f64, k: f64) -> f64 { (s - k).max(0.0) }
