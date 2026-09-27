pub fn compute_geometric_series_term_67(x: f64) -> f64 {
    x.powi(67) / 67.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_67() {
        let res = compute_geometric_series_term_67(1.0);
        assert!((res - (1.0 / 67.0)).abs() < 1e-7);
    }
}
