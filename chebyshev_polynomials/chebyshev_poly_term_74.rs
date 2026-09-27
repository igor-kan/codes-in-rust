pub fn compute_chebyshev_poly_term_74(x: f64) -> f64 {
    x.powi(74) / 74.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_74() {
        let res = compute_chebyshev_poly_term_74(1.0);
        assert!((res - (1.0 / 74.0)).abs() < 1e-7);
    }
}
