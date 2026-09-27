pub fn compute_power_series_term_140(x: f64) -> f64 {
    x.powi(140) / 140.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_140() {
        let res = compute_power_series_term_140(1.0);
        assert!((res - (1.0 / 140.0)).abs() < 1e-7);
    }
}
