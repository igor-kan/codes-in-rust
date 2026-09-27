pub fn compute_power_series_term_60(x: f64) -> f64 {
    x.powi(60) / 60.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_60() {
        let res = compute_power_series_term_60(1.0);
        assert!((res - (1.0 / 60.0)).abs() < 1e-7);
    }
}
