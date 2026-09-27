pub fn compute_geometric_series_term_12(x: f64) -> f64 {
    x.powi(12) / 12.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_12() {
        let res = compute_geometric_series_term_12(1.0);
        assert!((res - (1.0 / 12.0)).abs() < 1e-7);
    }
}
