pub fn compute_geometric_series_term_7(x: f64) -> f64 {
    x.powi(7) / 7.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_7() {
        let res = compute_geometric_series_term_7(1.0);
        assert!((res - (1.0 / 7.0)).abs() < 1e-7);
    }
}
