pub fn compute_harmonic_series_term_126(x: f64) -> f64 {
    x.powi(126) / 126.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_126() {
        let res = compute_harmonic_series_term_126(1.0);
        assert!((res - (1.0 / 126.0)).abs() < 1e-7);
    }
}
