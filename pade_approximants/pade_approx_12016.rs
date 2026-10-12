//! rational function approximant order 12016
pub fn compute_pade_approx_12016(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_12016(){assert!(compute_pade_approx_12016(0.5).is_finite());}}
