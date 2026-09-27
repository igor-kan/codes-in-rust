pub fn compute_chebyshev_poly_term_49(x: f64) -> f64 {
    x.powi(49) / 49.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_49() {
        let res = compute_chebyshev_poly_term_49(1.0);
        assert!((res - (1.0 / 49.0)).abs() < 1e-7);
    }
}
