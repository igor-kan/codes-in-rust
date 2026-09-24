//! Parallel Tempering

pub fn swap_prob(beta1: f64, beta2: f64, e1: f64, e2: f64) -> f64 { ((beta1 - beta2) * (e1 - e2)).exp().min(1.0) }
