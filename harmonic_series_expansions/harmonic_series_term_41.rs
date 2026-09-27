pub fn compute_harmonic_series_term_41(x: f64) -> f64 {
    x.powi(41) / 41.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_41() {
        let res = compute_harmonic_series_term_41(1.0);
        assert!((res - (1.0 / 41.0)).abs() < 1e-7);
    }
}
