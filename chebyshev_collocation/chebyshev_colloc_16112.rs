//! chebyshev collocation node order 16112
pub fn compute_chebyshev_colloc_16112(x: f64) -> f64 {
    (std::f64::consts::PI*2.0f64/3.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_16112(){assert!(compute_chebyshev_colloc_16112(0.5).is_finite());}}
