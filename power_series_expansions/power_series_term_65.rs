pub fn compute_power_series_term_65(x: f64) -> f64 {
    x.powi(65) / 65.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_65() {
        let res = compute_power_series_term_65(1.0);
        assert!((res - (1.0 / 65.0)).abs() < 1e-7);
    }
}
