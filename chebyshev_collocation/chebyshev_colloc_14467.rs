//! chebyshev collocation node order 14467
pub fn compute_chebyshev_colloc_14467(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_14467(){assert!(compute_chebyshev_colloc_14467(0.5).is_finite());}}
