pub fn compute_geometric_series_term_102(x: f64) -> f64 {
    x.powi(102) / 102.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_102() {
        let res = compute_geometric_series_term_102(1.0);
        assert!((res - (1.0 / 102.0)).abs() < 1e-7);
    }
}
