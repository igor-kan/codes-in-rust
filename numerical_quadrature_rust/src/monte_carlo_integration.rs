//! Monte Carlo Integration

pub fn mc_estimate(sum_f: f64, n: usize, vol: f64) -> f64 { vol * sum_f / (n as f64) }
