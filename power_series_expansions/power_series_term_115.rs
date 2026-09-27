pub fn compute_power_series_term_115(x: f64) -> f64 {
    x.powi(115) / 115.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_115() {
        let res = compute_power_series_term_115(1.0);
        assert!((res - (1.0 / 115.0)).abs() < 1e-7);
    }
}
