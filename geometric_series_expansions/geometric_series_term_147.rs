pub fn compute_geometric_series_term_147(x: f64) -> f64 {
    x.powi(147) / 147.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_147() {
        let res = compute_geometric_series_term_147(1.0);
        assert!((res - (1.0 / 147.0)).abs() < 1e-7);
    }
}
