pub fn compute_harmonic_series_term_81(x: f64) -> f64 {
    x.powi(81) / 81.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_81() {
        let res = compute_harmonic_series_term_81(1.0);
        assert!((res - (1.0 / 81.0)).abs() < 1e-7);
    }
}
