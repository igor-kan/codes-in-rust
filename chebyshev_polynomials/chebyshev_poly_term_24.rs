pub fn compute_chebyshev_poly_term_24(x: f64) -> f64 {
    x.powi(24) / 24.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_24() {
        let res = compute_chebyshev_poly_term_24(1.0);
        assert!((res - (1.0 / 24.0)).abs() < 1e-7);
    }
}
