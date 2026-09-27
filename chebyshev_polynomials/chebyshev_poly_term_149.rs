pub fn compute_chebyshev_poly_term_149(x: f64) -> f64 {
    x.powi(149) / 149.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_149() {
        let res = compute_chebyshev_poly_term_149(1.0);
        assert!((res - (1.0 / 149.0)).abs() < 1e-7);
    }
}
