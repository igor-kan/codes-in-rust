pub fn compute_power_series_term_75(x: f64) -> f64 {
    x.powi(75) / 75.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_75() {
        let res = compute_power_series_term_75(1.0);
        assert!((res - (1.0 / 75.0)).abs() < 1e-7);
    }
}
