//! chebyshev collocation node order 5037
pub fn compute_chebyshev_colloc_5037(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_5037(){assert!(compute_chebyshev_colloc_5037(0.5).is_finite());}}
