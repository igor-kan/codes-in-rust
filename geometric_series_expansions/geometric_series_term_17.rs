pub fn compute_geometric_series_term_17(x: f64) -> f64 {
    x.powi(17) / 17.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_17() {
        let res = compute_geometric_series_term_17(1.0);
        assert!((res - (1.0 / 17.0)).abs() < 1e-7);
    }
}
