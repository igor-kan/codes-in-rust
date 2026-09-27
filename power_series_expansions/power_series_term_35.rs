pub fn compute_power_series_term_35(x: f64) -> f64 {
    x.powi(35) / 35.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_35() {
        let res = compute_power_series_term_35(1.0);
        assert!((res - (1.0 / 35.0)).abs() < 1e-7);
    }
}
