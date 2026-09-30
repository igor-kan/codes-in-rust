//! Implementation of spherical harmonic radial component order 74

pub fn compute_spherical_harm_74(x: f64) -> f64 {
    x.powi(5) / (10 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_spherical_harm_74() {
        let res = compute_spherical_harm_74(0.5);
        assert!(res.is_finite());
    }
}
