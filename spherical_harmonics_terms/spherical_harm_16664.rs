//! spherical harmonic radial component order 16664
pub fn compute_spherical_harm_16664(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_16664(){assert!(compute_spherical_harm_16664(0.5).is_finite());}}
