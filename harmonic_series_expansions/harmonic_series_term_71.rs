pub fn compute_harmonic_series_term_71(x: f64) -> f64 {
    x.powi(71) / 71.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_71() {
        let res = compute_harmonic_series_term_71(1.0);
        assert!((res - (1.0 / 71.0)).abs() < 1e-7);
    }
}
