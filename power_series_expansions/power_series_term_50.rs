pub fn compute_power_series_term_50(x: f64) -> f64 {
    x.powi(50) / 50.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_50() {
        let res = compute_power_series_term_50(1.0);
        assert!((res - (1.0 / 50.0)).abs() < 1e-7);
    }
}
