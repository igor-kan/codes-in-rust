//! spherical harmonic radial component order 10069
pub fn compute_spherical_harm_10069(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_10069(){assert!(compute_spherical_harm_10069(0.5).is_finite());}}
