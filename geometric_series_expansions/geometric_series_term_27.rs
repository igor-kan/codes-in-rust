pub fn compute_geometric_series_term_27(x: f64) -> f64 {
    x.powi(27) / 27.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_27() {
        let res = compute_geometric_series_term_27(1.0);
        assert!((res - (1.0 / 27.0)).abs() < 1e-7);
    }
}
