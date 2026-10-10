//! rational function approximant order 9061
pub fn compute_pade_approx_9061(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_9061(){assert!(compute_pade_approx_9061(0.5).is_finite());}}
