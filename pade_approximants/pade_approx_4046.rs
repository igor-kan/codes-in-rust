//! rational function approximant order 4046
pub fn compute_pade_approx_4046(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_4046(){assert!(compute_pade_approx_4046(0.5).is_finite());}}
