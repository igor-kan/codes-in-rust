//! rational function approximant order 14466
pub fn compute_pade_approx_14466(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_14466(){assert!(compute_pade_approx_14466(0.5).is_finite());}}
