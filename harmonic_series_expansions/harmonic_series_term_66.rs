pub fn compute_harmonic_series_term_66(x: f64) -> f64 {
    x.powi(66) / 66.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_66() {
        let res = compute_harmonic_series_term_66(1.0);
        assert!((res - (1.0 / 66.0)).abs() < 1e-7);
    }
}
