//! Hull White Short Rate

pub fn hw_variance(sigma: f64, a: f64, t: f64) -> f64 { (sigma * sigma / (2.0 * a)) * (1.0 - (-2.0 * a * t).exp()) }
