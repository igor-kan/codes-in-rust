pub fn compute_harmonic_series_term_86(x: f64) -> f64 {
    x.powi(86) / 86.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_86() {
        let res = compute_harmonic_series_term_86(1.0);
        assert!((res - (1.0 / 86.0)).abs() < 1e-7);
    }
}
