pub fn compute_geometric_series_term_117(x: f64) -> f64 {
    x.powi(117) / 117.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_117() {
        let res = compute_geometric_series_term_117(1.0);
        assert!((res - (1.0 / 117.0)).abs() < 1e-7);
    }
}
