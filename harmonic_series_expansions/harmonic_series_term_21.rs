pub fn compute_harmonic_series_term_21(x: f64) -> f64 {
    x.powi(21) / 21.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_21() {
        let res = compute_harmonic_series_term_21(1.0);
        assert!((res - (1.0 / 21.0)).abs() < 1e-7);
    }
}
