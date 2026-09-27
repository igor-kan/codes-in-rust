pub fn compute_power_series_term_95(x: f64) -> f64 {
    x.powi(95) / 95.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_95() {
        let res = compute_power_series_term_95(1.0);
        assert!((res - (1.0 / 95.0)).abs() < 1e-7);
    }
}
