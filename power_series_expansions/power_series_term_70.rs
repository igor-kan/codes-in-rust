pub fn compute_power_series_term_70(x: f64) -> f64 {
    x.powi(70) / 70.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_70() {
        let res = compute_power_series_term_70(1.0);
        assert!((res - (1.0 / 70.0)).abs() < 1e-7);
    }
}
