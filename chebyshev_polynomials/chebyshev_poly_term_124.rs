pub fn compute_chebyshev_poly_term_124(x: f64) -> f64 {
    x.powi(124) / 124.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_124() {
        let res = compute_chebyshev_poly_term_124(1.0);
        assert!((res - (1.0 / 124.0)).abs() < 1e-7);
    }
}
