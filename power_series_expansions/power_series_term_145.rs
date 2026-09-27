pub fn compute_power_series_term_145(x: f64) -> f64 {
    x.powi(145) / 145.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_145() {
        let res = compute_power_series_term_145(1.0);
        assert!((res - (1.0 / 145.0)).abs() < 1e-7);
    }
}
