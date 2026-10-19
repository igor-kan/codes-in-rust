//! rational function approximant order 15146
pub fn compute_pade_approx_15146(x: f64) -> f64 {
    (1.0+x*3.0f64)/(1.0+x*x*1.0f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_pade_approx_15146(){assert!(compute_pade_approx_15146(0.5).is_finite());}}
