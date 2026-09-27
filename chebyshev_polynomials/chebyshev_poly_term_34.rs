pub fn compute_chebyshev_poly_term_34(x: f64) -> f64 {
    x.powi(34) / 34.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_34() {
        let res = compute_chebyshev_poly_term_34(1.0);
        assert!((res - (1.0 / 34.0)).abs() < 1e-7);
    }
}
