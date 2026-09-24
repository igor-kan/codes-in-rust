//! Hamiltonian Leapfrog

pub fn leapfrog_p(p: f64, grad_u: f64, eps: f64) -> f64 { p - 0.5 * eps * grad_u }
