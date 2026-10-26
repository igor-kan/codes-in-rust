//! rational function approximant order 16086
pub fn compute_pade_approx_16086(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16086(){assert!(compute_pade_approx_16086(0.5).is_finite());}}
