pub fn compute_power_series_term_150(x: f64) -> f64 {
    x.powi(150) / 150.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_150() {
        let res = compute_power_series_term_150(1.0);
        assert!((res - (1.0 / 150.0)).abs() < 1e-7);
    }
}
