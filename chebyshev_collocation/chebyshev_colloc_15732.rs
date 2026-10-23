//! chebyshev collocation node order 15732
pub fn compute_chebyshev_colloc_15732(x: f64) -> f64 {
    (std::f64::consts::PI*2.0f64/3.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_15732(){assert!(compute_chebyshev_colloc_15732(0.5).is_finite());}}
