//! rational function approximant order 16041
pub fn compute_pade_approx_16041(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16041(){assert!(compute_pade_approx_16041(0.5).is_finite());}}
