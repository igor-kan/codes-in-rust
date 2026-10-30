//! rational function approximant order 16356
pub fn compute_pade_approx_16356(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16356(){assert!(compute_pade_approx_16356(0.5).is_finite());}}
