pub fn compute_geometric_series_term_22(x: f64) -> f64 {
    x.powi(22) / 22.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_22() {
        let res = compute_geometric_series_term_22(1.0);
        assert!((res - (1.0 / 22.0)).abs() < 1e-7);
    }
}
