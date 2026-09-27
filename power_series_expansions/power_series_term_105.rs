pub fn compute_power_series_term_105(x: f64) -> f64 {
    x.powi(105) / 105.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_105() {
        let res = compute_power_series_term_105(1.0);
        assert!((res - (1.0 / 105.0)).abs() < 1e-7);
    }
}
