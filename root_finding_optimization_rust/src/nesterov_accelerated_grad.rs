//! Nesterov Accelerated Grad

pub fn nesterov_lookahead(theta: f64, v: f64, beta: f64) -> f64 { theta + beta * v }
