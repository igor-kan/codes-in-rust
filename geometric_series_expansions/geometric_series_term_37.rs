pub fn compute_geometric_series_term_37(x: f64) -> f64 {
    x.powi(37) / 37.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_37() {
        let res = compute_geometric_series_term_37(1.0);
        assert!((res - (1.0 / 37.0)).abs() < 1e-7);
    }
}
