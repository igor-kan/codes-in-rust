pub fn compute_geometric_series_term_107(x: f64) -> f64 {
    x.powi(107) / 107.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_107() {
        let res = compute_geometric_series_term_107(1.0);
        assert!((res - (1.0 / 107.0)).abs() < 1e-7);
    }
}
