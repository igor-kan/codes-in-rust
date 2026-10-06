//! chebyshev collocation node order 3017
pub fn compute_chebyshev_colloc_3017(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_3017(){assert!(compute_chebyshev_colloc_3017(0.5).is_finite());}}
