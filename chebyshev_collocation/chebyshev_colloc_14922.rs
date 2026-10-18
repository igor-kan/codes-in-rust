//! chebyshev collocation node order 14922
pub fn compute_chebyshev_colloc_14922(x: f64) -> f64 {
    (std::f64::consts::PI*2.0f64/3.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_14922(){assert!(compute_chebyshev_colloc_14922(0.5).is_finite());}}
