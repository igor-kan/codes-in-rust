pub fn compute_chebyshev_poly_term_14(x: f64) -> f64 {
    x.powi(14) / 14.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_14() {
        let res = compute_chebyshev_poly_term_14(1.0);
        assert!((res - (1.0 / 14.0)).abs() < 1e-7);
    }
}
