pub fn compute_harmonic_series_term_6(x: f64) -> f64 {
    x.powi(6) / 6.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_6() {
        let res = compute_harmonic_series_term_6(1.0);
        assert!((res - (1.0 / 6.0)).abs() < 1e-7);
    }
}
