//! rational function approximant order 2121
pub fn compute_pade_approx_2121(x: f64) -> f64 {
    (1.0+x*1.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_2121(){assert!(compute_pade_approx_2121(0.5).is_finite());}}
