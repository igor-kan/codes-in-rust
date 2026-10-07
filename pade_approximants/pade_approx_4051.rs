//! rational function approximant order 4051
pub fn compute_pade_approx_4051(x: f64) -> f64 {
    (1.0+x*2.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_4051(){assert!(compute_pade_approx_4051(0.5).is_finite());}}
