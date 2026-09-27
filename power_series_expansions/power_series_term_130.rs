pub fn compute_power_series_term_130(x: f64) -> f64 {
    x.powi(130) / 130.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_130() {
        let res = compute_power_series_term_130(1.0);
        assert!((res - (1.0 / 130.0)).abs() < 1e-7);
    }
}
