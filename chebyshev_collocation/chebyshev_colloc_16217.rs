//! chebyshev collocation node order 16217
pub fn compute_chebyshev_colloc_16217(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_16217(){assert!(compute_chebyshev_colloc_16217(0.5).is_finite());}}
