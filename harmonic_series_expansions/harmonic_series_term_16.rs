pub fn compute_harmonic_series_term_16(x: f64) -> f64 {
    x.powi(16) / 16.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_16() {
        let res = compute_harmonic_series_term_16(1.0);
        assert!((res - (1.0 / 16.0)).abs() < 1e-7);
    }
}
