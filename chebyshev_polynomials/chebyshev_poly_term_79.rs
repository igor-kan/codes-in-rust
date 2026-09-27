pub fn compute_chebyshev_poly_term_79(x: f64) -> f64 {
    x.powi(79) / 79.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_79() {
        let res = compute_chebyshev_poly_term_79(1.0);
        assert!((res - (1.0 / 79.0)).abs() < 1e-7);
    }
}
