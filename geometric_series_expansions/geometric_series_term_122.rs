pub fn compute_geometric_series_term_122(x: f64) -> f64 {
    x.powi(122) / 122.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_122() {
        let res = compute_geometric_series_term_122(1.0);
        assert!((res - (1.0 / 122.0)).abs() < 1e-7);
    }
}
