//! Ornstein Uhlenbeck Sde

pub fn ou_drift(theta: f64, mu: f64, x: f64) -> f64 { theta * (mu - x) }
