pub fn compute_harmonic_series_term_101(x: f64) -> f64 {
    x.powi(101) / 101.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_101() {
        let res = compute_harmonic_series_term_101(1.0);
        assert!((res - (1.0 / 101.0)).abs() < 1e-7);
    }
}
