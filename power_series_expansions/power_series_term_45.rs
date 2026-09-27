pub fn compute_power_series_term_45(x: f64) -> f64 {
    x.powi(45) / 45.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_45() {
        let res = compute_power_series_term_45(1.0);
        assert!((res - (1.0 / 45.0)).abs() < 1e-7);
    }
}
