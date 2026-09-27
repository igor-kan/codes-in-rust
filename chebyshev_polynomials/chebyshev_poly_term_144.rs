pub fn compute_chebyshev_poly_term_144(x: f64) -> f64 {
    x.powi(144) / 144.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_144() {
        let res = compute_chebyshev_poly_term_144(1.0);
        assert!((res - (1.0 / 144.0)).abs() < 1e-7);
    }
}
