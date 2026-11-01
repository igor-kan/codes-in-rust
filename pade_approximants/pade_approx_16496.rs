//! rational function approximant order 16496
pub fn compute_pade_approx_16496(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16496(){assert!(compute_pade_approx_16496(0.5).is_finite());}}
