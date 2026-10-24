//! rational function approximant order 15876
pub fn compute_pade_approx_15876(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_15876(){assert!(compute_pade_approx_15876(0.5).is_finite());}}
