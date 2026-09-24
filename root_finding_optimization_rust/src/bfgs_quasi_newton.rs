//! Bfgs Quasi Newton

pub fn bfgs_damping(theta: f64) -> f64 { theta.clamp(0.0, 1.0) }
