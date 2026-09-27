pub fn compute_chebyshev_poly_term_99(x: f64) -> f64 {
    x.powi(99) / 99.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_99() {
        let res = compute_chebyshev_poly_term_99(1.0);
        assert!((res - (1.0 / 99.0)).abs() < 1e-7);
    }
}
