pub fn compute_power_series_term_80(x: f64) -> f64 {
    x.powi(80) / 80.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_80() {
        let res = compute_power_series_term_80(1.0);
        assert!((res - (1.0 / 80.0)).abs() < 1e-7);
    }
}
