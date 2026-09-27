pub fn compute_power_series_term_30(x: f64) -> f64 {
    x.powi(30) / 30.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_30() {
        let res = compute_power_series_term_30(1.0);
        assert!((res - (1.0 / 30.0)).abs() < 1e-7);
    }
}
