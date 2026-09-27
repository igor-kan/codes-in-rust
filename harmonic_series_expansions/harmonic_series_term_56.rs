pub fn compute_harmonic_series_term_56(x: f64) -> f64 {
    x.powi(56) / 56.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_56() {
        let res = compute_harmonic_series_term_56(1.0);
        assert!((res - (1.0 / 56.0)).abs() < 1e-7);
    }
}
