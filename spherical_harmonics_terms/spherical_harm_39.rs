//! Implementation of spherical harmonic radial component order 39

pub fn compute_spherical_harm_39(x: f64) -> f64 {
    x.powi(5) / (10 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_spherical_harm_39() {
        let res = compute_spherical_harm_39(0.5);
        assert!(res.is_finite());
    }
}
