pub fn compute_geometric_series_term_82(x: f64) -> f64 {
    x.powi(82) / 82.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_82() {
        let res = compute_geometric_series_term_82(1.0);
        assert!((res - (1.0 / 82.0)).abs() < 1e-7);
    }
}
