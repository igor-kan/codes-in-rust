pub fn compute_power_series_term_100(x: f64) -> f64 {
    x.powi(100) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_100() {
        let res = compute_power_series_term_100(1.0);
        assert!((res - (1.0 / 100.0)).abs() < 1e-7);
    }
}
