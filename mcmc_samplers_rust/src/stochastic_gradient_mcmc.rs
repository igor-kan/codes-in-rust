//! Stochastic Gradient Mcmc

pub fn sgld_step(theta: f64, eps: f64, g: f64) -> f64 { theta - 0.5 * eps * g }
