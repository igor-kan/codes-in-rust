pub fn compute_harmonic_series_term_121(x: f64) -> f64 {
    x.powi(121) / 121.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_121() {
        let res = compute_harmonic_series_term_121(1.0);
        assert!((res - (1.0 / 121.0)).abs() < 1e-7);
    }
}
