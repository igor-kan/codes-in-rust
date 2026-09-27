pub fn compute_chebyshev_poly_term_89(x: f64) -> f64 {
    x.powi(89) / 89.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_89() {
        let res = compute_chebyshev_poly_term_89(1.0);
        assert!((res - (1.0 / 89.0)).abs() < 1e-7);
    }
}
