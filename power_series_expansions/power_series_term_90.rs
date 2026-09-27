pub fn compute_power_series_term_90(x: f64) -> f64 {
    x.powi(90) / 90.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_90() {
        let res = compute_power_series_term_90(1.0);
        assert!((res - (1.0 / 90.0)).abs() < 1e-7);
    }
}
