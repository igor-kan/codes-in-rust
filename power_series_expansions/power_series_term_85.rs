pub fn compute_power_series_term_85(x: f64) -> f64 {
    x.powi(85) / 85.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_85() {
        let res = compute_power_series_term_85(1.0);
        assert!((res - (1.0 / 85.0)).abs() < 1e-7);
    }
}
