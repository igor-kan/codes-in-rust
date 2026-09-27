pub fn compute_chebyshev_poly_term_59(x: f64) -> f64 {
    x.powi(59) / 59.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_59() {
        let res = compute_chebyshev_poly_term_59(1.0);
        assert!((res - (1.0 / 59.0)).abs() < 1e-7);
    }
}
