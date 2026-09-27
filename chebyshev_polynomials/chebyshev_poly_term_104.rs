pub fn compute_chebyshev_poly_term_104(x: f64) -> f64 {
    x.powi(104) / 104.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_104() {
        let res = compute_chebyshev_poly_term_104(1.0);
        assert!((res - (1.0 / 104.0)).abs() < 1e-7);
    }
}
