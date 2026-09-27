pub fn compute_chebyshev_poly_term_39(x: f64) -> f64 {
    x.powi(39) / 39.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_39() {
        let res = compute_chebyshev_poly_term_39(1.0);
        assert!((res - (1.0 / 39.0)).abs() < 1e-7);
    }
}
