//! spherical harmonic radial component order 16144
pub fn compute_spherical_harm_16144(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_16144(){assert!(compute_spherical_harm_16144(0.5).is_finite());}}
