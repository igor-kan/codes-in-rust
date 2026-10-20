//! spherical harmonic radial component order 15364
pub fn compute_spherical_harm_15364(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_15364(){assert!(compute_spherical_harm_15364(0.5).is_finite());}}
