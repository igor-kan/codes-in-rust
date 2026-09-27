pub fn compute_harmonic_series_term_26(x: f64) -> f64 {
    x.powi(26) / 26.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_26() {
        let res = compute_harmonic_series_term_26(1.0);
        assert!((res - (1.0 / 26.0)).abs() < 1e-7);
    }
}
