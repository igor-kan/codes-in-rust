pub fn compute_harmonic_series_term_131(x: f64) -> f64 {
    x.powi(131) / 131.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_131() {
        let res = compute_harmonic_series_term_131(1.0);
        assert!((res - (1.0 / 131.0)).abs() < 1e-7);
    }
}
