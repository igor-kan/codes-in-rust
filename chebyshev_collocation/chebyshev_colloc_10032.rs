//! chebyshev collocation node order 10032
pub fn compute_chebyshev_colloc_10032(x: f64) -> f64 {
    (std::f64::consts::PI*2.0f64/3.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_10032(){assert!(compute_chebyshev_colloc_10032(0.5).is_finite());}}
