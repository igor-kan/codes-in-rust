//! rational function approximant order 16131
pub fn compute_pade_approx_16131(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16131(){assert!(compute_pade_approx_16131(0.5).is_finite());}}
