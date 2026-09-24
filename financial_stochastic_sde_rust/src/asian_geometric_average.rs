//! Asian Geometric Average

pub fn geometric_mean_log(log_sum: f64, n: usize) -> f64 { (log_sum / (n as f64)).exp() }
