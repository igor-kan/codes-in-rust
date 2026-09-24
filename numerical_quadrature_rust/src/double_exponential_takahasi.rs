//! Double Exponential Takahasi

pub fn tanh_sinh_weight(t: f64) -> f64 { 0.5 * std::f64::consts::PI * t.cosh() / (0.5 * std::f64::consts::PI * t.sinh()).cosh().powi(2) }
