pub fn compute_power_series_term_110(x: f64) -> f64 {
    x.powi(110) / 110.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_110() {
        let res = compute_power_series_term_110(1.0);
        assert!((res - (1.0 / 110.0)).abs() < 1e-7);
    }
}
