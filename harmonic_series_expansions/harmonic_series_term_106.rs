pub fn compute_harmonic_series_term_106(x: f64) -> f64 {
    x.powi(106) / 106.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_106() {
        let res = compute_harmonic_series_term_106(1.0);
        assert!((res - (1.0 / 106.0)).abs() < 1e-7);
    }
}
