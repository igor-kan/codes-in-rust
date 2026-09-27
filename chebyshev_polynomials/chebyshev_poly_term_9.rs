pub fn compute_chebyshev_poly_term_9(x: f64) -> f64 {
    x.powi(9) / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_9() {
        let res = compute_chebyshev_poly_term_9(1.0);
        assert!((res - (1.0 / 9.0)).abs() < 1e-7);
    }
}
