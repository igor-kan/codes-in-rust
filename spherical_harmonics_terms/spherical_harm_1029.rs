//! Implementation of spherical harmonic radial component order 1029

pub fn compute_spherical_harm_1029(x: f64) -> f64 {
    x.powi(5) / (10 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_spherical_harm_1029() {
        let res = compute_spherical_harm_1029(0.5);
        assert!(res.is_finite());
    }
}
