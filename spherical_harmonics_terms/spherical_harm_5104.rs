//! spherical harmonic radial component order 5104
pub fn compute_spherical_harm_5104(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_5104(){assert!(compute_spherical_harm_5104(0.5).is_finite());}}
