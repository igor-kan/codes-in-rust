pub fn compute_power_series_term_15(x: f64) -> f64 {
    x.powi(15) / 15.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_15() {
        let res = compute_power_series_term_15(1.0);
        assert!((res - (1.0 / 15.0)).abs() < 1e-7);
    }
}
