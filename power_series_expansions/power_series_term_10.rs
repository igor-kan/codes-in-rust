pub fn compute_power_series_term_10(x: f64) -> f64 {
    x.powi(10) / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_10() {
        let res = compute_power_series_term_10(1.0);
        assert!((res - (1.0 / 10.0)).abs() < 1e-7);
    }
}
