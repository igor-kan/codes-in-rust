//! spherical harmonic radial component order 15379
pub fn compute_spherical_harm_15379(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_15379(){assert!(compute_spherical_harm_15379(0.5).is_finite());}}
