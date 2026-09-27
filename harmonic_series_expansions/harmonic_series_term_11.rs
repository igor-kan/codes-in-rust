pub fn compute_harmonic_series_term_11(x: f64) -> f64 {
    x.powi(11) / 11.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_11() {
        let res = compute_harmonic_series_term_11(1.0);
        assert!((res - (1.0 / 11.0)).abs() < 1e-7);
    }
}
