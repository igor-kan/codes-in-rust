//! rational function approximant order 16436
pub fn compute_pade_approx_16436(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16436(){assert!(compute_pade_approx_16436(0.5).is_finite());}}
