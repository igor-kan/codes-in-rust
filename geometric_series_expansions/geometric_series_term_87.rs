pub fn compute_geometric_series_term_87(x: f64) -> f64 {
    x.powi(87) / 87.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_87() {
        let res = compute_geometric_series_term_87(1.0);
        assert!((res - (1.0 / 87.0)).abs() < 1e-7);
    }
}
