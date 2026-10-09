//! rational function approximant order 8101
pub fn compute_pade_approx_8101(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_8101(){assert!(compute_pade_approx_8101(0.5).is_finite());}}
