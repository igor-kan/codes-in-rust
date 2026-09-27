pub fn compute_power_series_term_5(x: f64) -> f64 {
    x.powi(5) / 5.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_5() {
        let res = compute_power_series_term_5(1.0);
        assert!((res - (1.0 / 5.0)).abs() < 1e-7);
    }
}
