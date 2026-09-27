pub fn compute_harmonic_series_term_36(x: f64) -> f64 {
    x.powi(36) / 36.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_36() {
        let res = compute_harmonic_series_term_36(1.0);
        assert!((res - (1.0 / 36.0)).abs() < 1e-7);
    }
}
