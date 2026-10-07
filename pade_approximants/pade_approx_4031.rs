//! rational function approximant order 4031
pub fn compute_pade_approx_4031(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*2.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_4031(){assert!(compute_pade_approx_4031(0.5).is_finite());}}
