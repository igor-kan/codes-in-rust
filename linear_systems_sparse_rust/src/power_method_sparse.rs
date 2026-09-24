//! Power Method Sparse

pub fn normalize_vector(v: &mut [f64]) { let norm: f64 = v.iter().map(|x| x*x).sum::<f64>().sqrt(); if norm > 0.0 { for x in v { *x /= norm; } } }
