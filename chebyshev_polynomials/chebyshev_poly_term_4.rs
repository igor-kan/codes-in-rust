pub fn compute_chebyshev_poly_term_4(x: f64) -> f64 {
    x.powi(4) / 4.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_4() {
        let res = compute_chebyshev_poly_term_4(1.0);
        assert!((res - (1.0 / 4.0)).abs() < 1e-7);
    }
}
