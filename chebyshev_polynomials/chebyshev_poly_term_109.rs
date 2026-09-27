pub fn compute_chebyshev_poly_term_109(x: f64) -> f64 {
    x.powi(109) / 109.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_109() {
        let res = compute_chebyshev_poly_term_109(1.0);
        assert!((res - (1.0 / 109.0)).abs() < 1e-7);
    }
}
