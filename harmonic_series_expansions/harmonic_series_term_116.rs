pub fn compute_harmonic_series_term_116(x: f64) -> f64 {
    x.powi(116) / 116.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_116() {
        let res = compute_harmonic_series_term_116(1.0);
        assert!((res - (1.0 / 116.0)).abs() < 1e-7);
    }
}
