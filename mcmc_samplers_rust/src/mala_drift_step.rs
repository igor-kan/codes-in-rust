//! Mala Drift Step

pub fn mala_mean(x: f64, grad_log_p: f64, tau: f64) -> f64 { x + tau * grad_log_p }
