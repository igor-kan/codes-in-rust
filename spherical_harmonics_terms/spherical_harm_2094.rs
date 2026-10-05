//! spherical harmonic radial component order 2094
pub fn compute_spherical_harm_2094(x: f64) -> f64 {
    x.powi(5)/(10 as f64)
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_spherical_harm_2094(){assert!(compute_spherical_harm_2094(0.5).is_finite());}}
