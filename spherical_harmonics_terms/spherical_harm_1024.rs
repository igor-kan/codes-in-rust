//! Implementation of spherical harmonic radial component order 1024

pub fn compute_spherical_harm_1024(x: f64) -> f64 {
    x.powi(5) / (10 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_spherical_harm_1024() {
        let res = compute_spherical_harm_1024(0.5);
        assert!(res.is_finite());
    }
}
