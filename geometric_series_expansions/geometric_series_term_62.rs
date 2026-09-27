pub fn compute_geometric_series_term_62(x: f64) -> f64 {
    x.powi(62) / 62.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_62() {
        let res = compute_geometric_series_term_62(1.0);
        assert!((res - (1.0 / 62.0)).abs() < 1e-7);
    }
}
