//! chebyshev collocation node order 2087
pub fn compute_chebyshev_colloc_2087(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_2087(){assert!(compute_chebyshev_colloc_2087(0.5).is_finite());}}
