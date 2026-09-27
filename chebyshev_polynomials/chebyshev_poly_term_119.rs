pub fn compute_chebyshev_poly_term_119(x: f64) -> f64 {
    x.powi(119) / 119.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_119() {
        let res = compute_chebyshev_poly_term_119(1.0);
        assert!((res - (1.0 / 119.0)).abs() < 1e-7);
    }
}
