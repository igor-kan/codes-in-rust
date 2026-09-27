pub fn compute_harmonic_series_term_61(x: f64) -> f64 {
    x.powi(61) / 61.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_61() {
        let res = compute_harmonic_series_term_61(1.0);
        assert!((res - (1.0 / 61.0)).abs() < 1e-7);
    }
}
