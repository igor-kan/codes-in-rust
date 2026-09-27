pub fn compute_chebyshev_poly_term_44(x: f64) -> f64 {
    x.powi(44) / 44.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_44() {
        let res = compute_chebyshev_poly_term_44(1.0);
        assert!((res - (1.0 / 44.0)).abs() < 1e-7);
    }
}
