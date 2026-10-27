//! rational function approximant order 16136
pub fn compute_pade_approx_16136(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16136(){assert!(compute_pade_approx_16136(0.5).is_finite());}}
