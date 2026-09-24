//! Simulated Annealing Cool

pub fn boltzmann_accept(dE: f64, temp: f64) -> f64 { (-dE / temp).exp().min(1.0) }
