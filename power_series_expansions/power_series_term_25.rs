pub fn compute_power_series_term_25(x: f64) -> f64 {
    x.powi(25) / 25.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_25() {
        let res = compute_power_series_term_25(1.0);
        assert!((res - (1.0 / 25.0)).abs() < 1e-7);
    }
}
