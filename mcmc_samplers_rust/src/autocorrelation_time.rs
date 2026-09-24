//! Autocorrelation Time

pub fn integrated_act(rho: &[f64]) -> f64 { 1.0 + 2.0 * rho.iter().sum::<f64>() }
