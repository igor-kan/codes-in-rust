//! Cox Ingersoll Ross

pub fn cir_variance_diffusion(sigma: f64, r: f64) -> f64 { sigma * r.max(0.0).sqrt() }
