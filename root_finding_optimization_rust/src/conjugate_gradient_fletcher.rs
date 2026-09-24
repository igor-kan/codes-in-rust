//! Conjugate Gradient Fletcher

pub fn fletcher_reeves_beta(g_new_sq: f64, g_old_sq: f64) -> f64 { g_new_sq / g_old_sq }
