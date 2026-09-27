pub fn compute_geometric_series_term_42(x: f64) -> f64 {
    x.powi(42) / 42.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_42() {
        let res = compute_geometric_series_term_42(1.0);
        assert!((res - (1.0 / 42.0)).abs() < 1e-7);
    }
}
