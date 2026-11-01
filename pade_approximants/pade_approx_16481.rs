//! rational function approximant order 16481
pub fn compute_pade_approx_16481(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16481(){assert!(compute_pade_approx_16481(0.5).is_finite());}}
