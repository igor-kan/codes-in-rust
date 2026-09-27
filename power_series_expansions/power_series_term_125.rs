pub fn compute_power_series_term_125(x: f64) -> f64 {
    x.powi(125) / 125.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_125() {
        let res = compute_power_series_term_125(1.0);
        assert!((res - (1.0 / 125.0)).abs() < 1e-7);
    }
}
