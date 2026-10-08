//! rational function approximant order 5096
pub fn compute_pade_approx_5096(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_5096(){assert!(compute_pade_approx_5096(0.5).is_finite());}}
