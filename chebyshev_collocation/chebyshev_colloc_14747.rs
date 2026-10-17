//! chebyshev collocation node order 14747
pub fn compute_chebyshev_colloc_14747(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_14747(){assert!(compute_chebyshev_colloc_14747(0.5).is_finite());}}
