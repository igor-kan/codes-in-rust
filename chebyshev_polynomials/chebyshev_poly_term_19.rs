pub fn compute_chebyshev_poly_term_19(x: f64) -> f64 {
    x.powi(19) / 19.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_19() {
        let res = compute_chebyshev_poly_term_19(1.0);
        assert!((res - (1.0 / 19.0)).abs() < 1e-7);
    }
}
