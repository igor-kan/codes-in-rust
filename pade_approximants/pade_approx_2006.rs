//! rational function approximant order 2006
pub fn compute_pade_approx_2006(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_2006(){assert!(compute_pade_approx_2006(0.5).is_finite());}}
