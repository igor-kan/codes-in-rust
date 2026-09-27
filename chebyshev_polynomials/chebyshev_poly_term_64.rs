pub fn compute_chebyshev_poly_term_64(x: f64) -> f64 {
    x.powi(64) / 64.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_64() {
        let res = compute_chebyshev_poly_term_64(1.0);
        assert!((res - (1.0 / 64.0)).abs() < 1e-7);
    }
}
