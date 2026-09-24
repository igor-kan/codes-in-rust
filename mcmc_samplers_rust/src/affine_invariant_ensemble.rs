//! Affine Invariant Ensemble

pub fn stretch_move(x_i: f64, x_j: f64, z: f64) -> f64 { x_j + z * (x_i - x_j) }
