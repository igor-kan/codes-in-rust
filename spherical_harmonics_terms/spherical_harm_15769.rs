//! spherical harmonic radial component order 15769
pub fn compute_spherical_harm_15769(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_15769(){assert!(compute_spherical_harm_15769(0.5).is_finite());}}
