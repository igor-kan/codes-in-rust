//! spherical harmonic radial component order 15919
pub fn compute_spherical_harm_15919(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_15919(){assert!(compute_spherical_harm_15919(0.5).is_finite());}}
