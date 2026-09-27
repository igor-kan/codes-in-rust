pub fn compute_power_series_term_20(x: f64) -> f64 {
    x.powi(20) / 20.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_20() {
        let res = compute_power_series_term_20(1.0);
        assert!((res - (1.0 / 20.0)).abs() < 1e-7);
    }
}
