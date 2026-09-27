pub fn compute_chebyshev_poly_term_69(x: f64) -> f64 {
    x.powi(69) / 69.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_69() {
        let res = compute_chebyshev_poly_term_69(1.0);
        assert!((res - (1.0 / 69.0)).abs() < 1e-7);
    }
}
