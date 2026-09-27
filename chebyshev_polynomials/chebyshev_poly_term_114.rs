pub fn compute_chebyshev_poly_term_114(x: f64) -> f64 {
    x.powi(114) / 114.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_114() {
        let res = compute_chebyshev_poly_term_114(1.0);
        assert!((res - (1.0 / 114.0)).abs() < 1e-7);
    }
}
