//! Chebyshev Semi Iterative

pub fn cheb_iteration_omega(gamma: f64, omega_prev: f64) -> f64 { 1.0 / (1.0 - 0.25 * gamma * gamma * omega_prev) }
