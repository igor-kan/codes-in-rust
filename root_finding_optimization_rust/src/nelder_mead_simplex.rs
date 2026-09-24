//! Nelder Mead Simplex

pub fn reflection_point(centroid: f64, worst: f64, alpha: f64) -> f64 { centroid + alpha * (centroid - worst) }
