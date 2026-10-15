//! rational function approximant order 14416
pub fn compute_pade_approx_14416(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_14416(){assert!(compute_pade_approx_14416(0.5).is_finite());}}
