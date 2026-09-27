pub fn compute_chebyshev_poly_term_134(x: f64) -> f64 {
    x.powi(134) / 134.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_134() {
        let res = compute_chebyshev_poly_term_134(1.0);
        assert!((res - (1.0 / 134.0)).abs() < 1e-7);
    }
}
