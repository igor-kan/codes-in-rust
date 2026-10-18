//! chebyshev collocation node order 14932
pub fn compute_chebyshev_colloc_14932(x: f64) -> f64 {
    (std::f64::consts::PI*2.0f64/3.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_14932(){assert!(compute_chebyshev_colloc_14932(0.5).is_finite());}}
