pub fn compute_chebyshev_poly_term_29(x: f64) -> f64 {
    x.powi(29) / 29.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_29() {
        let res = compute_chebyshev_poly_term_29(1.0);
        assert!((res - (1.0 / 29.0)).abs() < 1e-7);
    }
}
