//! rational function approximant order 16656
pub fn compute_pade_approx_16656(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_16656(){assert!(compute_pade_approx_16656(0.5).is_finite());}}
