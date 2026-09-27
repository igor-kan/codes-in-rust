pub fn compute_power_series_term_55(x: f64) -> f64 {
    x.powi(55) / 55.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_55() {
        let res = compute_power_series_term_55(1.0);
        assert!((res - (1.0 / 55.0)).abs() < 1e-7);
    }
}
