//! rational function approximant order 16316
pub fn compute_pade_approx_16316(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16316(){assert!(compute_pade_approx_16316(0.5).is_finite());}}
