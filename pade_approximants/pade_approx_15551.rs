//! rational function approximant order 15551
pub fn compute_pade_approx_15551(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_15551(){assert!(compute_pade_approx_15551(0.5).is_finite());}}
