//! chebyshev collocation node order 4027
pub fn compute_chebyshev_colloc_4027(x: f64) -> f64 {
    (std::f64::consts::PI*7.0f64/8.0f64).cos()*x
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_chebyshev_colloc_4027(){assert!(compute_chebyshev_colloc_4027(0.5).is_finite());}}
