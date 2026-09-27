pub fn compute_power_series_term_40(x: f64) -> f64 {
    x.powi(40) / 40.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_40() {
        let res = compute_power_series_term_40(1.0);
        assert!((res - (1.0 / 40.0)).abs() < 1e-7);
    }
}
