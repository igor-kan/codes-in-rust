pub fn compute_chebyshev_poly_term_129(x: f64) -> f64 {
    x.powi(129) / 129.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_129() {
        let res = compute_chebyshev_poly_term_129(1.0);
        assert!((res - (1.0 / 129.0)).abs() < 1e-7);
    }
}
