pub fn compute_chebyshev_poly_term_54(x: f64) -> f64 {
    x.powi(54) / 54.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_54() {
        let res = compute_chebyshev_poly_term_54(1.0);
        assert!((res - (1.0 / 54.0)).abs() < 1e-7);
    }
}
