//! rational function approximant order 16816
pub fn compute_pade_approx_16816(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16816(){assert!(compute_pade_approx_16816(0.5).is_finite());}}
