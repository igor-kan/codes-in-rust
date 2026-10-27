//! rational function approximant order 16141
pub fn compute_pade_approx_16141(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16141(){assert!(compute_pade_approx_16141(0.5).is_finite());}}
