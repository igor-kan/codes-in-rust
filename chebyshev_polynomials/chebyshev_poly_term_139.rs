pub fn compute_chebyshev_poly_term_139(x: f64) -> f64 {
    x.powi(139) / 139.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_139() {
        let res = compute_chebyshev_poly_term_139(1.0);
        assert!((res - (1.0 / 139.0)).abs() < 1e-7);
    }
}
