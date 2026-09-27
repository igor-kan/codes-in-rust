pub fn compute_harmonic_series_term_1(x: f64) -> f64 {
    x.powi(1) / 1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_1() {
        let res = compute_harmonic_series_term_1(1.0);
        assert!((res - (1.0 / 1.0)).abs() < 1e-7);
    }
}
