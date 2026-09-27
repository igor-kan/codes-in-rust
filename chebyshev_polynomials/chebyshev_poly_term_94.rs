pub fn compute_chebyshev_poly_term_94(x: f64) -> f64 {
    x.powi(94) / 94.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_94() {
        let res = compute_chebyshev_poly_term_94(1.0);
        assert!((res - (1.0 / 94.0)).abs() < 1e-7);
    }
}
