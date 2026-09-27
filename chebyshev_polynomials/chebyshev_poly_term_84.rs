pub fn compute_chebyshev_poly_term_84(x: f64) -> f64 {
    x.powi(84) / 84.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_84() {
        let res = compute_chebyshev_poly_term_84(1.0);
        assert!((res - (1.0 / 84.0)).abs() < 1e-7);
    }
}
