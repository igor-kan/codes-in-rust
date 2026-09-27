pub fn compute_power_series_term_135(x: f64) -> f64 {
    x.powi(135) / 135.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_135() {
        let res = compute_power_series_term_135(1.0);
        assert!((res - (1.0 / 135.0)).abs() < 1e-7);
    }
}
