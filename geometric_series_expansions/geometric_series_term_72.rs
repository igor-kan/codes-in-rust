pub fn compute_geometric_series_term_72(x: f64) -> f64 {
    x.powi(72) / 72.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_72() {
        let res = compute_geometric_series_term_72(1.0);
        assert!((res - (1.0 / 72.0)).abs() < 1e-7);
    }
}
