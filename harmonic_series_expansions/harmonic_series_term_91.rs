pub fn compute_harmonic_series_term_91(x: f64) -> f64 {
    x.powi(91) / 91.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_91() {
        let res = compute_harmonic_series_term_91(1.0);
        assert!((res - (1.0 / 91.0)).abs() < 1e-7);
    }
}
