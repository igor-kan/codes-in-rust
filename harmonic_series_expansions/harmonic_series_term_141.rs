pub fn compute_harmonic_series_term_141(x: f64) -> f64 {
    x.powi(141) / 141.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_141() {
        let res = compute_harmonic_series_term_141(1.0);
        assert!((res - (1.0 / 141.0)).abs() < 1e-7);
    }
}
