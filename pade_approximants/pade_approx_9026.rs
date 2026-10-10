//! rational function approximant order 9026
pub fn compute_pade_approx_9026(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_9026(){assert!(compute_pade_approx_9026(0.5).is_finite());}}
