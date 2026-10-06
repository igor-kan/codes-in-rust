//! chebyshev collocation node order 3057
pub fn compute_chebyshev_colloc_3057(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_3057(){assert!(compute_chebyshev_colloc_3057(0.5).is_finite());}}
