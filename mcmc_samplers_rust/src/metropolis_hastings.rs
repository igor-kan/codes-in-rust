//! Metropolis Hastings

pub fn accept_prob(p_old: f64, p_new: f64) -> f64 { (p_new / p_old).min(1.0) }
