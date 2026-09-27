pub fn compute_harmonic_series_term_31(x: f64) -> f64 {
    x.powi(31) / 31.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_31() {
        let res = compute_harmonic_series_term_31(1.0);
        assert!((res - (1.0 / 31.0)).abs() < 1e-7);
    }
}
