pub fn compute_geometric_series_term_57(x: f64) -> f64 {
    x.powi(57) / 57.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_57() {
        let res = compute_geometric_series_term_57(1.0);
        assert!((res - (1.0 / 57.0)).abs() < 1e-7);
    }
}
