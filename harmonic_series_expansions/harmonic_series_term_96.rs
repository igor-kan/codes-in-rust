pub fn compute_harmonic_series_term_96(x: f64) -> f64 {
    x.powi(96) / 96.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_96() {
        let res = compute_harmonic_series_term_96(1.0);
        assert!((res - (1.0 / 96.0)).abs() < 1e-7);
    }
}
