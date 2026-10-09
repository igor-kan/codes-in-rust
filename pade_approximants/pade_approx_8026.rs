//! rational function approximant order 8026
pub fn compute_pade_approx_8026(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_8026(){assert!(compute_pade_approx_8026(0.5).is_finite());}}
