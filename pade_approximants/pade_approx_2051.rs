//! rational function approximant order 2051
pub fn compute_pade_approx_2051(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_2051(){assert!(compute_pade_approx_2051(0.5).is_finite());}}
